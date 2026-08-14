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

pub(crate) const PAGES_DIR: &str = "content/pages";
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
                slug: "wireless-media",
                label: "Wireless media",
            },
            SidebarEntry {
                slug: "wired-media",
                label: "Wired media",
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
                slug: "subnetting",
                label: "Subnetting, CIDR, and VLSM",
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
                slug: "software-defined-networking",
                label: "Software-defined networking",
            },
            SidebarEntry {
                slug: "zero-trust-architecture",
                label: "Zero-trust architecture",
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
                slug: "linux-streams",
                label: "Streams, redirection, and pipes",
            },
            SidebarEntry {
                slug: "linux-permissions",
                label: "File permissions and links",
            },
            SidebarEntry {
                slug: "linux-archives",
                label: "Archives and compression",
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

// List every slug the sidebar offers — the allowlist of servable /learn pages
pub(crate) fn sidebar_slugs() -> Vec<&'static str> {
    SIDEBAR
        .iter()
        .flat_map(|section| section.entries.iter().map(|entry| entry.slug))
        .collect()
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

    // End-to-end guard for the heading-anchor pass in models::markdown: a real
    // published page, rendered through the real template, must come out with
    // addressable section ids. Unit tests cover the slug rules; this covers the
    // wiring actually reaching the page a reader sees.
    #[test]
    fn published_pages_render_addressable_section_anchors() {
        let slug = "osi-model";
        let page = Page::find(&PathBuf::from(PAGES_DIR), slug).expect("OSI page must exist");
        let html = WikiPageTemplate {
            page,
            sidebar: SIDEBAR,
            active_slug: slug,
        }
        .render()
        .expect("template renders");

        assert!(
            html.contains(r#"<h2 id="overview""#),
            "article headings should carry generated ids"
        );
        assert!(
            html.contains(r##"class="heading-anchor" href="#overview""##),
            "each heading should trail a permalink to its own id"
        );
        assert!(
            html.contains(r#"id="encapsulation-and-decapsulation""#),
            "multi-word headings should slug predictably, so cross-page links stay stable"
        );
    }

    #[test]
    fn unknown_slug_returns_none() {
        assert!(lookup_sidebar_slug("does-not-exist").is_none());
    }

    // B5 gap G4. SIDEBAR and tests/wiki_pages.rs::WIKI_SLUGS are deliberately
    // separate so the test crate stays decoupled from the bin, but until now each
    // was only checked against disk — so the two copies could disagree with each
    // other silently as long as both happened to name real files. This is the
    // guard that makes the duplication honest (criteria 5A/5B).
    #[test]
    fn sidebar_and_the_test_crate_agree_on_the_page_list() {
        let source = std::fs::read_to_string("tests/wiki_pages.rs")
            .expect("the integration test file must exist");
        let list = source
            .split_once("const WIKI_SLUGS: &[&str] = &[")
            .and_then(|(_, rest)| rest.split_once("];"))
            .map(|(body, _)| body)
            .expect("WIKI_SLUGS must still be declared as a slice literal");

        let mut declared: Vec<String> = list
            .split(',')
            .map(|entry| entry.trim().trim_matches('"').to_string())
            .filter(|entry| !entry.is_empty())
            .collect();
        let mut offered: Vec<String> = sidebar_slugs().iter().map(|s| s.to_string()).collect();
        declared.sort();
        offered.sort();

        assert_eq!(
            offered, declared,
            "SIDEBAR and WIKI_SLUGS have drifted apart — every published page must appear \
             in both"
        );
    }

    // Cross-page links now address sections, so a renamed heading silently breaks
    // a link that still resolves to a real page. Checked here rather than in the
    // test crate because only this side can call the real renderer, and a second
    // copy of the slug rules would be the drift it is meant to prevent.
    #[test]
    fn every_section_anchor_in_the_corpus_resolves() {
        let pages_dir = PathBuf::from(PAGES_DIR);
        let mut ids: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for slug in sidebar_slugs() {
            let html = Page::find(&pages_dir, slug)
                .unwrap_or_else(|err| panic!("load {slug}: {err:?}"))
                .content_html;
            let found = html
                .match_indices(" id=\"")
                .map(|(index, marker)| {
                    html[index + marker.len()..]
                        .split('"')
                        .next()
                        .unwrap_or_default()
                        .to_string()
                })
                .collect();
            ids.insert(slug.to_string(), found);
        }

        let mut checked = 0;
        for slug in sidebar_slugs() {
            let raw = std::fs::read_to_string(pages_dir.join(format!("{slug}.md")))
                .unwrap_or_else(|err| panic!("read {slug}: {err}"));
            for (index, _) in raw.match_indices("/learn/") {
                let tail: String = raw[index + "/learn/".len()..]
                    .chars()
                    .take_while(|c| {
                        c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-' || *c == '#'
                    })
                    .collect();
                let Some((target, fragment)) = tail.split_once('#') else {
                    continue;
                };
                let known = ids
                    .get(target)
                    .unwrap_or_else(|| panic!("{slug}.md links to unknown page {target}"));
                assert!(
                    known.iter().any(|id| id == fragment),
                    "{slug}.md links to /learn/{target}#{fragment}, but that page has no such \
                     heading id. Available: {known:?}"
                );
                checked += 1;
            }
        }
        assert!(
            checked > 0,
            "anchor checker matched nothing — it has broken"
        );
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
