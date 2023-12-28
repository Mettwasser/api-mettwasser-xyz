use mettwasser_xyz::endpoints::round_image;

use mettwasser_xyz::router;

#[tokio::main]
async fn main() {
    let app = router! {
        "/image/round" => round_image GET,
    };

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
