//! Health check endpoint
//!
//! Provides a simple health check endpoint that returns server status and version.

use hyper::Response;

use super::HttpBody;

/// Handle GET /health request (no authentication required)
pub fn handle() -> Response<HttpBody> {
    let body = format!(
        r#"{{"status":"ok","version":"{}"}}"#,
        env!("CARGO_PKG_VERSION")
    );
    super::response::ok(&body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check() {
        let resp = handle();
        assert_eq!(resp.status(), hyper::StatusCode::OK);
    }
}
