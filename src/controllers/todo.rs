use crate::models::todo::{Todo, TodoError};
use axum::{extract::Query, Json};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;
/// List all todo items
#[utoipa::path(
        get,
        path = "/todo",
        context_path = "/api",
        responses(
            (status = 200, description = "List all todos successfully", body = [Todo])
        )
    )]
pub async fn list_todos() -> Json<Vec<Todo>> {
    let todos = vec![
        Todo {
            id: Uuid::new_v4(),
            value: "Done".to_string(),
            is_done: true,
        },
        Todo {
            id: Uuid::new_v4(),
            value: "Not done".to_string(),
            is_done: false,
        },
    ];
    Json(todos)
}

/// Todo search query
#[derive(Deserialize, IntoParams)]
pub struct TodoSearchQuery {
    /// Search by value. Search is case insensitive.
    value: Option<String>,
    /// Search by `done` status.
    done: Option<bool>,
}

/// Search Todos by query params.
#[utoipa::path(
        get,
        path = "/todo/search",
        context_path = "/api",
        params(
            TodoSearchQuery
        ),
        responses(
            (status = 200, description = "List matching todos by query", body = [Todo])
        )
    )]
pub async fn search_todos(query: Query<TodoSearchQuery>) -> Json<Vec<Todo>> {
    Json(vec![])
}
