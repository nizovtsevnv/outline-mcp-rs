//! # Outline MCP Server
//!
//! MCP (Model Context Protocol) server for Outline knowledge base interaction
//! with focus on simplicity and performance.
//!
//! ## Design Principles
//!
//! - **Simplicity**: Direct functions instead of complex abstractions
//! - **Performance**: Static builds and minimal dependencies
//! - **Elegance**: One file for each area of responsibility
//!
//! ## Usage Example
//!
//! ```no_run
//! use outline_mcp_rs::{Config, run_stdio, run_http};
//!
//! #[tokio::main]
//! async fn main() -> outline_mcp_rs::Result<()> {
//!     let config = Config::from_env()?;
//!
//!     // STDIO mode
//!     run_stdio(config.clone()).await?;
//!
//!     // Or HTTP mode
//!     run_http(config).await
//! }
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

// Public exports
pub use config::Config;
pub use error::{Error, Result};

// Modules
pub mod cli;
pub mod config;
pub mod error;
mod http;
mod mcp;
mod outline;
mod tools;

/// Run server in STDIO mode
///
/// Used for integration with MCP clients through standard input/output streams.
/// Requires `OUTLINE_API_KEY` environment variable to be set.
///
/// # Errors
///
/// Returns error on initialization or request processing problems.
pub async fn run_stdio(config: Config) -> Result<()> {
    use std::io::{self, Write};
    use tracing::{debug, error};

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // STDIO mode requires OUTLINE_API_KEY
    let api_key = config.outline_api_key.ok_or_else(|| Error::Config {
        message: "OUTLINE_API_KEY environment variable required for STDIO mode".to_string(),
        source: None,
    })?;

    // Initialize Outline API client
    let outline_client = outline::Client::new(api_key, config.outline_api_url)?;

    debug!("STDIO server ready");

    // Main STDIO processing loop
    loop {
        let input = {
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => line.trim_end().to_string(),
                Err(e) => {
                    error!("Error reading STDIN: {}", e);
                    break;
                }
            }
        };

        if input.trim().is_empty() {
            continue;
        }

        // Process JSON-RPC request
        match mcp::handle_request(&input, &outline_client).await {
            Ok(Some(response)) => {
                writeln!(stdout, "{response}")?;
                stdout.flush()?;
            }
            Ok(None) => {
                // No response needed (notification), just continue
            }
            Err(e) => {
                error!("Error processing request: {}", e);
                let error_response = mcp::create_error_response(&e);
                writeln!(stdout, "{error_response}")?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}

/// Run server in HTTP mode
///
/// Creates a Streamable HTTP server with multi-user support, authentication,
/// rate limiting, and session management.
///
/// Required environment variables:
/// - `MCP_AUTH_TOKENS` — comma-separated list of allowed MCP authentication tokens
///
/// Optional environment variables:
/// - `OUTLINE_API_URL` — Outline API base URL (default: `https://app.getoutline.com/api`)
/// - `HTTP_HOST` — bind address (default: `127.0.0.1`)
/// - `HTTP_PORT` — port number (default: `3000`)
/// - `HTTP_MAX_BODY_SIZE` — max request body in bytes (default: `1048576`)
/// - `HTTP_SESSION_TIMEOUT` — session TTL in seconds (default: `1800`)
/// - `HTTP_RATE_LIMIT` — requests per minute per IP (default: `60`)
///
/// # Errors
///
/// Returns error if there are problems binding to port, building the HTTP client,
/// or if `MCP_AUTH_TOKENS` is not set.
pub async fn run_http(config: Config) -> Result<()> {
    use tracing::warn;

    // Validate HTTP mode requirements
    if config.mcp_auth_tokens.is_empty() {
        return Err(Error::Config {
            message: "MCP_AUTH_TOKENS environment variable required for HTTP mode \
                      (comma-separated list of allowed tokens)"
                .to_string(),
            source: None,
        });
    }

    if config.outline_api_key.is_some() {
        warn!(
            "OUTLINE_API_KEY is set but ignored in HTTP mode. \
             Each client must provide their own key via Authorization header."
        );
    }

    let server = http::server::HttpServer::bind(&config).await?;
    server.run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::for_testing();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_error_types() {
        let _error = Error::Config {
            message: "test error".to_string(),
            source: None,
        };

        // Test that error types work correctly
        // Test passes if error creation doesn't panic
    }

    #[test]
    fn test_http_transport_error() {
        let _error = Error::HttpTransport {
            status: 401,
            message: "Unauthorized".to_string(),
        };
    }
}
