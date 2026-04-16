use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User not found")]
    NotFound,

    #[error("Username already exists")]
    Conflict,

    #[error("Database error")]
    Database,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "User not found").into_response(),

            AppError::Conflict => {
                (StatusCode::BAD_REQUEST, "Username already exists").into_response()
            }

            AppError::Database => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
