use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State};

use crate::{
    errors::{AppError, Result},
    handler::auth_handler::{AuthUser, generate_token},
    models::{JsonRes, LoginReq, LoginRes, ProfileRes},
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<JsonRes<LoginRes>> {
    let row: Option<(String, String)> =
        sqlx::query_as("SELECT username, password FROM users WHERE username = ?")
            .bind(&req.username)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| AppError::Message(format!("database error: {e}")))?;

    let (username, password_hash) =
        row.ok_or_else(|| AppError::Message("Invalid username or password".into()))?;

    let valid: bool = tokio::task::spawn_blocking(move || -> std::result::Result<bool, String> {
        let parsed =
            PasswordHash::new(&password_hash).map_err(|e| format!("password hash: {e}"))?;
        Ok(Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed)
            .is_ok())
    })
    .await
    .map_err(|e| AppError::Message(format!("join error: {e}")))?
    .map_err(AppError::Message)?;

    if !valid {
        return Err(AppError::Message("Invalid username or password".into()));
    }

    let token = generate_token(&username, &state.config)?;
    Ok(JsonRes::success_with_data(LoginRes { token }))
}

pub async fn get_profile(user: AuthUser) -> Result<JsonRes<ProfileRes>> {
    Ok(JsonRes::success_with_data(ProfileRes {
        username: user.username,
    }))
}
