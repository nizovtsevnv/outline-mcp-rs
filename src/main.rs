//! Outline MCP Server - main entry point
//!
//! MCP server with STDIO and HTTP transport support.

use std::env;
use std::io::{self, BufRead, Write};
use tokio::net::TcpListener;
use tracing::{error, info};

mod config;
mod error;
mod mcp;
mod outline;
mod tools;

use config::Config;
use error::Result;

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
    init_logging();

    // Load variables from .env file (ignore errors if file not found)
    if let Err(e) = dotenvy::dotenv() {
        info!(
            ".env file not found, using system environment variables: {}",
            e
        );
    } else {
        info!("Environment loaded from .env file");
    }

    let config = Config::from_env()?;

    match env::args().nth(1).as_deref() {
        Some("--http") => run_http(config).await,
        _ => run_stdio(config).await,
    }
}

/// Initialize logging with reasonable defaults
fn init_logging() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}

/// Run server in STDIO mode
async fn run_stdio(config: Config) -> Result<()> {
    info!("📡 STDIO MCP server starting...");
    info!("🔗 Connect using: your-mcp-client");

    let outline_client = outline::Client::new(config.outline_api_key, config.outline_api_url)?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        match mcp::handle_request(&line, &outline_client).await {
            Ok(response) => {
                writeln!(stdout, "{}", response)?;
                stdout.flush()?;
            }
            Err(e) => {
                let error_response = mcp::create_error_response(&e);
                writeln!(stdout, "{}", error_response)?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}

/// Run server in HTTP mode
async fn run_http(config: Config) -> Result<()> {
    let addr = format!("{}:{}", config.http_host, config.http_port.as_u16());
    let listener = TcpListener::bind(&addr).await?;

    info!("🌐 HTTP server started on {}", addr);
    info!("📡 Available at /mcp for MCP requests");

    let outline_client = outline::Client::new(config.outline_api_key, config.outline_api_url)?;

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let client = outline_client.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, client).await {
                        error!("Connection error from {}: {}", addr, e);
                    }
                });
            }
            Err(e) => error!("Accept error: {}", e),
        }
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    outline_client: outline::Client,
) -> Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let mut line = String::new();
    while reader.read_line(&mut line).await? > 0 {
        if line.trim().is_empty() {
            line.clear();
            continue;
        }

        match mcp::handle_request(&line, &outline_client).await {
            Ok(response) => {
                writer.write_all(response.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
            Err(e) => {
                let error_response = mcp::create_error_response(&e);
                writer.write_all(error_response.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
        }

        line.clear();
    }

    Ok(())
}
