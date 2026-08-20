// Author:      machinageist
// Date:        2026-07-12
// Description: Shared request counters plus the deliberately minimal public
//              availability response.
//
// Notes:       Counters remain internal. /status and /status.json do not expose
//              process lifetime, memory, build, listener, or traffic details.

use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

// -----------------------------------------------------------------------
// Shared state
// -----------------------------------------------------------------------

#[derive(Clone)]
pub struct AppState {
    requests_total: Arc<AtomicU64>,
    page_hits: Arc<Mutex<HashMap<String, u64>>>,
}

impl AppState {
    // Construct fresh state at process start
    pub fn new() -> Self {
        Self {
            requests_total: Arc::new(AtomicU64::new(0)),
            page_hits: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Preserve the startup API without retaining the listener address in state.
    pub fn with_bind_addr(_bind_addr: IpAddr) -> Self {
        Self::new()
    }

    // Count one request toward the running total
    pub fn record_request(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
    }

    // Count one view of a resolved route
    pub fn record_hit(&self, route: &str) {
        let mut hits = self.page_hits.lock().unwrap();
        *hits.entry(route.to_string()).or_insert(0) += 1;
    }

    // Read the running request total
    #[cfg(test)]
    pub fn requests_total(&self) -> u64 {
        self.requests_total.load(Ordering::Relaxed)
    }

    // Snapshot the per-route hit counters
    #[allow(dead_code)] // consumed by the /stats page in Phase 2
    pub fn hits(&self) -> HashMap<String, u64> {
        self.page_hits.lock().unwrap().clone()
    }

    // Confirm two handles point at the same live runtime
    fn shares_runtime_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.requests_total, &other.requests_total)
            && Arc::ptr_eq(&self.page_hits, &other.page_hits)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------
// Process-global handle — set once at startup, read by the footer/status
// -----------------------------------------------------------------------

static APP_STATE: OnceLock<AppState> = OnceLock::new();

// Publish the process state, rejecting any different router runtime
pub fn init_global(state: AppState) -> Result<(), &'static str> {
    if let Some(existing) = APP_STATE.get() {
        return if existing.shares_runtime_with(&state) {
            Ok(())
        } else {
            Err("global AppState already points at a different router runtime")
        };
    }

    match APP_STATE.set(state.clone()) {
        Ok(()) => Ok(()),
        Err(_) => APP_STATE
            .get()
            .filter(|installed| installed.shares_runtime_with(&state))
            .map(|_| ())
            .ok_or("global AppState initialization raced with a different runtime"),
    }
}

// -----------------------------------------------------------------------
// Public status allowlist. Availability is useful; process metadata is not.
// -----------------------------------------------------------------------

#[derive(Serialize)]
pub struct Status {
    pub status: &'static str,
}

impl Status {
    // Keep this response deliberately coarse. Monitoring details belong in a
    // private system, not in a public reconnaissance endpoint.
    pub fn current() -> Status {
        Status {
            status: "available",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clones_share_the_same_counters() {
        let original = AppState::new();
        let clone = original.clone();

        // Writes through the clone must be visible through the original —
        // this is the property the counting middleware relies on
        clone.record_request();
        clone.record_hit("/via-clone");
        assert_eq!(original.requests_total(), 1);
        assert_eq!(original.hits().get("/via-clone"), Some(&1));
    }

    #[test]
    fn request_and_hit_counters_increment() {
        let state = AppState::new();
        assert_eq!(state.requests_total(), 0);

        state.record_request();
        state.record_request();
        assert_eq!(state.requests_total(), 2);

        state.record_hit("/");
        state.record_hit("/");
        state.record_hit("/about");
        let hits = state.hits();
        assert_eq!(hits["/"], 2);
        assert_eq!(hits["/about"], 1);
    }

    #[test]
    fn status_snapshot_has_version_and_no_secrets() {
        let status = Status::current();
        assert_eq!(status.status, "available");
    }

    #[test]
    fn independent_states_are_detectably_different_runtimes() {
        let first = AppState::new();
        let different = AppState::new();
        assert!(first.shares_runtime_with(&first.clone()));
        assert!(!first.shares_runtime_with(&different));
    }
}
