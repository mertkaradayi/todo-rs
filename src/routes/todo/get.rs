use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: u64,
    pub username: String,
}

pub async fn get_todos() -> (StatusCode, Json<Vec<Todo>>) {
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
    (StatusCode::OK, Json(todos))
}

pub async fn get_todo(Path(id): Path<u64>) -> (StatusCode, Json<Todo>) {
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

    match todos.into_iter().find(|todo| todo.id == id) {
        Some(todo) => (StatusCode::OK, Json(todo)),
        None => (
            StatusCode::NOT_FOUND,
            Json(Todo {
                id: 0,
                username: "".to_string(),
            }),
        ),
    }
}
