#[macro_export]
macro_rules! router {
    ( $( $route:literal => $handler:ident $method:ident ),* $(,)? ) => {
        axum::Router::new()
        $(
            .route($route, router!(@mappings $handler, $method))
        )*
    };

    ( @mappings $handler:ident, GET ) => {
        axum::routing::get($handler)
    };

    ( @mappings $handler:ident, POST ) => {
        axum::routing::post($handler)
    };
}
