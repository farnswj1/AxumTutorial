mod handlers;
mod serializers;

use axum::{routing::get, serve, Router};
use tokio::net::TcpListener;
use crate::handlers::{root, plain, json};

fn get_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/plain", get(plain))
        .route("/json", get(json))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = get_router();
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("LISTENING on 0.0.0.0:8000");
    serve(listener, router).await.unwrap();
}
