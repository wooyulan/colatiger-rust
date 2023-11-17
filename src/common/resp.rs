//! 自定义响应状态码
//!
use serde:: Serialize;
use axum::http::StatusCode;

pub const CODE_SUCCESS: StatusCode = StatusCode::OK;
pub const CODE_FAIL: StatusCode = StatusCode::BAD_REQUEST;

#[derive(Debug, Serialize)]
pub struct RespVO<T: Serialize> {
    pub error_code: Option<u16>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize
{
    pub fn new(error_code: u16, status: &str, message: &str, data:T) -> Self {
        Self {
            error_code:Some(error_code),
            status: Some(status.to_string()),
            message: Some(message.to_string()),
            data: Some(data),
        }
    }
    pub fn ok(data: T) -> Self {
        Self::new(CODE_SUCCESS.as_u16(), "ok", "success", data)
    }


    pub fn ok_empty() -> Self {
        Self {
            error_code:Some(CODE_SUCCESS.as_u16()),
            status: Some("ok".to_string()),
            message: Some("success".to_string()),
            data: None
        }
    }

    pub fn fail(error_code: u16, message: &str) -> Self {
        Self {
            error_code:Some(error_code),
            status: Some("fail".to_string()),
            message: Some(message.to_string()),
            data: None
        }
    }

    pub fn fail_info(message: &str) -> Self {
        Self {
            error_code:Some(CODE_FAIL.as_u16()),
            status: Some("fail".to_string()),
            message: Some(message.to_string()),
            data: None
        }
    }
}

