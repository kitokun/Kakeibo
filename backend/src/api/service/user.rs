use crate::db::connect::{connect_db};

use crate::api::controller::request::user::User;
use crate::api::store::user::{
    save_user,
    retrieve_all_users,
    retrieve_user_by_id,
    retrieve_user_by_name,
};

pub async fn obtain_all_users() -> Result<Vec<User>, sqlx::Error> {
    let pool = connect_db().await?;

    let result = retrieve_all_users(&pool).await;
    
    match result {
        Ok(users) => {
            return Ok(users)
        },
        Err(error) => {
            return Err(error)
        },
    }
}

pub async fn obtain_user_by_id(id: &str) -> Result<User, sqlx::Error> {
    let pool = connect_db().await?;

    let result = retrieve_user_by_id(&pool, id).await;
    match result {
        Ok(user) => {
            return Ok(user)
        },
        Err(error) => {
            return Err(error)
        },
    }
}

pub async fn obtain_user_by_name(name: &str) -> Result<User, sqlx::Error> {
    let pool = connect_db().await?;

    let result = retrieve_user_by_name(&pool, name).await;
    match result {
        Ok(user) => {
            return Ok(user)
        },
        Err(error) => {
            return Err(error)
        },
    }
}

pub async fn create_user(user: &User) -> Result<(), sqlx::Error> {
    let pool = connect_db().await?;

    let result = save_user(&pool, user).await;
    match result {
        Ok(_) => {
            return Ok(())
        },
        Err(error) => {
            return Err(error)
        },
    }
}