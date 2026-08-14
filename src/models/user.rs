use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRes {
    pub token: String,
}
