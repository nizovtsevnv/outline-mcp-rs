//! User management tools

use serde_json::{json, Value};
use tracing::debug;

use super::common::{get_optional_number_arg, tool_definition};
use crate::error::Result;
use crate::outline::Client as OutlineClient;

/// Get all user tool definitions
pub fn get_user_tools() -> Vec<Value> {
    vec![tool_definition(
        "list_users",
        "List users",
        &[("limit", "number", "Number of users (optional)")],
    )]
}

/// Call user tool
pub async fn call_user_tool(name: &str, arguments: Value, client: &OutlineClient) -> Result<Value> {
    match name {
        "list_users" => list_users(arguments, client).await,
        _ => unreachable!("Unknown user tool: {}", name),
    }
}

async fn list_users(args: Value, client: &OutlineClient) -> Result<Value> {
    let limit = get_optional_number_arg(&args, "limit");

    debug!("Listing users");

    let mut request_body = json!({});
    if let Some(lim) = limit {
        request_body["limit"] = json!(lim);
    }

    let response = client.post("users.list", request_body).await?;
    Ok(response)
}
