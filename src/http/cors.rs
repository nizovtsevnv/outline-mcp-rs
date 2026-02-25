//! CORS header management
//!
//! Applies Cross-Origin Resource Sharing headers to HTTP responses.

use hyper::header::HeaderValue;
use hyper::Response;

use super::HttpBody;

/// Apply CORS headers to an HTTP response
pub fn apply<B>(response: &mut Response<B>) {
    let headers = response.headers_mut();
    headers.insert(
        hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        hyper::header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, DELETE, OPTIONS"),
    );
    headers.insert(
        hyper::header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static(
            "Content-Type, Accept, Authorization, X-MCP-Token, Mcp-Session-Id",
        ),
    );
    headers.insert(
        hyper::header::ACCESS_CONTROL_EXPOSE_HEADERS,
        HeaderValue::from_static("Mcp-Session-Id"),
    );
}

/// Create a CORS preflight response (OPTIONS)
pub fn preflight() -> Response<HttpBody> {
    let mut response = super::response::no_content();
    apply(&mut response);
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_preflight() {
        let resp = preflight();
        assert_eq!(resp.status(), hyper::StatusCode::NO_CONTENT);
        assert_eq!(
            resp.headers()
                .get(hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .unwrap(),
            "*"
        );
    }

    #[test]
    fn test_cors_apply() {
        let mut resp = super::super::response::ok(r#"{"test":true}"#);
        apply(&mut resp);
        assert_eq!(
            resp.headers()
                .get(hyper::header::ACCESS_CONTROL_ALLOW_METHODS)
                .unwrap(),
            "GET, POST, DELETE, OPTIONS"
        );
        assert_eq!(
            resp.headers()
                .get(hyper::header::ACCESS_CONTROL_EXPOSE_HEADERS)
                .unwrap(),
            "Mcp-Session-Id"
        );
    }
}
