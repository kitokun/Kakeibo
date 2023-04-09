use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    #[serde(default)]
    pub amount: i64,
}