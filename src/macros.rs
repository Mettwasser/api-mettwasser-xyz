#[macro_export]
macro_rules! router {
    ( ROUTES: $( $route:literal => $handler:ident $method:ident ),*$(,)? NESTED_SERVICES: $( $folder_name:literal ),* $(,)? ) => {
        axum::Router::new()
        $(
            .route($route, router!(@mappings $handler, $method))
        )*
        $(
            .nest_service(concat!("/", $folder_name), tower_http::services::ServeDir::new($folder_name))
        )*
    };

    ( @mappings $handler:ident, GET ) => {
        axum::routing::get($handler)
    };

    ( @mappings $handler:ident, POST ) => {
        axum::routing::post($handler)
    };
}
