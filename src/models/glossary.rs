// Author:      machinageist
// Date:        2026-08-14
// Description: Defines the glossary data model — terms (C2a) and commands
//              (C2b) — loaded from frontmatter-only Markdown under
//              content/glossary/. Both carry cross-links back into the /learn
//              corpus so a definition is a doorway rather than a dead end.
// Notes:       The files are data wearing a Markdown coat: YAML frontmatter,
//              empty body. That buys the existing gray_matter + serde idiom
//              from page.rs with no new dependency, at the cost of two
//              ceremonial `---` fences.
//              Anchors come from models::markdown::slugify, the same function
//              that ids headings, so a `see_also` or a `learn` anchor cannot
//              resolve in one place and not the other.

use crate::errors::SiteError;
use crate::models::markdown;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use std::fs;
use std::path::Path;

pub const GLOSSARY_DIR: &str = "content/glossary";

// The shell prompt shown before a command, matching the convention the wiki's
// fenced blocks use. An unprivileged prompt — nothing in the reference needs
// root, and a `#` would be a claim about the command rather than decoration.
const PROMPT: &str = "$";

// -----------------------------------------------------------------------
// Data types
// -----------------------------------------------------------------------

// The domain an entry belongs to; drives the text tag and the ?cat= filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Networking,
    Linux,
}

impl Category {
    // Name the domain in a word — never a colour alone
    pub fn label(&self) -> &'static str {
        match self {
            Category::Networking => "Networking",
            Category::Linux => "Linux",
        }
    }

    // The ?cat= value that selects this category
    pub fn slug(&self) -> &'static str {
        match self {
            Category::Networking => "networking",
            Category::Linux => "linux",
        }
    }
}

// A cross-link into the teaching corpus: /learn/<slug>#<anchor>
#[derive(Debug, Clone, Deserialize)]
pub struct LearnRef {
    pub slug: String,
    #[serde(default)]
    pub anchor: Option<String>,
    pub label: String,
}

impl LearnRef {
    // Build the href, with the anchor only when one was given
    pub fn href(&self) -> String {
        match &self.anchor {
            Some(anchor) => format!("/learn/{}#{}", self.slug, anchor),
            None => format!("/learn/{}", self.slug),
        }
    }
}

// One term definition — prose first, in ordinary language
#[derive(Debug, Clone, Deserialize)]
pub struct GlossaryTerm {
    pub term: String,
    // Synonyms and abbreviations, so a reader searching "MMU" lands somewhere
    #[serde(default)]
    pub aka: Vec<String>,
    pub category: Category,
    pub definition: String,
    #[serde(default)]
    pub see_also: Vec<String>,
    #[serde(default)]
    pub learn: Vec<LearnRef>,
}

impl GlossaryTerm {
    // The in-page anchor this term is addressable at
    pub fn anchor(&self) -> String {
        markdown::slugify(&self.term)
    }

    // Definitions are authored with inline code spans for commands, flags, and
    // addresses. Rendered, or the backticks reach the reader as backticks.
    pub fn definition_html(&self) -> String {
        markdown::to_inline_html(&self.definition)
    }
}

// One command — what it does, and when you would reach for it
#[derive(Debug, Clone, Deserialize)]
pub struct GlossaryCommand {
    pub name: String,
    // One runnable command per entry. A list rather than a joined string
    // because a code block reads as "paste this" — two commands sharing a line
    // with a separator between them is not something anyone can paste.
    pub synopsis: Vec<String>,
    pub category: Category,
    pub purpose: String,
    pub context: String,
    #[serde(default)]
    pub example: Option<String>,
    // Destructive or root-requiring notes. Rendered with a text label, never
    // signalled by colour alone.
    #[serde(default)]
    pub caution: Option<String>,
    #[serde(default)]
    pub see_also: Vec<String>,
    #[serde(default)]
    pub learn: Vec<LearnRef>,
    #[serde(default)]
    pub man: Option<String>,
}

impl GlossaryCommand {
    pub fn anchor(&self) -> String {
        markdown::slugify(&self.name)
    }

    pub fn purpose_html(&self) -> String {
        markdown::to_inline_html(&self.purpose)
    }

    pub fn context_html(&self) -> String {
        markdown::to_inline_html(&self.context)
    }

    // The synopsis as rendered HTML, produced by the same Markdown renderer the
    // wiki uses.
    //
    // This goes through markdown::to_html rather than being hand-built as a
    // <pre> in the template, because that is the only way the two surfaces
    // cannot drift. A template that assembles its own code block is a second
    // implementation of something the site already does, and it diverged four
    // times before this: bare commands, then a middot separator inside a block
    // that reads as "paste this", then no prompt, then its own padding and
    // font size. One pipeline, one result.
    pub fn synopsis_html(&self) -> String {
        markdown::to_html(&fenced(&self.synopsis))
    }

