use axum::{http::StatusCode, routing::get, Json, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let hello_routes = Router::new().route("/", get(|| async { "Hello, World!" }));
    let api_routes = Router::new()
        .nest("/hello", hello_routes)
        .fallback(api_fallback);

    // Build our application with a single route
    let app = Router::new().nest("/api", api_routes).fallback(fallback);

    // run our app with tokio, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Server listening on address {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
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
