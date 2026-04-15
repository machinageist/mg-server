// Author:      machinageist
// Date:        2026-04
// Description: Entry point for mg-server. Configures the tracing (logging)
//              subscriber, builds the Axum router with all routes and middleware
//              attached, binds a TCP listener to 0.0.0.0:3000, and hands it to
//              Axum's serve loop. Stays thin — all routing and middleware logic
//              lives in router.rs. This file is only responsible for startup.
//
// Notes:       RUST_LOG env var controls log verbosity at runtime:
//                RUST_LOG=info cargo run        — normal operation
//                RUST_LOG=debug cargo run       — full tower internals
//                RUST_LOG=mg_server=debug       — only this crate's debug output
//              Binding 0.0.0.0 accepts connections on all interfaces.
//              In production behind Caddy, change to 127.0.0.1 so the port
//              is only reachable from localhost — Caddy handles public traffic.

// Declare modules — tells Rust each file exists as part of this crate
mod router;
mod handlers;
mod models;
mod errors;
mod middleware;

use std::net::SocketAddr;

// Wrap main() in the Tokio async runtime — required for async/await to work
// Tokio provides the thread pool and task scheduler Rust does not ship built-in
#[tokio::main]
async fn main() {
    // Configure structured logging before anything else can produce output
    tracing_subscriber::fmt()
        .with_env_filter(
            // Read RUST_LOG from environment, fall back to info-level for this crate
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mg_server=info,tower_http=info".into()),
        )
        .init();

    // Build the complete application — routes, static files, middleware all wired inside
    let app = router::build();

    // Bind to all interfaces on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("server starting on http://{}", addr);

    // Open the TCP socket — equivalent to socket() + bind() + listen() in C
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    // Hand the socket to Axum — blocks here until the process is killed
    axum::serve(listener, app)
        .await
        .unwrap();
}
