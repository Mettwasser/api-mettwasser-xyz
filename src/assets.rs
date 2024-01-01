use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

static GLOBAL_CSS: &str = include_str!("../assets/global.css");

pub async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "global.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, GLOBAL_CSS)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}
