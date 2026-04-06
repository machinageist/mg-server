// &'static str is a string slice with the static lifetime —
// meaning the data lives for the entire duration of the program.
// For hardcoded string constants this is always correct and has zero runtime cost.
// Compare to String (heap-allocated, owned) which would require allocation
// just to store values that never change.
#[derive(Debug, Clone)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],  // a fixed-length slice of static string slices
    pub url: Option<&'static str>,      // Option = may or may not have a link
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Active,
    Complete,
    InProgress,
}

// Display trait implementation lets Askama render {{ project.status }} directly.
// Without this, you'd need a separate method call in every template.
// The match ensures every enum variant has a string representation —
// adding a new variant without updating this match is a compile error.
impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter)  -> std::fmt::Result {
        match self {
            ProjectStatus::Active       => write!(f, "active"),
            ProjectStatus::InProgress   => write!(f, "in progress"),
            ProjectStatus::Complete     => write!(f, "complete"),
        }
    }
}

pub fn all() -> Vec<Project> {
    vec![
        Project {
            name: "mg-server",
            description: "Personal site and web server built in rust. \
                          Axum, Askama templates, flat-file blog. \
                          The server you are looking at right now.",
            tags: &["rust", "c", "security", "memory-saftey"],
            url: None,
            status: ProjectStatus::InProgress,
        },
    ]
}

