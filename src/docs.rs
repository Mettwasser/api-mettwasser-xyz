use axum::response::Html;
use axum_swagger_ui::swagger_ui;

pub async fn docs_internal() -> &'static str {
    include_str!("../docs/OpenAPI.yml")
}

pub async fn docs() -> Html<String> {
    Html(swagger_ui("docs/api.mettwasser.xyz"))
}
