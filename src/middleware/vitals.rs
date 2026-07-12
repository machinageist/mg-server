// Author:      machinageist
// Date:        2026-07-12
// Description: Request-counting middleware. Increments the running request total
//              and the per-route hit counter in AppState. Static assets are not
//              portfolio page views, so they are skipped; unresolved paths (404s)
//              are counted toward the request total but not the per-route map, so
//              a bot hammering random URLs cannot inflate the route list.
//
// Notes:       Placed inside the rate limiter in router::build — requests the
//              limiter has already rejected never reach here, so a throttled
//              flood does not run up the counters. State is baked in via
//              from_fn_with_state, so the surrounding router stays state-free.

use crate::state::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

// Static assets are served straight from disk and are not page views
const STATIC_PREFIX: &str = "/static";

// Count the request and, for resolved pages, the per-route hit
pub async fn count(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let path = request.uri().path().to_string();
    let is_static = path.starts_with(STATIC_PREFIX);

    // Total counts every non-static request, resolved or not
    if !is_static {
        state.record_request();
    }

    let response = next.run(request).await;

    // Per-route map only records real, resolved pages — never 404 spam
    if !is_static && response.status() != StatusCode::NOT_FOUND {
        state.record_hit(&path);
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::get;
    use axum::{Router, middleware};
    use tower::ServiceExt;
    use tower_http::services::ServeDir;

    // Build a minimal router wrapping the counting middleware
    fn app(state: AppState) -> Router {
        Router::new()
            .route("/", get(|| async { "ok" }))
            .nest_service("/static", ServeDir::new("static"))
            .layer(middleware::from_fn_with_state(state, count))
    }

    #[tokio::test]
    async fn counts_pages_but_not_static_or_missing() {
        let state = AppState::new();
        let router = app(state.clone());

        // A real page is counted
        router
            .clone()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(state.requests_total(), 1);
        assert_eq!(state.hits().get("/"), Some(&1));

        // A static asset is excluded entirely
        router
            .clone()
            .oneshot(
                Request::get("/static/css/style.css")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(state.requests_total(), 1, "static must not count");

        // A missing page counts the request but not the route map
        router
            .oneshot(Request::get("/nope").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(state.requests_total(), 2);
        assert!(!state.hits().contains_key("/nope"), "404 must not map");
    }
}
