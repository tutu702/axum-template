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
    let cfg = state.config;
    if req.username != cfg.auth.username || req.password != cfg.auth.password {
        return Err(AppError::Message("Invalid username or password".into()));
    }

    let token = generate_token(&req.username, &cfg)?;
    Ok(JsonRes::success_with_data(LoginRes { token }))
}

pub async fn get_profile(user: AuthUser) -> Result<JsonRes<ProfileRes>> {
    Ok(JsonRes::success_with_data(ProfileRes {
        username: user.username,
    }))
}
