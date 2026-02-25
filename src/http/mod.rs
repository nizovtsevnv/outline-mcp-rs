//! Streamable HTTP transport
//!
//! Full MCP Streamable HTTP transport implementation with multi-user support,
//! authentication, rate limiting, session management, and SSE streaming.

mod auth;
mod cors;
mod handler;
mod health;
mod request;
mod response;
mod router;
pub mod server;
mod session;
mod sse;

use bytes::Bytes;
use http_body_util::Full;
use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};

/// HTTP response body type supporting both complete and streaming responses
pub enum HttpBody {
    /// Complete body (for JSON responses)
    Full(Full<Bytes>),
    /// Server-Sent Events streaming body
    Sse(sse::SseBody),
}

impl hyper::body::Body for HttpBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        // SAFETY: Both Full<Bytes> and SseBody are Unpin
        match self.get_mut() {
            Self::Full(body) => Pin::new(body).poll_frame(cx),
            Self::Sse(body) => Pin::new(body).poll_frame(cx),
        }
    }

    fn is_end_stream(&self) -> bool {
        match self {
            Self::Full(body) => body.is_end_stream(),
            Self::Sse(_) => false,
        }
    }
}
