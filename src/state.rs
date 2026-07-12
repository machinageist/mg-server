// Author:      machinageist
// Date:        2026-07-12
// Description: Shared application state and the process status snapshot. AppState
//              is cheap to clone — every field is Copy (the process-start Instant)
//              or an Arc — and holds the counters the vitals strip and /status
//              read: total requests, per-route hits, and uptime.
//
//              A process-global OnceLock holds a clone of AppState so the footer
//              vitals strip can render on every page without threading State into
//              every handler. Reads go through Status::current(); the middleware
//              still writes through its own clone — both share the same Arcs.
//
// Notes:       AtomicU64 for the request total — lock-free on the hot path. The
//              per-route map takes a Mutex because a HashMap needs one; at this
//              traffic level the lock is never contended, so dashmap is not
//              justified. RSS is read from /proc/self/status and is None off Linux
//              (dev is macOS) rather than panicking — a missing metric is fine.
//              Nothing here exposes a hostname, IP, or path.

use serde::Serialize;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
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
// Avoid reading /proc for every footer render
const RSS_CACHE_TTL: Duration = Duration::from_secs(5);

// -----------------------------------------------------------------------
// Shared state
// -----------------------------------------------------------------------

#[derive(Clone)]
pub struct AppState {
    started_at: Instant,
    requests_total: Arc<AtomicU64>,
    page_hits: Arc<Mutex<HashMap<String, u64>>>,
    rss_cache: Arc<Mutex<RssCache>>,
    bind_mode: BindMode,
}

impl AppState {
    // Construct fresh state at process start
    pub fn new() -> Self {
        Self::with_bind_addr(IpAddr::V4(Ipv4Addr::LOCALHOST))
    }

    // Construct fresh state with the address the listener actually resolved
    pub fn with_bind_addr(bind_addr: IpAddr) -> Self {
        Self {
            started_at: Instant::now(),
            requests_total: Arc::new(AtomicU64::new(0)),
            page_hits: Arc::new(Mutex::new(HashMap::new())),
            rss_cache: Arc::new(Mutex::new(RssCache::default())),
            bind_mode: BindMode::from(bind_addr),
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
    #[allow(dead_code)] // consumed by the /stats page in Phase 2
    pub fn hits(&self) -> HashMap<String, u64> {
        self.page_hits.lock().unwrap().clone()
    }

    // Time since the process started
    pub fn uptime(&self) -> Duration {
        self.started_at.elapsed()
    }

    // Build timestamp as epoch seconds
    pub fn build_ts(&self) -> i64 {
        BUILD_TS_EPOCH.parse().unwrap_or(0)
    }

    // Read a recent RSS sample, refreshing /proc at most once per cache window
    fn rss_mib(&self) -> Option<u64> {
        self.rss_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get_or_refresh(Instant::now(), rss_mib)
    }

    // Describe the resolved listener without exposing a custom address
    fn bind_description(&self) -> &'static str {
        self.bind_mode.description()
    }

    // Confirm two handles point at the same live runtime
    fn shares_runtime_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.requests_total, &other.requests_total)
            && Arc::ptr_eq(&self.page_hits, &other.page_hits)
            && Arc::ptr_eq(&self.rss_cache, &other.rss_cache)
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
// Cached process metrics and privacy-safe listener classification
// -----------------------------------------------------------------------

#[derive(Default)]
struct RssCache {
    sampled_at: Option<Instant>,
    value: Option<u64>,
}

impl RssCache {
    // Return a recent sample or invoke the reader once when the cache is stale
    fn get_or_refresh<F>(&mut self, now: Instant, read: F) -> Option<u64>
    where
        F: FnOnce() -> Option<u64>,
    {
        if self
            .sampled_at
            .is_some_and(|sampled| now.saturating_duration_since(sampled) < RSS_CACHE_TTL)
        {
            return self.value;
        }

        self.value = read();
        self.sampled_at = Some(now);
        self.value
    }
}

#[derive(Clone, Copy)]
enum BindMode {
    LoopbackV4,
    LoopbackV6,
    AllIpv4,
    AllIpv6,
    Custom,
}

impl From<IpAddr> for BindMode {
    fn from(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(addr) if addr.is_loopback() => Self::LoopbackV4,
            IpAddr::V6(addr) if addr.is_loopback() => Self::LoopbackV6,
            IpAddr::V4(addr) if addr.is_unspecified() => Self::AllIpv4,
            IpAddr::V6(addr) if addr.is_unspecified() => Self::AllIpv6,
            _ => Self::Custom,
        }
    }
}

impl BindMode {
    // Return a useful classification without publishing a custom address
    fn description(self) -> &'static str {
        match self {
            Self::LoopbackV4 => "loopback (127.0.0.1)",
            Self::LoopbackV6 => "loopback (::1)",
            Self::AllIpv4 => "all IPv4 interfaces",
            Self::AllIpv6 => "all IPv6 interfaces",
            Self::Custom => "custom interface",
        }
    }
}

