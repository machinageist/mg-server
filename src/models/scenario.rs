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

use crate::errors::SiteError;
use crate::models::markdown;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use std::fs;
use std::path::Path;

pub const PBQ_DIR: &str = "content/study/pbq";

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
    #[serde(skip)]
    pub slug: String,
}

impl Scenario {
    pub fn situation_html(&self) -> String {
        markdown::to_inline_html(&self.situation)
    }
}

// Load one scenario by slug
pub fn load(dir: &Path, slug: &str) -> Result<Scenario, SiteError> {
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
}

impl GradedStep {
    pub fn correct(&self) -> bool {
        self.step.accepts(&self.typed)
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

    fn step(accept: &[&str]) -> Step {
        Step {
            prompt: "Do the thing".to_string(),
            given: None,
            accept: accept.iter().map(|s| s.to_string()).collect(),
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
