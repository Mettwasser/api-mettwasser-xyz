pub mod preview_size;

use super::hex_color::HexColor;
use crate::{error::ApiError, extract::Query};
use axum::{http::header, response::AppendHeaders};
use image::ImageFormat;
use preview_size::PreviewSize;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use utoipa::IntoParams;

mod defaults {
    use super::preview_size::PreviewSize;

    #[inline(always)]
    pub fn preview_size() -> PreviewSize {
        PreviewSize::Small
    }
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PreviewColorQueryParams {
    hex: String,

    #[serde(default = "defaults::preview_size")]
    size: PreviewSize,
}

impl From<PreviewColorQueryParams> for (HexColor, PreviewSize) {
    fn from(val: PreviewColorQueryParams) -> Self {
        (HexColor::from(val.hex), val.size)
    }
}

#[utoipa::path(
    get,
    path = "/colorpreview", 
    params(PreviewColorQueryParams),
    responses(
        (status = 200, content_type = "image/png", description = "The raw image")
    )
)]
pub async fn preview_color(
    Query(params): Query<PreviewColorQueryParams>,
) -> Result<
    (
        AppendHeaders<[(header::HeaderName, &'static str); 1]>,
        Vec<u8>,
    ),
    ApiError,
> {
    let (hex, prevsize) = params.into();
    let img = hex.into_preview(prevsize);

    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;

    Ok((AppendHeaders([(header::CONTENT_TYPE, "image/png")]), buffer))
}
