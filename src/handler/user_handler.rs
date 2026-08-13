use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    errors::{AppError, Result},
    handler::auth_handler::generate_token,
    models::JsonRes,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginReq {
    username: String,
    password: String,
}

pub async fn login(Json(req): Json<LoginReq>) -> Result<JsonRes<String>> {
    if !((req.username == "admin") && (req.password == "123456")) {
        return Err(AppError::Message("username or password error".into()));
    }

    let token = generate_token(&req.username, "axum-teamplate", 30)?;

    Ok(JsonRes::new(200, true, "ok", Some(token)))
}
