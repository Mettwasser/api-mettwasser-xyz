use crate::error::ApiError;
use axum::extract::rejection::QueryRejection;
use axum::{extract::Query, Json};
use captcha_rs::CaptchaBuilder;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;

mod defaults {
    use rand::Rng;

    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ\
    abcdefghijkmnpqrstuvwxyz\
    23456789";

    pub fn difficulty() -> u32 {
        5
    }

    pub fn captcha_text() -> String {
        let mut rng = rand::thread_rng();
        (0..=5)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
pub struct CaptchaQueryParams {
    #[serde(default = "defaults::difficulty")]
    pub difficulty: u32,

    #[serde(default = "defaults::captcha_text")]
    pub text: String,

    #[serde(default)]
    #[serde(rename = "darkMode")]
    pub dark_mode: bool,
}

#[derive(Serialize)]
pub struct CaptchaResponse {
    pub solution: String,
    pub base64_image: String,
}

pub async fn generate_captcha_response(
    query: StdResult<Query<CaptchaQueryParams>, QueryRejection>,
) -> StdResult<Json<CaptchaResponse>, Json<ApiError>> {
    let Query(captcha_params) = query.map_err(ApiError::from)?;

    if !(1..=10).contains(&captcha_params.difficulty) {
        return Err(Json(
            ("The difficulty must be in between 1 and 10.", 400).into(),
        ));
    }

    if captcha_params.text.len() <= 5 {
        return Err(Json(ApiError::bad_arguments(
            "Captcha text length has to 5 or less.",
        )));
    }

    let captcha_text = captcha_params.text.clone();

    let captcha = tokio::task::spawn_blocking(move || {
        CaptchaBuilder::new()
            .compression(30)
            .text(captcha_text)
            .complexity(captcha_params.difficulty)
            .dark_mode(captcha_params.dark_mode)
            .width(160)
            .height(40)
            .build()
    })
    .await
    .map_err(|_err| Json(ApiError::internal_server_error("Something went wrong")))?;

    Ok(Json(CaptchaResponse {
        solution: captcha_params.text,
        base64_image: captcha.to_base64(),
    }))
}
