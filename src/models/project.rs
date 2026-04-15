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
    pub name:        &'static str,
    pub description: &'static str,
    // Fixed-size slice of static string slices — zero allocation
    pub tags:        &'static [&'static str],
    // None = not yet published or no public repo
    pub url:         Option<&'static str>,
    pub status:      ProjectStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Active,
    InProgress,
    Complete,
}

// Allow {{ project.status }} in Askama templates — renders the display string directly
impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProjectStatus::Active     => write!(f, "active"),
            ProjectStatus::InProgress => write!(f, "in progress"),
            ProjectStatus::Complete   => write!(f, "complete"),
        }
    }
}

// -----------------------------------------------------------------------
// Project list — add new entries here as projects are built
// -----------------------------------------------------------------------

// Return canonical project list — called by portfolio handler on each request
pub fn all() -> Vec<Project> {
    vec![
        Project {
            name:        "mg-server",
            description: "Personal site and web server built in Rust. \
                          Axum routing, Askama compile-time templates, flat-file \
                          blog with Markdown and YAML frontmatter. \
                          The server you are looking at right now.",
            tags:        &["rust", "axum", "web", "askama"],
            url:         Some("https://github.com/YOUR_USERNAME/mg-server"),
            status:      ProjectStatus::Active,
        },
        Project {
            name:        "port scanner",
            description: "TCP port scanner rewritten from Python to Rust. \
                          Async concurrent scanning with Tokio. \
                          Demonstrates ownership model vs Python's GIL.",
            tags:        &["rust", "networking", "security", "tokio"],
            url:         None,
            status:      ProjectStatus::InProgress,
        },
        Project {
            name:        "memory safety writeup",
            description: "Side-by-side C and Rust implementations of the same \
                          unsafe string pattern. Documents the CVE class each \
                          Rust feature eliminates at the language level.",
            tags:        &["rust", "c", "security", "memory-safety"],
            url:         None,
            status:      ProjectStatus::InProgress,
        },
    ]
}
