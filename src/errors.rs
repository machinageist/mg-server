// thiserror::Error derive macro generates Display and std::error::Error implementations.
// The #[error("...")] attribute defines the Display output for each variant.
// This is what appears in log output when you use `error!(error = %e, ...)`.
use thiserror::Error;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use askama::Template;
use tracing::error;

#[derive(Debug, Error)]
pub enum SiteError {
    // PostNotFound is a 404 — user's fault, not server's fault.
    // The slug is included so logs show exactly what was requested.
    #[error("post not found: {0}")]
    PostNotFound(String),

    #[error("invalid file path")]
    InvalidPath,

    #[error("missing frontmatter in post: {0}")]
    MissingFrontmatter(String),

    #[error("frontmatter parse error:  {0}")]
    FrontmatterParse(String),

    #[error("date parse error: {0}")]
    DateParse(String),

    // #[from] generates a From<std::io::Error> impl automatically.
    // This is what makes the ? operator work in functions that return Result<_, SiteError>
    // when the underlying operation returns std::io::Error.
    // Without #[from], you'd have to write .map_err(SiteError::Io) everywhere.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Template)]
#[template(path = "error_404.html")]
struct Error404Template;

impl Error404Template { 
    pub fn title(&self) -> &str { "404" } 
}

#[derive(Template)]
#[template(path = "error_500.html")]
struct Error500Template;

impl Error500Template { 
    pub fn title(&self) -> &str { "500" } 
}

impl IntoResponse for SiteError {
    fn into_response(self) -> Response {
        match self {
            SiteError::PostNotFound(ref slug) => {
                // Log internally with full context — the slug tells us what was requested.
                // error!() macro creates a structured log event at ERROR level.
                // %slug uses the Display impl (plain string output).
                error!(slug = %slug, "404 post not found");
                
                // Render the 404 template — user sees a proper page, not a raw error string.
                // Information disclosure principle: log everything, show users nothing specific.
                // The template render can theoretically fail — we fall back to plain text
                // rather than panicking. Template failures are extremely rare since
                // Askama validates them at compile time.
                match Error404Template.render() {
                    Ok(html) =>(StatusCode::NOT_FOUND, axum::response::Html(html)).into_response(),
                    Err(_) => (StatusCode::NOT_FOUND, "404 not found").into_response(),
                }
            }

            other => {
                // Log the full internal error — this is for the operator (you), not the user.
                // The user gets "something went wrong" — no stack traces, no file paths,
                // no library versions. All of those are recon data for an attacker.
                error!(error = %other, "500 internal server error");
                match Error500Template.render() {
                    Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, axum::response::Html(html)).into_response(),
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "500 internal server error").into_response(),
                }
            }
        }
    }
}
