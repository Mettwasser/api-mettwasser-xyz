pub(super) mod logic;

use crate::error::ApiError;
use axum::extract::rejection::QueryRejection;
use axum::{body::Bytes, extract::Query, http::header, response::AppendHeaders};
use image::{io::Reader, ImageFormat};
use logic::round;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use std::{io::Cursor, result::Result as StdResult};

async fn fetch_image(url: &str) -> Result<Bytes, ApiError> {
    Ok(reqwest::get(url).await?.bytes().await?)
}

fn image_from_bytes(
    bytes: Bytes,
) -> Result<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, ApiError> {
    Ok(Reader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?
        .to_rgba8())
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct RoundImageQueryParams {
    /// The URL to the image that should be rounded
    pub url: String,

    #[serde(default = "defaults::auto")]
    /// Whether the API tries to figure out the max. radius on its own. This means if width and height are the same, you'll get a perfectly round image. This will override everything else
    pub auto: bool,

    #[serde(default = "defaults::radius")]
    /// The radius to use when rounding the corners. Specifying specific corners overrides this for said corner.
    corner_radius: u32,

    top_left: Option<u32>,

    top_right: Option<u32>,

    bottom_left: Option<u32>,

    bottom_right: Option<u32>,
}

impl RoundImageQueryParams {
    pub fn top_left(&self) -> u32 {
        self.top_left.unwrap_or(self.corner_radius)
    }

    pub fn top_right(&self) -> u32 {
        self.top_right.unwrap_or(self.corner_radius)
    }

    pub fn bottom_left(&self) -> u32 {
        self.bottom_left.unwrap_or(self.corner_radius)
    }

    pub fn bottom_right(&self) -> u32 {
        self.bottom_right.unwrap_or(self.corner_radius)
    }

    pub fn list_corners(&self) -> (u32, u32, u32, u32) {
        (
            self.top_left(),
            self.top_right(),
            self.bottom_left(),
            self.bottom_right(),
        )
    }
}

#[utoipa::path(
    get,
    path = "/round",
    params(RoundImageQueryParams),
    responses(
        (status = 200, content_type = "image/png", description = "The raw image")
    )
)]
pub async fn round_image(
    query: std::result::Result<Query<RoundImageQueryParams>, QueryRejection>,
) -> StdResult<
    (
        AppendHeaders<[(header::HeaderName, &'static str); 1]>,
        Vec<u8>,
    ),
    ApiError,
> {
    let Query(round_image_params) = query.map_err(ApiError::from)?;
    let bytes = fetch_image(&round_image_params.url).await?;

    let mut img = image_from_bytes(bytes)?;

    round(&mut img, round_image_params)?;

    let mut buffer: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(ApiError::from)?;

    Ok((AppendHeaders([(header::CONTENT_TYPE, "image/png")]), buffer))
}

mod defaults {
    #[inline(always)]
    pub fn radius() -> u32 {
        3
    }

    #[inline(always)]
    pub fn auto() -> bool {
        false
    }
}
