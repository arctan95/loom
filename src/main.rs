use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
mod config;
mod router;
mod service;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    let app = router::create_app();
    let addr = "0.0.0.0:8080".to_string();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Server listening on {}...", addr);
    axum::serve(listener, app).await.unwrap();
}
