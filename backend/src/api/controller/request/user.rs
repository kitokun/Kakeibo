use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[serde(default)]
    pub id: i32,
    pub name: String, // ユーザ名
    pub create_date: Option<NaiveDateTime>,
    pub update_date: Option<NaiveDateTime>,
}