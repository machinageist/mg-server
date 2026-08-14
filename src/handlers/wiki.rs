// Author:      machinageist
// Date:        2026-05-15
// Description: Handlers for the wiki section.
//              `/learn` renders the overview page (content/pages/index.md).
//              `/learn/:slug` renders one tool page (content/pages/<slug>.md).
//              Both responses include a left navigation sidebar with the
//              active entry highlighted.
//              `/wiki` and `/wiki/:slug` are the pre-rename URLs; they permanently
//              redirect to the `/learn` equivalents so old links keep working.

use crate::errors::SiteError;
use crate::models::page::Page;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Path as AxumPath;
use axum::response::Redirect;
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

// Static education-wiki sidebar layout. New reviewed topics land here when published.
const SIDEBAR: &[SidebarSection] = &[
    SidebarSection {
        heading: "Overview",
        entries: &[SidebarEntry {
            slug: OVERVIEW_SLUG,
            label: "Education Wiki",
        }],
    },
    SidebarSection {
        heading: "Networking Foundations",
        entries: &[
            SidebarEntry {
                slug: "osi-model",
                label: "OSI model",
            },
            SidebarEntry {
                slug: "network-topologies",
                label: "Network topologies",
            },
            SidebarEntry {
                slug: "transmission-media",
                label: "Transmission media",
            },
            SidebarEntry {
                slug: "transceivers",
                label: "Transceivers and connectors",
            },
            SidebarEntry {
                slug: "network-appliances",
                label: "Network appliances",
            },
            SidebarEntry {
                slug: "network-applications",
                label: "Network applications",
            },
            SidebarEntry {
                slug: "network-functions",
                label: "Network functions",
            },
            SidebarEntry {
                slug: "network-protocols",
                label: "Network protocols and ports",
            },
            SidebarEntry {
                slug: "traffic-types",
                label: "Network traffic types",
            },
            SidebarEntry {
                slug: "ipv4-addressing",
                label: "IPv4 addressing",
            },
            SidebarEntry {
                slug: "ipv6-addressing",
                label: "IPv6 addressing",
            },
            SidebarEntry {
                slug: "cloud-computing",
                label: "Cloud computing concepts",
            },
            SidebarEntry {
                slug: "modern-network-environments",
                label: "Modern network environments",
            },
        ],
    },
    SidebarSection {
        heading: "Linux Foundations",
        entries: &[
            SidebarEntry {
                slug: "linux-abstraction-layers",
                label: "Linux abstraction layers",
            },
            SidebarEntry {
                slug: "linux-filesystem-hierarchy",
                label: "Filesystem hierarchy",
            },
            SidebarEntry {
                slug: "linux-shell",
                label: "The shell and the command line",
            },
            SidebarEntry {
                slug: "linux-permissions",
                label: "File permissions and links",
            },
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

// Render one education-wiki page selected by URL slug
pub async fn page(AxumPath(slug): AxumPath<String>) -> Result<impl IntoResponse, SiteError> {
    let allowed =
        lookup_sidebar_slug(&slug).ok_or_else(|| SiteError::PageNotFound(slug.clone()))?;
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

// Permanently redirect the legacy /wiki root to /learn
pub async fn redirect_index() -> Redirect {
    Redirect::permanent("/learn")
}

// Permanently redirect a legacy /wiki/:slug URL to its /learn/:slug equivalent
pub async fn redirect_page(AxumPath(slug): AxumPath<String>) -> Redirect {
    Redirect::permanent(&format!("/learn/{slug}"))
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
        let page =
            Page::find(&PathBuf::from(PAGES_DIR), OVERVIEW_SLUG).expect("overview page must exist");
        let html = WikiPageTemplate {
            page,
            sidebar: SIDEBAR,
            active_slug: OVERVIEW_SLUG,
        }
        .render()
        .expect("template renders");
        assert!(html.contains("wiki-layout"), "missing wiki layout shell");
        assert!(html.contains("wiki-sidebar"), "missing sidebar block");
        assert!(
            html.contains("/learn/network-appliances"),
            "sidebar should link to the networking education pages"
        );
        assert!(
            html.contains("class=\"active\""),
            "active class should appear on the overview entry"
        );
        assert!(
            html.contains("Education wiki"),
            "overview should frame the wiki as a public education resource"
        );
        assert!(
            html.contains("Understand → Practice → Evidence"),
            "overview should explain the education cluster model"
        );
    }

    #[test]
    fn rendering_education_page_marks_correct_active_entry() {
        let slug = "osi-model";
        let page = Page::find(&PathBuf::from(PAGES_DIR), slug).expect("OSI page must exist");
        let html = WikiPageTemplate {
            page,
            sidebar: SIDEBAR,
            active_slug: slug,
        }
        .render()
        .expect("template renders");
        // Exactly one entry is active, and its <li> wraps the OSI-model link.
        assert_eq!(
            html.matches("class=\"active\"").count(),
            1,
            "exactly one sidebar entry should be active"
        );
        let active_li = html
            .split("<li class=\"active\">")
            .nth(1)
            .and_then(|rest| rest.split("</li>").next())
            .expect("an active sidebar entry should exist");
        assert!(
            active_li.contains("/learn/osi-model"),
            "expected OSI model to be the active sidebar entry"
        );
    }

    #[test]
    fn unknown_slug_returns_none() {
        assert!(lookup_sidebar_slug("does-not-exist").is_none());
    }

    #[tokio::test]
    async fn legacy_wiki_root_redirects_to_learn() {
        let response = redirect_index().await.into_response();
        assert_eq!(
            response.status(),
            axum::http::StatusCode::PERMANENT_REDIRECT
        );
        assert_eq!(response.headers().get("location").unwrap(), "/learn");
    }

    #[tokio::test]
    async fn legacy_wiki_slug_redirects_to_matching_learn_slug() {
        let response = redirect_page(AxumPath("osi-model".to_string()))
            .await
            .into_response();
        assert_eq!(
            response.status(),
            axum::http::StatusCode::PERMANENT_REDIRECT
        );
        assert_eq!(
            response.headers().get("location").unwrap(),
            "/learn/osi-model"
        );
    }
}
