use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransactionEntity {
    #[serde(default)]
    pub id: i32,
    pub transaction_entity: String,      // 取引主体名
    pub transaction_entity_type: String, // 取引主体タイプ
    pub create_date: Option<NaiveDateTime>,
    pub update_date: Option<NaiveDateTime>,
}