use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct PostTodo {
    pub description: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::todos)]
pub struct PutTodo {
    pub description: Option<String>,
    pub status: Option<String>,
}
