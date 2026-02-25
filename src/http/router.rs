//! HTTP request router
//!
//! Routes incoming HTTP requests to the appropriate handler based on method and path.

use std::convert::Infallible;
use std::net::IpAddr;
use std::sync::Arc;

use hyper::body::Incoming;
use hyper::{Method, Response};
use tracing::debug;

use super::server::AppState;
use super::{cors, handler, health, response, HttpBody};

/// Route an incoming HTTP request to the appropriate handler
///
/// # Errors
///
/// This function never returns an error (Infallible). All error conditions
/// are mapped to appropriate HTTP error responses.
pub async fn route(
    req: hyper::Request<Incoming>,
    state: Arc<AppState>,
    client_ip: IpAddr,
) -> Result<Response<HttpBody>, Infallible> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    debug!("{} {} from {}", method, path, client_ip);

    let mut resp = match (&method, path.as_str()) {
        // CORS preflight for any path
        (&Method::OPTIONS, _) => cors::preflight(),

        // Health check (no auth)
        (&Method::GET, "/health") => health::handle(),

        // MCP endpoints (with auth)
        (&Method::POST, "/mcp") => handler::handle_post(req, state, client_ip).await,
        (&Method::GET, "/mcp") => handler::handle_get_sse(req, state, client_ip).await,
        (&Method::DELETE, "/mcp") => handler::handle_delete(req, state, client_ip).await,

        // Unknown path
        (_, "/mcp") => response::method_not_allowed(),
        _ => response::not_found(),
    };

    // Apply CORS to all responses (except preflight which already has them)
    if method != Method::OPTIONS {
        cors::apply(&mut resp);
    }

    Ok(resp)
}
