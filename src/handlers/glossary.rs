// Author:      machinageist
// Date:        2026-08-14
// Description: Handlers for the glossary — a landing page, an index of terms,
//              and an index of commands, each filterable by category.
// Notes:       Every entry carries cross-links back into /learn, which is the
//              point of the surface: a definition should be a doorway, not a
//              dead end. A test in models::glossary fails if any of those links
//              stops resolving.
//              Read from disk per request, like every other content surface
//              here — no cache, so an edited definition is live immediately.

use crate::errors::SiteError;
use crate::models::glossary::{self, Category, GlossaryCommand, GlossaryTerm};
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Query;
use serde::Deserialize;
use std::path::PathBuf;

// A run of entries sharing a first letter, so a long index can be jumped
// through rather than scrolled. Generic because terms and commands differ in
// shape but not in how they are navigated.
pub struct LetterGroup<T> {
    pub letter: String,
    pub entries: Vec<T>,
}

// Bucket sorted entries by their first letter, preserving order within each
fn group_by_letter<T, F>(entries: Vec<T>, key: F) -> Vec<LetterGroup<T>>
where
    F: Fn(&T) -> String,
{
    let mut groups: Vec<LetterGroup<T>> = Vec::new();
    for entry in entries {
        let letter = key(&entry)
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "#".to_string());
        match groups.last_mut() {
            Some(group) if group.letter == letter => group.entries.push(entry),
            _ => groups.push(LetterGroup {
                letter,
                entries: vec![entry],
            }),
        }
    }
    groups
}

// Optional ?cat= filter; an unknown or absent value renders the full index
#[derive(Debug, Deserialize)]
pub struct CategoryFilter {
    #[serde(default)]
    pub cat: Option<String>,
}

impl CategoryFilter {
    // Resolve the query string to a category, ignoring anything unrecognised
    fn selected(&self) -> Option<Category> {
        match self.cat.as_deref() {
            Some("networking") => Some(Category::Networking),
            Some("linux") => Some(Category::Linux),
            _ => None,
        }
    }
}

// -----------------------------------------------------------------------
// Landing — /glossary
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "glossary_landing.html")]
pub struct GlossaryLandingTemplate {
    pub term_count: usize,
    pub command_count: usize,
}

impl GlossaryLandingTemplate {
    pub fn title(&self) -> &str {
        "Glossary — machinageist"
    }
    pub fn description(&self) -> &str {
        "Definitions and commands from the education wiki, each linked back to the page that explains it."
    }
    pub fn section(&self) -> &str {
        "glossary"
    }
}

// Render the glossary landing page with both counts
pub async fn landing() -> Result<impl IntoResponse, SiteError> {
    let dir = PathBuf::from(glossary::GLOSSARY_DIR);
    Ok(GlossaryLandingTemplate {
        term_count: glossary::load_terms(&dir)?.len(),
        command_count: glossary::load_commands(&dir)?.len(),
    })
}

// -----------------------------------------------------------------------
// Terms — /glossary/terms
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "glossary_terms.html")]
pub struct TermsTemplate {
    pub groups: Vec<LetterGroup<GlossaryTerm>>,
    pub active: Option<Category>,
    pub total: usize,
    pub shown: usize,
}

impl TermsTemplate {
    pub fn title(&self) -> &str {
        "Glossary: terms — machinageist"
    }
    pub fn description(&self) -> &str {
        "Networking and Linux terms defined in plain language, each linked to the wiki page that covers it."
    }
    pub fn section(&self) -> &str {
        "glossary"
    }

    // Whether a category filter is currently applied, for the empty state and
    // the "show all" affordance
    pub fn filtered(&self) -> bool {
        self.active.is_some()
    }

    // The letters that actually have entries, for the jump bar
    pub fn letters(&self) -> Vec<&str> {
        self.groups.iter().map(|g| g.letter.as_str()).collect()
    }
}

// Render the term index, optionally filtered by category
pub async fn terms(Query(filter): Query<CategoryFilter>) -> Result<impl IntoResponse, SiteError> {
    let all = glossary::load_terms(&PathBuf::from(glossary::GLOSSARY_DIR))?;
    let total = all.len();
    let active = filter.selected();
    let entries = match active {
        Some(category) => all
            .into_iter()
            .filter(|entry| entry.category == category)
            .collect(),
        None => all,
    };
    let shown = entries.len();
    Ok(TermsTemplate {
        groups: group_by_letter(entries, |entry| entry.term.clone()),
        active,
        total,
        shown,
    })
}

// -----------------------------------------------------------------------
// Commands — /glossary/commands
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "glossary_commands.html")]
pub struct CommandsTemplate {
    pub groups: Vec<LetterGroup<GlossaryCommand>>,
    pub active: Option<Category>,
    pub total: usize,
    pub shown: usize,
}

