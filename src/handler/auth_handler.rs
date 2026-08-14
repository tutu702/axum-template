use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{HeaderValue, request::Parts},
};
use chrono::Utc;
use headers::{Authorization, Header as _, authorization::Bearer};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{
    config::AppConfig,
    errors::{AppError, AuthError},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub username: String,
    pub exp: i64,
}

/// Authenticated principal extracted from the `Authorization: Bearer …` header.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub username: String,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Arc<AppConfig>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cfg = Arc::<AppConfig>::from_ref(state);

        let auth_value: &HeaderValue = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or(AuthError::MissingToken)?;

        let header = Authorization::<Bearer>::decode(&mut std::iter::once(auth_value))
            .map_err(|_| AuthError::MissingToken)?;

        let claims = decode::<TokenClaims>(
            header.token(),
            &DecodingKey::from_secret(cfg.auth.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AuthError::InvalidToken(e.to_string()))?
        .claims;

        Ok(Self {
            username: claims.username,
        })
    }
}

pub(crate) fn generate_token(username: &str, cfg: &AppConfig) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now
        .checked_add_signed(chrono::Duration::minutes(cfg.auth.expire_minutes))
        .ok_or(AuthError::TokenExpirationOverflow)?
        .timestamp();

    let token = encode(
        &Header::default(),
        &TokenClaims {
            username: username.into(),
            exp,
        },
        &EncodingKey::from_secret(cfg.auth.secret.as_bytes()),
    )
    .map_err(|e| AuthError::TokenCreation(e.to_string()))?;

    Ok(token)
}
