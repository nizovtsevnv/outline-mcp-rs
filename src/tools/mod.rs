//! Tools for working with Outline API
//!
//! Modular implementation following "one structure - one file" principle.

use serde_json::Value;
use tracing::{debug, info};

use crate::error::{Error, Result};
use crate::outline::Client as OutlineClient;

// Submodules
mod collections;
mod comments;
mod common;
mod documents;
mod users;

/// Get list of all available tools
#[allow(clippy::too_many_lines)]
pub fn get_tools_list() -> Vec<Value> {
    let mut tools = Vec::new();

    // Document tools
    tools.extend(documents::get_document_tools());

    // Collection tools
    tools.extend(collections::get_collection_tools());

    // Comment tools
    tools.extend(comments::get_comment_tools());

    // User tools
    tools.extend(users::get_user_tools());

    tools
}

/// Call tool by name
pub async fn call_tool(name: &str, arguments: Value, client: &OutlineClient) -> Result<Value> {
    info!("🔨 Calling tool: {}", name);
    debug!("📊 Arguments: {}", arguments);

    match name {
        // Document tools
        "create_document"
        | "get_document"
        | "update_document"
        | "delete_document"
        | "list_documents"
        | "search_documents"
        | "ask_documents"
        | "archive_document"
        | "move_document"
        | "create_template_from_document" => {
            documents::call_document_tool(name, arguments, client).await
        }

        // Collection tools
        "create_collection" | "get_collection" | "update_collection" | "list_collections" => {
            collections::call_collection_tool(name, arguments, client).await
        }

        // Comment tools
        "create_comment" | "update_comment" | "delete_comment" => {
            comments::call_comment_tool(name, arguments, client).await
        }

        // User tools
        "list_users" => users::call_user_tool(name, arguments, client).await,

        _ => Err(Error::Tool {
            tool_name: name.to_string(),
            message: "Unknown tool".to_string(),
            source: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tools_list() {
        let tools = get_tools_list();
        assert_eq!(tools.len(), 18);

        // Check first tool is a document tool
        let first_tool = &tools[0];
        assert_eq!(first_tool["name"], "create_document");
        assert!(first_tool["description"]
            .as_str()
            .unwrap()
            .contains("Create"));
    }
}
