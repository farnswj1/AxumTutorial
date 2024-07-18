use axum::{
    http::StatusCode,
    response::Html,
    routing::get, 
    serve,
    Json,
    Router
};
use tokio::net::TcpListener;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello world!</h1>")
}

async fn plain() -> &'static str {
    "This is plain text!"
}

// Tuples can be return to specify status codes. The default code is 200 (OK)
async fn json() -> (StatusCode, Json<User>) {
    let data = User { id: 12, name: String::from("John Smith")};
    (StatusCode::OK, Json(data))
}

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
