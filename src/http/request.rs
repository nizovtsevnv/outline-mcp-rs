//! HTTP request validation
//!
//! Validates incoming HTTP requests for content type, size, and required headers.

use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::body::Incoming;

use crate::error::{Error, Result};

/// Validate that the request Content-Type is application/json
///
/// # Errors
///
/// Returns `HttpTransport` error with status 415 if Content-Type is not application/json.
pub fn validate_content_type(req: &hyper::Request<Incoming>) -> Result<()> {
    let content_type = req
        .headers()
        .get(hyper::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !content_type.starts_with("application/json") {
        return Err(Error::HttpTransport {
            status: 415,
            message: "Content-Type must be application/json".to_string(),
        });
    }

    Ok(())
}

/// Validate that the Accept header includes application/json or text/event-stream
///
/// # Errors
///
/// Returns `HttpTransport` error with status 400 if Accept header is incompatible.
pub fn validate_accept(req: &hyper::Request<Incoming>) -> Result<()> {
    if let Some(accept) = req
        .headers()
        .get(hyper::header::ACCEPT)
        .and_then(|v| v.to_str().ok())
    {
        let acceptable = accept.contains("application/json")
            || accept.contains("text/event-stream")
            || accept.contains("*/*");
        if !acceptable {
            return Err(Error::HttpTransport {
                status: 400,
                message: "Accept header must include application/json or text/event-stream"
                    .to_string(),
            });
        }
    }
    Ok(())
}

/// Read the request body with a size limit
///
/// # Errors
///
/// Returns `HttpTransport` error with status 413 if body exceeds `max_size`.
/// Returns transport error if body cannot be read.
pub async fn read_body(req: hyper::Request<Incoming>, max_size: usize) -> Result<(Bytes, String)> {
    // Check Content-Length header first for early rejection
    if let Some(content_length) = req
        .headers()
        .get(hyper::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
    {
        if content_length > max_size {
            return Err(Error::HttpTransport {
                status: 413,
                message: format!("Request body too large: {content_length} > {max_size}"),
            });
        }
    }

    let body_bytes = req
        .collect()
        .await
        .map_err(|e| Error::Transport {
            transport_type: "HTTP body read".to_string(),
            source: Box::new(e),
        })?
        .to_bytes();

    if body_bytes.len() > max_size {
        return Err(Error::HttpTransport {
            status: 413,
            message: format!("Request body too large: {} > {max_size}", body_bytes.len()),
        });
    }

    let body_str = String::from_utf8(body_bytes.to_vec()).map_err(|e| Error::Transport {
        transport_type: "UTF-8 body decode".to_string(),
        source: Box::new(e),
    })?;

    Ok((body_bytes, body_str))
}

/// Extract the Mcp-Session-Id header value
pub fn extract_session_id(req: &hyper::Request<Incoming>) -> Option<String> {
    req.headers()
        .get("Mcp-Session-Id")
        .and_then(|v| v.to_str().ok())
        .map(std::string::ToString::to_string)
}

// Note: validate_content_type, validate_accept, and read_body require
// hyper::body::Incoming which can only be created from actual HTTP connections.
// These are tested in integration tests (tests/integration/http_transport_tests.rs).
