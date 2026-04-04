#[derive(Debug, Clone)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub url: Option<&'static str>,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Active,
    Complete,
    InProgress,
}

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

