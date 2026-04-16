use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_session_auth::AuthSession;
use axum_session_sqlx::SessionSqlitePool;
use bcrypt::{hash, verify};
use sqlx::SqlitePool;

use crate::{
    core::error::AppError,
    features::auth::{
        model::{User, UserRequest},
        service::*,
    },
};

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<UserRequest>,
) -> Result<impl IntoResponse, AppError> {
    check_user_exists(&pool, &payload.username)
        .await
        .map_err(|_| AppError::Conflict(format!("Username '{}' is already taken", payload.username)))?;

    let hashed_password = hash(&payload.password, 10)
        .map_err(|_| AppError::Database("Failed to process password".into()))?;

    create_user(&pool, &payload.username, &hashed_password)
        .await
        .map_err(|_| AppError::Database("Could not create user, please try again".into()))?;

    Ok((StatusCode::CREATED, "User created"))
}

pub async fn login(
    auth: AuthSession<User, i64, SessionSqlitePool, SqlitePool>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_user(&pool, &payload.username)
        .await
        .map_err(|_| AppError::NotFound(format!("'{}' is not registered", payload.username)))?;

    if verify(&payload.password, &user.password).unwrap_or(false) {
        auth.login_user(user.id as i64);
        Ok((StatusCode::OK, "Login successful"))
    } else {
        Err(AppError::NotFound("Incorrect password".into()))
    }
}
pub async fn log_out(
    auth: AuthSession<User, i64, SessionSqlitePool, SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    auth.logout_user();
    Ok((StatusCode::OK, "Logged out successfully").into_response())
}

pub async fn protected(Extension(user): Extension<User>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("Hello {}, your id is {}", user.username, user.id),
    )
}
