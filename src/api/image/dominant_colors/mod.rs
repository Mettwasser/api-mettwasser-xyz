use {
    crate::{
        error::ApiError,
        extract::{Json, Query},
        utils::{fetch_raw_image, image_from_bytes, rgb_to_hex},
    },
    image::Pixel,
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    utoipa::{IntoParams, ToSchema},
};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DominantColorQueryParams {
    url: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DominantColorEntry {
    color: String,
    color_name: Option<&'static str>,
    pixels_counted: u32,
}

#[utoipa::path(
    get,
    path = "/dominant_colors", 
    params(DominantColorQueryParams),
    responses(
        (status = 200, description = "The pixels of the image (ordered by most-dominant)", body = inline(DominantColorEntry))
    )
)]
pub async fn dominant_colors(
    Query(query_params): Query<DominantColorQueryParams>,
) -> Result<Json<Vec<DominantColorEntry>>, ApiError> {
    let raw_img = fetch_raw_image(&query_params.url).await?;
    let img = image_from_bytes(raw_img)?;

    let mut color_count: HashMap<[u8; 3], u32> = HashMap::new();

    for pixel in img.pixels() {
        let rgb = pixel.to_rgb();
        *color_count.entry(rgb.0).or_insert(0) += 1;
    }

    let mut dominant_colors: Vec<DominantColorEntry> = Vec::with_capacity(color_count.len());

    for (rgb, count) in color_count {
        let dominant_color = DominantColorEntry {
            color: rgb_to_hex(&rgb),
            color_name: color_names::rgb_to_color_name(&rgb),
            pixels_counted: count,
        };
        dominant_colors.push(dominant_color);
    }

    dominant_colors.sort_by(|a, b| b.pixels_counted.cmp(&a.pixels_counted));

    Ok(Json(dominant_colors))
}
