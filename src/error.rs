use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
use image::ImageError;
use serde::Serialize;
use std::io;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl ErrorResponse {
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ApiError {
    QueryRejection(#[from] QueryRejection),
    JsonRejection(#[from] JsonRejection),
    ImageError(#[from] ImageError),
    Io(#[from] io::Error),
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    FetchError(String),
    #[error("{1}")]
    Any(StatusCode, String),
    #[error("{1}")]
    AnyStatic(StatusCode, &'static str),
}

impl ApiError {
    pub const INTERNAL_SERVER_ERROR: ApiError =
        ApiError::AnyStatic(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong");
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use ApiError::*;
        let (code, msg) = match self {
            QueryRejection(err) => (StatusCode::BAD_REQUEST, err.body_text()),
            JsonRejection(err) => (StatusCode::BAD_REQUEST, err.body_text()),
            ImageError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Io(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.kind().to_string()),
            Reqwest(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            FetchError(msg) => (StatusCode::BAD_REQUEST, msg),
            Any(code, msg) => (code, msg),
            AnyStatic(code, msg) => (code, msg.to_owned()),
        };

        (code, axum::Json(ErrorResponse::new(code.as_u16(), msg))).into_response()
    }
}
