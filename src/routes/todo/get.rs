use crate::errors::AppError;
use crate::DbPool;
use anyhow::Result;
use axum::{extract::Path, extract::State, http::StatusCode, Json};

use crate::domain::Todo;
use diesel::prelude::*;

pub async fn get_todos(State(db_pool): State<DbPool>) -> Result<Json<Vec<Todo>>, AppError> {
    use crate::schema::todos::dsl::*;

    let mut conn = db_pool
        .get()
        .map_err(|_| AppError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to get connection from pool".to_string(),
        })?;

    let result_todos: Vec<_> = todos
        .limit(10)
        .select(Todo::as_select())
        .load(&mut conn)
        .expect("Error loading todos");

    Ok(Json(result_todos))
}

pub async fn get_todo(
    State(db_pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, AppError> {
    use crate::schema::todos::dsl;

    let mut conn = db_pool
        .get()
        .map_err(|_| AppError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to get connection from pool".to_string(),
        })?;
    dsl::todos
        .filter(dsl::id.eq(id))
        .select(Todo::as_select())
        .first::<Todo>(&mut conn)
        .optional()?
        .map(Json)
        .ok_or_else(|| AppError {
            status: StatusCode::NOT_FOUND,
            message: format!("todo with id {:?} not found", id),
        })
}
