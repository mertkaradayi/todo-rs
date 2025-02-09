use axum::{
    routing::{get, post, put},
    Router,
};
use todo_rs::routes::{create_todo, get_todo, get_todos, health_check, update_todo};

use todo_rs::utils::establish_db_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = establish_db_pool();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/todos", post(create_todo))
        .route("/todos", get(get_todos))
        .route("/todos/{id}", get(get_todo))
        .route("/todos/{id}", put(update_todo))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8008").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
