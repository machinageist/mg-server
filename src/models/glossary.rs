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
}

// One command — what it does, and when you would reach for it
#[derive(Debug, Clone, Deserialize)]
pub struct GlossaryCommand {
    pub name: String,
    pub synopsis: String,
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
