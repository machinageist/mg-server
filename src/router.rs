// Author:      machinageist
// Date:        2026-04
// Description: Single source of truth for all URL routes and middleware ordering.
//              Builds and returns the complete Axum Router that main.rs hands to
//              the server. Every URL the site responds to is defined here.
//              Middleware layers are applied in reverse-stack order on the way in
//              and forward order on the way out — TraceLayer sees every request
//              first and last, security_headers stamps every outgoing response.
//
// Notes:       Route matching is top-down. More specific routes first.
//              :slug matches exactly one path segment — /blog/foo/bar won't match.
//              ServeDir sanitizes paths automatically — traversal attempts return 404.
//              Rate limiter is built here and captured by closure into the layer.
//              axum::middleware aliased to `mw` — avoids collision with our own
//              `middleware` module which occupies the same name in scope.

use axum::{Router, routing::get};
use axum::middleware as mw;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use crate::handlers::{pages, blog};
use crate::middleware::security_headers::add_security_headers;
use crate::middleware::rate_limit::{rate_limit, build_limiter};

// Build and return the fully configured Axum application
pub fn build() -> Router {
    // Construct rate limiter once — Arc inside allows cheap cloning across tasks
    let limiter = build_limiter();

    // -----------------------------------------------------------------------
    // Routes — URL pattern to handler function mappings
    // -----------------------------------------------------------------------
    Router::new()
        .route("/",            get(pages::home))
        .route("/about",       get(pages::about))
        .route("/portfolio",   get(pages::portfolio))
        .route("/blog",        get(blog::list))
        // Capture :slug as a single path segment, passed to handler as Path<String>
        .route("/blog/:slug",  get(blog::post))
        // Map /static/* URL prefix to ./static/ directory on disk
        .nest_service("/static", ServeDir::new("static"))

        // -----------------------------------------------------------------------
        // Middleware layers — applied bottom-up on request, top-down on response
        // -----------------------------------------------------------------------

        // Stamp security headers onto every outgoing response
        .layer(mw::from_fn(add_security_headers))
        // Check rate limit before request reaches any handler
        // Clone limiter into closure so each task gets a shared reference
        .layer(mw::from_fn(move |req, next| {
            let lim = limiter.clone();
            async move { rate_limit(lim, req, next).await }
        }))
        // Log every request: method, path, status code, response time
        .layer(TraceLayer::new_for_http())
}
