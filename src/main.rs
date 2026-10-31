use axum::{Router, routing::get};
use std::net::{Ipv6Addr, SocketAddr};

async fn index() -> &'static str {
    "Hello from Rust and Axum!\n"
}

async fn health() -> &'static str {
    "ok\n"
}

// Stop on SIGTERM (what the platform sends) or Ctrl-C.
async fn shutdown() {
    let mut term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("signal handler");
    tokio::select! {
        _ = term.recv() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health));

    // [::] accepts IPv4 too on Linux (IPV6_V6ONLY is off by default).
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    println!("listening on {addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("server");
}