// -----------------------------------------------------------------------
// Status snapshot — the shape rendered by the footer, /status, /status.json
// -----------------------------------------------------------------------

#[derive(Serialize)]
pub struct Status {
    pub uptime_secs: u64,
    pub uptime: String,
    pub requests: u64,
    pub rss_mib: Option<u64>,
    pub version: &'static str,
    pub build: String,
    pub bind: String,
}

impl Status {
    // Snapshot the live status from the global state
    pub fn current() -> Status {
        let state = APP_STATE.get();
        let uptime = state.map(|s| s.uptime()).unwrap_or_default();
        Status {
            uptime_secs: uptime.as_secs(),
            uptime: format_uptime(uptime),
            requests: state.map(|s| s.requests_total()).unwrap_or(0),
            rss_mib: state.and_then(AppState::rss_mib),
            version: CRATE_VERSION,
            build: format_build(state.map(|s| s.build_ts()).unwrap_or(0)),
            bind: state
                .map(AppState::bind_description)
                .unwrap_or("unknown")
                .to_string(),
        }
    }
}

// -----------------------------------------------------------------------
// Free helpers — process metrics and readout formatting
// -----------------------------------------------------------------------

// Read resident memory in MiB from /proc/self/status; None off Linux or on failure
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
pub fn format_uptime(uptime: Duration) -> String {
    let total = uptime.as_secs();
    let days = total / 86_400;
    let hours = (total % 86_400) / 3_600;
    let minutes = (total % 3_600) / 60;
    format!("{days:02}:{hours:02}:{minutes:02}")
}

// Format an epoch as YYYY-MM-DD HH:MM UTC
fn format_build(epoch: i64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp(epoch, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    #[test]
    fn clones_share_the_same_counters() {
        let original = AppState::new();
        let clone = original.clone();

        // Writes through the clone must be visible through the original —
        // this is the property the vitals middleware and footer rely on
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
    fn uptime_formats_as_dd_hh_mm() {
        // 1 day, 1 hour, 1 minute, 1 second
        assert_eq!(format_uptime(Duration::from_secs(90_061)), "01:01:01");
        assert_eq!(format_uptime(Duration::from_secs(0)), "00:00:00");
    }

    #[test]
    fn status_snapshot_has_version_and_no_secrets() {
        let status = Status::current();
        assert_eq!(status.version, env!("CARGO_PKG_VERSION"));
        // never leak an address in the bind description
        assert!(!status.bind.contains("0.0.0.0"));
    }

    #[test]
    fn independent_states_are_detectably_different_runtimes() {
        let first = AppState::new();
        let different = AppState::new();
        assert!(first.shares_runtime_with(&first.clone()));
        assert!(!first.shares_runtime_with(&different));
    }

    #[test]
    fn bind_description_comes_from_the_resolved_listener_address() {
        assert_eq!(
            BindMode::from(IpAddr::V4(Ipv4Addr::LOCALHOST)).description(),
            "loopback (127.0.0.1)"
        );
        assert_eq!(
            BindMode::from(IpAddr::V6(Ipv6Addr::LOCALHOST)).description(),
            "loopback (::1)"
        );
        assert_eq!(
            BindMode::from(IpAddr::V4(Ipv4Addr::UNSPECIFIED)).description(),
            "all IPv4 interfaces"
        );
        assert_eq!(
            BindMode::from(IpAddr::V6(Ipv6Addr::UNSPECIFIED)).description(),
            "all IPv6 interfaces"
        );
        assert_eq!(
            BindMode::from(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 10))).description(),
            "custom interface"
        );
    }

    #[test]
    fn rss_cache_reuses_a_recent_sample() {
        let mut cache = RssCache::default();
        let mut reads = 0;
        let started = Instant::now();

        let first = cache.get_or_refresh(started, || {
            reads += 1;
            Some(42)
        });
        let second = cache.get_or_refresh(started + Duration::from_secs(1), || {
            reads += 1;
            Some(99)
        });

        assert_eq!(first, Some(42));
        assert_eq!(second, Some(42));
        assert_eq!(reads, 1, "the /proc reader should not run per render");
    }

    #[test]
    fn rss_cache_refreshes_after_ttl_and_caches_failed_reads() {
        let mut cache = RssCache::default();
        let started = Instant::now();

        assert_eq!(cache.get_or_refresh(started, || None), None);
        assert_eq!(
            cache.get_or_refresh(started + Duration::from_secs(1), || Some(99)),
            None,
            "a failed /proc sample is cached during the TTL"
        );
        assert_eq!(
            cache.get_or_refresh(started + RSS_CACHE_TTL, || Some(99)),
            Some(99),
            "the next render at expiry refreshes the sample"
        );
    }
}
