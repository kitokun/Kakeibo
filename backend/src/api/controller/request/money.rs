use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}