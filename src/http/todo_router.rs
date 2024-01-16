use crate::controllers::todo;
use axum::{routing::get, Router};
pub fn router() -> Router {
    Router::new()
        .route("/api/todo", get(todo::list_todos))
        .route("/api/todo/search", get(todo::search_todos))
}
