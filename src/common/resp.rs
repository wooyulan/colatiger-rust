//! 自定义响应状态码
//!
use std::fmt::Debug;
use serde::Serialize;
use axum::{
    body::{self, Full},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

pub const CODE_SUCCESS: StatusCode = StatusCode::OK;
pub const CODE_FAIL: StatusCode = StatusCode::BAD_REQUEST;

#[derive(Debug, Serialize, Default)]
pub struct RespVO<T: Serialize> {
    pub error_code: Option<u16>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}

/// 填入到extensions中的数据
#[derive(Debug)]
pub struct ResJsonString(pub String);

impl<T> RespVO<T>
    where
        T: Serialize
{
    pub fn new(error_code: u16, status: &str, message: &str, data: T) -> Self {
        Self {
            error_code: Some(error_code),
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
            error_code: Some(CODE_SUCCESS.as_u16()),
            status: Some("ok".to_string()),
            message: Some("success".to_string()),
            data: None,
        }
    }

    pub fn fail(error_code: u16, message: &str) -> Self {
        Self {
            error_code: Some(error_code),
            status: Some("fail".to_string()),
            message: Some(message.to_string()),
            data: None,
        }
    }

    pub fn fail_info(message: &str) -> Self {
        Self {
            error_code: Some(CODE_FAIL.as_u16()),
            status: Some("fail".to_string()),
            message: Some(message.to_string()),
            data: None,
        }
    }
}


impl<T> IntoResponse for RespVO<T>
    where T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            error_code: self.error_code,
            status: self.status,
            message: self.message,
            data: self.data,
        };

        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()))
                    .body(body::boxed(Full::from(e.to_string())))
                    .unwrap();
            }
        };
        let res_json_string = ResJsonString(json_string.clone());
        let mut response = json_string.into_response();
        response.extensions_mut().insert(res_json_string);
        response
    }
}
