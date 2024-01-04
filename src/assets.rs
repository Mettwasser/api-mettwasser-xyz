use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::ASSETS;
use crate::BUILD;

pub async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "index.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (
            StatusCode::OK,
            headers,
            BUILD
                .get_file("index.css")
                .unwrap()
                .contents_utf8()
                .unwrap(),
        )
    } else if path == "index.js" {
        headers.insert(header::CONTENT_TYPE, "application/javascript".parse().unwrap());
        (
            StatusCode::OK,
            headers,
            ASSETS
                .get_file("scripts/index.js")
                .unwrap()
                .contents_utf8()
                .unwrap(),
        ) 
    } else if path == "typewriter.js" {
        headers.insert(header::CONTENT_TYPE, "application/javascript".parse().unwrap());
        (
            StatusCode::OK,
            headers,
            ASSETS
                .get_file("scripts/typewriter.js")
                .unwrap()
                .contents_utf8()
                .unwrap(),
        )  
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}
