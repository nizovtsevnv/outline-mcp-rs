//! Server-Sent Events (SSE) support
//!
//! SSE body implementation for streaming responses and encoding utilities.

use bytes::Bytes;
use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// SSE streaming body backed by an mpsc channel
#[derive(Debug)]
pub struct SseBody {
    rx: mpsc::Receiver<Bytes>,
}

impl SseBody {
    /// Create a new SSE body from a channel receiver
    pub const fn new(rx: mpsc::Receiver<Bytes>) -> Self {
        Self { rx }
    }
}

impl hyper::body::Body for SseBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        self.rx
            .poll_recv(cx)
            .map(|opt| opt.map(|data| Ok(hyper::body::Frame::data(data))))
    }
}

/// Encode a JSON message as an SSE event
#[must_use]
#[allow(dead_code)]
pub fn encode_event(json: &str) -> Bytes {
    Bytes::from(format!("event: message\ndata: {json}\n\n"))
}

/// Encode an SSE keepalive comment
#[must_use]
pub const fn encode_keepalive() -> Bytes {
    Bytes::from_static(b": keepalive\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_event() {
        let data = encode_event(r#"{"test":true}"#);
        let expected = "event: message\ndata: {\"test\":true}\n\n";
        assert_eq!(data, expected);
    }

    #[test]
    fn test_encode_keepalive() {
        let data = encode_keepalive();
        assert_eq!(data, ": keepalive\n\n");
    }

    #[tokio::test]
    async fn test_sse_body_receives_data() {
        let (tx, rx) = mpsc::channel(16);
        let mut body = SseBody::new(rx);

        use hyper::body::Body;

        tx.send(encode_event(r#"{"id":1}"#)).await.unwrap();
        drop(tx);

        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);
        let frame = Pin::new(&mut body).poll_frame(&mut cx);

        // Channel has data, should be ready
        assert!(matches!(frame, Poll::Ready(Some(Ok(_)))));
    }
}
