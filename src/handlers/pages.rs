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

use crate::errors::SiteError;
use crate::handlers::blog::POSTS_DIR;
use crate::models::markdown::Heading;
use crate::models::page::Page;
use crate::models::post::BlogPost;
use crate::models::project::{self, Project};
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Path as AxumPath;
use std::path::PathBuf;

// Project documents live beside the other content collections
pub(crate) const PROJECTS_DIR: &str = "content/projects";

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
        "Homelab, networking, and Linux notes from machinageist — practical educational material and the projects that come out of it."
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
        "About Jeff Cincoski — homelab, networking and Linux operations, small automation tools, and a public education wiki."
    }
    pub fn section(&self) -> &str {
        "about"
    }
}

// Render about page with bio text
pub async fn about() -> impl IntoResponse {
    AboutTemplate {
        bio: "I'm Jeff. I run a homelab, write about what breaks and how I fix it, and study \
              Linux systems administration and networking. Most of what's here comes out of \
              hardware I own and operate."
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

// -----------------------------------------------------------------------
// Project document — /portfolio/:slug
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "project_page.html")]
pub struct ProjectPageTemplate {
    pub project: Project,
    // The document itself, rendered from content/projects/<slug>.md
    pub page: Page,
    // Derived from the document rather than maintained by hand, so a renamed
    // section cannot leave a contents entry pointing at nothing
    pub outline: Vec<Heading>,
}

impl ProjectPageTemplate {
    pub fn title(&self) -> String {
        format!("{} — machinageist", self.project.name)
    }

    pub fn description(&self) -> &str {
        &self.page.summary
    }

    pub fn section(&self) -> &str {
        "portfolio"
    }
}

// Build one project's document view
//
// The slug is resolved against the model rather than passed to the filesystem,
// so an unknown slug is a 404 from the allowlist and never a path read — the
// same shape /labs/:slug and /learn/:slug use. A project carrying no document
// is a 404 too: the card for it renders without a link, so arriving here means
// the URL was guessed.
fn project_page_view(slug: &str) -> Result<ProjectPageTemplate, SiteError> {
    let project = project::all()
        .into_iter()
        .find(|entry| entry.slug == slug && entry.doc)
        .ok_or_else(|| SiteError::PageNotFound(slug.to_string()))?;
    let page = Page::find(&PathBuf::from(PROJECTS_DIR), project.slug)?;
    let outline = page.outline.clone();
    Ok(ProjectPageTemplate {
        project,
        page,
        outline,
    })
}

// Render one project's document, selected by URL slug
pub async fn project_page(
    AxumPath(slug): AxumPath<String>,
) -> Result<impl IntoResponse, SiteError> {
    project_page_view(&slug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;

    // Every project claiming a document has one, and every document has a
    // project. Both directions, because each failure is invisible from the
    // other side: a `doc: true` with no file is a card linking to a 404, and a
    // file with no entry is writing nothing links to.
    #[test]
    fn project_documents_and_entries_match_in_both_directions() {
        let projects = project::all();

        for entry in projects.iter().filter(|entry| entry.doc) {
            assert!(
                project_page_view(entry.slug).is_ok(),
                "{}: doc is true but content/projects/{}.md does not load",
                entry.name,
                entry.slug
            );
        }

        let dir = PathBuf::from(PROJECTS_DIR);
        for file in std::fs::read_dir(&dir).expect("read content/projects") {
            let path = file.expect("dir entry").path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }
            let slug = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .expect("utf-8 file stem");
            assert!(
                projects.iter().any(|entry| entry.slug == slug && entry.doc),
                "content/projects/{slug}.md has no project entry with doc: true, \
                 so nothing on the site links to it"
            );
        }
    }

    // An unknown slug must come back through the allowlist, never reach the
    // filesystem. Same guard /labs/:slug carries.
    #[test]
    fn an_unknown_project_slug_is_not_a_path_read() {
        assert!(project_page_view("../../etc/passwd").is_err());
        assert!(project_page_view("nonexistent").is_err());
        // A project with no document is a 404 too, not a blank page
        assert!(project_page_view("mg-server").is_err());
    }

    // The card links to the document when there is one, and the page renders
    // the contents a reader came for.
    #[test]
    fn a_project_card_links_to_its_document() {
        let html = PortfolioTemplate {
            projects: project::all(),
        }
        .render()
        .expect("portfolio renders");

        for entry in project::all().iter().filter(|entry| entry.doc) {
            assert!(
                html.contains(&format!("href=\"/portfolio/{}\"", entry.slug)),
                "the portfolio does not link to {}",
                entry.slug
            );
        }
    }

    #[test]
    fn a_project_page_renders_its_document_and_outline() {
        let view = project_page_view("geistos").expect("geistos document loads");
        assert!(!view.outline.is_empty(), "no outline was derived");
        let html = view.render().expect("project page renders");
        assert!(html.contains("<h1>geistos</h1>"));
        assert!(html.contains("Contents"));
    }

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
            content_text: String::new(),
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

        // Concrete work, shown not told. These strings must appear in the page
        // body, not only in <meta description> — asserting on a term that lives
        // solely in the metadata makes this test pass for the wrong reason and
        // break when unrelated copy is edited.
        let body = html
            .split_once("<main")
            .map(|(_, rest)| rest)
            .expect("about/home layout always renders a <main>");
        assert!(body.contains("homelab"));
        assert!(body.contains("virtualized systems"));
        assert!(body.contains("DNS"));
        assert!(body.contains("self-hosted Rust application"));
        // The landing page should not inventory the live hosting stack.
        assert!(!body.contains("Proxmox"));
        assert!(!body.contains("Cloudflare Tunnel"));
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
            bio: "I run a homelab and study Linux systems administration.".to_string(),
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

    // Every capability /about names must be backed by a published post. These five
    // were all claimed until 2026-08-14 while the site's own writing called them
    // absent or planned — the contradiction was one click away. Each banned term
    // is justified by the post that disowns it:
    //   monitoring, backup     hosting-machinageist-dev.md:103-106
    //   VLAN                   management-layer-first-network-migration.md:15
    //   auth-log               security-headers-on-machinageist-dev.md:96-98
    //   health check           no post mentions one
    // When one becomes real, the commit that makes it real updates the post, the
    // copy, and this list together.
    #[test]
    fn about_page_claims_no_capability_the_posts_call_planned() {
        let html = AboutTemplate {
            bio: "I run a homelab and study Linux systems administration.".to_string(),
        }
        .render()
        .expect("about template renders");
        let body = html
            .split_once("<main")
            .map(|(_, rest)| rest)
            .expect("layout always renders a <main>");

        for term in ["monitoring", "backup", "VLAN", "auth-log", "health check"] {
            assert!(
                !body.contains(term),
                "/about claims {term:?}, which the blog posts record as absent or planned"
            );
        }
    }
}
