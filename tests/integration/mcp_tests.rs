//! Integration tests for MCP protocol

mod common;

use common::{create_mcp_request, parse_mcp_response, is_success_response};
use serde_json::json;

#[tokio::test]
async fn test_mcp_initialize() {
    // Test MCP initialization protocol
    let request = create_mcp_request("initialize", Some(json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {}
    })));
    
    // In a real test, we would send this to the server
    // For now, just verify the request format
    let parsed = parse_mcp_response(&request).unwrap();
    assert_eq!(parsed["method"], "initialize");
}

#[tokio::test]
async fn test_tools_list_request() {
    let request = create_mcp_request("tools/list", None);
    
    // Verify request structure
    let parsed = parse_mcp_response(&request).unwrap();
    assert_eq!(parsed["method"], "tools/list");
    assert!(parsed["params"].is_object());
}

#[tokio::test]
async fn test_tools_call_request() {
    let request = create_mcp_request("tools/call", Some(json!({
        "name": "create_document",
        "arguments": {
            "title": "Test Document",
            "text": "Test content"
        }
    })));
    
    // Verify request structure
    let parsed = parse_mcp_response(&request).unwrap();
    assert_eq!(parsed["method"], "tools/call");
    assert_eq!(parsed["params"]["name"], "create_document");
}

#[test]
fn test_mcp_response_parsing() {
    // Test success response
    let success_response = r#"{"jsonrpc":"2.0","id":1,"result":{"tools":[]}}"#;
    let parsed = parse_mcp_response(success_response).unwrap();
    assert!(is_success_response(&parsed));
    
    // Test error response
    let error_response = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32601,"message":"Method not found"}}"#;
    let parsed = parse_mcp_response(error_response).unwrap();
    assert!(!is_success_response(&parsed));
} 