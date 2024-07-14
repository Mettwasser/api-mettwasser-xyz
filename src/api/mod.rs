pub mod utility;
// home

use axum::Router;
// Image
pub use image::generate_captcha_image;
pub use image::{generate_captcha_response, preview_color, round_image};
// Utility
pub use utility::random_color::random_color;
use utoipa::OpenApi;

mod image;

use image::ImageDocs;
use utility::UtilityDocs;

#[derive(OpenApi)]
// RELEASE
#[openapi(
    nest(
        (path = "/utility", api = UtilityDocs, tags = ["utility"]),
        (path = "/image", api = ImageDocs, tags = ["image"]),
    )
)]
// RELEASE
#[cfg_attr(not(debug_assertions), 
    openapi(
        servers(
            (url = "https://api.mettwasser.xyz")
        )
    )
)]
// DEBUG
#[cfg_attr(debug_assertions, 
    openapi(
        servers(
            (url = "http://127.0.0.1:3000"),
        )
    )
)]
pub struct ApiDocs;

pub fn router() -> Router {
    Router::new()
        .nest("/utility", utility::router())
        .nest("/image", image::router())
}