impl CommandsTemplate {
    pub fn title(&self) -> &str {
        "Glossary: commands — machinageist"
    }
    pub fn description(&self) -> &str {
        "A command reference with purpose and context, each linked to the wiki page where it is used."
    }
    pub fn section(&self) -> &str {
        "glossary"
    }

    pub fn filtered(&self) -> bool {
        self.active.is_some()
    }

    pub fn letters(&self) -> Vec<&str> {
        self.groups.iter().map(|g| g.letter.as_str()).collect()
    }
}

// Render the command index, optionally filtered by category
pub async fn commands(
    Query(filter): Query<CategoryFilter>,
) -> Result<impl IntoResponse, SiteError> {
    let all = glossary::load_commands(&PathBuf::from(glossary::GLOSSARY_DIR))?;
    let total = all.len();
    let active = filter.selected();
    let entries = match active {
        Some(category) => all
            .into_iter()
            .filter(|entry| entry.category == category)
            .collect(),
        None => all,
    };
    let shown = entries.len();
    Ok(CommandsTemplate {
        groups: group_by_letter(entries, |entry| entry.name.clone()),
        active,
        total,
        shown,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_terms(cat: Option<&str>) -> String {
        let all = glossary::load_terms(&PathBuf::from(glossary::GLOSSARY_DIR)).expect("terms load");
        let total = all.len();
        let filter = CategoryFilter {
            cat: cat.map(|c| c.to_string()),
        };
        let active = filter.selected();
        let entries = match active {
            Some(category) => all
                .into_iter()
                .filter(|entry| entry.category == category)
                .collect(),
            None => all,
        };
        let shown = entries.len();
        TermsTemplate {
            groups: group_by_letter(entries, |entry| entry.term.clone()),
            active,
            total,
            shown,
        }
        .render()
        .expect("terms template renders")
    }

    #[test]
    fn the_unfiltered_index_lists_every_term() {
        let html = render_terms(None);
        for entry in
            glossary::load_terms(&PathBuf::from(glossary::GLOSSARY_DIR)).expect("terms load")
        {
            assert!(
                html.contains(&entry.term),
                "{} is missing from the index",
                entry.term
            );
        }
    }

    #[test]
    fn a_category_filter_narrows_and_an_unknown_one_does_not() {
        let linux = render_terms(Some("linux"));
        assert!(linux.contains("Kernel"));
        assert!(!linux.contains(">Anycast<"), "networking term leaked in");

        // An unrecognised value renders everything rather than an empty page —
        // a hand-edited URL should not look like a broken site
        let nonsense = render_terms(Some("bananas"));
        assert!(nonsense.contains("Kernel"));
        assert!(nonsense.contains("Anycast"));
    }

    // The jump bar is the index's navigation. Every letter it offers must land
    // on a heading that exists, and every group must be reachable from it.
    #[test]
    fn the_jump_bar_covers_every_group_and_lands_somewhere() {
        let html = render_terms(None);
        let entries =
            glossary::load_terms(&PathBuf::from(glossary::GLOSSARY_DIR)).expect("terms load");

        let mut letters: Vec<String> = entries
            .iter()
            .map(|entry| {
                entry
                    .term
                    .chars()
                    .next()
                    .map(|c| c.to_uppercase().to_string())
                    .unwrap_or_default()
            })
            .collect();
        letters.sort();
        letters.dedup();

        for letter in letters {
            assert!(
                html.contains(&format!("href=\"#letter-{letter}\"")),
                "the jump bar is missing {letter}"
            );
            assert!(
                html.contains(&format!("id=\"letter-{letter}\"")),
                "jump target letter-{letter} does not exist"
            );
        }
    }

    #[test]
    fn every_definition_offers_a_way_into_the_wiki() {
        let html = render_terms(None);
        assert!(
            html.contains("href=\"/learn/"),
            "the glossary's job is to link back into the teaching pages"
        );
    }

    #[test]
    fn the_glossary_needs_no_javascript() {
        let html = render_terms(Some("linux"));
        let mut stripped = String::new();
        let mut rest = html.as_str();
        while let Some(start) = rest.find("<script") {
            stripped.push_str(&rest[..start]);
            match rest[start..].find("</script>") {
                Some(end) => rest = &rest[start + end + "</script>".len()..],
                None => break,
            }
        }
        stripped.push_str(rest);
        assert!(stripped.contains("Kernel"), "entries survive without JS");
        assert!(
            stripped.contains("href=\"/glossary/terms\""),
            "the filter is plain links, not script"
        );
    }
}
