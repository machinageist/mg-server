// Author:      machinageist
// Date:        2026-04
// Description: Rate limiting middleware using a token bucket algorithm.
//              build_limiter() constructs a governor RateLimiter allowing
//              60 requests per minute. rate_limit() consumes one token per
//              request and returns 429 Too Many Requests when the bucket empties.
//              Each trusted Cloudflare client address gets an independent
//              bucket. SharedRateLimiter wraps the keyed limiter in Arc.
//
// Notes:       Token bucket — bucket holds 60 tokens, replenishes 1/sec.
//              Allows short bursts up to 60 before throttling begins.
//              The origin binds to loopback and is reached through the local
//              reverse-proxy/tunnel path. That trust boundary is what makes the
//              Cloudflare-managed CF-Connecting-IP header usable here.
//
//              Red team context: 429 is what a brute-force scanner sees when
//              rate limiting fires. Tools like hydra and ffuf handle 429 by
//              slowing down — the rate limiter forces the attack to take longer
//              than a password list would otherwise require.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use std::net::{IpAddr, Ipv4Addr};
use std::num::NonZeroU32;
use std::sync::Arc;

// -----------------------------------------------------------------------
// Shared type alias — Arc allows cheap clone into each async task
// -----------------------------------------------------------------------

pub type SharedRateLimiter = Arc<DefaultKeyedRateLimiter<IpAddr>>;

// -----------------------------------------------------------------------
// Limiter construction — called once at startup in router::build()
// -----------------------------------------------------------------------

// Build token bucket allowing 60 requests per minute
pub fn build_limiter() -> SharedRateLimiter {
    // 60 requests per minute = replenish 1 token per second
    let quota = Quota::per_minute(NonZeroU32::new(60).unwrap());
    Arc::new(RateLimiter::keyed(quota))
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
    let client = request
        .headers()
        .get("cf-connecting-ip")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<IpAddr>().ok())
        // Missing or malformed trusted metadata shares one fail-closed bucket;
        // arbitrary header text never creates unlimited keys.
        .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));

    match limiter.check_key(&client) {
        // Token available — pass request through to next middleware or handler
        Ok(_) => next.run(request).await,
        // Bucket empty — return 429 without reaching any handler
        Err(_) => {
            tracing::warn!(client = %client, "rate limit exceeded");
            Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body(Body::from("too many requests"))
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::middleware;
    use axum::routing::get;
    use tower::ServiceExt;

    async fn request(app: Router, address: &str) -> StatusCode {
        app.oneshot(
            Request::get("/")
                .header("cf-connecting-ip", address)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
        .status()
    }

    #[tokio::test]
    async fn one_client_cannot_spend_another_clients_quota() {
        let limiter = build_limiter();
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(middleware::from_fn(move |req, next| {
                let limiter = limiter.clone();
                async move { rate_limit(limiter, req, next).await }
            }));

        for _ in 0..60 {
            assert_eq!(request(app.clone(), "198.51.100.10").await, StatusCode::OK);
        }
        assert_eq!(
            request(app.clone(), "198.51.100.10").await,
            StatusCode::TOO_MANY_REQUESTS
        );
        assert_eq!(
            request(app, "203.0.113.20").await,
            StatusCode::OK,
            "a different client must have an independent bucket"
        );
    }
}
