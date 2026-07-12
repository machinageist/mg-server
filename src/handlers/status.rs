// Author:      machinageist
// Date:        2026-07-12
// Description: Handlers for the /status page and /status.json endpoint. Both
//              render the same Status snapshot from state.rs — the page for
//              humans, the JSON for machines (the /tty terminal consumes it).
//              Values are read at request time; this is a stamp, not a feed.
//
// Notes:       The snapshot deliberately exposes only uptime, request count,
//              RSS, version, build timestamp, and bind mode — no hostname, no
//              addresses, no paths. Anything added here ships to the public
//              internet, so the Status struct is the allowlist.

use crate::state::Status;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Json;
use axum::http::header;

// Live readouts must never be cached — every response is a fresh snapshot
const CACHE_POLICY: (header::HeaderName, &str) = (header::CACHE_CONTROL, "no-store");

// -----------------------------------------------------------------------
// Status page — status.html
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "status.html")]
pub struct StatusTemplate {
    pub status: Status,
}

impl StatusTemplate {
    // Supply page title to base.html <title> slot
    pub fn title(&self) -> &str {
        "Status — machinageist"
    }
    pub fn description(&self) -> &str {
        "Live process vitals for machinageist.dev — uptime, request count, memory, and build metadata from the Rust process serving this page."
    }
    pub fn section(&self) -> &str {
        "status"
    }
}

// Render the full status readout page
pub async fn page() -> impl IntoResponse {
    (
        [CACHE_POLICY],
        StatusTemplate {
            status: Status::current(),
        },
    )
}

// Serve the same snapshot as JSON for machine consumers
pub async fn json() -> impl IntoResponse {
    ([CACHE_POLICY], Json(Status::current()))
}

#[cfg(test)]
mod tests {
    use crate::router;
    use crate::state::AppState;
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, Response, StatusCode, header};
    use tower::ServiceExt;

    // Drive a router instance and return the raw response
    async fn get(app: Router, path: &str) -> Response<Body> {
        app.oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    // Collect a response body into a string
    async fn body_string(response: Response<Body>) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    // Drive a fresh router and return status code plus body
    async fn get_body(path: &str) -> (StatusCode, String) {
        let response = get(router::build(AppState::new()), path).await;
        let status = response.status();
        (status, body_string(response).await)
    }

    #[tokio::test]
    async fn status_page_renders_vitals() {
        let (code, body) = get_body("/status").await;
        assert_eq!(code, StatusCode::OK);
        assert!(body.contains("UP"), "status page must show uptime");
        assert!(body.contains("REQ"), "status page must show request count");
        // privacy allowlist — nothing internal leaks
        assert!(!body.contains("0.0.0.0"));
        assert!(!body.contains("/Users/"));
        assert!(!body.contains("/home/"));
    }

    #[tokio::test]
    async fn status_json_parses_and_has_expected_fields() {
        let (code, body) = get_body("/status.json").await;
        assert_eq!(code, StatusCode::OK);
        let parsed: serde_json::Value = serde_json::from_str(&body).expect("valid JSON");
        assert!(parsed["uptime_secs"].is_u64());
        assert!(parsed["requests"].is_u64());
        assert_eq!(parsed["version"], env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn vitals_strip_appears_on_pages() {
        for path in ["/", "/blog"] {
            let (code, body) = get_body(path).await;
            assert_eq!(code, StatusCode::OK, "{path} must be 200");
            assert!(
                body.contains("vitals-strip"),
                "{path} must include the footer vitals strip"
            );
        }
    }

    #[tokio::test]
    async fn live_status_responses_are_never_cached() {
        for path in ["/status", "/status.json"] {
            let response = get(router::build(AppState::new()), path).await;
            assert_eq!(
                response.headers().get(header::CACHE_CONTROL).unwrap(),
                "no-store",
                "{path} must carry Cache-Control: no-store"
            );
        }
    }

    // The one test allowed to publish to the process-global OnceLock —
    // init_global is idempotent only for clones of the same runtime, so a
    // second test claiming it with its own state would fail the first claim
    #[tokio::test]
    async fn status_json_reflects_requests_counted_through_the_router() {
        let state = AppState::new();
        crate::state::init_global(state.clone()).expect("test publishes the router state");
        let app = router::build(state.clone());

        // Three page views through the real middleware stack
        for _ in 0..3 {
            get(app.clone(), "/").await;
        }

        // The snapshot must read the same Arcs the middleware wrote through:
        // 3 page views plus the /status.json request itself
        let response = get(app, "/status.json").await;
        let parsed: serde_json::Value =
            serde_json::from_str(&body_string(response).await).expect("valid JSON");
        assert_eq!(parsed["requests"], 4, "snapshot must see middleware writes");
        assert_eq!(
            parsed["requests"].as_u64().unwrap(),
            state.requests_total(),
            "global snapshot and local clone must share state"
        );
    }
}
