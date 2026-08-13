use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRes<T> {
    code: i32,
    success: bool,
    msg: String,
    // 返回数据(业务接口定义具体数据结构)
    data: Option<T>,
}

impl<T> JsonRes<T> {
    pub fn new(code: i32, success: bool, msg: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code,
            success,
            msg: msg.into(),
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for JsonRes<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
