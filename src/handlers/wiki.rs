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
            SidebarEntry {
                slug: "mg-engagement",
                label: "mg-engagement",
            },
            SidebarEntry {
                slug: "mg-harness",
                label: "mg-harness",
            },
            SidebarEntry {
                slug: "mg-tui",
                label: "mg-tui",
            },
        ],
    },
    SidebarSection {
        heading: "Recon",
        entries: &[
            SidebarEntry {
                slug: "subdomain-enum",
                label: "subdomain-enum",
            },
            SidebarEntry {
                slug: "mg-scan",
                label: "mg-scan",
            },
            SidebarEntry {
                slug: "mg-fingerprint",
                label: "mg-fingerprint",
            },
            SidebarEntry {
                slug: "mg-recon",
                label: "mg-recon",
            },
            SidebarEntry {
                slug: "corpus-builder",
                label: "corpus-builder",
            },
            SidebarEntry {
                slug: "mg-whois",
                label: "mg-whois",
            },
            SidebarEntry {
                slug: "mg-shodan",
                label: "mg-shodan",
            },
            SidebarEntry {
                slug: "mg-dns-enum",
                label: "mg-dns-enum",
            },
            SidebarEntry {
                slug: "mg-dns-history",
                label: "mg-dns-history",
            },
            SidebarEntry {
                slug: "mg-cloud-enum",
                label: "mg-cloud-enum",
            },
            SidebarEntry {
                slug: "mg-cname-chain",
                label: "mg-cname-chain",
            },
            SidebarEntry {
                slug: "mg-udp-scan",
                label: "mg-udp-scan",
            },
        ],
    },
    SidebarSection {
        heading: "OSINT",
        entries: &[
            SidebarEntry {
                slug: "mg-github",
                label: "mg-github",
            },
            SidebarEntry {
                slug: "mg-breach",
                label: "mg-breach",
            },
            SidebarEntry {
                slug: "mg-google-dork",
                label: "mg-google-dork",
            },
            SidebarEntry {
                slug: "mg-leak-monitor",
                label: "mg-leak-monitor",
            },
        ],
    },
    SidebarSection {
        heading: "Web & Active",
        entries: &[
            SidebarEntry {
                slug: "mg-crawl",
                label: "mg-crawl",
            },
            SidebarEntry {
                slug: "mg-probe",
                label: "mg-probe",
            },
            SidebarEntry {
                slug: "mg-fuzz",
                label: "mg-fuzz",
            },
            SidebarEntry {
                slug: "mg-replay",
                label: "mg-replay",
            },
            SidebarEntry {
                slug: "ai-prioritize",
                label: "ai-prioritize",
            },
        ],
    },
    SidebarSection {
        heading: "Web Vulnerability Scanning",
        entries: &[SidebarEntry {
            slug: "mg-webscan",
            label: "mg-webscan",
        }],
    },
    SidebarSection {
        heading: "Auth & Session",
        entries: &[
            SidebarEntry {
                slug: "mg-jwt",
                label: "mg-jwt",
            },
            SidebarEntry {
                slug: "mg-authz",
                label: "mg-authz",
            },
            SidebarEntry {
                slug: "mg-oauth",
                label: "mg-oauth",
            },
            SidebarEntry {
                slug: "mg-session-audit",
                label: "mg-session-audit",
            },
            SidebarEntry {
                slug: "mg-brute",
                label: "mg-brute",
            },
        ],
    },
    SidebarSection {
        heading: "OOB & Cloud SSRF",
        entries: &[
            SidebarEntry {
                slug: "mg-oob",
                label: "mg-oob",
            },
            SidebarEntry {
                slug: "mg-aws",
                label: "mg-aws",
            },
            SidebarEntry {
                slug: "mg-gcp",
                label: "mg-gcp",
            },
            SidebarEntry {
                slug: "mg-azure",
                label: "mg-azure",
            },
            SidebarEntry {
                slug: "mg-serverless",
                label: "mg-serverless",
            },
        ],
    },
    SidebarSection {
        heading: "Network Services",
        entries: &[
            SidebarEntry {
                slug: "mg-tls-scan",
                label: "mg-tls-scan",
            },
            SidebarEntry {
                slug: "mg-ssh-audit",
                label: "mg-ssh-audit",
            },
            SidebarEntry {
                slug: "mg-smtp",
                label: "mg-smtp",
            },
            SidebarEntry {
                slug: "mg-snmp",
                label: "mg-snmp",
            },
            SidebarEntry {
                slug: "mg-smb",
                label: "mg-smb",
            },
            SidebarEntry {
                slug: "mg-http2",
                label: "mg-http2",
            },
        ],
    },
    SidebarSection {
        heading: "API Surface",
        entries: &[
            SidebarEntry {
                slug: "mg-graphql",
                label: "mg-graphql",
            },
            SidebarEntry {
                slug: "mg-openapi",
                label: "mg-openapi",
            },
            SidebarEntry {
                slug: "mg-grpc",
                label: "mg-grpc",
            },
            SidebarEntry {
                slug: "mg-websocket",
                label: "mg-websocket",
            },
        ],
    },
    SidebarSection {
        heading: "Cloud & Infra",
        entries: &[
            SidebarEntry {
                slug: "mg-k8s",
                label: "mg-k8s",
            },
            SidebarEntry {
                slug: "mg-docker",
                label: "mg-docker",
            },
            SidebarEntry {
                slug: "mg-vhost",
                label: "mg-vhost",
            },
            SidebarEntry {
                slug: "mg-takeover",
                label: "mg-takeover",
            },
        ],
    },
    SidebarSection {
        heading: "Mobile & Static Analysis",
        entries: &[
            SidebarEntry {
                slug: "mg-artifact-audit",
                label: "mg-artifact-audit",
            },
            SidebarEntry {
                slug: "mg-secret-validate",
                label: "mg-secret-validate",
            },
            SidebarEntry {
                slug: "mg-csp",
                label: "mg-csp",
            },
        ],
    },
    SidebarSection {
        heading: "Retired pages",
        entries: &[
            SidebarEntry {
                slug: "mg-apikey",
                label: "mg-apikey → mg-artifact-audit",
            },
            SidebarEntry {
                slug: "mg-apk",
                label: "mg-apk → mg-artifact-audit",
            },
            SidebarEntry {
                slug: "mg-ipa",
                label: "mg-ipa → mg-artifact-audit",
            },
            SidebarEntry {
                slug: "mg-js-analyze",
                label: "mg-js-analyze → mg-artifact-audit",
            },
            SidebarEntry {
                slug: "mg-metadata",
                label: "mg-metadata → mg-artifact-audit",
            },
            SidebarEntry {
                slug: "mg-sourcemap",
                label: "mg-sourcemap → mg-artifact-audit",
            },
            SidebarEntry {
                slug: "mg-xss",
                label: "mg-xss → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-sqli",
                label: "mg-sqli → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-ssrf",
                label: "mg-ssrf → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-ssti",
                label: "mg-ssti → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-xxe",
                label: "mg-xxe → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-traversal",
                label: "mg-traversal → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-redirect",
                label: "mg-redirect → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-csrf",
                label: "mg-csrf → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-cmdinject",
                label: "mg-cmdinject → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-cors-exploit",
                label: "mg-cors-exploit → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-cache-poison",
                label: "mg-cache-poison → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-proto-pollute",
                label: "mg-proto-pollute → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-deser",
                label: "mg-deser → mg-webscan",
            },
            SidebarEntry {
                slug: "mg-smuggle",
                label: "mg-smuggle → mg-webscan",
            },
        ],
    },
    SidebarSection {
        heading: "Post-Access",
        entries: &[
            SidebarEntry {
                slug: "mg-privesc-linux",
                label: "mg-privesc-linux",
            },
            SidebarEntry {
                slug: "mg-privesc-windows",
                label: "mg-privesc-windows",
            },
            SidebarEntry {
                slug: "mg-loot",
                label: "mg-loot",
            },
        ],
    },
    SidebarSection {
        heading: "Workflow",
        entries: &[
            SidebarEntry {
                slug: "mg-diff",
                label: "mg-diff",
            },
            SidebarEntry {
                slug: "mg-notify",
                label: "mg-notify",
            },
            SidebarEntry {
                slug: "mg-timeline",
                label: "mg-timeline",
            },
            SidebarEntry {
                slug: "mg-dns-rebind",
                label: "mg-dns-rebind",
            },
        ],
    },
    SidebarSection {
        heading: "Reporting",
        entries: &[SidebarEntry {
            slug: "mg-report",
            label: "mg-report",
        }],
    },
    SidebarSection {
        heading: "Analysis & Exploit Dev",
        entries: &[
            SidebarEntry {
                slug: "mg-recopilot",
                label: "mg-recopilot",
            },
            SidebarEntry {
                slug: "mg-aifuzz",
                label: "mg-aifuzz",
            },
            SidebarEntry {
                slug: "mg-exploitgen",
                label: "mg-exploitgen",
            },
        ],
    },
    SidebarSection {
        heading: "Libraries",
        entries: &[SidebarEntry {
            slug: "libraries",
            label: "Shared libraries",
        }],
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
            html.contains("/wiki/mg-recopilot"),
            "sidebar should link to mg-recopilot page"
        );
        assert!(
            html.contains("class=\"active\""),
            "active class should appear on the overview entry"
        );
        assert!(
            html.contains("Tool Suite"),
            "overview content should render"
        );
    }

    #[test]
    fn rendering_tool_page_marks_correct_active_entry() {
        let slug = "mg-aifuzz";
        let page = Page::find(&PathBuf::from(PAGES_DIR), slug).expect("mg-aifuzz page must exist");
        let html = WikiPageTemplate {
            page,
            sidebar: SIDEBAR,
            active_slug: slug,
        }
        .render()
        .expect("template renders");
        // Exactly one entry is active, and its <li> wraps the mg-aifuzz link
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
            active_li.contains("/wiki/mg-aifuzz"),
            "expected mg-aifuzz to be the active sidebar entry"
        );
    }

    #[test]
    fn unknown_slug_returns_none() {
        assert!(lookup_sidebar_slug("does-not-exist").is_none());
    }
}
