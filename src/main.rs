use mettwasser_xyz::assets::assets;
use mettwasser_xyz::docs::{docs, docs_internal};
use mettwasser_xyz::endpoints::home;
use mettwasser_xyz::endpoints::round_image;
use mettwasser_xyz::router;

#[cfg(debug_assertions)]
const HOST_IP: &str = "127.0.0.1:3000";

#[cfg(not(debug_assertions))]
const HOST_IP: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app = router! {
        // assets endpoints
        "/assets/*path" => assets GET,

        // documentation endpoints
        "/docs/mettwasser.xyz" => docs_internal GET, // "internal" endpoint (string representation for the SwaggerUI Template - needs an actual url)
        "/docs" => docs GET,

        // other endpoints
        "/" => home GET,
        "/image/round" => round_image GET,
    };

    let listener = tokio::net::TcpListener::bind(HOST_IP).await.unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
