use crate::controllers::hello;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/api/hello", get(hello::hello))
}
