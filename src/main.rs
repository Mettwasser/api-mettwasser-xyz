use {api_mettwasser_xyz::api::ApiDocs, axum::Router, utoipa_swagger_ui::SwaggerUi};
use {tower_http::services::ServeDir, utoipa::OpenApi};

#[cfg(debug_assertions)]
const HOST_IP: &str = "127.0.0.1:3000";

#[cfg(not(debug_assertions))]
const HOST_IP: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(api_mettwasser_xyz::router())
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/build", ServeDir::new("build"))
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDocs::openapi()));

    let listener = tokio::net::TcpListener::bind(HOST_IP).await.unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
