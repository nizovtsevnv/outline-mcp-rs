//! HTTP response builder
//!
//! Helper functions for creating HTTP responses with proper status codes and headers.

use bytes::Bytes;
use http_body_util::Full;
use hyper::Response;

use super::HttpBody;

/// Build a JSON response with the given status code and body
pub fn json(status: hyper::StatusCode, body: &str) -> Response<HttpBody> {
    Response::builder()
        .status(status)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(HttpBody::Full(Full::new(Bytes::from(body.to_string()))))
        .unwrap_or_else(|_| internal_server_error())
}

/// 200 OK with JSON body
pub fn ok(body: &str) -> Response<HttpBody> {
    json(hyper::StatusCode::OK, body)
}

/// 202 Accepted (for notifications that don't need a response body)
pub fn accepted() -> Response<HttpBody> {
    Response::builder()
        .status(hyper::StatusCode::ACCEPTED)
        .body(HttpBody::Full(Full::default()))
        .unwrap_or_else(|_| internal_server_error())
}

/// 204 No Content
pub fn no_content() -> Response<HttpBody> {
    Response::builder()
        .status(hyper::StatusCode::NO_CONTENT)
        .body(HttpBody::Full(Full::default()))
        .unwrap_or_else(|_| internal_server_error())
}

/// 400 Bad Request
pub fn bad_request(message: &str) -> Response<HttpBody> {
    json(
        hyper::StatusCode::BAD_REQUEST,
        &format!(r#"{{"error":"{message}"}}"#),
    )
}

/// 401 Unauthorized
pub fn unauthorized(message: &str) -> Response<HttpBody> {
    json(
        hyper::StatusCode::UNAUTHORIZED,
        &format!(r#"{{"error":"{message}"}}"#),
    )
}

/// 404 Not Found
pub fn not_found() -> Response<HttpBody> {
    json(hyper::StatusCode::NOT_FOUND, r#"{"error":"Not Found"}"#)
}

/// 405 Method Not Allowed
pub fn method_not_allowed() -> Response<HttpBody> {
    json(
        hyper::StatusCode::METHOD_NOT_ALLOWED,
        r#"{"error":"Method Not Allowed"}"#,
    )
}

/// 413 Payload Too Large
pub fn payload_too_large() -> Response<HttpBody> {
    json(
        hyper::StatusCode::PAYLOAD_TOO_LARGE,
        r#"{"error":"Payload Too Large"}"#,
    )
}

/// 415 Unsupported Media Type
pub fn unsupported_media_type() -> Response<HttpBody> {
    json(
        hyper::StatusCode::UNSUPPORTED_MEDIA_TYPE,
        r#"{"error":"Unsupported Media Type"}"#,
    )
}

/// 429 Too Many Requests
pub fn too_many_requests() -> Response<HttpBody> {
    json(
        hyper::StatusCode::TOO_MANY_REQUESTS,
        r#"{"error":"Too Many Requests"}"#,
    )
}

/// 500 Internal Server Error
pub fn internal_server_error() -> Response<HttpBody> {
    Response::builder()
        .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(HttpBody::Full(Full::new(Bytes::from(
            r#"{"error":"Internal Server Error"}"#,
        ))))
        .expect("static response must be valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_response() {
        let resp = ok(r#"{"result":"success"}"#);
        assert_eq!(resp.status(), hyper::StatusCode::OK);
    }

    #[test]
    fn test_unauthorized_response() {
        let resp = unauthorized("Invalid token");
        assert_eq!(resp.status(), hyper::StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_not_found_response() {
        let resp = not_found();
        assert_eq!(resp.status(), hyper::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_too_many_requests_response() {
        let resp = too_many_requests();
        assert_eq!(resp.status(), hyper::StatusCode::TOO_MANY_REQUESTS);
    }

    #[test]
    fn test_payload_too_large_response() {
        let resp = payload_too_large();
        assert_eq!(resp.status(), hyper::StatusCode::PAYLOAD_TOO_LARGE);
    }
}
