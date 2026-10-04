// Author:      machinageist
// Date:        2026-05-15
// Description: Handlers for the wiki section.
//              `/learn` renders the overview page (content/pages/index.md).
//              `/learn/:slug` renders one tool page (content/pages/<slug>.md).
//              Both responses include a left navigation sidebar with the
//              active entry highlighted.
//              `/wiki` and `/wiki/:slug` are the pre-rename URLs; they permanently
//              redirect to the `/learn` equivalents so old links keep working.
//
// Notes:       A page with `draft: true` in its frontmatter is registered in
//              SIDEBAR like any other, but it is left out of every sidebar
//              view and out of search, and a release build answers its URL
//              with a 404. A debug build serves it by direct URL so it can be
//              read while it is being written. Publishing is one frontmatter edit.
//
//              The sidebar has two views of the same pages: CCNA, which is the
//              default, and Network+. Each lists the exam's domains in objective
//              order and then every page that exam does not cover, so either
//              view reaches the whole wiki.

use crate::errors::SiteError;
use crate::models::markdown::Heading;
use crate::models::page::Page;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Path as AxumPath;
use axum::response::{Redirect, Response};
use chrono::{NaiveDate, Utc};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) const PAGES_DIR: &str = "content/pages";
const OVERVIEW_SLUG: &str = "index";

// Draft pages answer by direct URL in a debug build only. There is no setting
// to leave switched on, so a release build can never serve one
const SERVE_DRAFTS: bool = cfg!(debug_assertions);

// What kind of page a registered page is, which decides where a sidebar view
// puts it when the exam being shown does not list it
#[derive(Clone, Copy, PartialEq)]
pub enum Group {
    Overview,
    Networking,
    Linux,
}

// One registered /learn page
pub struct SidebarEntry {
    pub slug: &'static str,
    pub label: &'static str,
    pub group: Group,
}

// Headings a sidebar view uses outside the exam's own domains
const OVERVIEW_HEADING: &str = "Overview";
const OFF_EXAM_HEADING: &str = "Not on this exam";
const LINUX_HEADING: &str = "Linux foundations";

// Slugs retired when a page was split or renamed, and the page each now points at.
// Published URLs keep working through a permanent redirect.
const RENAMED_SLUGS: &[(&str, &str)] = &[
    ("network-functions", "vpns-and-ipsec"),
    ("network-applications", "content-delivery-networks"),
];

