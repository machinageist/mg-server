// Author:      machinageist
// Date:        2026-04
// Description: Defines Project and ProjectStatus — the data model for portfolio
//              entries. Projects are hardcoded in all() as a static Vec rather
//              than loaded from files because they are structured, typed, and
//              change rarely. The compiler validates every field at build time.
//              ProjectStatus implements Display so templates can render
//              {{ project.status }} directly without a helper method.
//
// Notes:       &'static str fields have zero runtime allocation cost — the data
//              is embedded in the binary at compile time and lives for the entire
//              program. Appropriate for values that never change at runtime.
//              Adding a new ProjectStatus variant without updating the Display
//              match is a compile error — exhaustive matching enforced by Rust.
//              url is Option<&'static str> — None projects render without a link.

// -----------------------------------------------------------------------
// Data types
// -----------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    // Fixed-size slice of static string slices — zero allocation
    pub tags: &'static [&'static str],
    // None = not yet published or no public repo
    pub url: Option<&'static str>,
    pub status: ProjectStatus,
}

impl Project {
    pub fn status_class(&self) -> &'static str {
        self.status.class_name()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Active,
    #[allow(dead_code)]
    InProgress,
    #[allow(dead_code)]
    Complete,
}

impl ProjectStatus {
    pub fn class_name(&self) -> &'static str {
        match self {
            ProjectStatus::Active => "active",
            ProjectStatus::InProgress => "in-progress",
            ProjectStatus::Complete => "complete",
        }
    }
}

// Allow {{ project.status }} in Askama templates — renders the display string directly
impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProjectStatus::Active => write!(f, "active"),
            ProjectStatus::InProgress => write!(f, "in progress"),
            ProjectStatus::Complete => write!(f, "complete"),
        }
    }
}

// -----------------------------------------------------------------------
// Project list — add new entries here as projects are built
// -----------------------------------------------------------------------

// Return canonical project list — called by portfolio handler on each request
// Order is deliberate: homelab operations lead, then this site, the cert track,
// and finally the demoted GeistScope retrospective.
pub fn all() -> Vec<Project> {
    vec![
        Project {
            name: "Homelab project 1 — internal DNS + network map",
            description: "An internal DNS resolver (Pi-hole/dnsmasq) for the Proxmox lab, a subnet/VLAN and \
                          service map now spanning a three-node cluster over a shared managed switch, and a \
                          dig/nslookup/ss name-resolution writeup captured before and after breaking a record. \
                          Anchored to Network+.",
            tags: &["homelab", "networking", "dns", "proxmox", "network+"],
            url: None,
            status: ProjectStatus::InProgress,
        },
        Project {
            name: "Homelab project 2 — harden & monitor the homelab",
            description: "A hardening pass across a lab VM and this server — key-only SSH, a host firewall, \
                          non-root service users, unattended updates — a security-headers audit of the site, and \
                          a log-based failed-login detector with triage notes. Anchored to Security+.",
            tags: &["homelab", "security", "linux", "hardening", "security+"],
            url: None,
            status: ProjectStatus::InProgress,
        },
        Project {
            name: "Homelab project 3 — Proxmox cluster ops: backup/restore, monitoring, HA",
            description: "The three-node Proxmox cluster's operations: a baseline and asset inventory, a validated \
                          VM backup and restore with RPO/RTO notes, a monitoring stack, structured incident \
                          write-ups, and high availability as the capstone. Anchored to Server+ and Linux+.",
            tags: &["homelab", "proxmox", "backup", "monitoring", "server+"],
            url: None,
            status: ProjectStatus::InProgress,
        },
        Project {
            name: "mg-server",
            description: "The Rust/Axum app that serves this site: routes, Askama templates, flat-file Markdown, \
                          request tracing, defensive response headers, and rate limiting, on a Proxmox Debian VM \
                          behind Caddy and a Cloudflare Tunnel.",
            tags: &["rust", "axum", "linux-service", "self-hosting", "headers"],
            url: Some("https://github.com/machinageist/mg-server"),
            status: ProjectStatus::Active,
        },
        Project {
            name: "Certification track — Network+ to Server+ by Jan 2027",
            description: "Network+, Security+, Linux+, then Server+, each one anchored to a homelab project above. \
                          Writeups link from each as the work lands.",
            tags: &["comptia", "network+", "security+", "linux+", "server+"],
            url: None,
            status: ProjectStatus::InProgress,
        },
        Project {
            name: "GeistScope (retrospective)",
            description: "An early AI-assisted-coding security-tooling experiment that over-scoped; the project has \
                          been removed from the public tool catalog and is not presented as professional security \
                          work. The retrospective records what was real, what was aspirational, and the publication \
                          gate future tools must meet.",
            tags: &["rust", "ai-assisted", "retrospective", "scope-control"],
            url: Some("/blog/geistscope-retrospective"),
            status: ProjectStatus::Complete,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portfolio_leads_with_homelab_and_cert_work_and_demotes_geistscope() {
        let projects = all();

        // The first three cards are the homelab operations projects
        assert!(projects[0].name.contains("Homelab project 1"));
        assert!(projects[1].name.contains("Homelab project 2"));
        assert!(projects[2].name.contains("Homelab project 3"));

        let combined = projects
            .iter()
            .map(|project| format!("{} {}", project.name, project.description))
            .collect::<Vec<_>>()
            .join("\n");

        // New lead framing: homelab / Proxmox / networking / certs
        assert!(combined.contains("homelab") || combined.contains("Homelab"));
        assert!(combined.contains("Proxmox"));
        assert!(combined.contains("Network+"));
        assert!(combined.contains("Certification track"));

        // GeistScope is present but demoted to one retrospective, framed as an experiment.
        assert!(combined.contains("GeistScope"));
        assert!(combined.contains("retrospective"));
        assert!(combined.contains("experiment"));

        // Anti-overclaim guards
        assert!(!combined.contains("bug-bounty"));
        assert!(!combined.contains("red-team"));
        assert!(!combined.contains("offensive security"));
    }
}
