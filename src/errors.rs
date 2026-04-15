// Author:      machinageist
// Date:        2026-04
// Description: Defines SiteError — the single error type for the entire crate.
//              Every way the server can fail is an explicit named variant.
//              Implements Axum's IntoResponse so handlers can return
//              Result<T, SiteError> and Axum knows what HTTP response to send.
//              Logs full internal error details, returns sanitized HTML pages —
//              users never see stack traces, file paths, or library versions.
//
// Notes:       Information disclosure principle: log everything internally,
//              show users nothing specific. Verbose error responses are recon
//              data — they reveal framework, version, and filesystem layout.
//              thiserror generates Display + std::error::Error from annotations.
//              #[from] on Io generates From<io::Error> — enables ? operator in
//              any function returning Result<_, SiteError> that does file I/O.

use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use thiserror::Error;
use tracing::error;

// -----------------------------------------------------------------------
// Error variants — every named failure mode in the application
// -----------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum SiteError {
    // User requested a slug with no matching file on disk — 404
    #[error("post not found: {0}")]
    PostNotFound(String),

    // File path had no valid stem — shouldn't occur in normal operation
    #[error("invalid file path")]
    InvalidPath,

    // .md file exists but has no --- frontmatter block
    #[error("missing frontmatter in post: {0}")]
    MissingFrontmatter(String),

    // Frontmatter block exists but couldn't be deserialized into Frontmatter struct
    #[error("frontmatter parse error: {0}")]
    FrontmatterParse(String),

    // date: field didn't match expected YYYY-MM-DD format
    #[error("date parse error: {0}")]
    DateParse(String),

    // Filesystem errors — directory listing, file reads
    // #[from] auto-generates From<io::Error> so ? works in file I/O functions
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

// -----------------------------------------------------------------------
// Error page templates — rendered by Askama at compile time
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "error_404.html")]
struct Error404Template;

impl Error404Template {
    // Return page title for base.html <title> tag
    pub fn title(&self) -> &str { "404" }
}

#[derive(Template)]
#[template(path = "error_500.html")]
struct Error500Template;

impl Error500Template {
    // Return page title for base.html <title> tag
    pub fn title(&self) -> &str { "500" }
}

// -----------------------------------------------------------------------
// HTTP response mapping — converts errors into browser-appropriate responses
// -----------------------------------------------------------------------

impl IntoResponse for SiteError {
    fn into_response(self) -> Response {
        match self {
            // Log slug for operator visibility, render 404 page for user
            SiteError::PostNotFound(ref slug) => {
                error!(slug = %slug, "404 post not found");
                // Render template or fall back to plain text if template itself fails
                match Error404Template.render() {
                    Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
                    Err(_)   => (StatusCode::NOT_FOUND, "404 not found").into_response(),
                }
            }
            // Log full internal error, return generic 500 — no internals exposed to user
            other => {
                error!(error = %other, "500 internal server error");
                match Error500Template.render() {
                    Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response(),
                    Err(_)   => (StatusCode::INTERNAL_SERVER_ERROR, "500 internal server error").into_response(),
                }
            }
        }
    }
}
