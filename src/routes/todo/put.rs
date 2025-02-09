use crate::domain::{PutTodo, Todo};
use crate::errors::AppError;
use crate::DbPool;
use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;

#[axum::debug_handler]
pub async fn update_todo(
    State(db_pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(update_todo): Json<PutTodo>,
) -> Result<Json<Todo>, AppError> {
    use crate::schema::todos::dsl::{id as todo_id, todos};

    let mut conn = db_pool.get().map_err(|_| AppError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "Failed to get connection from pool".to_string(),
    })?;

    let updated_todo = diesel::update(todos.filter(todo_id.eq(id)))
        .set(&update_todo)
        .get_result(&mut conn)?;
    Ok(Json(updated_todo))
}
