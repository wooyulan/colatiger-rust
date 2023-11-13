//! 自定义响应状态码
//!
use serde::Serialize;
use axum::Json;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub error_code: i32,
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub fn new(error_code: i32, status: &str, message: &str, data: Option<T>) -> Self {
        Self {
            error_code,
            status: status.to_string(),
            message: message.to_string(),
            data,
        }
    }
    pub fn ok(data: Option<T>) -> Self {
        Self::new(200, "ok", "success", data)
    }

    pub fn fail(error_code: i32, message: &str) -> Self {
        Self::new(error_code, "fail", message, None)
    }

    pub fn to_json(self) -> JsonResult<T> {
        Json(self)
    }

}

pub type JsonResult<T> = Json<Response<T>>;