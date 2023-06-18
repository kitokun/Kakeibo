use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransactionEntity {
    #[serde(default)]
    pub id: i32,
    pub transaction_entity: String,      // 取引主体名
    pub transaction_entity_type: String, // 取引主体タイプ
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}