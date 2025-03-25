#![forbid(unsafe_code)]

use dotenvy::dotenv;
use retro;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt};

fn main() {
    dotenv().ok();
    let _guard = setup_logging();
    retro::run_game();
}

fn setup_logging() -> WorkerGuard {
    // Write logs to stdout with a non-blocking writer.
    let (non_blocking_writer, guard) = non_blocking(std::io::stdout());

    // Use an environment filter for dynamic log levels.
    // e.g., set RUST_LOG=debug before running.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_writer(non_blocking_writer)
        .with_env_filter(env_filter)
        .init();
    guard
}
