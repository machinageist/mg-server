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
// The homelab, cert-track, and GeistScope entries are archived pending a
// long-form rewrite (see content/drafts/portfolio-entries.md) — this list
// only carries entries with verifiable status and evidence.
pub fn all() -> Vec<Project> {
    vec![Project {
        name: "mg-server",
        description: "The Rust/Axum app that serves this site: routes, Askama templates, flat-file Markdown, \
                      request tracing, defensive response headers, and rate limiting, on a Proxmox Debian VM \
                      behind Caddy and a Cloudflare Tunnel.",
        tags: &["rust", "axum", "linux-service", "self-hosting", "headers"],
        url: Some("https://github.com/machinageist/mg-server"),
        status: ProjectStatus::Active,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portfolio_only_carries_entries_with_verifiable_status_and_evidence() {
        let projects = all();

        // Homelab, cert-track, and GeistScope entries are archived pending a
        // rewrite — only mg-server, an Active entry with a real URL, remains.
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "mg-server");
        assert_eq!(projects[0].status, ProjectStatus::Active);
        assert!(projects[0].url.is_some());

        let combined = projects
            .iter()
            .map(|project| format!("{} {}", project.name, project.description))
            .collect::<Vec<_>>()
            .join("\n");

        // Anti-overclaim guards: nothing archived-but-unwritten should reappear here
        assert!(!combined.contains("Homelab"));
        assert!(!combined.contains("GeistScope"));
        assert!(!combined.contains("Certification track"));
        assert!(!combined.contains("bug-bounty"));
        assert!(!combined.contains("red-team"));
        assert!(!combined.contains("offensive security"));
    }
}
