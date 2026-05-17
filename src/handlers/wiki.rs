// Author:      machinageist
// Date:        2026-05-15
// Description: Handlers for the wiki section.
//              `/wiki` renders the overview page (content/pages/index.md).
//              `/wiki/:slug` renders one tool page (content/pages/<slug>.md).
//              Both responses include a left navigation sidebar with the
//              active entry highlighted.

use crate::errors::SiteError;
use crate::models::page::Page;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Path as AxumPath;
use std::path::PathBuf;

const PAGES_DIR: &str = "content/pages";
const OVERVIEW_SLUG: &str = "index";

// One entry in the left wiki sidebar
pub struct SidebarEntry {
    pub slug: &'static str,
    pub label: &'static str,
}

// One section in the left wiki sidebar
pub struct SidebarSection {
    pub heading: &'static str,
    pub entries: &'static [SidebarEntry],
}

// Static wiki sidebar layout. New tools land here when they ship.
const SIDEBAR: &[SidebarSection] = &[
    SidebarSection {
        heading: "Overview",
        entries: &[SidebarEntry {
            slug: OVERVIEW_SLUG,
            label: "Tool Suite Overview",
        }],
    },
    SidebarSection {
        heading: "Engagement & Harness",
        entries: &[
            SidebarEntry { slug: "mg-engagement", label: "mg-engagement" },
            SidebarEntry { slug: "mg-harness", label: "mg-harness" },
            SidebarEntry { slug: "mg-tui", label: "mg-tui" },
        ],
    },
    SidebarSection {
        heading: "Recon",
        entries: &[
            SidebarEntry { slug: "subdomain-enum", label: "subdomain-enum" },
            SidebarEntry { slug: "mg-scan", label: "mg-scan" },
            SidebarEntry { slug: "mg-fingerprint", label: "mg-fingerprint" },
            SidebarEntry { slug: "mg-recon", label: "mg-recon" },
            SidebarEntry { slug: "corpus-builder", label: "corpus-builder" },
        ],
    },
    SidebarSection {
        heading: "Web & Active",
        entries: &[
            SidebarEntry { slug: "mg-crawl", label: "mg-crawl" },
            SidebarEntry { slug: "mg-probe", label: "mg-probe" },
            SidebarEntry { slug: "mg-fuzz", label: "mg-fuzz" },
            SidebarEntry { slug: "mg-replay", label: "mg-replay" },
            SidebarEntry { slug: "ai-prioritize", label: "ai-prioritize" },
        ],
    },
    SidebarSection {
        heading: "Reporting",
        entries: &[
            SidebarEntry { slug: "mg-report", label: "mg-report" },
        ],
    },
    SidebarSection {
        heading: "Analysis & Exploit Dev",
        entries: &[
            SidebarEntry { slug: "mg-recopilot", label: "mg-recopilot" },
            SidebarEntry { slug: "mg-aifuzz", label: "mg-aifuzz" },
            SidebarEntry { slug: "mg-exploitgen", label: "mg-exploitgen" },
        ],
    },
    SidebarSection {
        heading: "Libraries",
        entries: &[
            SidebarEntry { slug: "libraries", label: "Shared libraries" },
        ],
    },
];

#[derive(Template)]
#[template(path = "wiki_page.html")]
pub struct WikiPageTemplate {
    pub page: Page,
    pub sidebar: &'static [SidebarSection],
    pub active_slug: &'static str,
}

impl WikiPageTemplate {
    pub fn title(&self) -> &str {
        &self.page.title
    }

    pub fn description(&self) -> &str {
        &self.page.summary
    }

    pub fn section(&self) -> &str {
        "wiki"
    }
}

// Render the wiki overview page
pub async fn index() -> Result<impl IntoResponse, SiteError> {
    render_for_slug(OVERVIEW_SLUG).await
}

// Render one tool wiki page selected by URL slug
pub async fn page(AxumPath(slug): AxumPath<String>) -> Result<impl IntoResponse, SiteError> {
    let allowed = lookup_sidebar_slug(&slug)
        .ok_or_else(|| SiteError::PageNotFound(slug.clone()))?;
    render_for_slug(allowed).await
}

// Load a page from disk and wrap it with the sidebar context
async fn render_for_slug(slug: &'static str) -> Result<WikiPageTemplate, SiteError> {
    let pages_dir = PathBuf::from(PAGES_DIR);
    let page = Page::find(&pages_dir, slug)?;
    Ok(WikiPageTemplate {
        page,
        sidebar: SIDEBAR,
        active_slug: slug,
    })
}

// Look up a slug in the sidebar; returns the static slug reference if known
fn lookup_sidebar_slug(slug: &str) -> Option<&'static str> {
    for section in SIDEBAR {
        for entry in section.entries {
            if entry.slug == slug {
                return Some(entry.slug);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;

    #[test]
    fn rendering_overview_template_includes_sidebar_and_content() {
        let page = Page::find(&PathBuf::from(PAGES_DIR), OVERVIEW_SLUG)
            .expect("overview page must exist");
        let html = WikiPageTemplate {
            page,
            sidebar: SIDEBAR,
            active_slug: OVERVIEW_SLUG,
        }
        .render()
        .expect("template renders");
        assert!(html.contains("wiki-layout"), "missing wiki layout shell");
        assert!(html.contains("wiki-sidebar"), "missing sidebar block");
        assert!(html.contains("/wiki/mg-recopilot"), "sidebar should link to mg-recopilot page");
        assert!(html.contains("class=\"active\""), "active class should appear on the overview entry");
        assert!(html.contains("Tool Suite"), "overview content should render");
    }

    #[test]
    fn rendering_tool_page_marks_correct_active_entry() {
        let slug = "mg-aifuzz";
        let page = Page::find(&PathBuf::from(PAGES_DIR), slug)
            .expect("mg-aifuzz page must exist");
        let html = WikiPageTemplate {
            page,
            sidebar: SIDEBAR,
            active_slug: slug,
        }
        .render()
        .expect("template renders");
        // The active entry's <li> carries class="active" and contains the slug
        let needle = "<li class=\"active\">\n            \n            <a href=\"/wiki/mg-aifuzz\">mg-aifuzz</a>";
        assert!(
            html.contains(needle),
            "expected mg-aifuzz to be the active sidebar entry"
        );
    }

    #[test]
    fn unknown_slug_returns_none() {
        assert!(lookup_sidebar_slug("does-not-exist").is_none());
    }
}
