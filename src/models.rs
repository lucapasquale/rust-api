use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct TodoModel {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub done: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
