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

use crate::handlers::blog::POSTS_DIR;
use crate::models::post::BlogPost;
use crate::models::project::{self, Project};
use askama::Template;
use askama_axum::IntoResponse;
use std::path::PathBuf;

// -----------------------------------------------------------------------
// Home page — index.html
// -----------------------------------------------------------------------

// How many recent posts the home page teases before sending readers to /blog
const HOME_POST_COUNT: usize = 3;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: String,
    // Newest-first, capped at HOME_POST_COUNT — empty renders no section at all
    pub posts: Vec<BlogPost>,
}

impl IndexTemplate {
    // Supply page title to base.html <title> slot
    pub fn title(&self) -> &str {
        "machinageist"
    }
    pub fn description(&self) -> &str {
        "Homelab, networking, and Linux notes from machinageist — a Proxmox lab, CompTIA study, and the projects that come out of it."
    }
    pub fn section(&self) -> &str {
        "home"
    }
}

// Render home page with owner name and the most recent posts
// Post loading degrades to an empty list rather than propagating SiteError — the
// front door should still answer if content/posts is unreadable, and /blog is
// the route that surfaces that failure honestly
pub async fn home() -> impl IntoResponse {
    let mut posts = BlogPost::load_all(&PathBuf::from(POSTS_DIR)).unwrap_or_default();
    posts.truncate(HOME_POST_COUNT);

    IndexTemplate {
        name: "machinageist".to_string(),
        posts,
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
    pub fn title(&self) -> &str {
        "About — machinageist"
    }
    pub fn description(&self) -> &str {
        "About Jeff Cincoski — a Proxmox homelab, networking and Linux operations, small automation, and CompTIA study."
    }
    pub fn section(&self) -> &str {
        "about"
    }
}

// Render about page with bio text
pub async fn about() -> impl IntoResponse {
    AboutTemplate {
        bio: "I'm Jeff. I run a homelab, write about what breaks and how I fix it, and I'm working \
              through the CompTIA stack. Most of what's here comes out of hardware I own and operate."
            .to_string(),
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
    pub fn title(&self) -> &str {
        "Portfolio — machinageist"
    }
    pub fn description(&self) -> &str {
        "What machinageist builds and runs — verified, evidenced work only."
    }
    pub fn section(&self) -> &str {
        "portfolio"
    }
}

// Load project list from models and render portfolio page
pub async fn portfolio() -> impl IntoResponse {
    PortfolioTemplate {
        projects: project::all(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;

    // Build a minimal post carrying only the fields the home page teaser renders
    fn teaser_post(slug: &str, title: &str) -> BlogPost {
        BlogPost {
            slug: slug.to_string(),
            title: title.to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
            summary: "Summary line.".to_string(),
            tags: Vec::new(),
            category: None,
            content_html: String::new(),
        }
    }

    #[test]
    fn home_page_shows_concrete_work_without_strategy_narration() {
        let html = IndexTemplate {
            name: "machinageist".to_string(),
            posts: Vec::new(),
        }
        .render()
        .expect("home template renders");

        // Concrete work, shown not told
        assert!(html.contains("homelab"));
        assert!(html.contains("Proxmox"));
        assert!(html.contains("CompTIA"));
        // Quiet confidence: no self-describing strategy meta-copy
        assert!(!html.contains("infrastructure-support"));
        assert!(!html.contains("in training"));
        assert!(!html.contains("evidence-first"));
        // Anti-overclaim guards — no offensive/senior first-person framing
        assert!(!html.contains("security engineer"));
        assert!(!html.contains("offensive security"));
        assert!(!html.contains("red-team"));
    }

    #[test]
    fn home_page_teases_recent_posts_and_links_to_the_full_list() {
        let html = IndexTemplate {
            name: "machinageist".to_string(),
            posts: vec![teaser_post("network-migration", "Moving My Homelab")],
        }
        .render()
        .expect("home template renders");

        assert!(html.contains("Latest writing"));
        assert!(html.contains("/blog/network-migration"));
        assert!(html.contains("Moving My Homelab"));
        // The teaser always offers a way through to everything else
        assert!(html.contains("All writing"));
        assert!(html.contains("/learn"));
    }

    #[test]
    fn home_page_omits_the_writing_section_when_no_posts_load() {
        let html = IndexTemplate {
            name: "machinageist".to_string(),
            posts: Vec::new(),
        }
        .render()
        .expect("home template renders");

        // Post loading degrades to empty rather than 500ing — the section simply
        // disappears instead of rendering an empty list with a dangling heading
        assert!(!html.contains("Latest writing"));
        assert!(!html.contains("All writing"));
        // The rest of the page is unaffected
        assert!(html.contains("Lately"));
        assert!(html.contains("/learn"));
    }

    #[test]
    fn about_page_describes_work_plainly_without_disclaimers() {
        let html = AboutTemplate {
            bio: "I run a homelab and work through the CompTIA stack.".to_string(),
        }
        .render()
        .expect("about template renders");

        // Plain description of the actual work
        assert!(html.contains("What I work with"));
        assert!(html.contains("homelab"));
        // Show, don't tell: no defensive/strategy sections
        assert!(!html.contains("What I am not claiming yet"));
        assert!(!html.contains("in training"));
        assert!(!html.contains("evidence-first"));
        // Anti-overclaim guards still hold
        assert!(!html.contains("security engineer"));
        assert!(!html.contains("red-team"));
        assert!(!html.contains("offensive security"));
    }
}
