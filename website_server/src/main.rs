#[cfg(feature = "ssr")]
mod server_context;
use leptos::prelude::*;
use serde::Deserialize;

#[cfg(feature = "ssr")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use axum::{Router, routing::get};
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use server_context::{ServerConfig, ServerContextData};
    use std::env;
    use tower::ServiceBuilder;
    use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
    use tracing::Level;
    use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};
    use website_server::site::*;

    use crate::server_context::ServerContext;

    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();
    tracing::info!("Initializing server");

    let leptos_config =
        get_configuration(None).expect("to successfully load configuration on startup");
    let server_address = leptos_config.leptos_options.site_addr;
    let leptos_options = leptos_config.leptos_options;
    let routes = generate_route_list(Site);

    let google_oauth_client_id =
        env::var("GOOGLE_OAUTH_CLIENT_ID").expect("to have google oauth client id in environment");
    let google_oauth_client_secret = env::var("GOOGLE_OAUTH_CLIENT_SECRET")
        .expect("to have google oauth client secret in environment");

    let server_context = ServerContext::new(ServerContextData {
        config: ServerConfig {
            google_oauth_client_id,
            google_oauth_client_secret,
        },
    });

    let server_middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    let server_router = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(server_context.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .route("/healthcheck", get(|| async { "OK" })) //intentionally showing the usage of a non-axum route or server_fn being integrated
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(server_middleware)
        .with_state(leptos_options);

    tracing::info!("Beginning to listen on http://{}", server_address);
    let server_listener = tokio::net::TcpListener::bind(server_address)
        .await
        .expect("Unable to bind listener to server_address: {}");

    axum::serve(server_listener, server_router)
        .await
        .expect("Unable to start server");
}

#[cfg(feature = "ssr")]
use server_fn::codec::GetUrl;

#[derive(Deserialize, Debug)]
struct MyQuery {
    foo: String,
}

#[server(endpoint = "simple_test", input = GetUrl)]
async fn simple_test() -> Result<String, ServerFnError> {
    use crate::server_context::ServerContext;
    let state = expect_context::<ServerContext>();

    //warn to both stand out in logs and more easily remind us to remove this when done tinkering
    tracing::warn!(
        "Client ID: {}, Secret Peek: {:?}",
        &state.config.google_oauth_client_id,
        &state.config.google_oauth_client_secret[..4]
    );
    Ok("Hi!".to_string())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    //Inentional no-op
}
