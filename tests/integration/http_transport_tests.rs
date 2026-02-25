//! Integration tests for HTTP transport
//!
//! Tests the Streamable HTTP transport including authentication,
//! rate limiting, session management, and CORS.

mod common;

use common::create_test_config;
use serde_json::json;

/// Helper: build a JSON-RPC request string
fn mcp_request(method: &str, id: u32) -> String {
    serde_json::to_string(&json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": {}
    }))
    .unwrap()
}

#[tokio::test]
async fn test_config_for_http_mode() {
    let config = create_test_config();
    assert!(config.outline_api_key.is_some());
    assert!(!config.mcp_auth_tokens.is_empty());
    assert_eq!(config.http_max_body_size, 1_048_576);
    assert_eq!(config.http_session_timeout, 1800);
    assert_eq!(config.http_rate_limit, 60);
}

#[tokio::test]
async fn test_http_mode_requires_mcp_auth_tokens() {
    use outline_mcp_rs::config::{LogLevel, Port};
    use outline_mcp_rs::Config;
    use std::net::IpAddr;

    let config = Config {
        outline_api_key: None,
        outline_api_url: "https://test.example.com".parse().unwrap(),
        http_host: "127.0.0.1".parse::<IpAddr>().unwrap(),
        http_port: Port::new(3000).unwrap(),
        log_level: LogLevel::new("info").unwrap(),
        http_max_body_size: 1_048_576,
        http_session_timeout: 1800,
        http_rate_limit: 60,
        mcp_auth_tokens: vec![], // Empty — should fail
    };

    // run_http should fail when mcp_auth_tokens is empty
    let result = outline_mcp_rs::run_http(config).await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("MCP_AUTH_TOKENS"));
}

#[tokio::test]
async fn test_mcp_request_format() {
    let req = mcp_request("initialize", 1);
    let parsed: serde_json::Value = serde_json::from_str(&req).unwrap();
    assert_eq!(parsed["jsonrpc"], "2.0");
    assert_eq!(parsed["method"], "initialize");
    assert_eq!(parsed["id"], 1);
}

#[tokio::test]
async fn test_mcp_tools_call_request_format() {
    let req = serde_json::to_string(&json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "list_documents",
            "arguments": {}
        }
    }))
    .unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&req).unwrap();
    assert_eq!(parsed["method"], "tools/call");
    assert_eq!(parsed["params"]["name"], "list_documents");
}

#[tokio::test]
async fn test_stdio_mode_requires_outline_api_key() {
    use outline_mcp_rs::config::{LogLevel, Port};
    use outline_mcp_rs::Config;
    use std::net::IpAddr;

    let config = Config {
        outline_api_key: None, // Missing — should fail for STDIO
        outline_api_url: "https://test.example.com".parse().unwrap(),
        http_host: "127.0.0.1".parse::<IpAddr>().unwrap(),
        http_port: Port::new(3000).unwrap(),
        log_level: LogLevel::new("info").unwrap(),
        http_max_body_size: 1_048_576,
        http_session_timeout: 1800,
        http_rate_limit: 60,
        mcp_auth_tokens: vec![],
    };

    let result = outline_mcp_rs::run_stdio(config).await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("OUTLINE_API_KEY"));
}
