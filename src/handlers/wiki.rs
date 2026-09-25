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
use axum::response::{Redirect, Response};
use chrono::{NaiveDate, Utc};
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

// Slugs retired when a page was split or renamed, and the page each now points at.
// Published URLs keep working through a permanent redirect.
const RENAMED_SLUGS: &[(&str, &str)] = &[
    ("network-functions", "vpns-and-ipsec"),
    ("network-applications", "content-delivery-networks"),
];

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
        heading: "Models and patterns",
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
                slug: "traffic-types",
                label: "Network traffic types",
            },
        ],
    },
    SidebarSection {
        heading: "Physical layer",
        entries: &[
            SidebarEntry {
                slug: "transmission-media",
                label: "Transmission media",
            },
            SidebarEntry {
                slug: "wired-media",
                label: "Wired media",
            },
            SidebarEntry {
                slug: "wireless-media",
                label: "Wireless media",
            },
            SidebarEntry {
                slug: "transceivers",
                label: "Transceivers and connectors",
            },
        ],
    },
    SidebarSection {
        heading: "Addressing",
        entries: &[
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
        ],
    },
    SidebarSection {
        heading: "Local networks",
        entries: &[
            SidebarEntry {
                slug: "switching-technologies",
                label: "Switching technologies",
            },
            SidebarEntry {
                slug: "wireless-technologies",
                label: "Wireless technologies",
            },
        ],
    },
    SidebarSection {
        heading: "Between networks",
        entries: &[
            SidebarEntry {
                slug: "routing-technologies",
                label: "Routing technologies and route selection",
            },
            SidebarEntry {
                slug: "vpns-and-ipsec",
                label: "VPNs and IPsec",
            },
            SidebarEntry {
                slug: "quality-of-service",
                label: "Quality of service",
            },
        ],
    },
    SidebarSection {
        heading: "Services and devices",
        entries: &[
            SidebarEntry {
                slug: "network-protocols",
                label: "Network protocols and ports",
            },
            SidebarEntry {
                slug: "network-appliances",
                label: "Network appliances",
            },
            SidebarEntry {
                slug: "content-delivery-networks",
                label: "Content delivery networks",
            },
        ],
    },
    SidebarSection {
        heading: "Modern environments",
        entries: &[
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
        heading: "Linux foundations",
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

// One page in an exam ordering, filed under the objective it maps to
pub struct ExamEntry {
    pub objective: &'static str,
    pub slug: &'static str,
}

// One exam domain, with its pages in objective order
pub struct ExamSection {
    pub heading: &'static str,
    pub entries: &'static [ExamEntry],
}

// Network+ N10-009 ordering. Every networking page maps to it, because the
// notes these pages come from follow its objectives
const NETWORK_PLUS: &[ExamSection] = &[
    ExamSection {
        heading: "1.0 Networking concepts",
        entries: &[
            ExamEntry {
                objective: "1.1",
                slug: "osi-model",
            },
            ExamEntry {
                objective: "1.2",
                slug: "network-appliances",
            },
            ExamEntry {
                objective: "1.2",
                slug: "content-delivery-networks",
            },
            ExamEntry {
                objective: "1.2",
                slug: "vpns-and-ipsec",
            },
            ExamEntry {
                objective: "1.2",
                slug: "quality-of-service",
            },
            ExamEntry {
                objective: "1.3",
                slug: "cloud-computing",
            },
            ExamEntry {
                objective: "1.4",
                slug: "network-protocols",
            },
            ExamEntry {
                objective: "1.4",
                slug: "traffic-types",
            },
            ExamEntry {
                objective: "1.5",
                slug: "transmission-media",
            },
            ExamEntry {
                objective: "1.5",
                slug: "wired-media",
            },
            ExamEntry {
                objective: "1.5",
                slug: "wireless-media",
            },
            ExamEntry {
                objective: "1.5",
                slug: "transceivers",
            },
            ExamEntry {
                objective: "1.6",
                slug: "network-topologies",
            },
            ExamEntry {
                objective: "1.7",
                slug: "ipv4-addressing",
            },
            ExamEntry {
                objective: "1.7",
                slug: "subnetting",
            },
            ExamEntry {
                objective: "1.8",
                slug: "ipv6-addressing",
            },
            ExamEntry {
                objective: "1.8",
                slug: "software-defined-networking",
            },
            ExamEntry {
                objective: "1.8",
                slug: "zero-trust-architecture",
            },
        ],
    },
    ExamSection {
        heading: "2.0 Network implementation",
        entries: &[
            ExamEntry {
                objective: "2.1",
                slug: "routing-technologies",
            },
            ExamEntry {
                objective: "2.2",
                slug: "switching-technologies",
            },
            ExamEntry {
                objective: "2.3",
                slug: "wireless-technologies",
            },
        ],
    },
];

// CCNA 200-301 v1.1 ordering. Pages whose material is not on the blueprint are
// left out: content delivery networks, zero trust, and the Linux section
const CCNA_V1_1: &[ExamSection] = &[
    ExamSection {
        heading: "1.0 Network fundamentals",
        entries: &[
            ExamEntry {
                objective: "1.1",
                slug: "network-appliances",
            },
            ExamEntry {
                objective: "1.2",
                slug: "network-topologies",
            },
            ExamEntry {
                objective: "1.3",
                slug: "transmission-media",
            },
            ExamEntry {
                objective: "1.3",
                slug: "wired-media",
            },
            ExamEntry {
                objective: "1.3",
                slug: "transceivers",
            },
            ExamEntry {
                objective: "1.5",
                slug: "osi-model",
            },
            ExamEntry {
                objective: "1.6",
                slug: "ipv4-addressing",
            },
            ExamEntry {
                objective: "1.6",
                slug: "subnetting",
            },
            ExamEntry {
                objective: "1.8",
                slug: "ipv6-addressing",
            },
            ExamEntry {
                objective: "1.9",
                slug: "traffic-types",
            },
            ExamEntry {
                objective: "1.11",
                slug: "wireless-media",
            },
            ExamEntry {
                objective: "1.11",
                slug: "wireless-technologies",
            },
            ExamEntry {
                objective: "1.12",
                slug: "cloud-computing",
            },
        ],
    },
    ExamSection {
        heading: "2.0 Network access",
        entries: &[ExamEntry {
            objective: "2.1",
            slug: "switching-technologies",
        }],
    },
    ExamSection {
        heading: "3.0 IP connectivity",
        entries: &[ExamEntry {
            objective: "3.1",
            slug: "routing-technologies",
        }],
    },
    ExamSection {
        heading: "4.0 IP services",
        entries: &[
            ExamEntry {
                objective: "4.3",
                slug: "network-protocols",
            },
            ExamEntry {
                objective: "4.7",
                slug: "quality-of-service",
            },
        ],
    },
    ExamSection {
        heading: "5.0 Security fundamentals",
        entries: &[ExamEntry {
            objective: "5.5",
            slug: "vpns-and-ipsec",
        }],
    },
    ExamSection {
        heading: "6.0 Automation and programmability",
        entries: &[ExamEntry {
            objective: "6.2",
            slug: "software-defined-networking",
        }],
    },
];

// CCNA 200-301 v2.0 ordering. v2.0 also drops topologies, the OSI model as a
// TCP-versus-UDP objective, traffic types, and QoS
const CCNA_V2_0: &[ExamSection] = &[
    ExamSection {
        heading: "1.0 Network infrastructure and connectivity",
        entries: &[
            ExamEntry {
                objective: "1.1",
                slug: "transmission-media",
            },
            ExamEntry {
                objective: "1.1",
                slug: "wired-media",
            },
            ExamEntry {
                objective: "1.1",
                slug: "transceivers",
            },
            ExamEntry {
                objective: "1.2",
                slug: "cloud-computing",
            },
            ExamEntry {
                objective: "1.3",
                slug: "ipv4-addressing",
            },
            ExamEntry {
                objective: "1.3",
                slug: "subnetting",
            },
            ExamEntry {
                objective: "1.4",
                slug: "ipv6-addressing",
            },
            ExamEntry {
                objective: "1.5",
                slug: "wireless-media",
            },
            ExamEntry {
                objective: "1.5",
                slug: "wireless-technologies",
            },
        ],
    },
    ExamSection {
        heading: "2.0 Switching and network access",
        entries: &[
            ExamEntry {
                objective: "2.1",
                slug: "switching-technologies",
            },
            ExamEntry {
                objective: "2.2",
                slug: "network-appliances",
            },
        ],
    },
    ExamSection {
        heading: "3.0 IP routing",
        entries: &[ExamEntry {
            objective: "3.1",
            slug: "routing-technologies",
        }],
    },
    ExamSection {
        heading: "4.0 Network services and security",
        entries: &[
            ExamEntry {
                objective: "4.4",
                slug: "network-protocols",
            },
            ExamEntry {
                objective: "4.5",
                slug: "vpns-and-ipsec",
            },
        ],
    },
    ExamSection {
        heading: "5.0 AI, network operations, and management",
        entries: &[ExamEntry {
            objective: "5.3",
            slug: "software-defined-networking",
        }],
    },
];

// First day CCNA 200-301 v2.0 is delivered. v1.1's last day is the day before,
// and the CCNA ordering switches blueprints on this date without a redeploy
const CCNA_V2_FIRST_DAY: (i32, u32, u32) = (2027, 2, 3);

// One link in a rendered sidebar ordering. Exam orderings carry the objective
pub struct NavEntry {
    pub slug: &'static str,
    pub label: &'static str,
    pub objective: Option<&'static str>,
}

// One heading's worth of links in a rendered sidebar ordering
pub struct NavSection {
    pub heading: &'static str,
    pub entries: Vec<NavEntry>,
}

// One of the three sidebar orderings the toggle switches between
pub struct NavView {
    pub key: &'static str,
    pub name: &'static str,
    pub notes: Vec<String>,
    pub sections: Vec<NavSection>,
}

#[derive(Template)]
#[template(path = "wiki_page.html")]
pub struct WikiPageTemplate {
    pub page: Page,
    pub views: Vec<NavView>,
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

// Render one education-wiki page selected by URL slug, or redirect a retired one
pub async fn page(AxumPath(slug): AxumPath<String>) -> Result<Response, SiteError> {
    if let Some(allowed) = lookup_sidebar_slug(&slug) {
        return Ok(render_for_slug(allowed).await?.into_response());
    }
    if let Some(target) = renamed_slug(&slug) {
        return Ok(Redirect::permanent(&format!("/learn/{target}")).into_response());
    }
    Err(SiteError::PageNotFound(slug))
}

// Load a page from disk and wrap it with the sidebar context
async fn render_for_slug(slug: &'static str) -> Result<WikiPageTemplate, SiteError> {
    let pages_dir = PathBuf::from(PAGES_DIR);
    let page = Page::find(&pages_dir, slug)?;
    Ok(WikiPageTemplate {
        page,
        views: nav_views(slug, Utc::now().date_naive()),
        active_slug: slug,
    })
}

// Pick the CCNA blueprint that is live on a given day, with its version name
fn ccna_blueprint(today: NaiveDate) -> (&'static str, &'static [ExamSection]) {
    let (year, month, day) = CCNA_V2_FIRST_DAY;
    let first_day = NaiveDate::from_ymd_opt(year, month, day).expect("cutover date is valid");
    if today >= first_day {
        ("v2.0", CCNA_V2_0)
    } else {
        ("v1.1", CCNA_V1_1)
    }
}

// Build the topic, CCNA, and Network+ orderings for the page being shown
fn nav_views(active: &str, today: NaiveDate) -> Vec<NavView> {
    let topic = NavView {
        key: "topic",
        name: "Topic",
        notes: Vec::new(),
        sections: SIDEBAR
            .iter()
            .map(|section| NavSection {
                heading: section.heading,
                entries: section
                    .entries
                    .iter()
                    .map(|entry| NavEntry {
                        slug: entry.slug,
                        label: entry.label,
                        objective: None,
                    })
                    .collect(),
            })
            .collect(),
    };
    let (version, ccna) = ccna_blueprint(today);
    vec![
        topic,
        exam_view(
            "ccna",
            "CCNA",
            &format!("CCNA 200-301 {version}"),
            ccna,
            active,
        ),
        exam_view(
            "netplus",
            "Network+",
            "Network+ N10-009",
            NETWORK_PLUS,
            active,
        ),
    ]
}

// Build one exam ordering. It lists only pages on that exam, and says so when
// the page being shown is not one of them
fn exam_view(
    key: &'static str,
    name: &'static str,
    exam: &str,
    sections: &'static [ExamSection],
    active: &str,
) -> NavView {
    let mut notes = vec![format!("Pages on {exam}, by objective.")];
    let listed = active == OVERVIEW_SLUG
        || sections
            .iter()
            .any(|section| section.entries.iter().any(|entry| entry.slug == active));
    if !listed {
        notes.push("This page is not on this exam.".to_string());
    }

    let overview = NavSection {
        heading: SIDEBAR[0].heading,
        entries: vec![NavEntry {
            slug: OVERVIEW_SLUG,
            label: SIDEBAR[0].entries[0].label,
            objective: None,
        }],
    };
    let domains = sections.iter().map(|section| NavSection {
        heading: section.heading,
        entries: section
            .entries
            .iter()
            .map(|entry| NavEntry {
                slug: entry.slug,
                label: sidebar_label(entry.slug).unwrap_or(entry.slug),
                objective: Some(entry.objective),
            })
            .collect(),
    });

    NavView {
        key,
        name,
        notes,
        sections: std::iter::once(overview).chain(domains).collect(),
    }
}

// Look up a page's sidebar label, so exam orderings never restate one
fn sidebar_label(slug: &str) -> Option<&'static str> {
    SIDEBAR
        .iter()
        .flat_map(|section| section.entries.iter())
        .find(|entry| entry.slug == slug)
        .map(|entry| entry.label)
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

// Look up where a retired slug now lives
fn renamed_slug(slug: &str) -> Option<&'static str> {
    RENAMED_SLUGS
        .iter()
        .find(|(old, _)| *old == slug)
        .map(|(_, new)| *new)
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
            views: nav_views(OVERVIEW_SLUG, before_cutover()),
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
            views: nav_views(slug, before_cutover()),
            active_slug: slug,
        }
        .render()
        .expect("template renders");
        // The page is in all three orderings. Each marks exactly one entry
        // active, and that entry's <li> wraps the OSI-model link.
        for view in ["topic", "ccna", "netplus"] {
            let block = html
                .split(&format!("data-view=\"{view}\""))
                .nth(1)
                .and_then(|rest| rest.split("data-view=").next())
                .unwrap_or_else(|| panic!("the {view} ordering should render"));
            assert_eq!(
                block.matches("class=\"active\"").count(),
                1,
                "exactly one entry should be active in the {view} ordering"
            );
            let active_li = block
                .split("<li class=\"active\">")
                .nth(1)
                .and_then(|rest| rest.split("</li>").next())
                .expect("an active sidebar entry should exist");
            assert!(
                active_li.contains("/learn/osi-model"),
                "expected OSI model to be the active entry in the {view} ordering"
            );
        }
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
            views: nav_views(slug, before_cutover()),
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

    // A retired slug must redirect somewhere real, and must not also be served
    #[test]
    fn renamed_slugs_point_at_published_pages() {
        for (old, new) in RENAMED_SLUGS {
            assert!(
                lookup_sidebar_slug(old).is_none(),
                "{old} is retired but still in the sidebar"
            );
            assert!(
                lookup_sidebar_slug(new).is_some(),
                "{old} redirects to {new}, which is not a published page"
            );
        }
    }

    // A fixed day on the v1.1 side of the CCNA cutover, so tests do not change
    // behavior when the calendar does
    fn before_cutover() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 9, 25).expect("valid date")
    }

    // Split an objective like "1.11" into (1, 11) so it sorts numerically
    fn objective_key(objective: &str) -> (u32, u32) {
        let (domain, item) = objective
            .split_once('.')
            .unwrap_or_else(|| panic!("objective {objective} is not domain.item"));
        (
            domain.parse().expect("numeric domain"),
            item.parse().expect("numeric item"),
        )
    }

    const EXAM_ORDERINGS: &[(&str, &[ExamSection])] = &[
        ("Network+ N10-009", NETWORK_PLUS),
        ("CCNA v1.1", CCNA_V1_1),
        ("CCNA v2.0", CCNA_V2_0),
    ];

    // An exam ordering can only point at published pages, each once, filed
    // under the right domain and in objective order
    #[test]
    fn exam_orderings_list_published_pages_once_in_objective_order() {
        for (exam, sections) in EXAM_ORDERINGS {
            let mut seen = Vec::new();
            let mut previous = (0, 0);
            for section in *sections {
                let domain: u32 = section
                    .heading
                    .split('.')
                    .next()
                    .and_then(|d| d.parse().ok())
                    .unwrap_or_else(|| panic!("{exam}: heading {} has no domain", section.heading));
                for entry in section.entries {
                    assert!(
                        lookup_sidebar_slug(entry.slug).is_some() && entry.slug != OVERVIEW_SLUG,
                        "{exam}: {} is not a published topic page",
                        entry.slug
                    );
                    assert!(
                        !seen.contains(&entry.slug),
                        "{exam}: {} is listed twice",
                        entry.slug
                    );
                    seen.push(entry.slug);
                    let key = objective_key(entry.objective);
                    assert_eq!(
                        key.0, domain,
                        "{exam}: {} is filed under the wrong domain",
                        entry.slug
                    );
                    assert!(
                        key >= previous,
                        "{exam}: {} is out of objective order",
                        entry.slug
                    );
                    previous = key;
                }
            }
        }
    }

    // Every networking page maps to Network+, because the notes follow its
    // objectives. Pages that cite the Network+ textbook must appear there
    #[test]
    fn network_plus_ordering_covers_every_networking_page() {
        let listed: Vec<&str> = NETWORK_PLUS
            .iter()
            .flat_map(|section| section.entries.iter().map(|entry| entry.slug))
            .collect();
        for slug in sidebar_slugs() {
            if slug == OVERVIEW_SLUG {
                continue;
            }
            let body = std::fs::read_to_string(format!("{PAGES_DIR}/{slug}.md"))
                .expect("every sidebar page exists");
            let networking = body.contains("Ian Neil");
            assert_eq!(
                listed.contains(&slug),
                networking,
                "{slug}: the Network+ ordering should list exactly the networking pages"
            );
        }
    }

    // The exam orderings leave out what the exam does not cover
    #[test]
    fn ccna_orderings_leave_out_material_off_the_blueprint() {
        for sections in [CCNA_V1_1, CCNA_V2_0] {
            for section in sections {
                for entry in section.entries {
                    assert!(
                        !entry.slug.starts_with("linux-")
                            && entry.slug != "content-delivery-networks"
                            && entry.slug != "zero-trust-architecture",
                        "{} is not on the CCNA blueprint",
                        entry.slug
                    );
                }
            }
        }
    }

    // The CCNA ordering follows whichever blueprint is being delivered that day
    #[test]
    fn ccna_ordering_switches_to_v2_on_its_first_day() {
        let last_v1 = NaiveDate::from_ymd_opt(2027, 2, 2).expect("valid date");
        let first_v2 = NaiveDate::from_ymd_opt(2027, 2, 3).expect("valid date");
        assert_eq!(ccna_blueprint(last_v1).0, "v1.1");
        assert_eq!(ccna_blueprint(first_v2).0, "v2.0");
    }

    // A reader on a page the exam does not cover is told so, not left looking
    // for a highlighted entry that is not there
    #[test]
    fn a_page_off_an_exam_says_so_in_that_ordering() {
        let views = nav_views("content-delivery-networks", before_cutover());
        let notes = |key: &str| {
            views
                .iter()
                .find(|view| view.key == key)
                .map(|view| view.notes.join(" "))
                .expect("ordering exists")
        };
        assert!(notes("ccna").contains("not on this exam"));
        assert!(!notes("netplus").contains("not on this exam"));
        assert!(notes("topic").is_empty());
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
