use serde::{Deserialize};

#[derive(Deserialize)]
pub struct TransactionId {
    pub id: i32,
}