// Every /learn page on disk is registered here. This is the allowlist of
// servable pages and the one place a page's sidebar label is written. A page
// whose frontmatter says draft: true is registered and not shown
const SIDEBAR: &[SidebarEntry] = &[
    SidebarEntry {
        slug: OVERVIEW_SLUG,
        label: "Education Wiki",
        group: Group::Overview,
    },
    SidebarEntry {
        slug: "osi-model",
        label: "OSI model",
        group: Group::Overview,
    },
    SidebarEntry {
        slug: "network-topologies",
        label: "Network topologies",
        group: Group::Overview,
    },
    SidebarEntry {
        slug: "traffic-types",
        label: "Network traffic types",
        group: Group::Overview,
    },
    SidebarEntry {
        slug: "troubleshooting-method",
        label: "A troubleshooting method",
        group: Group::Overview,
    },
    SidebarEntry {
        slug: "transmission-media",
        label: "Transmission media",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "wired-media",
        label: "Wired media",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "wireless-media",
        label: "Wireless media",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "transceivers",
        label: "Transceivers and connectors",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "ipv4-addressing",
        label: "IPv4 addressing",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "subnetting",
        label: "Subnetting, CIDR, and VLSM",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "ipv6-addressing",
        label: "IPv6 addressing",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "ethernet-and-arp",
        label: "Ethernet frames and ARP",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "switching-technologies",
        label: "Switching technologies",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "spanning-tree",
        label: "Spanning tree: STP and RSTP",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "etherchannel",
        label: "EtherChannel",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "layer-2-security",
        label: "Layer 2 security",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "wireless-technologies",
        label: "Wireless technologies",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "routing-technologies",
        label: "Routing technologies and route selection",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "ospf",
        label: "OSPF, single area",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "first-hop-redundancy",
        label: "First-hop redundancy",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "nat",
        label: "NAT and PAT",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "access-control-lists",
        label: "ACLs and wildcard masks",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "vpns-and-ipsec",
        label: "VPNs and IPsec",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "quality-of-service",
        label: "Quality of service",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "network-protocols",
        label: "Network protocols and ports",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "dhcp-and-dns-services",
        label: "DHCP, DHCP relay, and DNS",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "device-management-protocols",
        label: "NTP, SNMP, syslog, CDP and LLDP",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "securing-device-access",
        label: "Securing device access",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "network-appliances",
        label: "Network appliances",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "content-delivery-networks",
        label: "Content delivery networks",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "cloud-computing",
        label: "Cloud computing concepts",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "software-defined-networking",
        label: "Software-defined networking",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "rest-json-and-config-management",
        label: "REST, JSON, and config management",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "ai-in-network-operations",
        label: "AI in network operations",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "zero-trust-architecture",
        label: "Zero-trust architecture",
        group: Group::Networking,
    },
    SidebarEntry {
        slug: "linux-abstraction-layers",
        label: "Linux abstraction layers",
        group: Group::Linux,
    },
    SidebarEntry {
        slug: "linux-filesystem-hierarchy",
        label: "Filesystem hierarchy",
        group: Group::Linux,
    },
    SidebarEntry {
        slug: "linux-shell",
        label: "The shell and the command line",
        group: Group::Linux,
    },
    SidebarEntry {
        slug: "linux-streams",
        label: "Streams, redirection, and pipes",
        group: Group::Linux,
    },
    SidebarEntry {
        slug: "linux-permissions",
        label: "File permissions and links",
        group: Group::Linux,
    },
    SidebarEntry {
        slug: "linux-archives",
        label: "Archives and compression",
        group: Group::Linux,
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
// A page that spans several objectives is listed once, under the lowest one in
// its main domain. troubleshooting-method has no v1.1 objective of its own
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
            ExamEntry {
                objective: "1.13",
                slug: "ethernet-and-arp",
            },
        ],
    },
    ExamSection {
        heading: "2.0 Network access",
        entries: &[
            ExamEntry {
                objective: "2.1",
                slug: "switching-technologies",
            },
            ExamEntry {
                objective: "2.4",
                slug: "etherchannel",
            },
            ExamEntry {
                objective: "2.5",
                slug: "spanning-tree",
            },
        ],
    },
    ExamSection {
        heading: "3.0 IP connectivity",
        entries: &[
            ExamEntry {
                objective: "3.1",
                slug: "routing-technologies",
            },
            ExamEntry {
                objective: "3.4",
                slug: "ospf",
            },
            ExamEntry {
                objective: "3.5",
                slug: "first-hop-redundancy",
            },
        ],
    },
    ExamSection {
        heading: "4.0 IP services",
        entries: &[
            ExamEntry {
                objective: "4.1",
                slug: "nat",
            },
            ExamEntry {
                objective: "4.2",
                slug: "device-management-protocols",
            },
            ExamEntry {
                objective: "4.3",
                slug: "network-protocols",
            },
            ExamEntry {
                objective: "4.3",
                slug: "dhcp-and-dns-services",
            },
            ExamEntry {
                objective: "4.7",
                slug: "quality-of-service",
            },
        ],
    },
    ExamSection {
        heading: "5.0 Security fundamentals",
        entries: &[
            ExamEntry {
                objective: "5.3",
                slug: "securing-device-access",
            },
            ExamEntry {
                objective: "5.5",
                slug: "vpns-and-ipsec",
            },
            ExamEntry {
                objective: "5.6",
                slug: "access-control-lists",
            },
            ExamEntry {
                objective: "5.7",
                slug: "layer-2-security",
            },
        ],
    },
    ExamSection {
        heading: "6.0 Automation and programmability",
        entries: &[
            ExamEntry {
                objective: "6.2",
                slug: "software-defined-networking",
            },
            ExamEntry {
                objective: "6.4",
                slug: "ai-in-network-operations",
            },
            ExamEntry {
                objective: "6.5",
                slug: "rest-json-and-config-management",
            },
        ],
    },
];

