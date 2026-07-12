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

    // User requested a standalone page slug with no matching file on disk — 404
    #[error("page not found: {0}")]
    PageNotFound(String),

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
struct Error404Template {
    // The path the visitor asked for — Askama auto-escapes it on render
    path: String,
}

impl Error404Template {
    // Return page title for base.html <title> tag
    pub fn title(&self) -> &str {
        "404"
    }
    pub fn description(&self) -> &str {
        "The requested page could not be found."
    }
    pub fn section(&self) -> &str {
        "error"
    }
}

#[derive(Template)]
#[template(path = "error_500.html")]
struct Error500Template;

impl Error500Template {
    // Return page title for base.html <title> tag
    pub fn title(&self) -> &str {
        "500"
    }
    pub fn description(&self) -> &str {
        "The server could not complete the request."
    }
    pub fn section(&self) -> &str {
        "error"
    }
}

// -----------------------------------------------------------------------
// HTTP response mapping — converts errors into browser-appropriate responses
// -----------------------------------------------------------------------

impl IntoResponse for SiteError {
    fn into_response(self) -> Response {
        match self {
            // Log slug for operator visibility, render 404 page for user
            SiteError::PostNotFound(ref slug) | SiteError::PageNotFound(ref slug) => {
                error!(slug = %slug, "404 not found");
                render_404(slug.clone())
            }
            // Log full internal error, return generic 500 — no internals exposed to user
            other => {
                error!(error = %other, "500 internal server error");
                match Error500Template.render() {
                    Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response(),
                    Err(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "500 internal server error",
                    )
                        .into_response(),
                }
            }
        }
    }
}

// Render the boot-sequence 404 for a requested path or slug
fn render_404(path: String) -> Response {
    // Render template or fall back to plain text if template itself fails
    match (Error404Template { path }).render() {
        Ok(html) => (StatusCode::NOT_FOUND, Html(html)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "404 not found").into_response(),
    }
}

// Catch every unmatched route with the themed 404 — registered as the
// router fallback so bare Axum defaults never reach a visitor
pub async fn fallback_404(uri: axum::http::Uri) -> Response {
    render_404(uri.path().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router;
    use crate::state::AppState;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    // Collect a response body into a string
    async fn body_string(response: Response) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[test]
    fn requested_path_is_html_escaped() {
        let html = Error404Template {
            path: "<script>alert(1)</script>".to_string(),
        }
        .render()
        .expect("404 template renders");
        assert!(html.contains("&lt;script&gt;"), "probe must be escaped");
        assert!(!html.contains("<script>alert"), "probe must never be live");
    }

    #[tokio::test]
    async fn unknown_route_returns_boot_sequence_404() {
        let app = router::build(AppState::new());
        let response = app
            .oneshot(Request::get("/no-such-page").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = body_string(response).await;
        assert!(body.contains("SECTOR NOT FOUND"));
        assert!(body.contains("/no-such-page"), "path must echo back");
    }

    #[tokio::test]
    async fn internal_error_page_leaks_nothing() {
        let inner = std::io::Error::other("secret-internal-detail at src/models/post.rs");
        let response = SiteError::Io(inner).into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = body_string(response).await;
        assert!(!body.contains("secret-internal-detail"));
        assert!(!body.contains("src/"));
        assert!(!body.contains(".rs"));
    }
}
