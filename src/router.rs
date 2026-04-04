// All route definitions owned by this file
use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use crate::handlers::{pages, blog};

pub fn build() -> Router {
    Router::new()
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/portfolio", get(pages::portfolio))
        // Blog routes goes here
        .route("/blog", get(blog::list))
        .route("/blog/:slug", get(blog::post))
        .nest_service("/static", ServeDir::new("static"))
}
