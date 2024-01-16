use anyhow::Context;
use axum::Extension;
use axum::{http::StatusCode, Json, Router};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::controllers::{hello, todo};
use crate::models;

mod hello_router;
mod todo_router;

#[derive(OpenApi)]
#[openapi(
    paths(hello::hello, todo::list_todos, todo::search_todos),
    tags(
    (name="Sample Project", description="This is a sample AXUM swagger integration")
    ),
    components(
        schemas(models::todo::Todo)
    )
)]
struct ApiDoc;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let listen_url = format!("0.0.0.0:{}", config.listen_port.clone());
    let state = ApiContext {
        config: Arc::new(config),
        db,
    };

    let api = api_router().fallback(api_fallback);
    let app = Router::new()
        .merge(api)
        .merge(SwaggerUi::new("/docs/swagger-ui").url("/docs/openapi.json", ApiDoc::openapi()))
        .layer(Extension(state))
        .fallback(fallback);

    let listener = TcpListener::bind(&listen_url)
        .await
        .context(format!("Failed to listen on url {}", listen_url))?;

    println!(
        "Server listening on address {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    Router::new()
        .merge(hello_router::router())
        .merge(todo_router::router())
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

async fn api_fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"status": "Not Found"})),
    )
}
