mod router;

use std::net::SocketAddr;
use axum::Router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = router::build();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

