mod server_app_state;

#[cfg(feature = "ssr")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use axum::{Router, routing::get};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use server_app_state::ServerAppState;
    use std::env;
    use tower::ServiceBuilder;
    use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
    use tracing::Level;
    use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();
    tracing::info!("Initializing server");

    let port = env::var("PORT").unwrap_or("4242".to_string());
    let server_address = format!("0.0.0.0:{}", port);
    tracing::info!("Beginning to listen on http://{}", server_address);
    let server_listener = tokio::net::TcpListener::bind(server_address)
        .await
        .expect("Unable to bind listener to server_address: {}");

    let server_middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    let server_router = Router::new()
        .route("/healthcheck", get(|| async { "OK" }))
        .layer(server_middleware);

    axum::serve(server_listener, server_router)
        .await
        .expect("Unable to start server");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    //Inentional no-op
}
