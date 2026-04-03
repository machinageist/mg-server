// All route definitions owned by this file

use axum::{Router, routing::get};
// use crate::handlers::pages;

pub fn build() -> Router {
    Router::new()
        .route("/", get(hello))
        // .route("/", get(pages::home))
        // .route("/about", get(pages::about))
        // .route("/portfolio", get(pages::portfolio))
        // Blog routes goes here
        // Static file serving goes here
}

async fn hello() -> &'static str {
    "Hello from your Rust server."
}
