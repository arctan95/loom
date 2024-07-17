use axum::Router;

mod web;

pub fn create_app() -> Router {
    Router::new().merge(web::create_web_router())
}
