use axum::{routing::get, Router};
use todo_rs::routes::get_todo;
use todo_rs::routes::get_todos;
use todo_rs::routes::health_check;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/todos", get(get_todos))
        .route("/todos", get(get_todo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8008").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
