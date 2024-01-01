use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::ASSETS;

pub async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "index.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (
            StatusCode::OK,
            headers,
            ASSETS
                .get_file("index.css")
                .unwrap()
                .contents_utf8()
                .unwrap(),
        )
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}
