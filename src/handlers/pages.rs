use askama::Template;
use askama_axum::IntoResponse;
use crate::models::project::{self, Project};

// -- Home page --

#[derive(Template)] 
#[template(path = "index.html")] 
pub struct IndexTemplate {
    pub name: String,
}

impl IndexTemplate {
    pub fn title(&self) -> &str {
        "home"
    }
}

pub async fn home() -> impl IntoResponse {
    IndexTemplate {
        name: "machinageist".to_string(),
    }
}

// --- About page ---

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub bio: String,
}

impl AboutTemplate {
    pub fn title(&self) -> &str {
        "about"
    }
}

pub async fn about() -> impl IntoResponse {
    AboutTemplate {bio: "CS background. learning offensive and defensive security. building tools in rust and c.".to_string(),
    }
}

// -- Portfolio page --

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct PortfolioTemplate {
    pub projects: Vec<Project>,
}

impl PortfolioTemplate {
    pub fn title(&self) -> &str { "portfolio" }
}

pub async fn portfolio() -> impl IntoResponse {
    PortfolioTemplate {
        projects: project::all(),
    }
}

    