    // The worked example, through the same pipeline
    pub fn example_html(&self) -> String {
        match &self.example {
            Some(command) => markdown::to_html(&fenced(std::slice::from_ref(command))),
            None => String::new(),
        }
    }

    pub fn caution_html(&self) -> String {
        self.caution
            .as_deref()
            .map(markdown::to_inline_html)
            .unwrap_or_default()
    }
}

// Build a fenced Markdown code block from a list of commands
//
// The prompt is added here rather than stored: a command is a command, and the
// scenario matcher already strips a leading prompt precisely because people
// copy them along with the command.
fn fenced(commands: &[String]) -> String {
    let body = commands
        .iter()
        .map(|command| format!("{PROMPT} {command}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("```text\n{body}\n```")
}

// -----------------------------------------------------------------------
// Loading — frontmatter-only Markdown, same parse Page::from_file uses
// -----------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct TermFile {
    entries: Vec<GlossaryTerm>,
}

#[derive(Debug, Deserialize)]
struct CommandFile {
    entries: Vec<GlossaryCommand>,
}

// Read every term, sorted for stable rendering
pub fn load_terms(dir: &Path) -> Result<Vec<GlossaryTerm>, SiteError> {
    let raw = fs::read_to_string(dir.join("terms.md"))?;
    let parsed = Matter::<YAML>::new().parse(&raw);
    let file: TermFile = parsed
        .data
        .ok_or_else(|| SiteError::MissingFrontmatter("glossary terms".to_string()))?
        .deserialize()
        .map_err(|err| SiteError::FrontmatterParse(err.to_string()))?;

    let mut entries = file.entries;
    entries.sort_by_key(|entry| entry.term.to_lowercase());
    Ok(entries)
}

// Read every command, sorted for stable rendering
pub fn load_commands(dir: &Path) -> Result<Vec<GlossaryCommand>, SiteError> {
    let raw = fs::read_to_string(dir.join("commands.md"))?;
    let parsed = Matter::<YAML>::new().parse(&raw);
    let file: CommandFile = parsed
        .data
        .ok_or_else(|| SiteError::MissingFrontmatter("glossary commands".to_string()))?
        .deserialize()
        .map_err(|err| SiteError::FrontmatterParse(err.to_string()))?;

    let mut entries = file.entries;
    entries.sort_by_key(|entry| entry.name.to_lowercase());
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dir() -> PathBuf {
        PathBuf::from(GLOSSARY_DIR)
    }

    #[test]
    fn the_glossary_files_parse_and_are_not_empty() {
        let terms = load_terms(&dir()).expect("terms.md must parse");
        let commands = load_commands(&dir()).expect("commands.md must parse");
        assert!(!terms.is_empty(), "the glossary should define terms");
        assert!(!commands.is_empty(), "the glossary should define commands");
    }

    #[test]
    fn entries_are_sorted_so_rendering_is_stable() {
        let terms = load_terms(&dir()).expect("terms parse");
        let mut sorted = terms.clone();
        sorted.sort_by_key(|entry| entry.term.to_lowercase());
        let names: Vec<&str> = terms.iter().map(|e| e.term.as_str()).collect();
        let expected: Vec<&str> = sorted.iter().map(|e| e.term.as_str()).collect();
        assert_eq!(names, expected);
    }

    // A code block is an instruction to paste. Every line in one has to be a
    // command on its own, which is why the synopsis is a list and why a
    // display separator must never creep back into it.
    #[test]
    fn every_synopsis_line_is_a_single_runnable_command() {
        for entry in load_commands(&dir()).expect("commands parse") {
            assert!(!entry.synopsis.is_empty(), "{}: no synopsis", entry.name);
            for line in &entry.synopsis {
                assert!(
                    !line.contains('·') && !line.contains(" | ") || line.contains('|'),
                    "{}: synopsis line {line:?} joins commands with a separator",
                    entry.name
                );
                assert!(
                    !line.trim().is_empty(),
                    "{}: an empty synopsis line",
                    entry.name
                );
                assert!(
                    line.trim() == line,
                    "{}: synopsis line {line:?} has stray whitespace",
                    entry.name
                );
            }
        }
    }

    // Commands are shown the way the wiki shows them — prompted — so a reader
    // moving between the two surfaces sees one convention. The prompt is added
    // at render time, so it must never also live in the data.
    #[test]
    fn command_blocks_carry_a_prompt_that_is_not_stored_in_the_data() {
        for entry in load_commands(&dir()).expect("commands parse") {
            let rendered = entry.synopsis_html();
            for command in &entry.synopsis {
                assert!(
                    rendered.contains(&format!("$ {command}")),
                    "{}: {command:?} is not prompted in the rendered block",
                    entry.name
                );
            }
            for stored in &entry.synopsis {
                assert!(
                    !stored.starts_with('$') && !stored.starts_with('#'),
                    "{}: {stored:?} stores the prompt — it belongs to rendering",
                    entry.name
                );
            }
            if let Some(example) = &entry.example {
                assert!(
                    entry.example_html().contains(&format!("$ {example}")),
                    "{}: the example is not prompted",
                    entry.name
                );
            }
        }
    }

    // The reason this surface kept diverging from the wiki was that the
    // template assembled its own <pre>. This pins the glossary's command block
    // to what the wiki's Markdown pipeline produces for the same content, so a
    // hand-built block cannot creep back in without failing here.
    #[test]
    fn a_command_block_is_byte_identical_to_the_wiki_pipeline() {
        let entry = load_commands(&dir())
            .expect("commands parse")
            .into_iter()
            .next()
            .expect("at least one command");

        let through_the_wiki_renderer = markdown::to_html(&format!(
            "```text\n{}\n```",
            entry
                .synopsis
                .iter()
                .map(|c| format!("$ {c}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));

        assert_eq!(
            entry.synopsis_html(),
            through_the_wiki_renderer,
            "the glossary must render commands through the same pipeline as /learn"
        );
    }

    #[test]
    fn anchors_are_unique_within_each_index() {
        for anchors in [
            load_terms(&dir())
                .expect("terms parse")
                .iter()
                .map(|e| e.anchor())
                .collect::<Vec<_>>(),
            load_commands(&dir())
                .expect("commands parse")
                .iter()
                .map(|e| e.anchor())
                .collect::<Vec<_>>(),
        ] {
            let mut seen = anchors.clone();
            seen.sort();
            seen.dedup();
            assert_eq!(
                seen.len(),
                anchors.len(),
                "two entries share an anchor, so one is unreachable"
            );
        }
    }

    // The glossary's value is that a definition is a doorway. A learn ref
    // pointing at a page that does not exist, or at an anchor the renderer
    // never generates, is a doorway into a wall — and it fails silently,
    // because the link still looks fine.
    #[test]
    fn every_learn_reference_resolves_to_a_real_page_and_anchor() {
        let pages = std::path::PathBuf::from(crate::handlers::wiki::PAGES_DIR);
        let mut checked = 0;

        let refs = load_terms(&dir())
            .expect("terms parse")
            .into_iter()
            .flat_map(|entry| entry.learn)
            .chain(
                load_commands(&dir())
                    .expect("commands parse")
                    .into_iter()
                    .flat_map(|entry| entry.learn),
            );

        for reference in refs {
            let page = crate::models::page::Page::find(&pages, &reference.slug)
                .unwrap_or_else(|err| panic!("learn ref {:?}: {err:?}", reference.slug));
            if let Some(anchor) = &reference.anchor {
                let ids: Vec<&str> = page
                    .outline
                    .iter()
                    .map(|heading| heading.id.as_str())
                    .collect();
                assert!(
                    ids.contains(&anchor.as_str()),
                    "learn ref /learn/{}#{} has no such heading. Available: {ids:?}",
                    reference.slug,
                    anchor
                );
            }
            checked += 1;
        }
        assert!(checked > 0, "the guard matched nothing — it has broken");
    }

    // Every cross-link is a promise the reader can follow. A see_also pointing
    // at an anchor that does not exist is a broken link inside the page that
    // exists to connect things.
    #[test]
    fn every_see_also_resolves_within_its_own_index() {
        let terms = load_terms(&dir()).expect("terms parse");
        let term_anchors: Vec<String> = terms.iter().map(|e| e.anchor()).collect();
        for entry in &terms {
            for reference in &entry.see_also {
                assert!(
                    term_anchors.contains(reference),
                    "term {:?} points at see_also {:?}, which is not a term",
                    entry.term,
                    reference
                );
            }
        }

        let commands = load_commands(&dir()).expect("commands parse");
        let command_anchors: Vec<String> = commands.iter().map(|e| e.anchor()).collect();
        for entry in &commands {
            for reference in &entry.see_also {
                assert!(
                    command_anchors.contains(reference),
                    "command {:?} points at see_also {:?}, which is not a command",
                    entry.name,
                    reference
                );
            }
        }
    }
}
