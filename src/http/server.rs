//! HTTP server
//!
//! Binds a TCP listener and serves HTTP connections with graceful shutdown support.

use std::sync::Arc;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};
use url::Url;

use super::auth::AuthGuard;
use super::router;
use super::session::SessionManager;
use crate::config::Config;
use crate::error::Result;

/// Shared application state accessible by all request handlers
#[derive(Debug)]
pub struct AppState {
    /// Authentication and rate limiting guard
    pub auth: AuthGuard,
    /// Session manager for MCP sessions
    pub sessions: SessionManager,
    /// Base URL for the Outline API
    pub outline_base_url: Url,
    /// Shared HTTP client for Outline API requests
    pub shared_http_client: reqwest::Client,
    /// Maximum allowed request body size in bytes
    pub max_body_size: usize,
}

/// HTTP server with graceful shutdown support
pub struct HttpServer {
    listener: TcpListener,
    state: Arc<AppState>,
}

impl HttpServer {
    /// Bind the server to the configured address and prepare shared state
    ///
    /// # Errors
    ///
    /// Returns error if binding to the address fails or the HTTP client cannot be built.
    pub async fn bind(config: &Config) -> Result<Self> {
        let addr = format!("{}:{}", config.http_host, config.http_port.as_u16());
        let listener = TcpListener::bind(&addr).await?;

        let shared_http_client = crate::outline::Client::build_http_client()?;

        let state = Arc::new(AppState {
            auth: AuthGuard::new(config.mcp_auth_tokens.clone(), config.http_rate_limit),
            sessions: SessionManager::new(config.http_session_timeout),
            outline_base_url: config.outline_api_url.clone(),
            shared_http_client,
            max_body_size: config.http_max_body_size,
        });

        info!("HTTP server bound to {}", addr);
        info!("Available at POST/GET/DELETE /mcp for MCP requests");
        info!("Health check at GET /health");

        Ok(Self { listener, state })
    }

    /// Run the server, accepting connections until shutdown signal
    ///
    /// Spawns background tasks for session cleanup (every 60s) and
    /// rate limit cleanup (every 5 min). Handles SIGINT/SIGTERM for
    /// graceful shutdown.
    ///
    /// # Errors
    ///
    /// Returns error if there are critical server-level failures.
    pub async fn run(self) -> Result<()> {
        let state = self.state;

        // Background task: cleanup expired sessions
        let session_state = Arc::clone(&state);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                let removed = session_state.sessions.cleanup_expired().await;
                if removed > 0 {
                    debug!("Cleaned up {} expired sessions", removed);
                }
            }
        });

        // Background task: cleanup stale rate limit entries
        let rate_state = Arc::clone(&state);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
            loop {
                interval.tick().await;
                rate_state.auth.cleanup_stale().await;
                debug!("Cleaned up stale rate limit entries");
            }
        });

        // Main accept loop with graceful shutdown
        info!("HTTP server ready, waiting for connections...");

        loop {
            tokio::select! {
                accept_result = self.listener.accept() => {
                    match accept_result {
                        Ok((stream, addr)) => {
                            debug!("New connection from {}", addr);
                            let conn_state = Arc::clone(&state);
                            let client_ip = addr.ip();

                            tokio::spawn(async move {
                                let io = TokioIo::new(stream);
                                let service = service_fn(move |req| {
                                    let state = Arc::clone(&conn_state);
                                    async move {
                                        router::route(req, state, client_ip).await
                                    }
                                });

                                if let Err(e) = http1::Builder::new()
                                    .serve_connection(io, service)
                                    .await
                                {
                                    // Connection errors are expected (client disconnect, etc.)
                                    debug!("Connection error from {}: {}", client_ip, e);
                                }
                            });
                        }
                        Err(e) => {
                            error!("Error accepting connection: {}", e);
                        }
                    }
                }
                () = shutdown_signal() => {
                    warn!("Shutdown signal received, stopping HTTP server");
                    break;
                }
            }
        }

        info!("HTTP server stopped");
        Ok(())
    }
}

/// Wait for a shutdown signal (SIGINT or SIGTERM on Unix, Ctrl+C on Windows)
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = sigterm.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    }
}
