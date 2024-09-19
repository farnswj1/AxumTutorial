mod handlers;
mod serializers;

use axum::{routing::get, serve, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use crate::handlers::{root, plain, json};

fn get_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/plain", get(plain))
        .route("/json", get(json))
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    let router = get_router();
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("LISTENING on 0.0.0.0:8000");
    serve(listener, router).await.unwrap();
}
