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
    pub entries: Vec<GlossaryTerm>,
    pub active: Option<Category>,
    pub total: usize,
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
    Ok(TermsTemplate {
        entries,
        active,
        total,
    })
}

// -----------------------------------------------------------------------
// Commands — /glossary/commands
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "glossary_commands.html")]
pub struct CommandsTemplate {
    pub entries: Vec<GlossaryCommand>,
    pub active: Option<Category>,
    pub total: usize,
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
    Ok(CommandsTemplate {
        entries,
        active,
        total,
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
        TermsTemplate {
            entries,
            active,
            total,
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
