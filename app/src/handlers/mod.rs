use axum::{http::StatusCode, response::Html, Json};
use crate::serializers::User;

pub async fn root() -> Html<&'static str> {
    Html("<h1>Hello world!</h1>")
}

pub async fn plain() -> &'static str {
    "This is plain text!"
}

// Tuples can be returned to specify status codes. The default code is 200 (OK)
pub async fn json() -> (StatusCode, Json<User>) {
    let data = User { id: 12, name: String::from("John Smith")};
    (StatusCode::OK, Json(data))
}
