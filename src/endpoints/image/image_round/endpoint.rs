use super::logic::round;
use super::logic::RoundImageQueryParams;
use crate::error::{ApiError, IntoApiError};
use axum::{body::Bytes, extract::Query, http::header, response::AppendHeaders, Json};
use image::{io::Reader, ImageFormat};
use std::{io::Cursor, result::Result as StdResult};

type Result<T> = std::result::Result<T, ApiError>;

async fn fetch_image(url: &str) -> Result<Bytes> {
    reqwest::get(url)
        .await
        .map_err(|err| err.into_api_error())?
        .bytes()
        .await
        .map_err(|err| err.into_api_error())
}

async fn image_from_bytes(bytes: Bytes) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    Ok(Reader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|err| err.into_api_error())?
        .decode()
        .map_err(|err| err.into_api_error())?
        .to_rgba8())
}

pub async fn round_image(
    Query(round_image_params): Query<RoundImageQueryParams>,
) -> StdResult<
    (
        AppendHeaders<[(header::HeaderName, &'static str); 1]>,
        Vec<u8>,
    ),
    Json<ApiError>,
> {
    let bytes = fetch_image(&round_image_params.url).await?;

    let mut img = image_from_bytes(bytes).await?;

    round(&mut img, round_image_params)?;

    let mut buffer: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(|err| err.into_api_error())?;

    Ok((AppendHeaders([(header::CONTENT_TYPE, "image/png")]), buffer))
}
