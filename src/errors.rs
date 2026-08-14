use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::models::JsonRes;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing token")]
    MissingToken,
    #[error("Token creation error: {0}")]
    TokenCreation(String),
    #[error("Token has expired")]
    TokenExpired,
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Token expiration calculation overflowed")]
    TokenExpirationOverflow,
}

impl AuthError {
    /// HTTP status code that this auth error should surface as.
    const fn status(&self) -> StatusCode {
        match self {
            Self::MissingToken | Self::TokenExpired | Self::InvalidToken(_) => {
                StatusCode::UNAUTHORIZED
            }
            Self::TokenCreation(_) | Self::TokenExpirationOverflow => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            AppError::Auth(err) => (
                err.status(),
                JsonRes::fail(err.status().as_u16() as i32, err.to_string()),
            ),
            AppError::Message(msg) => (
                StatusCode::BAD_REQUEST,
                JsonRes::fail(StatusCode::BAD_REQUEST.as_u16() as i32, msg.clone()),
            ),
        };
        (status, Json(body)).into_response()
    }
}
