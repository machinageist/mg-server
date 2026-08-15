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

use crate::errors;
use crate::handlers::{blog, glossary, labs, pages, search, status, study, well_known, wiki};
use crate::middleware::rate_limit::{build_limiter, rate_limit};
use crate::middleware::security_headers::add_security_headers;
use crate::middleware::vitals;
use crate::state::AppState;
use axum::middleware as mw;
use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

// Build and return the fully configured Axum application
pub fn build(state: AppState) -> Router {
    // Construct rate limiter once — Arc inside allows cheap cloning across tasks
    let limiter = build_limiter();

    // -----------------------------------------------------------------------
    // Routes — URL pattern to handler function mappings
    // -----------------------------------------------------------------------
    Router::new()
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/portfolio", get(pages::portfolio))
        .route("/labs", get(labs::labs))
        .route("/labs/:slug", get(labs::lab_page))
        .route("/study", get(study::index))
        // More specific first — /study/cards/:slug must not be shadowed
        .route("/study/cards/:slug", get(study::cards))
        .route(
            "/study/pbq/:slug",
            get(study::scenario_page).post(study::grade_scenario),
        )
        .route("/study/:slug", get(study::quiz).post(study::grade))
        .route("/glossary", get(glossary::landing))
        .route("/glossary/terms", get(glossary::terms))
        .route("/glossary/commands", get(glossary::commands))
        .route("/blog", get(blog::list))
        .route("/search", get(search::search))
        .route("/learn", get(wiki::index))
        .route("/learn/:slug", get(wiki::page))
        // /wiki is the pre-rename URL — keep old links and bookmarks working
        .route("/wiki", get(wiki::redirect_index))
        .route("/wiki/:slug", get(wiki::redirect_page))
        // Capture :slug as a single path segment, passed to handler as Path<String>
        .route("/blog/:slug", get(blog::post))
        // Process vitals — human page and the JSON the /tty terminal consumes
        .route("/status", get(status::page))
        .route("/status.json", get(status::json))
        // RFC 9116 security disclosure contact. Serve both the canonical
        // well-known URL and root fallback for scanners that check either path.
        .route("/.well-known/security.txt", get(well_known::security_txt))
        .route("/security.txt", get(well_known::security_txt))
        // Explicit crawler policy for AI bots that ignore Cloudflare's content signals
        .route("/robots.txt", get(well_known::robots_txt))
        // Map /static/* URL prefix to ./static/ directory on disk
        .nest_service("/static", ServeDir::new("static"))
        // Every unmatched route renders the themed 404 with the requested path
        .fallback(errors::fallback_404)
        // -----------------------------------------------------------------------
        // Middleware layers — applied bottom-up on request, top-down on response
        // -----------------------------------------------------------------------
        // Stamp security headers onto every outgoing response
        .layer(mw::from_fn(add_security_headers))
        // Count requests and per-route hits — inside the limiter so throttled
        // floods (already rejected below) never reach the counters
        .layer(mw::from_fn_with_state(state.clone(), vitals::count))
        // Check rate limit before request reaches any handler
        // Clone limiter into closure so each task gets a shared reference
        .layer(mw::from_fn(move |req, next| {
            let lim = limiter.clone();
            async move { rate_limit(lim, req, next).await }
        }))
        // Log every request: method, path, status code, response time
        .layer(TraceLayer::new_for_http())
}
