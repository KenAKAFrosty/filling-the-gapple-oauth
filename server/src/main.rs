use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();

    tracing::info!("Hello, Oauth!");
}
