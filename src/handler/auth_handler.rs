use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::errors::{AuthError, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    username: String,
    exp: i64,
}

pub(crate) fn generate_token(username: &str, secret: &str, expire: usize) -> Result<String> {
    let now = Utc::now();
    let exp = now
        .checked_add_signed(chrono::Duration::minutes(expire as i64))
        .ok_or(AuthError::TokenExpirationOverflow)?
        .timestamp();

    let claims = TokenClaims {
        username: username.into(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AuthError::TokenCration(e.to_string()))?;

    Ok(token)
}