// CCNA 200-301 v2.0 ordering. v2.0 also drops topologies, the OSI model as a
// TCP-versus-UDP objective, traffic types, and QoS
// ethernet-and-arp is left out: v2.0 has no objective for switching concepts.
// Numbers were checked against Cisco's v2.0 exam topics PDF on 2026-10-04
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
            ExamEntry {
                objective: "1.7",
                slug: "dhcp-and-dns-services",
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
                objective: "2.1",
                slug: "etherchannel",
            },
            ExamEntry {
                objective: "2.2",
                slug: "network-appliances",
            },
            ExamEntry {
                objective: "2.4",
                slug: "troubleshooting-method",
            },
            ExamEntry {
                objective: "2.5",
                slug: "spanning-tree",
            },
        ],
    },
    ExamSection {
        heading: "3.0 IP routing",
        entries: &[
            ExamEntry {
                objective: "3.1",
                slug: "routing-technologies",
            },
            ExamEntry {
                objective: "3.3",
                slug: "ospf",
            },
            ExamEntry {
                objective: "3.4",
                slug: "first-hop-redundancy",
            },
        ],
    },
    ExamSection {
        heading: "4.0 Network services and security",
        entries: &[
            ExamEntry {
                objective: "4.1",
                slug: "securing-device-access",
            },
            ExamEntry {
                objective: "4.3",
                slug: "nat",
            },
            ExamEntry {
                objective: "4.4",
                slug: "network-protocols",
            },
            ExamEntry {
                objective: "4.5",
                slug: "vpns-and-ipsec",
            },
            ExamEntry {
                objective: "4.6",
                slug: "access-control-lists",
            },
            ExamEntry {
                objective: "4.7",
                slug: "layer-2-security",
            },
        ],
    },
    ExamSection {
        heading: "5.0 AI, network operations, and management",
        entries: &[
            ExamEntry {
                objective: "5.1",
                slug: "ai-in-network-operations",
            },
            ExamEntry {
                objective: "5.3",
                slug: "software-defined-networking",
            },
            ExamEntry {
                objective: "5.4",
                slug: "device-management-protocols",
            },
            ExamEntry {
                objective: "5.5",
                slug: "rest-json-and-config-management",
            },
        ],
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
    // The page's main sections, shown when the entry is expanded
    pub anchors: Vec<Heading>,
}

impl NavEntry {
    // The page's URL. The overview lives at the wiki root
    pub fn href(&self) -> String {
        if self.slug == OVERVIEW_SLUG {
            "/learn".to_string()
        } else {
            format!("/learn/{}", self.slug)
        }
    }
}

// Heading level a sidebar entry lists when expanded. Subsections are left to
// the page itself
const ANCHOR_LEVEL: u8 = 2;

// Collect the main sections of every published page, keyed by slug
fn page_anchors(drafts: &[&str]) -> HashMap<&'static str, Vec<Heading>> {
    SIDEBAR
        .iter()
        .filter(|entry| !drafts.contains(&entry.slug))
        .map(|entry| {
            let anchors = Page::outline_of(Path::new(PAGES_DIR), entry.slug)
                .into_iter()
                .filter(|heading| heading.level == ANCHOR_LEVEL)
                .collect();
            (entry.slug, anchors)
        })
        .collect()
}

// One heading's worth of links in a rendered sidebar ordering
pub struct NavSection {
    pub heading: &'static str,
    pub entries: Vec<NavEntry>,
}

