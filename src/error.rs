use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
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

pub trait IntoApiError {
    fn into_api_error(self) -> ApiError;
}

impl IntoApiError for reqwest::Error {
    fn into_api_error(self) -> ApiError {
        (
            format!("Error from requested URL: {}", self),
            self.status().map_or(500, |status| status.as_u16()),
        )
            .into()
    }
}
