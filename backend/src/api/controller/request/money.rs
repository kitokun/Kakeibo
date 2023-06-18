use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    #[serde(default)]
    pub id: i32,
    pub amount: i32,         // 金額
    pub destination: String, // 支払い先
    pub source: String,      // 支払い元
    pub nominal: String,     // 名目
    pub description: String, // 説明
    pub user_id: i32,        // ユーザID
    pub create_date: Option<NaiveDateTime>,
    pub update_date: Option<NaiveDateTime>,
}