// Author:      machinageist
// Date:        2026-07-12
// Description: Shared application state. Cheap to clone — every field is either
//              Copy (the process-start Instant) or an Arc, so the whole struct
//              clones by bumping reference counts. Holds the counters the vitals
//              strip and /status page read: total requests, per-route hits, and
//              process uptime, plus compile-time build metadata.
//
// Notes:       AtomicU64 for the request total — lock-free on the hot path where
//              every request touches it. The per-route map takes a Mutex because
//              a HashMap needs one; at this traffic level the lock is never
//              contended, so dashmap's complexity is not justified.
//              RSS is read from /proc/self/status and returns None off Linux
//              (dev is macOS) rather than panicking — a missing metric is fine.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

// -----------------------------------------------------------------------
// Compile-time metadata — injected by build.rs and Cargo
// -----------------------------------------------------------------------

// Build timestamp as epoch seconds, stamped by build.rs
const BUILD_TS_EPOCH: &str = env!("BUILD_TS");
// Crate version from Cargo.toml
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
// Linux proc file exposing this process's memory usage
const PROC_STATUS_PATH: &str = "/proc/self/status";
// Kibibytes per mebibyte — VmRSS is reported in kB
const KIB_PER_MIB: u64 = 1024;

// -----------------------------------------------------------------------
// Shared state
// -----------------------------------------------------------------------

#[derive(Clone)]
pub struct AppState {
    started_at: Instant,
    requests_total: Arc<AtomicU64>,
    page_hits: Arc<Mutex<HashMap<String, u64>>>,
}

// The read-side accessors are consumed by the /status handler in Phase 1;
// allow dead_code until that feature lands in the next commit.
#[allow(dead_code)]
impl AppState {
    // Construct fresh state at process start
    pub fn new() -> Self {
        Self {
            started_at: Instant::now(),
            requests_total: Arc::new(AtomicU64::new(0)),
            page_hits: Arc::new(Mutex::new(HashMap::new())),
        }
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
    pub fn requests_total(&self) -> u64 {
        self.requests_total.load(Ordering::Relaxed)
    }

    // Snapshot the per-route hit counters
    pub fn hits(&self) -> HashMap<String, u64> {
        self.page_hits.lock().unwrap().clone()
    }

    // Time since the process started
    pub fn uptime(&self) -> Duration {
        self.started_at.elapsed()
    }

    // Crate version string
    pub fn version(&self) -> &'static str {
        CRATE_VERSION
    }

    // Build timestamp as epoch seconds
    pub fn build_ts(&self) -> i64 {
        BUILD_TS_EPOCH.parse().unwrap_or(0)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------
// Free helpers — process metrics and readout formatting
// -----------------------------------------------------------------------

// Read resident memory in MiB from /proc/self/status; None off Linux or on failure
#[allow(dead_code)] // consumed by the /status handler in Phase 1
pub fn rss_mib() -> Option<u64> {
    let text = std::fs::read_to_string(PROC_STATUS_PATH).ok()?;
    for line in text.lines() {
        // line looks like: VmRSS:     12345 kB
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            let kib: u64 = rest.split_whitespace().next()?.parse().ok()?;
            return Some(kib / KIB_PER_MIB);
        }
    }
    None
}

// Format a duration as dd:hh:mm for the vitals readout
#[allow(dead_code)] // consumed by the /status handler in Phase 1
pub fn format_uptime(uptime: Duration) -> String {
    let total = uptime.as_secs();
    let days = total / 86_400;
    let hours = (total % 86_400) / 3_600;
    let minutes = (total % 3_600) / 60;
    format!("{days:02}:{hours:02}:{minutes:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn uptime_formats_as_dd_hh_mm() {
        // 1 day, 1 hour, 1 minute, 1 second
        assert_eq!(format_uptime(Duration::from_secs(90_061)), "01:01:01");
        assert_eq!(format_uptime(Duration::from_secs(0)), "00:00:00");
    }
}
