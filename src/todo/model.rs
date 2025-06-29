use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
