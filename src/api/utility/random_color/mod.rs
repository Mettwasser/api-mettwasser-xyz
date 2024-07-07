use axum::response::Json;

use color_names::{rgb_to_color_name, COLOR_MAP};
use rand::seq::IteratorRandom;
use rand::thread_rng;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, PartialEq, PartialOrd, Serialize, ToSchema)]
pub struct RandomColorResponse {
    color_hex: String,
    preview_url: String,
    color_name: Option<&'static str>,
}

impl RandomColorResponse {
    pub fn new_random() -> Self {
        let random_color = COLOR_MAP.keys().choose(&mut thread_rng()).unwrap();
        let color_name = rgb_to_color_name(random_color);

        let color_hex: String = format!(
            "{:02x}{:02x}{:02x}",
            random_color[0], random_color[1], random_color[2]
        );

        Self {
            color_hex: format!("#{}", &color_hex),
            preview_url: format!(
                "https://api.mettwasser.xyz/image/colorpreview?hex={}",
                color_hex
            ),
            color_name,
        }
    }
}

#[utoipa::path(get, path = "/randomcolor", responses(
    (
        status = 200,
        body = inline(RandomColorResponse), 
        example = json!({
            "color_hex": "#6384b8",
            "preview_url": "https://api.mettwasser.xyz/image/colorpreview?hex=6384b8",
            "color_name": "Marine Ink"
        })
    )
))]
pub async fn random_color() -> Json<RandomColorResponse> {
    Json(RandomColorResponse::new_random())
}
