//! Common utilities for tool implementations

use crate::error::{Error, Result};
use serde_json::{json, Value};

/// Create tool definition JSON
pub fn tool_definition(name: &str, description: &str, params: &[(&str, &str, &str)]) -> Value {
    let properties = params
        .iter()
        .map(|(name, param_type, desc)| {
            (
                (*name).to_string(),
                json!({
                    "type": param_type,
                    "description": desc
                }),
            )
        })
        .collect::<serde_json::Map<String, Value>>();

    json!({
        "name": name,
        "description": description,
        "inputSchema": {
            "type": "object",
            "properties": properties,
            "required": params.iter().map(|(name, _, _)| name).collect::<Vec<_>>()
        }
    })
}

/// Extract string argument from JSON arguments
pub fn get_string_arg(args: &Value, name: &str) -> Result<String> {
    args.get(name)
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string)
        .ok_or_else(|| Error::Tool {
            tool_name: "argument_parser".to_string(),
            message: format!("Missing required parameter '{name}'"),
            source: None,
        })
}

/// Extract optional string argument from JSON arguments
pub fn get_optional_string_arg(args: &Value, name: &str) -> Option<String> {
    args.get(name)
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string)
}

/// Extract optional number argument from JSON arguments
pub fn get_optional_number_arg(args: &Value, name: &str) -> Option<i64> {
    args.get(name).and_then(serde_json::Value::as_i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_string_arg() {
        let args = json!({
            "title": "Test Title"
        });

        assert_eq!(get_string_arg(&args, "title").unwrap(), "Test Title");
        assert!(get_string_arg(&args, "missing").is_err());
    }

    #[test]
    fn test_get_optional_string_arg() {
        let args = json!({
            "title": "Test Title"
        });

        assert_eq!(
            get_optional_string_arg(&args, "title"),
            Some("Test Title".to_string())
        );
        assert_eq!(get_optional_string_arg(&args, "missing"), None);
    }

    #[test]
    fn test_tool_definition() {
        let tool = tool_definition(
            "test_tool",
            "Test description",
            &[("param1", "string", "First parameter")],
        );

        assert_eq!(tool["name"], "test_tool");
        assert_eq!(tool["description"], "Test description");
        assert!(tool["inputSchema"]["properties"]["param1"].is_object());
    }
}
