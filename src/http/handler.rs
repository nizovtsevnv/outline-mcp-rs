//! MCP request handler
//!
//! Handles POST, GET (SSE), and DELETE requests on the /mcp endpoint.

use std::sync::Arc;
use std::time::Duration;

use hyper::body::Incoming;
use hyper::Response;
use tokio::sync::mpsc;
use tracing::{debug, error, warn};

use super::server::AppState;
use super::{auth, cors, request, response, sse, HttpBody};
use crate::config::ApiKey;
use crate::{mcp, outline};

/// Handle POST /mcp — process a JSON-RPC MCP request
pub async fn handle_post(
    req: hyper::Request<Incoming>,
    state: Arc<AppState>,
    client_ip: std::net::IpAddr,
) -> Response<HttpBody> {
    // Auth: validate MCP token
    let mcp_token = match auth::AuthGuard::extract_mcp_token(&req) {
        Some(token) => token.to_string(),
        None => return response::unauthorized("Missing X-MCP-Token header"),
    };
    if !state.auth.validate_token(&mcp_token) {
        warn!("Invalid MCP token from {}", client_ip);
        return response::unauthorized("Invalid MCP token");
    }

    // Rate limiting
    if !state.auth.check_rate_limit(client_ip).await {
        warn!("Rate limit exceeded for {}", client_ip);
        return response::too_many_requests();
    }

    // Extract Outline API key from Authorization header
    let Some(outline_key) = auth::AuthGuard::extract_outline_key(&req) else {
        return response::unauthorized("Missing Authorization header with Outline API key");
    };

    // Validate API key
    let Ok(api_key) = ApiKey::new(outline_key) else {
        return response::bad_request("Invalid Outline API key");
    };

    // Validate Content-Type
    if let Err(e) = request::validate_content_type(&req) {
        return match e {
            crate::error::Error::HttpTransport { status: 415, .. } => {
                response::unsupported_media_type()
            }
            _ => response::bad_request("Invalid Content-Type"),
        };
    }

    // Validate Accept header
    if let Err(_e) = request::validate_accept(&req) {
        return response::bad_request("Invalid Accept header");
    }

    // Handle session: check for existing session or create new one on initialize
    let session_id = request::extract_session_id(&req);

    // Read body with size limit
    let (_body_bytes, body_str) = match request::read_body(req, state.max_body_size).await {
        Ok(result) => result,
        Err(crate::error::Error::HttpTransport { status: 413, .. }) => {
            return response::payload_too_large();
        }
        Err(_) => return response::bad_request("Failed to read request body"),
    };

    // Check if this is an initialize request (to create a session)
    let is_initialize = body_str.contains("\"method\"")
        && (body_str.contains("\"initialize\"") || body_str.contains("'initialize'"));

    // Session management
    let response_session_id = if is_initialize {
        // Create a new session for initialize
        let new_id = state.sessions.create().await;
        debug!("Created new session: {}", new_id);
        Some(new_id)
    } else if let Some(ref sid) = session_id {
        // Validate existing session
        if !state.sessions.touch(sid).await {
            return response::bad_request("Invalid or expired Mcp-Session-Id");
        }
        Some(sid.clone())
    } else {
        // No session required for simple requests
        None
    };

    // Create per-request Outline client
    let outline_client = outline::Client::from_parts(
        state.shared_http_client.clone(),
        api_key,
        state.outline_base_url.clone(),
    );

    // Process MCP request
    match mcp::handle_request(&body_str, &outline_client).await {
        Ok(Some(mcp_response)) => {
            let mut resp = response::ok(&mcp_response);
            if let Some(sid) = response_session_id {
                resp.headers_mut().insert(
                    "Mcp-Session-Id",
                    hyper::header::HeaderValue::from_str(&sid)
                        .unwrap_or_else(|_| hyper::header::HeaderValue::from_static("invalid")),
                );
            }
            cors::apply(&mut resp);
            resp
        }
        Ok(None) => {
            // Notification — no response body needed
            let mut resp = response::accepted();
            cors::apply(&mut resp);
            resp
        }
        Err(e) => {
            error!("MCP request processing error: {}", e);
            let error_response = mcp::create_error_response(&e);
            let mut resp =
                response::json(hyper::StatusCode::INTERNAL_SERVER_ERROR, &error_response);
            cors::apply(&mut resp);
            resp
        }
    }
}

/// Handle GET /mcp — open an SSE stream for server-to-client communication
pub async fn handle_get_sse(
    req: hyper::Request<Incoming>,
    state: Arc<AppState>,
    client_ip: std::net::IpAddr,
) -> Response<HttpBody> {
    // Auth: validate MCP token
    let mcp_token = match auth::AuthGuard::extract_mcp_token(&req) {
        Some(token) => token.to_string(),
        None => return response::unauthorized("Missing X-MCP-Token header"),
    };
    if !state.auth.validate_token(&mcp_token) {
        return response::unauthorized("Invalid MCP token");
    }

    // Rate limiting
    if !state.auth.check_rate_limit(client_ip).await {
        return response::too_many_requests();
    }

    // Session validation
    let session_id = match request::extract_session_id(&req) {
        Some(sid) => {
            if !state.sessions.touch(&sid).await {
                return response::bad_request("Invalid or expired Mcp-Session-Id");
            }
            sid
        }
        None => return response::bad_request("Mcp-Session-Id required for SSE"),
    };

    // Create SSE channel
    let (tx, rx) = mpsc::channel::<bytes::Bytes>(32);

    // Spawn keepalive task
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            if tx.send(sse::encode_keepalive()).await.is_err() {
                break; // Client disconnected
            }
        }
    });

    // Build SSE response
    let sse_body = sse::SseBody::new(rx);
    let mut resp = Response::builder()
        .status(hyper::StatusCode::OK)
        .header(hyper::header::CONTENT_TYPE, "text/event-stream")
        .header(hyper::header::CACHE_CONTROL, "no-cache")
        .header("Connection", "keep-alive")
        .header("Mcp-Session-Id", &session_id)
        .body(HttpBody::Sse(sse_body))
        .unwrap_or_else(|_| response::internal_server_error());

    cors::apply(&mut resp);
    resp
}

/// Handle DELETE /mcp — terminate a session
pub async fn handle_delete(
    req: hyper::Request<Incoming>,
    state: Arc<AppState>,
    client_ip: std::net::IpAddr,
) -> Response<HttpBody> {
    // Auth: validate MCP token
    let mcp_token = match auth::AuthGuard::extract_mcp_token(&req) {
        Some(token) => token.to_string(),
        None => return response::unauthorized("Missing X-MCP-Token header"),
    };
    if !state.auth.validate_token(&mcp_token) {
        return response::unauthorized("Invalid MCP token");
    }

    // Rate limiting
    if !state.auth.check_rate_limit(client_ip).await {
        return response::too_many_requests();
    }

    // Session removal
    let Some(session_id) = request::extract_session_id(&req) else {
        return response::bad_request("Mcp-Session-Id required for DELETE");
    };

    if state.sessions.remove(&session_id).await {
        debug!("Deleted session: {}", session_id);
        let mut resp = response::ok(r#"{"status":"session_deleted"}"#);
        cors::apply(&mut resp);
        resp
    } else {
        response::not_found()
    }
}
