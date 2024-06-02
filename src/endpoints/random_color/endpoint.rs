use super::model::RandomColorResponse;
use axum::response::Json;

pub async fn random_color() -> Json<RandomColorResponse> {
    Json(RandomColorResponse::new_random())
}
