// Author:      machinageist
// Date:        2026-08-14
// Description: Performance-based scenarios — multi-step problems where the
//              answer is a command you type rather than an option you pick.
//              Loaded from content/study/pbq/<slug>.md.
// Notes:       This is the honest version of "performance-based" without a
//              terminal: it tests whether you can produce the right command
//              for a stated situation, which is the skill, without pretending
//              to be a shell.
//
//              Matching is normalised rather than exact — whitespace collapses
//              and short flags may be given in any order, because `chmod -Rv`
//              and `chmod -vR` are the same command and marking one wrong
//              would be teaching a superstition.
//
//              A scenario with `dialect: ios` also accepts Cisco IOS
//              abbreviations. IOS takes any prefix of a keyword that no other
//              keyword valid at that point shares, so `sh ip int br` is
//              `show ip interface brief`. The site does not know the IOS
//              grammar, so each scenario lists the keywords that compete at
//              each point, and a token with no list is matched exactly. A
//              list that is too short accepts an abbreviation a real device
//              would reject, so the lists are checked against a device.

use crate::errors::SiteError;
use crate::models::markdown;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use std::collections::{BTreeMap, VecDeque};
use std::fs;
use std::path::Path;

pub const PBQ_DIR: &str = "content/study/pbq";

// Keyword group for the first token of a command when a step names no mode
const DEFAULT_MODE: &str = "exec";

// Joins the tokens typed so far into the name of the next keyword group
const GROUP_SEPARATOR: &str = "-";

// IOS interface types. A name such as GigabitEthernet0/1 is a type and a
// number, and the type abbreviates like any keyword: g0/1, gi0/1, gig 0/1
const INTERFACE_TYPES: &[&str] = &[
    "ethernet",
    "fastethernet",
    "gigabitethernet",
    "tengigabitethernet",
    "serial",
    "loopback",
    "vlan",
    "port-channel",
    "tunnel",
];

// Which command language a scenario's steps are written in
#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
    #[default]
    Shell,
    Ios,
}

// What a scenario grades its steps against: the dialect, and for IOS the
// keywords that compete at each point in a command, by group name
#[derive(Debug, Clone, Default)]
pub struct Grammar {
    pub dialect: Dialect,
    pub keywords: BTreeMap<String, Vec<String>>,
}

// One step of a scenario: a situation, and the command that resolves it
#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    pub prompt: String,
    // Optional fixed-width context — an `ls -l` listing, a config excerpt
    #[serde(default)]
    pub given: Option<String>,
    // Accepted answers. More than one because there is usually more than one
    // correct command, and insisting on a favourite is not assessment.
    pub accept: Vec<String>,
    // IOS only. The mode the command is typed in, which names the keyword
    // group its first token is matched against. `sh` is `show` at the exec
    // prompt and `shutdown` on an interface
    #[serde(default)]
    pub mode: Option<String>,
    pub explanation: String,
    pub learn_slug: String,
    pub learn_anchor: String,
}

impl Step {
    pub fn prompt_html(&self) -> String {
        markdown::to_inline_html(&self.prompt)
    }

    pub fn explanation_html(&self) -> String {
        markdown::to_inline_html(&self.explanation)
    }

    pub fn learn_href(&self) -> String {
        format!("/learn/{}#{}", self.learn_slug, self.learn_anchor)
    }

    // The answer shown when someone gets it wrong — the first accepted form
    pub fn canonical(&self) -> &str {
        self.accept.first().map(String::as_str).unwrap_or_default()
    }

    // Whether a typed answer is one of the accepted commands
    pub fn accepts(&self, typed: &str) -> bool {
        let given = normalize(typed);
        if given.is_empty() {
            return false;
        }
        self.accept
            .iter()
            .any(|candidate| normalize(candidate) == given)
    }

    // Whether a typed answer is accepted under the scenario's grammar. The
    // accepted list is always checked first, so a shell scenario grades
    // exactly as `accepts` does
    pub fn accepts_in(&self, typed: &str, grammar: &Grammar) -> bool {
        if self.accepts(typed) {
            return true;
        }
        if grammar.dialect != Dialect::Ios || typed.trim().is_empty() {
            return false;
        }
        let mode = self.mode.as_deref().unwrap_or(DEFAULT_MODE);
        self.accept
            .iter()
            .any(|candidate| ios_matches(typed, candidate, mode, &grammar.keywords))
    }
}

// One token of an accepted IOS command
enum Wanted {
    // An interface name written whole, split into its type and its number
    Interface { kind: String, number: String },
    // A keyword or an argument. Which one depends on the keyword lists
    Token(String),
}

