use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::service::user::greet;

pub async fn home() -> impl IntoResponse {
    Html("<h1>Welcome to the Joyland!</h1>")
}

pub fn create_web_router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/hello", get(greet))
}
