// Author:      machinageist
// Date:        2026-04
// Description: Handlers for all static pages: home, about, and portfolio.
//              Each handler builds an Askama template struct, populates its
//              fields, and returns it. askama_axum's IntoResponse impl converts
//              the rendered HTML into an HTTP 200 response automatically.
//              Portfolio data is pulled from models::project::all() so project
//              entries are defined in one place and used everywhere.
//
// Notes:       Template structs are linked to .html files via #[template(path)].
//              Askama validates field references at compile time — a typo in a
//              template variable name is a build error, not a runtime panic.
//              title() is called by {{ self.title() }} in base.html to set
//              the per-page <title> tag without repeating the base layout.

use askama::Template;
use askama_axum::IntoResponse;
use crate::models::project::{self, Project};

// -----------------------------------------------------------------------
// Home page — index.html
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: String,
}

impl IndexTemplate {
    // Supply page title to base.html <title> slot
    pub fn title(&self) -> &str { "home" }
}

// Render home page with owner name injected into hero section
pub async fn home() -> impl IntoResponse {
    IndexTemplate {
        name: "machinageist".to_string(),
    }
}

// -----------------------------------------------------------------------
// About page — about.html
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub bio: String,
}

impl AboutTemplate {
    // Supply page title to base.html <title> slot
    pub fn title(&self) -> &str { "about" }
}

// Render about page with bio text
pub async fn about() -> impl IntoResponse {
    AboutTemplate {
        bio: "CS background. learning offensive and defensive security. \
              building tools in rust and c.".to_string(),
    }
}

// -----------------------------------------------------------------------
// Portfolio page — portfolio.html
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct PortfolioTemplate {
    // Owned Vec — Askama iterates over projects in the template
    pub projects: Vec<Project>,
}

impl PortfolioTemplate {
    // Supply page title to base.html <title> slot
    pub fn title(&self) -> &str { "portfolio" }
}

// Load project list from models and render portfolio page
pub async fn portfolio() -> impl IntoResponse {
    PortfolioTemplate {
        projects: project::all(),
    }
}