// One of the sidebar views the toggle switches between
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
    if let Some(allowed) = lookup_sidebar_slug(&slug)
        && is_servable(allowed, SERVE_DRAFTS)
    {
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
        views: nav_views(Utc::now().date_naive(), &draft_slugs()),
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

// Build the CCNA and Network+ views of the sidebar. CCNA is first, which makes
// it the view that shows by default and with JavaScript off. Draft pages are
// left out of both
fn nav_views(today: NaiveDate, drafts: &[&str]) -> Vec<NavView> {
    let (version, ccna) = ccna_blueprint(today);
    let anchors = page_anchors(drafts);
    vec![
        exam_view(
            "ccna",
            "CCNA",
            &format!("CCNA 200-301 {version}"),
            ccna,
            drafts,
            &anchors,
        ),
        exam_view(
            "netplus",
            "Network+",
            "Network+ N10-009",
            NETWORK_PLUS,
            drafts,
            &anchors,
        ),
    ]
}

// Build one sidebar view: the overview, the exam's domains in objective order,
// then every other published page, so each view reaches the whole wiki
fn exam_view(
    key: &'static str,
    name: &'static str,
    exam: &str,
    sections: &'static [ExamSection],
    drafts: &[&str],
    anchors: &HashMap<&'static str, Vec<Heading>>,
) -> NavView {
    let listed: Vec<&str> = sections
        .iter()
        .flat_map(|section| section.entries.iter().map(|entry| entry.slug))
        .collect();

    // Registered pages of one group that the exam does not list
    let unlisted = |group: Group, heading: &'static str| NavSection {
        heading,
        entries: SIDEBAR
            .iter()
            .filter(|entry| entry.group == group)
            .filter(|entry| !listed.contains(&entry.slug) && !drafts.contains(&entry.slug))
            .map(|entry| NavEntry {
                slug: entry.slug,
                label: entry.label,
                objective: None,
                anchors: anchors.get(entry.slug).cloned().unwrap_or_default(),
            })
            .collect(),
    };

    let domains = sections.iter().map(|section| NavSection {
        heading: section.heading,
        entries: section
            .entries
            .iter()
            .filter(|entry| !drafts.contains(&entry.slug))
            .map(|entry| NavEntry {
                slug: entry.slug,
                label: sidebar_label(entry.slug).unwrap_or(entry.slug),
                objective: Some(entry.objective),
                anchors: anchors.get(entry.slug).cloned().unwrap_or_default(),
            })
            .collect(),
    });

    NavView {
        key,
        name,
        notes: vec![format!("Pages on {exam}, by objective.")],
        sections: std::iter::once(unlisted(Group::Overview, OVERVIEW_HEADING))
            .chain(domains)
            .chain([
                unlisted(Group::Networking, OFF_EXAM_HEADING),
                unlisted(Group::Linux, LINUX_HEADING),
            ])
            .filter(|section| !section.entries.is_empty())
            .collect(),
    }
}

// Look up a page's sidebar label, so exam orderings never restate one
fn sidebar_label(slug: &str) -> Option<&'static str> {
    SIDEBAR
        .iter()
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

// List every slug registered in the sidebar, drafts included
pub(crate) fn sidebar_slugs() -> Vec<&'static str> {
    SIDEBAR.iter().map(|entry| entry.slug).collect()
}

// List the registered slugs whose page is still a draft
fn draft_slugs() -> Vec<&'static str> {
    sidebar_slugs()
        .into_iter()
        .filter(|slug| Page::is_draft(Path::new(PAGES_DIR), slug))
        .collect()
}

// List the slugs a reader can reach: the allowlist for search and for links
pub(crate) fn published_slugs() -> Vec<&'static str> {
    let drafts = draft_slugs();
    sidebar_slugs()
        .into_iter()
        .filter(|slug| !drafts.contains(slug))
        .collect()
}

