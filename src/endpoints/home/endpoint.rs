use crate::TEMPLATES;
use axum::response::Html;

pub async fn home() -> Html<&'static str> {
    Html(
        TEMPLATES
            .get_file("index.html")
            .unwrap()
            .contents_utf8()
            .unwrap(),
    )
}