// Split an interface name such as GigabitEthernet0/1 into type and number
fn split_interface(token: &str) -> Option<(String, String)> {
    let digit = token.find(|c: char| c.is_ascii_digit())?;
    let (kind, number) = token.split_at(digit);
    let kind = kind.to_ascii_lowercase();
    (INTERFACE_TYPES.contains(&kind.as_str()) && is_interface_number(number))
        .then(|| (kind, number.to_string()))
}

// Report whether a token is interface numbering such as 0/1 or 0/0/0.100
fn is_interface_number(token: &str) -> bool {
    token.starts_with(|c: char| c.is_ascii_digit())
        && token
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '/' | '.' | ':'))
}

// Decide whether a typed token stands for a keyword, the way IOS does: the
// whole word, or a prefix of it that no other keyword in the group shares
fn abbreviates<S: AsRef<str>>(typed: &str, keyword: &str, group: &[S]) -> bool {
    let typed = typed.to_ascii_lowercase();
    let keyword = keyword.to_ascii_lowercase();
    if typed.is_empty() || !keyword.starts_with(&typed) {
        return false;
    }
    typed == keyword
        || group
            .iter()
            .filter(|other| other.as_ref().to_ascii_lowercase().starts_with(&typed))
            .count()
            == 1
}

// Decide whether a typed IOS command is an accepted one, allowing abbreviation
//
// Tokens are compared in order. A token listed in the keyword group for its
// position may be abbreviated. Any other token is an argument and must match
// exactly, because a hostname or a description is case-sensitive. The group
// for a position is named by the mode and the accepted tokens before it.
fn ios_matches(
    typed: &str,
    accepted: &str,
    mode: &str,
    keywords: &BTreeMap<String, Vec<String>>,
) -> bool {
    let wanted = accepted.split_whitespace().map(|token| {
        split_interface(token)
            .map(|(kind, number)| Wanted::Interface { kind, number })
            .unwrap_or_else(|| Wanted::Token(token.to_string()))
    });
    let mut given: VecDeque<String> = typed
        .trim()
        .trim_start_matches(['$', '#'])
        .split_whitespace()
        .map(str::to_string)
        .collect();
    // Accepted tokens matched so far, which name the next keyword group
    let mut path: Vec<String> = Vec::new();

    for want in wanted {
        let Some(token) = given.pop_front() else {
            return false;
        };
        // The first token's group is the mode. Later groups are named by the
        // accepted tokens before them: show, then show-ip
        let group_name = if path.is_empty() {
            mode.to_string()
        } else {
            path.join(GROUP_SEPARATOR)
        };
        let matched = match &want {
            Wanted::Interface { kind, number } => {
                // The type and number may be typed together or apart
                let digit = token
                    .find(|c: char| c.is_ascii_digit())
                    .unwrap_or(token.len());
                let (typed_kind, glued_number) = token.split_at(digit);
                let typed_number = if glued_number.is_empty() {
                    given.pop_front().unwrap_or_default()
                } else {
                    glued_number.to_string()
                };
                abbreviates(typed_kind, kind, INTERFACE_TYPES) && typed_number == *number
            }
            Wanted::Token(word) => match keywords.get(&group_name) {
                Some(group) if group.iter().any(|k| k.eq_ignore_ascii_case(word)) => {
                    abbreviates(&token, word, group)
                }
                _ => token == *word,
            },
        };
        if !matched {
            return false;
        }
        path.push(match want {
            Wanted::Interface { kind, .. } => kind,
            Wanted::Token(word) => word.to_ascii_lowercase(),
        });
    }
    given.is_empty()
}

