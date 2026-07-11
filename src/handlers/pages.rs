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

use crate::models::project::{self, Project};
use askama::Template;
use askama_axum::IntoResponse;

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
    pub fn title(&self) -> &str {
        "machinageist"
    }
    pub fn description(&self) -> &str {
        "Systems Administrator / NOC Technician (in training). An evidence-first infrastructure-support portfolio around a Proxmox homelab, networking, Linux, and a four-CompTIA-cert journey."
    }
    pub fn section(&self) -> &str {
        "home"
    }
}

// Render home page with owner name injected into hero section
pub async fn home() -> impl IntoResponse {
    IndexTemplate {
        name: "machinageist".to_string(),
    }
}

// -----------------------------------------------------------------------
// Start Here page — start_here.html
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "start_here.html")]
pub struct StartHereTemplate;

impl StartHereTemplate {
    // Supply page title to base.html <title> slot
    pub fn title(&self) -> &str {
        "Start Here — machinageist"
    }
    pub fn description(&self) -> &str {
        "Start here for machinageist.dev: an evidence-first portfolio for Systems Administrator and NOC roles, built around a Proxmox homelab, networking, Linux, and a four-CompTIA-cert journey."
    }
    pub fn section(&self) -> &str {
        "start"
    }
}

// Render reviewer orientation page
pub async fn start_here() -> impl IntoResponse {
    StartHereTemplate
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
        "About Jeff Cincoski, a Systems Administrator / NOC Technician in training documenting a Proxmox homelab, networking, Linux service operations, and a four-CompTIA-cert journey."
    }
    pub fn section(&self) -> &str {
        "about"
    }
}

// Render about page with bio text
pub async fn about() -> impl IntoResponse {
    AboutTemplate {
        bio: "Systems Administrator / NOC Technician in training. I run a Proxmox homelab as an \
              operations lab and document the work: homelab and networking writeups as the content \
              engine, Linux service operations, and small support automation. The through-line is a \
              four-CompTIA-cert journey — Network+ then Security+ then Linux+ then Server+, targeted \
              for January 2027 — with each cert anchored to a homelab project. Portland, OR, \
              relocating to the South Bay Area."
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
        "Homelab operations projects, this self-hosted site, and a four-CompTIA-cert track — the SysAdmin/NOC portfolio artifacts machinageist is building with real commands and verification."
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

    #[test]
    fn home_page_frames_site_as_sysadmin_noc_portfolio() {
        let html = IndexTemplate {
            name: "machinageist".to_string(),
        }
        .render()
        .expect("home template renders");

        // Lead identity + pillars + cert through-line
        assert!(html.contains("Systems Administrator"));
        assert!(html.contains("NOC"));
        assert!(html.contains("infrastructure-support portfolio"));
        assert!(html.contains("homelab"));
        assert!(html.contains("certification journey"));
        // Anti-overclaim guards — no offensive/senior first-person framing
        assert!(!html.contains("security engineer"));
        assert!(!html.contains("offensive security"));
        assert!(!html.contains("red-team"));
    }

    #[test]
    fn about_page_keeps_claims_conservative_and_role_aligned() {
        let html = AboutTemplate {
            bio: "Systems Administrator / NOC Technician in training.".to_string(),
        }
        .render()
        .expect("about template renders");

        // Lead roles + pillar structure
        assert!(html.contains("Systems Administrator"));
        assert!(html.contains("NOC"));
        assert!(html.contains("Homelab"));
        assert!(html.contains("Certification journey"));
        assert!(html.contains("What I am not claiming yet"));
        // Long-term aspirations are framed as interests, not commitments
        assert!(html.contains("fields I'm drawn to"));
        assert!(html.contains("not current skills"));
        // Anti-overclaim guards
        assert!(!html.contains("security engineer"));
        assert!(!html.contains("red-team"));
        assert!(!html.contains("offensive security"));
    }

    #[test]
    fn start_here_page_orients_reviewers_to_evidence_and_next_paths() {
        let html = StartHereTemplate
            .render()
            .expect("start-here template renders");

        assert!(html.contains("Start Here"));
        assert!(html.contains("~/tech-skill-up/"));
        assert!(html.contains("Evidence standard"));
        assert!(html.contains("First reviewer paths"));
        assert!(html.contains("What this portfolio is not claiming"));
        assert!(html.contains("Systems Administrator"));
        assert!(html.contains("NOC"));
        assert!(html.contains("Certification journey"));
        assert!(html.contains("not claiming production-grade infrastructure"));
        // Anti-overclaim guards
        assert!(!html.contains("red-team"));
        assert!(!html.contains("offensive security"));
        assert!(!html.contains(">Releases</a>"));
    }
}