// Decide whether a registered page may be served. A draft is served only
// when the build allows it
fn is_servable(slug: &str, serve_drafts: bool) -> bool {
    serve_drafts || !Page::is_draft(Path::new(PAGES_DIR), slug)
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
    SIDEBAR
        .iter()
        .find(|entry| entry.slug == slug)
        .map(|entry| entry.slug)
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
            views: nav_views(before_cutover(), &[]),
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
            views: nav_views(before_cutover(), &[]),
            active_slug: slug,
        }
        .render()
        .expect("template renders");
        // The page is in both views. Each marks exactly one entry
        // active, and that entry's <li> wraps the OSI-model link.
        for view in ["ccna", "netplus"] {
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
            views: nav_views(before_cutover(), &[]),
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

    // Each sidebar entry expands to the page's own sections. Only the page
    // being read starts expanded, and the links are plain anchors, so it all
    // works with JavaScript off
    #[test]
    fn sidebar_entries_expand_to_the_page_sections() {
        let slug = "osi-model";
        let page = Page::find(&PathBuf::from(PAGES_DIR), slug).expect("OSI page must exist");
        let views = nav_views(before_cutover(), &[]);
        let html = WikiPageTemplate {
            page,
            views,
            active_slug: slug,
        }
        .render()
        .expect("template renders");

        assert!(html.contains("href=\"/learn/osi-model#encapsulation-and-decapsulation\""));
        assert!(html.contains("href=\"/learn/subnetting#subnet-masks\""));
        assert!(
            html.contains("href=\"/learn#"),
            "the overview links from the wiki root"
        );
        // One open entry in each of the two views
        assert_eq!(
            html.matches("<details class=\"wiki-entry\" open>").count(),
            2
        );
        assert!(html.matches("<details class=\"wiki-entry\">").count() > 40);
        // Subsections are left to the page
        assert!(!html.contains("href=\"/learn/subnetting#ccna-200-301\""));
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

    // Find the heading a page sits under in one view
    fn heading_of(views: &[NavView], key: &str, slug: &str) -> Option<&'static str> {
        views
            .iter()
            .find(|view| view.key == key)
            .expect("view exists")
            .sections
            .iter()
            .find(|section| section.entries.iter().any(|entry| entry.slug == slug))
            .map(|section| section.heading)
    }

    // A page the exam does not cover is still in that view, under a heading
    // that says so, and a Linux page is under its own
    #[test]
    fn a_page_off_an_exam_is_listed_under_a_heading_that_says_so() {
        let views = nav_views(before_cutover(), &[]);
        let cdn = "content-delivery-networks";
        assert_eq!(heading_of(&views, "ccna", cdn), Some(OFF_EXAM_HEADING));
        assert_eq!(
            heading_of(&views, "netplus", cdn),
            Some("1.0 Networking concepts")
        );
        for key in ["ccna", "netplus"] {
            assert_eq!(heading_of(&views, key, "linux-shell"), Some(LINUX_HEADING));
            assert_eq!(
                heading_of(&views, key, OVERVIEW_SLUG),
                Some(OVERVIEW_HEADING)
            );
        }
    }

    // CCNA is the main way through the wiki, so it is the first view, which
    // the stylesheet shows by default and with JavaScript off. Both views
    // must reach every published page, each exactly once
    #[test]
    fn each_view_lists_every_published_page_once_and_ccna_leads() {
        let drafts = draft_slugs();
        let views = nav_views(before_cutover(), &drafts);
        assert_eq!(
            views.iter().map(|view| view.key).collect::<Vec<_>>(),
            ["ccna", "netplus"]
        );
        let mut published = published_slugs();
        published.sort_unstable();
        for view in &views {
            let mut listed: Vec<&str> = view
                .sections
                .iter()
                .flat_map(|section| section.entries.iter().map(|entry| entry.slug))
                .collect();
            listed.sort_unstable();
            assert_eq!(
                listed, published,
                "{}: the view should list each published page exactly once",
                view.key
            );
        }
    }

    // A draft is in no ordering, and a section it leaves empty is dropped
    #[test]
    fn a_draft_page_is_left_out_of_every_ordering() {
        // Treat a published page and the whole v1.1 3.0 domain as drafts, so
        // that domain should go with them
        let drafts = [
            "osi-model",
            "routing-technologies",
            "ospf",
            "first-hop-redundancy",
        ];
        for view in nav_views(before_cutover(), &drafts) {
            for section in &view.sections {
                assert!(
                    !section.entries.is_empty(),
                    "{}: {} was left empty instead of dropped",
                    view.key,
                    section.heading
                );
                for entry in &section.entries {
                    assert!(
                        !drafts.contains(&entry.slug),
                        "{}: draft {} is listed",
                        view.key,
                        entry.slug
                    );
                }
            }
            if view.key == "ccna" {
                assert!(
                    !view
                        .sections
                        .iter()
                        .any(|s| s.heading == "3.0 IP connectivity"),
                    "a domain with only draft pages should not render"
                );
            }
        }
    }

    // The gate itself, against a real draft on disk. A release build passes
    // false, a debug build passes true
    #[test]
    fn a_draft_page_is_served_only_when_the_build_allows_it() {
        let draft = *draft_slugs()
            .first()
            .expect("at least one page is a draft while the CCNA pages are being written");
        assert!(!is_servable(draft, false), "{draft} must 404 in release");
        assert!(is_servable(draft, true), "{draft} should load in debug");
        assert!(
            is_servable("osi-model", false),
            "published pages always load"
        );
    }

    // Drafts are registered but never offered, and the published list is the
    // rest of the sidebar
    #[test]
    fn published_slugs_are_the_sidebar_minus_drafts() {
        let drafts = draft_slugs();
        let published = published_slugs();
        assert_eq!(published.len() + drafts.len(), sidebar_slugs().len());
        for slug in &drafts {
            assert!(
                !published.contains(slug),
                "{slug} is a draft and is offered"
            );
        }
        let views = nav_views(before_cutover(), &drafts);
        for view in &views {
            for entry in view.sections.iter().flat_map(|s| s.entries.iter()) {
                assert!(
                    published.contains(&entry.slug),
                    "{}: {} is not published",
                    view.key,
                    entry.slug
                );
            }
        }
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
