use diesel;
use diesel::{
    deserialize::FromSql,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
    AsExpression, FromSqlRow,
};
use std::fmt;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "UPPERCASE")]
pub enum TodoStatus {
    Pending,
    Completed,
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoStatus::Pending => write!(f, "PENDING"),
            TodoStatus::Completed => write!(f, "COMPLETED"),
        }
    }
}

impl TryFrom<&str> for TodoStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "PENDING" => Ok(TodoStatus::Pending),
            "COMPLETED" => Ok(TodoStatus::Completed),
            _ => Err(format!("Unknown todo status: {}", value)),
        }
    }
}

impl FromSql<Text, Sqlite> for TodoStatus {
    fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(t.as_str().try_into()?)
    }
}

impl ToSql<Text, Sqlite> for TodoStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub status: TodoStatus,
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
    pub status: Option<TodoStatus>,
}
