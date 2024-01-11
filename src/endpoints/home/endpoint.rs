use crate::{GetFileContentsFromDir, TEMPLATES};
use axum::response::Html;

pub async fn home() -> Html<&'static str> {
    Html(TEMPLATES.get_str("index.html"))
}
