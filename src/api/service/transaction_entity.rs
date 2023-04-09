use crate::db::connect::{connect_db};
use crate::api::store::transaction_entity::
    {
        create_new_transaction_entity,
        select_all_transaction_entities
    };
use crate::api::controller::request::transaction_entity::TransactionEntity;

pub async fn store_transaction_entity(transaction_entity: &TransactionEntity) -> Result<(), sqlx::Error> {
    let pool = connect_db().await?;
    let result = create_new_transaction_entity(&pool, transaction_entity).await;

    match result {
        Ok(_) => {
            return Ok(())
        },
        Err(error) => {
            return Err(error)
        },
    }
}

pub async fn get_all_transaction_entities() -> Result<Vec<TransactionEntity>, sqlx::Error> {
    let pool = connect_db().await?;
    let transaction_entities = select_all_transaction_entities(&pool).await;

    match transaction_entities {
        Ok(transaction_entities) => {
            return Ok(transaction_entities)
        },
        Err(error) => {
            return Err(error)
        },
    }
}