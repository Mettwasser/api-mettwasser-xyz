use std::io::Cursor;

use axum::{
    extract::{rejection::QueryRejection, Query},
    http::header,
    response::AppendHeaders,
    Json,
};
use image::ImageFormat;
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ToApiErrorResult};

use super::{hex_color::HexColor, preview_size::PreviewSize};

mod defaults {
    #[inline(always)]
    pub fn preview_size() -> u8 {
        1
    }
}

#[derive(Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct PreviewColorQueryParams {
    hex: String,

    #[serde(default = "defaults::preview_size")]
    size: u8,
}

impl From<PreviewColorQueryParams> for (HexColor, PreviewSize) {
    fn from(val: PreviewColorQueryParams) -> Self {
        (HexColor::from(val.hex), PreviewSize::from(val.size))
    }
}

pub async fn preview_color(
    query: Result<Query<PreviewColorQueryParams>, QueryRejection>,
) -> Result<
    (
        AppendHeaders<[(header::HeaderName, &'static str); 1]>,
        Vec<u8>,
    ),
    Json<ApiError>,
> {
    let Query(params) = query.to_api_error_result()?;
    let (hex, prevsize) = params.into();
    let img = hex.into_preview(prevsize);

    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(ApiError::from)?;

    Ok((AppendHeaders([(header::CONTENT_TYPE, "image/png")]), buffer))
}
