use axum::response::IntoResponse;
use crate::Response;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(message: String, cause: Option<Box<dyn std::error::Error>>) -> Self {
        Self {
            message,
            cause,
        }
    }

    pub fn with_cause(cause: Box<dyn std::error::Error>) -> Self {
        Self::new(cause.to_string(), Some(cause))
    }

}



impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Response::<()>::fail(0, &self.message)
        .to_json()
        .into_response()
    }
}



impl From<milvus::error::Error> for Error {
    fn from(e: milvus::error::Error) -> Self {
        Self::with_cause(Box::new(e))
    }
}