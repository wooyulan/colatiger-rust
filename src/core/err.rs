use snafu::prelude::*;
use axum::response::IntoResponse;
use crate::Response;

#[derive(Debug, Snafu)]
#[snafu(whatever, display("Error was: {message}"))]
pub struct Error {
    pub message: String,
    #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
    pub source: Option<Box<dyn std::error::Error>>,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Response::<()>::fail(0, &self.message)
            .to_json()
            .into_response()
    }
}



