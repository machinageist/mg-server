use axum::{Router, routing::get};
use axum::middleware as mw;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use crate::handlers::{pages, blog};
use crate::middleware::security_headers::add_security_headers;
use crate::middleware::rate_limit::{rate_limit, SharedRateLimiter, build_limiter};

pub fn build() -> Router {
    let limiter = build_limiter();

    Router::new()
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/portfolio", get(pages::portfolio))
        .route("/blog", get(blog::list))
        .route("/blog/:slug", get(blog::post))
        .nest_service("/static", ServeDir::new("static"))
        .layer(mw::from_fn(add_security_headers))
        .layer(mw::from_fn(move |req, next| {
            let lim = limiter.clone();
            async move { rate_limit(lim, req, next).await }
        }))
        .layer(TraceLayer::new_for_http())
}
