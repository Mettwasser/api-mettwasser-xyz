use std::io::Cursor;

use axum::{extract::Query, http::header, response::AppendHeaders, Json};
use image::ImageFormat;

use crate::error::{ApiError, IntoApiError};

use super::logic::PreviewColorQueryParams;

pub async fn preview_color(
    Query(params): Query<PreviewColorQueryParams>,
) -> Result<
    (
        AppendHeaders<[(header::HeaderName, &'static str); 1]>,
        Vec<u8>,
    ),
    Json<ApiError>,
> {
    let (hex, prevsize) = params.into();
    let img = hex.into_preview(prevsize);

    let mut buffer: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(|err| err.into_api_error())?;

    Ok((AppendHeaders([(header::CONTENT_TYPE, "image/png")]), buffer))
}
