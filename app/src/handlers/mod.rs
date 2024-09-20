use askama_axum::Template;
use axum::{http::StatusCode, Json};
use crate::serializers::User;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    name: &'static str
}

pub async fn root() -> IndexTemplate {
    IndexTemplate { name: "world" }
}

pub async fn plain() -> &'static str {
    "This is plain text!"
}

// Tuples can be returned to specify status codes. The default code is 200 (OK)
pub async fn json() -> (StatusCode, Json<User>) {
    let data = User { id: 12, name: "John Smith" };
    (StatusCode::OK, Json(data))
}
