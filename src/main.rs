use api_mettwasser_xyz::{
    docs::{docs, docs_internal},
    endpoints::{
        generate_captcha_image, generate_captcha_response, home, preview_color, random_color,
        round_image,
    },
    router,
};

#[cfg(debug_assertions)]
const HOST_IP: &str = "127.0.0.1:3000";

#[cfg(not(debug_assertions))]
const HOST_IP: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app = router! {
        ROUTES:
            // documentation endpoints
            "/docs/api.mettwasser.xyz" => docs_internal GET, // "internal" endpoint (string representation for the SwaggerUI Template - needs an actual url)
            "/docs" => docs GET,

            // other endpoints
            "/" => home GET,

            // image endpoints
            "/image/round" => round_image GET,
            "/image/colorpreview" => preview_color GET,
            "/image/captcha" => generate_captcha_image GET,
            "/captcha" => generate_captcha_response GET,

            // utility endpoints
            "/randomcolor" => random_color GET,

        NESTED_SERVICES:
            "assets",
            "build"
    };

    let listener = tokio::net::TcpListener::bind(HOST_IP).await.unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
