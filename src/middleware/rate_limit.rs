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

pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>;

pub fn build_limiter() -> SharedRateLimiter {
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
            tracing::warn!("rate limit exceeded");
            Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body(Body::from("too  many requests"))
                .unwrap()
        }
    }
}
