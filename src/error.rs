use std::{error::Error, fmt};

use axum::{extract::rejection::QueryRejection, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
}

impl Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ApiError occurred with message `{}` and status code `{}`.",
            self.message, self.code
        )
    }
}

impl ApiError {
    pub fn new(message: impl Into<String>, code: u16) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }

    pub fn bad_arguments(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: 400,
        }
    }
    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: 500,
        }
    }
}

impl From<ApiError> for String {
    fn from(value: ApiError) -> Self {
        value.message
    }
}

impl From<(String, u16)> for ApiError {
    fn from((message, code): (String, u16)) -> Self {
        Self { message, code }
    }
}

impl From<(&'static str, u16)> for ApiError {
    fn from((message, code): (&'static str, u16)) -> Self {
        Self {
            message: message.to_string(),
            code,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        (
            format!("Error from requested URL: {}", value),
            value.status().map_or(500, |status| status.as_u16()),
        )
            .into()
    }
}

impl From<image::ImageError> for ApiError {
    fn from(value: image::ImageError) -> Self {
        (
            format!("Error trying to pass image: {}", value),
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        )
            .into()
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        (
            format!("Error occurred: {}", value),
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        )
            .into()
    }
}

pub trait ToApiErrorResult<T> {
    fn to_api_error_result(self) -> Result<T, ApiError>;
}

impl<T, E> ToApiErrorResult<T> for Result<T, E>
where
    E: Into<ApiError>,
{
    fn to_api_error_result(self) -> Result<T, ApiError> {
        self.map_err(|err| err.into())
    }
}

impl From<QueryRejection> for ApiError {
    fn from(value: QueryRejection) -> Self {
        match value {
            QueryRejection::FailedToDeserializeQueryString(s) => (
                format!("{} (Bad Arguments)", s.body_text()),
                StatusCode::BAD_REQUEST.as_u16(),
            )
                .into(),
            _ => ("Internal Server Error: <QueryRejection>", 500).into(),
        }
    }
}