// Reduce a command to a comparable form
//
// Collapses whitespace, drops a leading prompt character someone copied along
// with the command, and sorts the letters inside a combined short flag so
// `-Rv` and `-vR` compare equal. Long flags and arguments keep their order,
// because there it can matter.
fn normalize(command: &str) -> String {
    command
        .trim()
        .trim_start_matches(['$', '#'])
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .map(|token| {
            if token.starts_with('-') && !token.starts_with("--") && token.len() > 2 {
                let mut letters: Vec<char> = token[1..].chars().collect();
                letters.sort_unstable();
                format!("-{}", letters.into_iter().collect::<String>())
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// A whole scenario — a situation worked through in order
#[derive(Debug, Clone, Deserialize)]
pub struct Scenario {
    pub title: String,
    pub situation: String,
    pub steps: Vec<Step>,
    // Absent on a shell scenario, which is the default
    #[serde(default)]
    pub dialect: Dialect,
    // IOS only. Keyword group name to the keywords that compete there
    #[serde(default)]
    pub keywords: BTreeMap<String, Vec<String>>,
    #[serde(skip)]
    pub slug: String,
}

impl Scenario {
    pub fn situation_html(&self) -> String {
        markdown::to_inline_html(&self.situation)
    }

    // What this scenario's steps are graded against
    pub fn grammar(&self) -> Grammar {
        Grammar {
            dialect: self.dialect,
            keywords: self.keywords.clone(),
        }
    }
}

// Load one scenario by slug
pub fn load(dir: &Path, slug: &str) -> Result<Scenario, SiteError> {
    if !crate::models::slug::is_safe(slug) {
        return Err(SiteError::PageNotFound(slug.to_string()));
    }
    let path = dir.join(format!("{slug}.md"));
    if !path.exists() {
        return Err(SiteError::PageNotFound(slug.to_string()));
    }
    let raw = fs::read_to_string(&path)?;
    let parsed = Matter::<YAML>::new().parse(&raw);
    let mut scenario: Scenario = parsed
        .data
        .ok_or_else(|| SiteError::MissingFrontmatter(slug.to_string()))?
        .deserialize()
        .map_err(|err| SiteError::FrontmatterParse(err.to_string()))?;
    scenario.slug = slug.to_string();
    Ok(scenario)
}

// Every scenario, sorted for a stable index
pub fn all(dir: &Path) -> Vec<Scenario> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut scenarios: Vec<Scenario> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                return None;
            }
            load(dir, path.file_stem()?.to_str()?).ok()
        })
        .collect();
    scenarios.sort_by(|a, b| a.title.cmp(&b.title));
    scenarios
}

// One graded step
pub struct GradedStep {
    pub step: Step,
    pub typed: String,
    pub grammar: Grammar,
}

impl GradedStep {
    pub fn correct(&self) -> bool {
        self.step.accepts_in(&self.typed, &self.grammar)
    }

    pub fn unanswered(&self) -> bool {
        self.typed.trim().is_empty()
    }

