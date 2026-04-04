use thiserror::Error;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

#[derive(Debug, Error)]
pub enum SiteError {
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

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for SiteError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            SiteError::PostNotFound(slug) =>
                (StatusCode::NOT_FOUND, format!("post not foudn: {}", slug)),
            SiteError::Io(e) =>
                (StatusCode::INTERNAL_SERVER_ERROR, format!("io error: {}", e)),
            other =>
                (StatusCode::INTERNAL_SERVER_ERROR, other.to_string()),
        };

    (status, message).into_response()

    }
}
