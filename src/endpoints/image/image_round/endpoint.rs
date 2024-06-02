use super::logic::round;
use super::logic::RoundImageQueryParams;
use crate::error::ApiError;
use axum::extract::rejection::QueryRejection;
use axum::{body::Bytes, extract::Query, http::header, response::AppendHeaders, Json};
use image::{io::Reader, ImageFormat};
use std::{io::Cursor, result::Result as StdResult};

async fn fetch_image(url: &str) -> Result<Bytes, ApiError> {
    reqwest::get(url)
        .await
        .map_err(ApiError::from)?
        .bytes()
        .await
        .map_err(ApiError::from)
}

async fn image_from_bytes(
    bytes: Bytes,
) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, ApiError> {
    Ok(Reader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(ApiError::from)?
        .decode()
        .map_err(ApiError::from)?
        .to_rgba8())
}

pub async fn round_image(
    query: std::result::Result<Query<RoundImageQueryParams>, QueryRejection>,
) -> StdResult<
    (
        AppendHeaders<[(header::HeaderName, &'static str); 1]>,
        Vec<u8>,
    ),
    Json<ApiError>,
> {
    let Query(round_image_params) = query.map_err(ApiError::from)?;
    let bytes = fetch_image(&round_image_params.url).await?;

    let mut img = image_from_bytes(bytes).await?;

    round(&mut img, round_image_params)?;

    let mut buffer: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(ApiError::from)?;

    Ok((AppendHeaders([(header::CONTENT_TYPE, "image/png")]), buffer))
}
