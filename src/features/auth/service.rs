use sqlx::SqlitePool;

use crate::features::auth::{model::UserSql, repository};

#[derive(Debug)]
pub enum UserServiceError {
    UserAlreadyExists,
    UserNotFound,
    DatabaseError,
}

pub async fn check_user_exists(pool: &SqlitePool, username: &str) -> Result<(), UserServiceError> {
    let user = repository::user_exists(pool, username).await;
    if user {
        Err(UserServiceError::UserAlreadyExists)
    } else {
        Ok(())
    }
}

pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    hashed_password: &str,
) -> Result<(), UserServiceError> {
    repository::insert_user(pool, username, hashed_password)
        .await
        .map_err(|_| UserServiceError::DatabaseError)
}

pub async fn get_user(pool: &SqlitePool, username: &str) -> Result<UserSql, UserServiceError> {
    repository::get_user_by_username(pool, username)
        .await
        .map_err(|_| UserServiceError::DatabaseError)?
        .ok_or(UserServiceError::UserNotFound)
}
