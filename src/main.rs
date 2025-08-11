//! Outline MCP Server - main entry point
//!
//! MCP server with STDIO and HTTP transport support.

use tracing::debug;

use outline_mcp_rs::{cli, run_http, run_stdio, Config, Result};

/// Application entry point
#[cfg(not(windows))]
#[tokio::main]
async fn main() -> Result<()> {
    main_impl().await
}

#[cfg(windows)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    main_impl().await
}

async fn main_impl() -> Result<()> {
    // Parse CLI arguments first (handles help/version internally)
    let command = cli::parse_args();

    // Initialize logging based on the command mode
    match command {
        cli::CliCommand::Http => {
            // For HTTP mode, initialize full logging immediately
            init_logging();
        }
        cli::CliCommand::Stdio => {
            // For STDIO mode, initialize minimal logging to stderr only
            init_stdio_logging();
        }
        // Help and Version are handled in parse_args() and exit
        cli::CliCommand::Help | cli::CliCommand::Version => unreachable!(),
    }

    // Load variables from .env file (ignore errors if file not found)
    if let Err(e) = dotenvy::dotenv() {
        debug!(
            ".env file not found, using system environment variables: {}",
            e
        );
    } else {
        debug!("Environment loaded from .env file");
    }

    let config = Config::from_env()?;

    match command {
        cli::CliCommand::Http => run_http(config).await,
        cli::CliCommand::Stdio => run_stdio(config).await,
        // Help and Version are handled in parse_args() and exit
        cli::CliCommand::Help | cli::CliCommand::Version => unreachable!(),
    }
}

/// Initialize logging for STDIO mode (stderr only, minimal level)
fn init_stdio_logging() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    // Use error level by default for STDIO, but allow override with RUST_LOG
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error"));

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(std::io::stderr) // Force stderr to avoid JSON pollution
                .with_ansi(false), // Disable colors for cleaner output
        )
        .with(filter)
        .init();
}

/// Initialize logging with reasonable defaults (for HTTP mode)
fn init_logging() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}
