use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)] // tells Askama to generate  rendering code for struct
#[template(path = "index.html")] // links struct to index.html
pub struct IndexTemplate {
    // fields availeable as {{ field__name }} in the template
    pub name: String,
}

// title() is called {{ self.title() }} in base.html
// this is how pages set thier own <title>
impl IndexTemplate {
    pub fn title(&self) -> &str {
        "home"
    }
}

// Axum calles handler when GET / is requested
// builds struct, fills fields, returns IndexTemplate
// askama_axum makes Template implement IntoResponse for auto HTTP resposne
pub async fn home() -> impl IntoResponse {
    IndexTemplate {
        name: "machinageist".to_string(),
    }
}

// --- about page ---

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

// -- portfolio page --
#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct PortfolioTemplate;

impl PortfolioTemplate {
    pub fn title(&self) -> &str {
        "portfolio"
    }
}

pub async fn portfolio() -> impl IntoResponse {
    PortfolioTemplate
}

    

