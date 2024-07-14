pub mod captcha;
pub mod image_round;
pub mod preview_color;
use axum::{routing::get, Router};
pub use captcha::{generate_captcha_image, generate_captcha_response};
use dominant_colors::dominant_colors;
pub use image_round::round_image;
pub use preview_color::preview_color;
mod dominant_colors;
mod hex_color;

mod docs {
    use super::{captcha::*, dominant_colors::*, image_round::*, preview_color::*};
    use preview_size::PreviewSize;
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(
        paths(preview_color, generate_captcha_image, round_image, dominant_colors),
        components(schemas(PreviewSize))
    )]
    pub struct ImageDocs;
}

pub use docs::ImageDocs;

// Starts with `/image/{...}`
pub fn router() -> Router {
    Router::new()
        .route("/gen_captcha", get(generate_captcha_image))
        .route("/round", get(round_image))
        .route("/colorpreview", get(preview_color))
        .route("/dominant_colors", get(dominant_colors))
}
