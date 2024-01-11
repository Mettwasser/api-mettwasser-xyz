use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::BUILD;
use crate::{GetFileContentsFromDir, ASSETS};

pub async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    match path.as_str() {
        "index.css" => {
            headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
            (StatusCode::OK, headers, BUILD.get_str("index.css"))
        }
        "index.js" => {
            headers.insert(
                header::CONTENT_TYPE,
                "application/javascript".parse().unwrap(),
            );
            (StatusCode::OK, headers, ASSETS.get_str("scripts/index.js"))
        }
        "typewriter.js" => {
            headers.insert(
                header::CONTENT_TYPE,
                "application/javascript".parse().unwrap(),
            );
            (
                StatusCode::OK,
                headers,
                ASSETS.get_str("scripts/typewriter.js"),
            )
        }
        _ => (StatusCode::NOT_FOUND, headers, ""),
    }
}
