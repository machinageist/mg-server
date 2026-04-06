// mod declarations tell Rust these source files exist as modules.
// Without these lines the compiler doesn't know the files are part of the crate.
// Order matters for nothing — but grouping them at the top is conventional.mod router;
mod handlers;
mod models;
mod errors;
mod middleware;

use std::net::SocketAddr;
use crate::middleware::rate_limit::build_limiter;

// #[tokio::main] is a procedural macro that wraps main() in a Tokio async runtime.
// Rust has no built-in async executor — Tokio provides the thread pool and scheduler.
// Without this macro, `async fn main()` would be a compile error.
// Think of it as: start the engine before the car can move.
#[tokio::main]
async fn main() {
    // Configure structured logging before anything else runs.
    // tracing_subscriber reads the RUST_LOG environment variable at startup:
    //   RUST_LOG=info cargo run    — shows INFO and above (normal operation)
    //   RUST_LOG=debug cargo run   — shows everything including tower internals
    //   RUST_LOG=mg_server=debug   — shows only this crate's debug output
    // The fallback "mg_server=info,tower_http=info" applies when RUST_LOG is not set.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mg_server=info,tower_http=info".into())
        )
        .init();

    // Build the router — all routes and middleware defined in router.rs.
    // Keeping this in one line here means main.rs is only responsible for
    // startup and shutdown — not for routing decisions.
    let limiter = build_limiter();
    let app = router::build();

    // SocketAddr::from(([0, 0, 0, 0], 3000)) binds to all network interfaces on port 3000.
    // 0.0.0.0 means: accept connections from anywhere (localhost AND the network).
    // 127.0.0.1 would mean: local connections only.
    // In production, behind a reverse proxy, you'd bind to 127.0.0.1 —
    // Caddy handles public traffic, your Rust server is only reachable locally.
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server starting on http://{}", addr);

    // TcpListener::bind performs socket() + bind() + listen() in C terms.
    // .await yields control until the OS confirms the port is bound.
    // .unwrap() crashes if the port is already in use — acceptable at startup.
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    // axum::serve hands the listener to Axum and starts the accept loop.
    // This blocks until the process is killed — the server runs here.    
    axum::serve(listener, app)
        .await
        .unwrap();
}

