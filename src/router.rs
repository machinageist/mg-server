// axum::middleware aliased to `mw` to avoid collision with our own `middleware` module.
// Both `mod middleware` (our files) and `use axum::middleware` (Axum's module)
// would occupy the same name in scope — Rust rejects that.
// Convention: alias the library, keep your own names unqualified.
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
        // Routes are matched top-to-bottom. More specific routes should come first.
        // :slug is a path parameter — any single path segment.
        // /blog/my-post  → slug = "my-post"
        // /blog/foo/bar  → no match (two segments)
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/portfolio", get(pages::portfolio))
        .route("/blog", get(blog::list))
        .route("/blog/:slug", get(blog::post))
        
        // nest_service mounts a complete service at a URL prefix.
        // ServeDir::new("static") maps /static/* to the ./static/ folder on disk.
        // Path is relative to the working directory (project root when using cargo run).
        // ServeDir sanitizes paths automatically — directory traversal attempts return 404.
        .nest_service("/static", ServeDir::new("static"))

        // layer() applies middleware to all routes defined above it.
        // Middleware runs in reverse layer order on the way in,
        // and in layer order on the way out.
        // Request:  TraceLayer → rate_limit → security_headers → handler
        // Response: handler → security_headers → rate_limit → TraceLayer
        .layer(mw::from_fn(add_security_headers))

        // The rate limiter is captured by move closure — shared across all requests.
        // Arc (atomic reference counting) allows safe sharing across async tasks
        // without a mutex — the rate limiter itself handles interior synchronization.
        .layer(mw::from_fn(move |req, next| {
            let lim = limiter.clone();
            async move { rate_limit(lim, req, next).await }
        }))

        // TraceLayer logs every request: method, path, status code, latency.
        // Produces output like: GET /blog 200 OK in 1.2ms
        .layer(TraceLayer::new_for_http())
}