    pub fn verdict(&self) -> &'static str {
        if self.unanswered() {
            "skipped"
        } else if self.correct() {
            "correct"
        } else {
            "incorrect"
        }
    }

    pub fn verdict_class(&self) -> &'static str {
        if self.unanswered() {
            "is-skipped"
        } else if self.correct() {
            "is-correct"
        } else {
            "is-wrong"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dir() -> PathBuf {
        PathBuf::from(PBQ_DIR)
    }

    #[test]
    fn traversal_slug_is_rejected_before_any_file_is_read() {
        let error = load(&dir(), "../../posts/hosting-machinageist-dev")
            .expect_err("a path-like slug must not reach an out-of-directory Markdown file");

        assert!(matches!(error, SiteError::PageNotFound(_)));
    }

    fn step(accept: &[&str]) -> Step {
        Step {
            prompt: "Do the thing".to_string(),
            given: None,
            accept: accept.iter().map(|s| s.to_string()).collect(),
            mode: None,
            explanation: "x".repeat(50),
            learn_slug: "linux-permissions".to_string(),
            learn_anchor: "numeric-notation".to_string(),
        }
    }

    #[test]
    fn matching_ignores_spacing_and_a_copied_prompt_character() {
        let s = step(&["chmod 644 file"]);
        assert!(s.accepts("chmod 644 file"));
        assert!(s.accepts("  chmod   644   file  "));
        assert!(s.accepts("$ chmod 644 file"));
        assert!(!s.accepts("chmod 640 file"));
    }

    // -Rv and -vR are the same command; marking one wrong teaches a
    // superstition rather than the skill
    #[test]
    fn combined_short_flags_compare_regardless_of_order() {
        let s = step(&["chmod -Rv 755 dir"]);
        assert!(s.accepts("chmod -vR 755 dir"));
        assert!(s.accepts("chmod -Rv 755 dir"));
    }

    #[test]
    fn long_flags_and_arguments_keep_their_meaning() {
        let s = step(&["ls --color=auto /etc"]);
        assert!(s.accepts("ls --color=auto /etc"));
        assert!(
            !s.accepts("ls /etc --color=auto"),
            "argument order is not something to silently forgive"
        );
    }

    #[test]
    fn an_empty_answer_is_skipped_not_wrong() {
        let graded = GradedStep {
            step: step(&["ls"]),
            typed: "   ".to_string(),
            grammar: Grammar::default(),
        };
        assert!(graded.unanswered());
        assert!(!graded.correct());
        assert_eq!(graded.verdict(), "skipped");
    }

    #[test]
    fn several_accepted_answers_all_pass() {
        let s = step(&["chmod 644 file", "chmod u=rw,go=r file"]);
        assert!(s.accepts("chmod 644 file"));
        assert!(s.accepts("chmod u=rw,go=r file"));
    }

    // -------------------------------------------------------------------
    // IOS abbreviation
    // -------------------------------------------------------------------

    // Unpublished scenarios kept for format testing. Never routed or listed
    const DRAFT_PBQ_DIR: &str = "content/drafts/study/pbq";

    // A small keyword set with real collisions in it
    fn ios() -> Grammar {
        let groups: &[(&str, &[&str])] = &[
            ("exec", &["show", "configure", "copy", "ping", "ssh"]),
            ("show", &["ip", "interfaces", "vlan", "version"]),
            ("show-ip", &["interface", "route", "inspect"]),
            ("show-ip-interface", &["brief"]),
            ("config", &["interface", "hostname", "ip"]),
            ("config-if", &["shutdown", "switchport", "speed"]),
        ];
        Grammar {
            dialect: Dialect::Ios,
            keywords: groups
                .iter()
                .map(|(name, words)| {
                    (
                        name.to_string(),
                        words.iter().map(|w| w.to_string()).collect(),
                    )
                })
                .collect(),
        }
    }

    fn ios_step(mode: Option<&str>, accept: &[&str]) -> Step {
        Step {
            mode: mode.map(str::to_string),
            ..step(accept)
        }
    }

    #[test]
    fn ios_keywords_may_be_abbreviated_to_an_unambiguous_prefix() {
        let s = ios_step(None, &["show ip interface brief"]);
        let g = ios();
        assert!(s.accepts_in("sh ip int br", &g));
        assert!(s.accepts_in("show ip interface brief", &g));
        assert!(s.accepts_in("SHOW IP INTERFACE BRIEF", &g));
        assert!(s.accepts_in("  sho  ip  interf  b ", &g));
    }

    #[test]
    fn an_ambiguous_ios_prefix_is_rejected() {
        let s = ios_step(None, &["show ip interface brief"]);
        let g = ios();
        // s could be show or ssh
        assert!(!s.accepts_in("s ip int br", &g));
        // in could be interface or inspect
        assert!(!s.accepts_in("sh ip in br", &g));
        // i could be ip or interfaces
        assert!(!s.accepts_in("sh i int br", &g));
    }

    #[test]
    fn a_whole_keyword_is_accepted_even_when_it_prefixes_another() {
        let mut g = ios();
        g.keywords
            .insert("show".to_string(), vec!["ip".into(), "ipv6".into()]);
        let s = ios_step(None, &["show ip route"]);
        assert!(s.accepts_in("show ip route", &g));
        assert!(!s.accepts_in("show i route", &g));
    }

    #[test]
    fn ios_token_counts_must_agree() {
        let s = ios_step(None, &["show ip interface brief"]);
        let g = ios();
        assert!(!s.accepts_in("sh ip int", &g));
        assert!(!s.accepts_in("sh ip int br extra", &g));
        assert!(!s.accepts_in("sh ip route", &g));
    }

    #[test]
    fn ios_arguments_and_unlisted_tokens_match_exactly() {
        let g = ios();
        let s = ios_step(Some("config"), &["hostname R1"]);
        assert!(s.accepts_in("host R1", &g));
        assert!(!s.accepts_in("host r1", &g), "a hostname is case-sensitive");
        assert!(!s.accepts_in("host R2", &g));

        // version is listed under show. detail is in no group, so it cannot
        // be abbreviated
        let s = ios_step(None, &["show version detail"]);
        assert!(s.accepts_in("sh ver detail", &g));
        assert!(!s.accepts_in("sh ver det", &g));
    }

    #[test]
    fn ios_interface_names_abbreviate_with_or_without_a_space() {
        let g = ios();
        let s = ios_step(Some("config"), &["interface GigabitEthernet0/1"]);
        for typed in [
            "interface GigabitEthernet0/1",
            "int g0/1",
            "int gi0/1",
            "interface gig 0/1",
            "int GigabitEthernet 0/1",
        ] {
            assert!(s.accepts_in(typed, &g), "{typed} should match");
        }
        for typed in ["int g0/2", "int f0/1", "int g", "int 0/1", "int g0/1 extra"] {
            assert!(!s.accepts_in(typed, &g), "{typed} should not match");
        }

        // t could be TenGigabitEthernet or Tunnel
        let s = ios_step(Some("config"), &["interface Tunnel0"]);
        assert!(s.accepts_in("int tu0", &g));
        assert!(!s.accepts_in("int t0", &g));
    }

    #[test]
    fn the_mode_decides_what_a_first_token_abbreviates() {
        let g = ios();
        // sh is show at the exec prompt and shutdown on an interface
        assert!(ios_step(None, &["show version"]).accepts_in("sh ver", &g));
        let shutdown = ios_step(Some("config-if"), &["shutdown"]);
        assert!(shutdown.accepts_in("sh", &g));
        assert!(shutdown.accepts_in("shut", &g));
        assert!(
            !shutdown.accepts_in("s", &g),
            "s could be speed or switchport"
        );
    }

    #[test]
    fn a_shell_scenario_never_gets_ios_abbreviation() {
        let s = step(&["show ip interface brief"]);
        let mut shell = ios();
        shell.dialect = Dialect::Shell;
        assert!(!s.accepts_in("sh ip int br", &shell));
        assert!(s.accepts_in("show ip interface brief", &shell));
        assert!(!s.accepts_in("", &ios()));
    }

    #[test]
    fn a_blank_ios_answer_is_skipped_not_wrong() {
        let graded = GradedStep {
            step: ios_step(None, &["show version"]),
            typed: " ".to_string(),
            grammar: ios(),
        };
        assert_eq!(graded.verdict(), "skipped");
    }

    // The fixture shows the IOS file format and is loaded by nothing else.
    // Every accepted command must match itself and its documented short form
    #[test]
    fn the_draft_ios_fixture_parses_and_grades() {
        let scenario = load(Path::new(DRAFT_PBQ_DIR), "ios-fixture").expect("fixture loads");
        assert_eq!(scenario.dialect, Dialect::Ios);
        let grammar = scenario.grammar();
        for step in &scenario.steps {
            for accepted in &step.accept {
                assert!(
                    step.accepts_in(accepted, &grammar),
                    "{accepted} does not match itself"
                );
            }
        }
        assert!(scenario.steps[0].accepts_in("sh ip int br", &grammar));
        assert!(scenario.steps[1].accepts_in("conf t", &grammar));
        assert!(scenario.steps[2].accepts_in("int g0/1", &grammar));
        assert!(scenario.steps[3].accepts_in("no shut", &grammar));
    }

    // Only an IOS scenario may carry IOS fields. On a shell scenario they
    // would be silently ignored, which hides a mistake
    #[test]
    fn only_ios_scenarios_carry_keywords_and_modes() {
        for scenario in all(&dir()) {
            if scenario.dialect == Dialect::Shell {
                assert!(
                    scenario.keywords.is_empty(),
                    "{}: keywords on a shell scenario",
                    scenario.slug
                );
                assert!(
                    scenario.steps.iter().all(|step| step.mode.is_none()),
                    "{}: mode on a shell scenario",
                    scenario.slug
                );
            }
        }
    }

    // Same provenance rule the question bank has: a scenario that cannot point
    // at material on this site does not ship
    #[test]
    fn every_step_cites_a_real_page_and_heading() {
        let pages = PathBuf::from(crate::handlers::wiki::PAGES_DIR);
        for scenario in all(&dir()) {
            for step in &scenario.steps {
                let page = crate::models::page::Page::find(&pages, &step.learn_slug)
                    .unwrap_or_else(|err| {
                        panic!(
                            "{}: cites /learn/{}: {err:?}",
                            scenario.slug, step.learn_slug
                        )
                    });
                // A draft page is a 404 in production, so the link would be dead
                assert!(
                    !page.draft,
                    "{}: cites /learn/{}, which is still a draft",
                    scenario.slug, step.learn_slug
                );
                let ids: Vec<&str> = page.outline.iter().map(|h| h.id.as_str()).collect();
                assert!(
                    ids.contains(&step.learn_anchor.as_str()),
                    "{}: cites /learn/{}#{}, which has no such heading. Available: {ids:?}",
                    scenario.slug,
                    step.learn_slug,
                    step.learn_anchor
                );
            }
        }
    }

    #[test]
    fn every_scenario_is_well_formed() {
        for scenario in all(&dir()) {
            assert!(!scenario.steps.is_empty(), "{}: no steps", scenario.slug);
            for step in &scenario.steps {
                assert!(
                    !step.accept.is_empty(),
                    "{}: a step with no accepted answer",
                    scenario.slug
                );
                assert!(
                    step.explanation.len() > 40,
                    "{}: a step with no real explanation",
                    scenario.slug
                );
                // The stated answer must pass its own matcher, or the scenario
                // marks its own solution wrong
                assert!(
                    step.accepts(step.canonical()),
                    "{}: canonical answer {:?} does not satisfy its own matcher",
                    scenario.slug,
                    step.canonical()
                );
            }
        }
    }
}
