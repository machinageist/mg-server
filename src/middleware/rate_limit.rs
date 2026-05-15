// Author:      machinageist
// Date:        2026-04
// Description: Rate limiting middleware using a token bucket algorithm.
//              build_limiter() constructs a governor RateLimiter allowing
//              60 requests per minute. rate_limit() consumes one token per
//              request and returns 429 Too Many Requests when the bucket empties.
//              SharedRateLimiter wraps the limiter in Arc for safe sharing
//              across concurrent async tasks without a mutex on the pointer.
//
// Notes:       Token bucket — bucket holds 60 tokens, replenishes 1/sec.
//              Allows short bursts up to 60 before throttling begins.
//              This implementation is per-server-instance, not per-IP.
//              Per-IP limiting requires extracting the client IP and keying
//              the limiter on it — governor supports this via DashMap-backed
//              state; see governor docs for the keyed limiter pattern.
//
//              Red team context: 429 is what a brute-force scanner sees when
//              rate limiting fires. Tools like hydra and ffuf handle 429 by
//              slowing down — the rate limiter forces the attack to take longer
//              than a password list would otherwise require.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use governor::clock::DefaultClock;
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;

// -----------------------------------------------------------------------
// Shared type alias — Arc allows cheap clone into each async task
// -----------------------------------------------------------------------

// Full type spelled out — NotKeyed = single bucket, InMemoryState = RAM storage
pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>;

// -----------------------------------------------------------------------
// Limiter construction — called once at startup in router::build()
// -----------------------------------------------------------------------

// Build token bucket allowing 60 requests per minute
pub fn build_limiter() -> SharedRateLimiter {
    // 60 requests per minute = replenish 1 token per second
    let quota = Quota::per_minute(NonZeroU32::new(60).unwrap());
    Arc::new(RateLimiter::direct(quota))
}

// -----------------------------------------------------------------------
// Middleware function — check bucket before passing request to handler
// -----------------------------------------------------------------------

// Consume one token or return 429 if bucket is empty
pub async fn rate_limit(
    limiter: SharedRateLimiter,
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    match limiter.check() {
        // Token available — pass request through to next middleware or handler
        Ok(_)  => next.run(request).await,
        // Bucket empty — return 429 without reaching any handler
        Err(_) => {
            tracing::warn!("rate limit exceeded");
            Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body(Body::from("too many requests"))
                .unwrap()
        }
    }
}
