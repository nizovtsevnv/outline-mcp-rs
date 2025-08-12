//! MCP (Model Context Protocol) handler
//!
//! Simple JSON-RPC 2.0 and MCP protocol implementation without complex abstractions.

use serde_json::{json, Map, Value};
use tracing::{debug, error};

use crate::error::{Error, Result};
use crate::outline::Client as OutlineClient;
use crate::tools;

/// Handle MCP request
pub async fn handle_request(request: &str, outline_client: &OutlineClient) -> Result<Option<String>> {
    debug!("📨 Received request: {}", request);

    // Parse JSON-RPC request
    let request_json: Value = serde_json::from_str(request)?;

    // Extract main JSON-RPC fields
    let method = request_json
        .get("method")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Protocol {
            protocol: "JSON-RPC".to_string(),
            message: "Missing 'method' field".to_string(),
            code: Some(-32600),
        })?;

    let params = request_json.get("params").cloned().unwrap_or(Value::Null);
    let id = request_json.get("id").cloned();

    debug!("🔧 Processing method: {}", method);

    // Dispatch MCP methods
    let result = match method {
        // MCP initialization
        "initialize" => Ok(Some(handle_initialize(params))),

        // Get tools list
        "tools/list" => Ok(Some(handle_tools_list(params))),

        // Call tool
        "tools/call" => handle_tools_call(params, outline_client).await.map(Some),

        // Notifications (no response required)
        "notifications/initialized" => {
            debug!("🔔 Client initialization notification received");
            return Ok(None); // Return None for notifications - no response needed
        }

        // Unknown method
        _ => {
            error!("❌ Unknown method: {}", method);
            Err(Error::Protocol {
                protocol: "MCP".to_string(),
                message: format!("Unknown method: {method}"),
                code: Some(-32601),
            })
        }
    };

    // Create JSON-RPC response
    match result {
        Ok(Some(result_value)) => {
            let response = create_success_response(id.as_ref(), &result_value);
            let response_str = serde_json::to_string(&response)?;
            debug!("📤 Sending response: {}", response_str);
            Ok(Some(response_str))
        }
        Ok(None) => {
            // No response needed (for notifications)
            Ok(None)
        }
        Err(error) => {
            let response = create_error_response_with_id(id.as_ref(), &error);
            let response_str = serde_json::to_string(&response)?;
            debug!("📤 Sending error response: {}", response_str);
            Ok(Some(response_str))
        }
    }
}

/// Create success response
pub fn create_success_response(id: Option<&Value>, result: &Value) -> Value {
    id.map_or_else(
        || {
            json!({
                "jsonrpc": "2.0",
                "result": result
            })
        },
        |id_val| {
            json!({
                "jsonrpc": "2.0",
                "result": result,
                "id": id_val
            })
        },
    )
}

/// Create error response
pub fn create_error_response(error: &Error) -> String {
    let response = json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32603,
            "message": error.to_string()
        },
        "id": null
    });

    serde_json::to_string(&response).unwrap_or_else(|_| {
        r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"},"id":null}"#
            .to_string()
    })
}

/// Create error response with ID
pub fn create_error_response_with_id(id: Option<&Value>, error: &Error) -> Value {
    id.map_or_else(
        || {
            json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": -32603,
                    "message": error.to_string()
                }
            })
        },
        |id_val| {
            json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": -32603,
                    "message": error.to_string()
                },
                "id": id_val
            })
        },
    )
}

/// Handle MCP initialization
fn handle_initialize(_params: Value) -> Value {
    debug!("🚀 MCP server initialization");

    json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": env!("CARGO_PKG_NAME"),
            "version": env!("CARGO_PKG_VERSION")
        }
    })
}

/// Handle tools list request
fn handle_tools_list(_params: Value) -> Value {
    debug!("📋 Getting tools list");

    let tools_list = tools::get_tools_list();

    json!({
        "tools": tools_list
    })
}

/// Handle tool call
async fn handle_tools_call(params: Value, outline_client: &OutlineClient) -> Result<Value> {
    // Extract tool name and arguments
    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Protocol {
            protocol: "MCP".to_string(),
            message: "Missing 'name' parameter".to_string(),
            code: Some(-32602),
        })?;

    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| Value::Object(Map::new()));

    debug!("🔨 Calling tool: {}", name);
    debug!("📊 Arguments: {}", arguments);

    // Call appropriate tool
    tools::call_tool(name, arguments, outline_client).await
}
