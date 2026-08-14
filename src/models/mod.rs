pub mod user;
pub use user::*;

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

/// HTTP status code used for success responses.
pub const SUCCESS_CODE: i32 = 200;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRes<T> {
    code: i32,
    success: bool,
    msg: String,
    /// Payload — `None` for "no data" responses (e.g. errors).
    data: Option<T>,
}

impl<T> JsonRes<T> {
    /// Lower-level constructor. Prefer [`JsonRes::success`] / [`JsonRes::fail`]
    /// for the common success / failure shapes.
    pub fn new(code: i32, success: bool, msg: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code,
            success,
            msg: msg.into(),
            data,
        }
    }

    /// Successful response carrying `data` with status `200` and message `"ok"`.
    pub fn success_with_data(data: T) -> Self {
        Self::new(SUCCESS_CODE, true, "ok", Some(data))
    }

    /// Successful response carrying `data` with a custom message and status `200`.
    pub fn success_with_msg_data(msg: impl Into<String>, data: T) -> Self {
        Self::new(SUCCESS_CODE, true, msg, Some(data))
    }
}

impl JsonRes<()> {
    /// Successful response without a payload (`data = null`), e.g. for
    /// `204 No Content`-style endpoints.
    pub fn success() -> Self {
        Self::new(SUCCESS_CODE, true, "ok", None)
    }

    /// Failure response with a custom code, message and no payload.
    pub fn fail(code: i32, msg: impl Into<String>) -> Self {
        Self::new(code, false, msg, None)
    }
}

impl<T: Serialize> IntoResponse for JsonRes<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
