// Author:      machinageist
// Date:        2026-07-12
// Description: Handlers for the /status page and /status.json endpoint. Both
//              render the same coarse availability value from state.rs. Process
//              counters, memory, build metadata, and bind details stay private.

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
        "A minimal availability check for machinageist.dev."
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
    async fn status_page_reports_availability_without_process_vitals() {
        let (code, body) = get_body("/status").await;
        assert_eq!(code, StatusCode::OK);
        assert!(body.contains("available"));
        for private_label in ["UP", "REQ", "MEM", "VER", "BUILT", "BIND"] {
            assert!(
                !body.contains(private_label),
                "status page leaks {private_label}"
            );
        }
    }

    #[tokio::test]
    async fn status_json_parses_and_has_expected_fields() {
        let (code, body) = get_body("/status.json").await;
        assert_eq!(code, StatusCode::OK);
        let parsed: serde_json::Value = serde_json::from_str(&body).expect("valid JSON");
        assert_eq!(parsed["status"], "available");
        for private_field in [
            "uptime_secs",
            "uptime",
            "requests",
            "rss_mib",
            "version",
            "build",
            "bind",
        ] {
            assert!(
                parsed.get(private_field).is_none(),
                "public status JSON leaks {private_field}"
            );
        }
    }

    #[tokio::test]
    async fn process_vitals_do_not_appear_on_pages() {
        for path in ["/", "/blog"] {
            let (code, body) = get_body(path).await;
            assert_eq!(code, StatusCode::OK, "{path} must be 200");
            assert!(
                !body.contains("vitals-strip"),
                "{path} leaks process vitals"
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
}
