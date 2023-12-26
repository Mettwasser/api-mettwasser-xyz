use axum::{
    extract::Path,
    response::Html,
    routing::{get, post},
    Router,
};

use mettwasser_xyz::endpoints::round_image;

use mettwasser_xyz::router;

#[tokio::main]
async fn main() {
    let app = router! {
        "/image/round/:image_url" => round_image GET,
    };

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
