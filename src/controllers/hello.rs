#[utoipa::path(
        get,
        path = "/hello",
        context_path = "/api",
        responses(
            (status = 200, description = "Returns hello", body = String),
        ),
    )]
pub async fn hello() -> String {
    "Hello, World!".to_string()
}
