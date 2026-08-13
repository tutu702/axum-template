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
    AuthError(#[from] AuthError),
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing token")]
    MissingToken,
    #[error("Token creation error")]
    TokenCration(String),
    #[error("Token has expired")]
    TokenExpired,
    #[error("Invalid token")]
    InvalidToken(String),
    #[error("Token expiration calculation overflowed")]
    TokenExpirationOverflow,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            // AuthError 子错误
            AppError::AuthError(AuthError::MissingToken) => {
                (StatusCode::UNAUTHORIZED, "missing token".to_string())
            }
            AppError::AuthError(AuthError::TokenExpired) => {
                (StatusCode::UNAUTHORIZED, "token has expired".to_string())
            }
            AppError::AuthError(AuthError::InvalidToken(_)) => {
                (StatusCode::UNAUTHORIZED, "invalid token".to_string())
            }
            AppError::AuthError(AuthError::TokenCration(e)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("token creation error: {e}"),
            ),
            AppError::AuthError(AuthError::TokenExpirationOverflow) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "token expiration overflow".to_string(),
            ),

            // 通用业务消息
            AppError::Message(m) => (StatusCode::BAD_REQUEST, m.clone()),
        };

        let body = JsonRes::new(status.as_u16() as i32, false, msg, None::<()>);
        (status, Json(body)).into_response()
    }
}
