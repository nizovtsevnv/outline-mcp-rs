//! Collection management tools

use serde_json::{json, Value};
use tracing::debug;

use super::common::{
    create_mcp_success_response, get_optional_number_arg, get_optional_string_arg, get_string_arg,
    tool_definition,
};
use crate::error::Result;
use crate::outline::{create_collection_request, Client as OutlineClient};

/// Get all collection tool definitions
pub fn get_collection_tools() -> Vec<Value> {
    vec![
        tool_definition(
            "create_collection",
            "Create collection",
            &[
                ("name", "string", "Collection name"),
                ("description", "string", "Description (optional)"),
            ],
        ),
        tool_definition(
            "get_collection",
            "Get collection",
            &[("id", "string", "Collection ID")],
        ),
        tool_definition(
            "update_collection",
            "Update collection",
            &[
                ("id", "string", "Collection ID"),
                ("name", "string", "New name (optional)"),
                ("description", "string", "New description (optional)"),
            ],
        ),
        tool_definition(
            "list_collections",
            "List collections",
            &[("limit", "number", "Number of collections (optional)")],
        ),
    ]
}

/// Call collection tool
pub async fn call_collection_tool(
    name: &str,
    arguments: Value,
    client: &OutlineClient,
) -> Result<Value> {
    match name {
        "create_collection" => create_collection(arguments, client).await,
        "get_collection" => get_collection(arguments, client).await,
        "update_collection" => update_collection(arguments, client).await,
        "list_collections" => list_collections(arguments, client).await,
        _ => unreachable!("Unknown collection tool: {}", name),
    }
}

async fn create_collection(args: Value, client: &OutlineClient) -> Result<Value> {
    let name = get_string_arg(&args, "name")?;
    let description = get_optional_string_arg(&args, "description");

    debug!("Creating collection: {}", name);

    let request_body = create_collection_request(&name, description.as_deref());
    let response = client.post("collections.create", request_body).await?;

    Ok(create_mcp_success_response(
        "Collection created successfully",
        Some(response),
    ))
}

async fn get_collection(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;

    debug!("Getting collection: {}", id);

    let request_body = json!({ "id": id });
    let response = client.post("collections.info", request_body).await?;

    Ok(create_mcp_success_response(
        "Collection retrieved successfully",
        Some(response),
    ))
}

async fn update_collection(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let name = get_optional_string_arg(&args, "name");
    let description = get_optional_string_arg(&args, "description");

    debug!("Updating collection: {}", id);

    let mut request_body = json!({ "id": id });
    if let Some(n) = name {
        request_body["name"] = json!(n);
    }
    if let Some(d) = description {
        request_body["description"] = json!(d);
    }

    let response = client.post("collections.update", request_body).await?;

    Ok(create_mcp_success_response(
        "Collection updated successfully",
        Some(response),
    ))
}

async fn list_collections(args: Value, client: &OutlineClient) -> Result<Value> {
    let limit = get_optional_number_arg(&args, "limit");

    debug!("Listing collections");

    let mut request_body = json!({});
    if let Some(lim) = limit {
        request_body["limit"] = json!(lim);
    }

    let response = client.post("collections.list", request_body).await?;
    Ok(create_mcp_success_response(
        "Collection retrieved successfully",
        Some(response),
    ))
}
