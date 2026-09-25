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
    // URL slug, and the file stem of the project document when one exists
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    // Fixed-size slice of static string slices — zero allocation
    pub tags: &'static [&'static str],
    // None = not yet published or no public repo
    pub url: Option<&'static str>,
    pub status: ProjectStatus,
    // Whether content/projects/<slug>.md exists. A card links to the document
    // only when this is true, and the drift guard in handlers::pages checks the
    // claim in both directions — a true with no file, or a file with no entry,
    // fails the build rather than shipping a dead link.
    pub doc: bool,
}

impl Project {
    pub fn status_class(&self) -> &'static str {
        self.status.class_name()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Active,
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
    vec![
        Project {
            slug: "mg-server",
            name: "mg-server",
            description: "The Rust/Axum app that serves this site: routes, Askama templates, flat-file Markdown, \
                          request tracing, defensive response headers, and rate limiting, deployed as a managed \
                          Linux service.",
            tags: &["rust", "axum", "linux-service", "self-hosting", "headers"],
            url: Some("https://github.com/machinageist/mg-server"),
            status: ProjectStatus::Active,
            doc: false,
        },
        Project {
            slug: "geistos",
            name: "geistos",
            description: "A local-first Linux workstation: a Quickshell desktop replacing the usual bar, launcher \
                          and notification daemon, a palette roster shared with this site, and Hyprland \
                          configured in Lua, one file per concern.",
            tags: &["linux", "wayland", "hyprland", "quickshell", "theming"],
            url: Some("https://github.com/machinageist/geistos"),
            status: ProjectStatus::InProgress,
            doc: true,
        },
        Project {
            slug: "mg-suite",
            name: "mg-suite",
            description: "An AI-assisted study in application architecture: a set of small local-first tools, each \
                          with its own data, connected through explicit interfaces instead of a shared database. \
                          Some are usable and some are not.",
            tags: &[
                "rust",
                "sqlite",
                "postgresql",
                "architecture",
                "local-first",
            ],
            url: Some("https://github.com/machinageist/mg-suite"),
            status: ProjectStatus::InProgress,
            doc: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portfolio_only_carries_entries_with_verifiable_status_and_evidence() {
        let projects = all();

        // Every entry leaves a reader somewhere to check: a public repository,
        // a project document on this site, or both. An entry that offers
        // neither is a claim with nothing behind it, which is the exact shape
        // this list exists to prevent.
        for project in &projects {
            assert!(
                project.url.is_some() || project.doc,
                "{}: no repository and no document — nothing a reader can check",
                project.name
            );
            assert!(!project.slug.is_empty(), "{}: no slug", project.name);
            assert!(
                project
                    .slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{}: slug {:?} is not url-safe",
                project.name,
                project.slug
            );
        }

        // Homelab, cert-track, and GeistScope entries are archived pending a
        // rewrite. mg-server remains the one Active entry with a public URL.
        let server = projects
            .iter()
            .find(|project| project.name == "mg-server")
            .expect("mg-server is the anchor entry");
        assert_eq!(server.status, ProjectStatus::Active);
        assert!(server.url.is_some());

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
