use axum::{routing::get, Router};
pub mod random_color;
use super::image::captcha;

mod docs {
    use super::random_color::*;
    use crate::api::image::captcha::*;
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(paths(random_color, generate_captcha_response))]
    pub struct UtilityDocs;
}

pub use docs::UtilityDocs;

// Starts with `/utility/{...}`
pub fn router() -> Router {
    Router::new()
        .route("/randomcolor", get(random_color::random_color))
        .route("/captcha", get(captcha::generate_captcha_response))
}
