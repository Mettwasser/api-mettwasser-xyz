pub(super) mod logic;

use crate::error::ApiError;
use crate::extract::Query;
use crate::utils::{fetch_raw_image, image_from_bytes};
use axum::{http::header, response::AppendHeaders};
use image::ImageFormat;
use logic::round;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use std::{io::Cursor, result::Result as StdResult};

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
    Query(round_image_params): Query<RoundImageQueryParams>,
) -> StdResult<
    (
        AppendHeaders<[(header::HeaderName, &'static str); 1]>,
        Vec<u8>,
    ),
    ApiError,
> {
    let bytes = fetch_raw_image(&round_image_params.url).await?;

    let mut img = image_from_bytes(bytes)?;

    round(&mut img, round_image_params)?;

    let mut buffer: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;

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
