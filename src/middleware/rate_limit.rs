use std::sync::Arc;
use std::num::NonZeroU32;
use governor::{Quota, RateLimiter};
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::middleware::NoOpMiddleware;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    extract::ConnectInfo,
};

// Arc = Atomically Reference Counted — a smart pointer for shared ownership.
// Multiple async tasks can hold a clone of Arc<T> safely.
// The reference count is updated atomically — no mutex needed for the pointer itself.
// When the last Arc clone is dropped, the value is freed.
//
// The rate limiter's internal state (token bucket) is synchronized by governor itself —
// Arc just allows sharing the limiter across tasks without copying it.
pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>;

pub fn build_limiter() -> SharedRateLimiter {
    / Token bucket algorithm:
    // - Bucket starts full (60 tokens)
    // - Each request consumes one token
    // - Tokens replenish at the quota rate (1 per second for 60/min)
    // - When the bucket is empty, requests are rejected with 429
    // This allows short bursts (up to 60 requests) while enforcing the average rate.
    let quota = Quota::per_minute(NonZeroU32::new(60).unwrap());
    Arc::new(RateLimiter::direct(quota))
}

pub async fn rate_limit(
    limiter: SharedRateLimiter,
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    match limiter.check() {
        Ok(_) => {
            next.run(request).await
        }
        Err(__) => {
            // 429 Too Many Requests — the standard HTTP status for rate limiting.
            // Returning 429 instead of silently dropping tells legitimate clients
            // to back off and retry later (many HTTP clients handle this automatically).
            // Red team context: this is exactly what a scanner sees when rate limiting fires.
            // A scanner that doesn't handle 429 will stall here.
            tracing::warn!("rate limit exceeded");
            Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body(Body::from("too  many requests"))
                .unwrap()
        }
    }
}
