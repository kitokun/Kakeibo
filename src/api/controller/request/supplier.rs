use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Supplier {
    #[serde(default)]
    pub id: i32,
    pub supplier: String, // 取引先
    pub create_date: Option<NaiveDateTime>,
    pub update_date: Option<NaiveDateTime>,
}