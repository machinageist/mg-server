// Author:      machinageist
// Date:        2026-04
// Description: Entry point for mg-server. Configures the tracing (logging)
//              subscriber, builds the Axum router with all routes and middleware
//              attached, binds a TCP listener to loopback by default, and hands it to
//              Axum's serve loop. Stays thin — all routing and middleware logic
//              lives in router.rs. This file is only responsible for startup.
//
// Notes:       RUST_LOG env var controls log verbosity at runtime:
//                RUST_LOG=info cargo run        — normal operation
//                RUST_LOG=debug cargo run       — full tower internals
//                RUST_LOG=mg_server=debug       — only this crate's debug output
//              MG_BIND_ADDR can override loopback for an explicit deployment need.

// Declare modules — tells Rust each file exists as part of this crate
mod assets;
mod errors;
mod handlers;
mod middleware;
mod models;
mod router;
mod search;
mod state;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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

    // Resolve bind address — default is 127.0.0.1 so only Caddy (same host) can reach us.
    // MG_BIND_ADDR overrides for local dev when binding 0.0.0.0 is needed.
    // Defaulting to loopback closes the LAN-side bypass of Caddy and the Cloudflare Tunnel.
    let host: IpAddr = std::env::var("MG_BIND_ADDR")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let addr = SocketAddr::from((host, 3000));

    // Construct state from the resolved listener configuration, then publish the
    // exact same runtime to the footer/status read handle before building the router
    let state = state::AppState::with_bind_addr(host);
    state::init_global(state.clone()).expect("global state must match router state");

    // Build the complete application — routes, static files, middleware all wired inside
    let app = router::build(state);

    tracing::info!("server starting on http://{}", addr);

    // Open the TCP socket — equivalent to socket() + bind() + listen() in C
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Hand the socket to Axum — blocks here until the process is killed
    axum::serve(listener, app).await.unwrap();
}
