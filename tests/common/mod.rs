//! Common test utilities

use outline_mcp_rs::{Config, Error, Result};
use serde_json::{json, Value};

/// Create test configuration
pub fn create_test_config() -> Config {
    use outline_mcp_rs::config::{ApiKey, LogLevel, Port};
    use std::net::IpAddr;

    Config {
        outline_api_key: ApiKey::new("test_api_key".to_string()).unwrap(),
        outline_api_url: "https://test.example.com".parse().unwrap(),
        http_host: "127.0.0.1".parse::<IpAddr>().unwrap(),
        http_port: Port::new(3000).unwrap(),
        log_level: LogLevel::new("info").unwrap(),
    }
}

/// Create mock MCP request
pub fn create_mcp_request(method: &str, params: Option<Value>) -> String {
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params.unwrap_or(json!({}))
    });

    serde_json::to_string(&request).unwrap()
}

/// Parse MCP response
pub fn parse_mcp_response(response: &str) -> Result<Value> {
    serde_json::from_str(response).map_err(|e| Error::Json {
        context: format!("Failed to parse response: {}", e),
        source: e,
    })
}

/// Check if response is success
pub fn is_success_response(response: &Value) -> bool {
    response.get("error").is_none() && response.get("result").is_some()
}

/// Extract result from MCP response
pub fn extract_result(response: &Value) -> Option<&Value> {
    response.get("result")
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_config() {
        let config = create_test_config();
        assert_eq!(config.outline_api_key.as_str(), "test_api_key");
        assert_eq!(config.http_host.to_string(), "127.0.0.1");
    }

    #[test]
    fn test_create_mcp_request() {
        let request = create_mcp_request("test_method", Some(json!({"key": "value"})));
        let parsed: Value = serde_json::from_str(&request).unwrap();

        assert_eq!(parsed["method"], "test_method");
        assert_eq!(parsed["params"]["key"], "value");
    }

    #[test]
    fn test_parse_mcp_response() {
        let response_str = r#"{"jsonrpc":"2.0","id":1,"result":{"success":true}}"#;
        let response = parse_mcp_response(response_str).unwrap();

        assert!(is_success_response(&response));
        assert_eq!(extract_result(&response).unwrap()["success"], true);
    }
}
