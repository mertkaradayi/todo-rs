use axum::{routing::get, Router};
use todo_rs::routes::{get_todo, get_todos, health_check};

use todo_rs::establish_connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = establish_connection();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/todos", get(get_todos))
        .route("/todos/{id}", get(get_todo))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8008").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
