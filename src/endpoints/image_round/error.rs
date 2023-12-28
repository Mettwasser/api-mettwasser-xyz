use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[derive(Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct RoundQueryParamError {
    query_params: [u32; 4],
    message: String,
    code: u16,
}

impl RoundQueryParamError {
    pub fn new(query_params: [u32; 4], message: String) -> Self {
        Self {
            query_params,
            message,
            code: 400,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RoundError {
    GeneralError(ApiError),
    QueryParamError(RoundQueryParamError),
}
