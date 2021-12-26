use dotenv::dotenv;
use tracing_subscriber::EnvFilter;

// Initiate the tracing subscriber for RUST_LOG
pub fn start_tracing() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}