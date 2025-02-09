use crate::domain::{PostTodo, Todo};
use crate::errors::AppError;
use crate::DbPool;
use anyhow::Result;
use axum::{extract::State, http::StatusCode, Json};

use diesel::prelude::*;

pub async fn create_todo(
    State(db_pool): State<DbPool>,
    Json(new_todo): Json<PostTodo>,
) -> Result<Json<Todo>, AppError> {
    use crate::schema::todos::dsl::*;

    let mut conn = db_pool.get().map_err(|_| AppError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Failed to get connection from pool".to_string(),
    })?;

    let inserted_todo = diesel::insert_into(todos)
        .values(description.eq(new_todo.description))
        .get_result(&mut conn)?;

    Ok(Json(inserted_todo))
}
