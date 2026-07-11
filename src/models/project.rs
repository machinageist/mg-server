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
// and finally the demoted GeistScope archive line.
pub fn all() -> Vec<Project> {
    vec![
        Project {
            name: "Homelab project 1 — internal DNS + network map",
            description: "Anchors Network+. Building an internal resolver (Pi-hole/dnsmasq) with local records \
                          for the Proxmox lab, a subnet/VLAN + service map that now spans a three-node Proxmox \
                          cluster over a shared managed switch, the bridges and VM IPs, and the Cloudflare Tunnel \
                          to Caddy to mg-server path, plus a dig/nslookup/ping/curl/ss troubleshooting writeup \
                          captured before and after breaking one record. Safe claim: implemented and validated \
                          internal DNS for a Proxmox homelab and documented its subnet layout and \
                          name-resolution troubleshooting. Evidence in progress — the two new nodes and switch \
                          are being brought up now; not resume-facing until the writeup and command output are \
                          captured.",
            tags: &["homelab", "networking", "dns", "proxmox", "network+"],
            url: None,
            status: ProjectStatus::InProgress,
        },
        Project {
            name: "Homelab project 2 — harden & monitor the homelab",
            description: "Anchors Security+ (planned for the Security+ phase). A host-hardening pass on a VM and \
                          mg-server (key-only SSH, host firewall, non-root service users, unattended updates), a \
                          security-headers audit of machinageist.dev, and a log-based failed-login detector with \
                          triage notes. Safe claim: hardened Linux hosts and built a failed-login detector on an \
                          owned homelab. Planned — artifacts not yet captured.",
            tags: &["homelab", "security", "linux", "hardening", "security+"],
            url: None,
            status: ProjectStatus::InProgress,
        },
        Project {
            name: "Homelab project 3 — Proxmox cluster ops: backup/restore, monitoring, HA",
            description: "Anchors Server+ and pulls in Linux+ automation (planned for the Server+/Linux+ phase). \
                          A Proxmox baseline and asset inventory across the three-node cluster, a validated VM \
                          backup and restore with RPO/RTO notes, a monitoring stack, structured NOC-style incident \
                          reports, and high availability as the later capstone. Honest sequencing: nodes joined and \
                          quorum validated first, HA claimed only once failover is actually measured — not HA yet. \
                          Safe claim: documented a small Proxmox cluster and validated VM backup/restore with \
                          monitoring and incident reports. Planned — artifacts not yet captured.",
            tags: &["homelab", "proxmox", "backup", "monitoring", "server+"],
            url: None,
            status: ProjectStatus::InProgress,
        },
        Project {
            name: "mg-server",
            description: "The Rust/Axum application that serves this site — a narrow, honest self-hosting artifact. \
                          Axum routes, Askama templates, flat-file Markdown content, request tracing, defensive \
                          response headers, and rate limiting, deployed on a Proxmox Debian VM behind Caddy and a \
                          Cloudflare Tunnel. Supports Linux service-operations and request-path discussions without \
                          overstating backend engineering seniority.",
            tags: &["rust", "axum", "linux-service", "self-hosting", "headers"],
            url: Some("https://github.com/machinageist/mg-server"),
            status: ProjectStatus::Active,
        },
        Project {
            name: "Certification track — Network+ to Server+ by Jan 2027",
            description: "The through-line for this portfolio: Network+ then Security+ then Linux+ then Server+, \
                          each cert anchored to one of the homelab projects above. Progress is stated honestly — a \
                          cert is listed as passed only once it is passed. Writeups link back from each cert phase \
                          as the homelab evidence is captured.",
            tags: &["comptia", "network+", "security+", "linux+", "server+"],
            url: None,
            status: ProjectStatus::InProgress,
        },
        Project {
            name: "GeistScope (archived reference)",
            description: "An early AI-assisted-coding security-tooling experiment that over-scoped; the project has \
                          been narrowed and archived as reference, not presented as professional security work. See \
                          the retrospective on the blog and the archived notes in the Archive, which are kept in \
                          full but labeled on every page as beginner generative-AI experimentation.",
            tags: &["rust", "archive", "retrospective", "scope-control"],
            url: None,
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

        // GeistScope is present but demoted to one archived line, framed as an experiment
        assert!(combined.contains("GeistScope"));
        assert!(combined.contains("archived"));
        assert!(combined.contains("experiment"));

        // Anti-overclaim guards
        assert!(!combined.contains("bug-bounty"));
        assert!(!combined.contains("red-team"));
        assert!(!combined.contains("offensive security"));
    }
}
