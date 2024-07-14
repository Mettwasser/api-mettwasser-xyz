use crate::{
    error::ApiError,
    extract::{Json, Query},
    ApiResult,
};
use axum::{
    http::{header, StatusCode},
    response::AppendHeaders,
};
use captcha_rs::CaptchaBuilder;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use serde_default_utils::default_u32;
use std::io::Cursor;
use utoipa::{IntoParams, ToSchema};

mod defaults {
    use rand::Rng;

    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ\
    abcdefghijkmnpqrstuvwxyz\
    23456789";

    pub fn captcha_text() -> String {
        let mut rng = rand::thread_rng();
        (0..5)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CaptchaQueryParams {
    #[serde(default = "default_u32::<5>")]
    #[param(minimum = 1, maximum = 10, default = 5)]
    pub difficulty: u32,

    #[serde(default = "defaults::captcha_text")]
    #[param(required = false, min_length = 1, max_length = 5)]
    /// Defaults to a random string of length 5
    pub text: String,

    #[serde(default)]
    #[serde(rename = "darkMode")]
    #[param(default = false)]
    pub dark_mode: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GenCaptchaQueryParams {
    #[param(minimum = 1, maximum = 10)]
    pub difficulty: u32,

    #[param(required = true, min_length = 1, max_length = 5)]
    pub text: String,

    #[serde(rename = "darkMode")]
    pub dark_mode: bool,
}

#[derive(Serialize, ToSchema)]
pub struct CaptchaResponse {
    pub solution: String,
    pub url: String,
}

#[utoipa::path(
    get,
    path = "/captcha", 
    params(CaptchaQueryParams),
    responses(
        (status = 200, content_type = "image/png", description = "The raw image")
    )
)]
pub async fn generate_captcha_response(
    Query(captcha_params): Query<CaptchaQueryParams>,
) -> ApiResult<CaptchaResponse> {
    if !(1..=10).contains(&captcha_params.difficulty) {
        return Err(ApiError::Any(
            StatusCode::BAD_REQUEST,
            "The difficulty must be in between 1 and 10.".to_owned(),
        ));
    }

    if !(1..=5).contains(&captcha_params.text.len()) {
        return Err(ApiError::Any(
            StatusCode::BAD_REQUEST,
            "Captcha text length has to 5 or less.".to_owned(),
        ));
    }

    Ok(Json(CaptchaResponse {
        url: format!(
            "https://api.mettwasser.xyz/image/gen_captcha?text={}&difficulty={}&darkMode={}",
            &captcha_params.text, captcha_params.difficulty, captcha_params.dark_mode
        ),
        solution: captcha_params.text,
    }))
}

#[utoipa::path(
    get,
    path = "/gen_captcha",
    params(GenCaptchaQueryParams),
    responses(
        (status = 200, body = inline(CaptchaResponse))
    )
)]
pub async fn generate_captcha_image(
    Query(captcha_params): Query<GenCaptchaQueryParams>,
) -> Result<
    (
        AppendHeaders<[(header::HeaderName, &'static str); 1]>,
        Vec<u8>,
    ),
    ApiError,
> {
    if !(1..=10).contains(&captcha_params.difficulty) {
        return Err(ApiError::Any(
            StatusCode::BAD_REQUEST,
            "The difficulty must be in between 1 and 10.".to_owned(),
        ));
    }

    if !(1..=5).contains(&captcha_params.text.len()) {
        return Err(ApiError::Any(
            StatusCode::BAD_REQUEST,
            "Captcha text length has to 5 or less.".to_owned(),
        ));
    }

    let captcha = tokio::task::spawn_blocking(move || {
        CaptchaBuilder::new()
            .compression(30)
            .text(captcha_params.text)
            .complexity(captcha_params.difficulty)
            .dark_mode(captcha_params.dark_mode)
            .width(160)
            .height(40)
            .build()
    })
    .await
    .map_err(|_err| {
        ApiError::Any(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong during captcha generation.".to_owned(),
        )
    })?;

    let mut buffer: Vec<u8> = Vec::new();

    captcha
        .image
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(ApiError::from)?;

    Ok((AppendHeaders([(header::CONTENT_TYPE, "image/png")]), buffer))
}
