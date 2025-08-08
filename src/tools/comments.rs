//! Comment management tools

use serde_json::{json, Value};
use tracing::debug;

use super::common::{get_string_arg, tool_definition};
use crate::error::Result;
use crate::outline::{create_comment_request, Client as OutlineClient};

/// Get all comment tool definitions
pub fn get_comment_tools() -> Vec<Value> {
    vec![
        tool_definition(
            "create_comment",
            "Create comment",
            &[
                ("document_id", "string", "Document ID"),
                ("data", "string", "Comment content"),
            ],
        ),
        tool_definition(
            "update_comment",
            "Update comment",
            &[
                ("id", "string", "Comment ID"),
                ("data", "string", "New content"),
            ],
        ),
        tool_definition(
            "delete_comment",
            "Delete comment",
            &[("id", "string", "Comment ID")],
        ),
    ]
}

/// Call comment tool
pub async fn call_comment_tool(
    name: &str,
    arguments: Value,
    client: &OutlineClient,
) -> Result<Value> {
    match name {
        "create_comment" => create_comment(arguments, client).await,
        "update_comment" => update_comment(arguments, client).await,
        "delete_comment" => delete_comment(arguments, client).await,
        _ => unreachable!("Unknown comment tool: {}", name),
    }
}

async fn create_comment(args: Value, client: &OutlineClient) -> Result<Value> {
    let document_id = get_string_arg(&args, "document_id")?;
    let data = get_string_arg(&args, "data")?;

    debug!("Creating comment for document: {}", document_id);

    let request_body = create_comment_request(&document_id, &data);
    let response = client.post("comments.create", request_body).await?;

    Ok(json!({
        "success": true,
        "comment": response
    }))
}

async fn update_comment(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let data = get_string_arg(&args, "data")?;

    debug!("Updating comment: {}", id);

    let request_body = json!({
        "id": id,
        "data": data
    });
    let response = client.post("comments.update", request_body).await?;

    Ok(json!({
        "success": true,
        "comment": response
    }))
}

async fn delete_comment(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;

    debug!("Deleting comment: {}", id);

    let request_body = json!({ "id": id });
    let response = client.post("comments.delete", request_body).await?;

    Ok(json!({
        "success": true,
        "result": response
    }))
}
