use crate::errors::AppError;
use anyhow::Result;
use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: u64,
    pub username: String,
}

pub async fn get_todos() -> Result<Json<Vec<Todo>>, AppError> {
    let todos = vec![
        Todo {
            id: 1337,
            username: "lalala".to_string(),
        },
        Todo {
            id: 1859,
            username: "lalaladjkfjkd".to_string(),
        },
    ];
    Ok(Json(todos))
}

pub async fn get_todo(Path(id): Path<u64>) -> Result<Json<Todo>, AppError> {
    let todos = vec![
        Todo {
            id: 1337,
            username: "lalala".to_string(),
        },
        Todo {
            id: 1859,
            username: "lalaladjkfjkd".to_string(),
        },
    ];

    todos
        .into_iter()
        .find(|todo| todo.id == id)
        .map(Json)
        .ok_or_else(|| AppError {
            status: StatusCode::NOT_FOUND,
            message: format!("todo with id {} not found", id),
        })
}
