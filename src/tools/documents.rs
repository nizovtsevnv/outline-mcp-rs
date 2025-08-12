//! Document management tools

use serde_json::{json, Value};
use tracing::debug;

use super::common::{
    create_mcp_success_response, get_optional_number_arg, get_optional_string_arg, get_string_arg,
    tool_definition,
};
use crate::error::Result;
use crate::outline::{
    create_document_request, search_documents_request, update_document_request,
    Client as OutlineClient,
};

/// Get all document tool definitions
pub fn get_document_tools() -> Vec<Value> {
    vec![
        tool_definition(
            "create_document",
            "Create new document",
            &[
                ("title", "string", "Document title"),
                ("text", "string", "Document content"),
                ("collection_id", "string", "Collection ID (optional)"),
            ],
        ),
        tool_definition(
            "get_document",
            "Get document by ID",
            &[("id", "string", "Document ID")],
        ),
        tool_definition(
            "update_document",
            "Update document",
            &[
                ("id", "string", "Document ID"),
                ("title", "string", "New title (optional)"),
                ("text", "string", "New content (optional)"),
            ],
        ),
        tool_definition(
            "delete_document",
            "Delete document",
            &[("id", "string", "Document ID")],
        ),
        tool_definition(
            "list_documents",
            "List documents",
            &[
                ("collection_id", "string", "Collection ID (optional)"),
                ("limit", "number", "Number of documents (optional)"),
            ],
        ),
        tool_definition(
            "search_documents",
            "Search documents",
            &[
                ("query", "string", "Search query"),
                ("limit", "number", "Number of results (optional)"),
            ],
        ),
        tool_definition(
            "ask_documents",
            "AI query to documents",
            &[
                ("query", "string", "Question to documents"),
                ("document_ids", "array", "List of document IDs (optional)"),
            ],
        ),
        tool_definition(
            "archive_document",
            "Archive document",
            &[("id", "string", "Document ID")],
        ),
        tool_definition(
            "move_document",
            "Move document",
            &[
                ("id", "string", "Document ID"),
                ("collection_id", "string", "Target collection ID"),
            ],
        ),
        tool_definition(
            "create_template_from_document",
            "Create template from document",
            &[
                ("id", "string", "Document ID"),
                ("name", "string", "Template name"),
            ],
        ),
    ]
}

/// Call document tool
pub async fn call_document_tool(
    name: &str,
    arguments: Value,
    client: &OutlineClient,
) -> Result<Value> {
    match name {
        "create_document" => create_document(arguments, client).await,
        "get_document" => get_document(arguments, client).await,
        "update_document" => update_document(arguments, client).await,
        "delete_document" => delete_document(arguments, client).await,
        "list_documents" => list_documents(arguments, client).await,
        "search_documents" => search_documents(arguments, client).await,
        "ask_documents" => ask_documents(arguments, client).await,
        "archive_document" => archive_document(arguments, client).await,
        "move_document" => move_document(arguments, client).await,
        "create_template_from_document" => create_template_from_document(arguments, client).await,
        _ => unreachable!("Unknown document tool: {}", name),
    }
}

async fn create_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let title = get_string_arg(&args, "title")?;
    let text = get_string_arg(&args, "text")?;
    let collection_id = get_optional_string_arg(&args, "collection_id");

    debug!("Creating document: {}", title);

    let request_body = create_document_request(&title, &text, collection_id.as_deref());
    let response = client.post("documents.create", request_body).await?;

    Ok(create_mcp_success_response(
        "Document created successfully",
        Some(response),
    ))
}

async fn get_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;

    debug!("Getting document: {}", id);

    let request_body = json!({ "id": id });
    let response = client.post("documents.info", request_body).await?;

    Ok(create_mcp_success_response(
        "Document retrieved successfully",
        Some(response),
    ))
}

async fn update_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let title = get_optional_string_arg(&args, "title");
    let text = get_optional_string_arg(&args, "text");

    debug!("Updating document: {}", id);

    let request_body = update_document_request(&id, title.as_deref(), text.as_deref());
    let response = client.post("documents.update", request_body).await?;

    Ok(create_mcp_success_response(
        "Document updated successfully",
        Some(response),
    ))
}

async fn delete_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;

    debug!("Deleting document: {}", id);

    let request_body = json!({ "id": id });
    let response = client.post("documents.delete", request_body).await?;

    Ok(create_mcp_success_response(
        "Document deleted successfully",
        Some(response),
    ))
}

async fn list_documents(args: Value, client: &OutlineClient) -> Result<Value> {
    let collection_id = get_optional_string_arg(&args, "collection_id");
    let limit = get_optional_number_arg(&args, "limit");

    debug!("Listing documents");

    let mut request_body = json!({});
    if let Some(cid) = collection_id {
        request_body["collection_id"] = json!(cid);
    }
    if let Some(lim) = limit {
        request_body["limit"] = json!(lim);
    }

    let response = client.post("documents.list", request_body).await?;
    Ok(create_mcp_success_response(
        "Documents listed successfully",
        Some(response),
    ))
}

async fn search_documents(args: Value, client: &OutlineClient) -> Result<Value> {
    let query = get_string_arg(&args, "query")?;
    let limit = get_optional_number_arg(&args, "limit").and_then(|l| u32::try_from(l).ok());

    debug!("Searching documents: {}", query);

    let request_body = search_documents_request(&query, limit);
    let response = client.post("documents.search", request_body).await?;

    Ok(create_mcp_success_response(
        "Documents searched successfully",
        Some(response),
    ))
}

async fn ask_documents(args: Value, client: &OutlineClient) -> Result<Value> {
    let query = get_string_arg(&args, "query")?;
    let document_ids = args
        .get("document_ids")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>());

    debug!("AI query to documents: {}", query);

    let mut request_body = json!({ "query": query });
    if let Some(ids) = document_ids {
        request_body["document_ids"] = json!(ids);
    }

    let response = client.post("documents.ask", request_body).await?;
    Ok(create_mcp_success_response(
        "AI query completed successfully",
        Some(response),
    ))
}

async fn archive_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;

    debug!("Archiving document: {}", id);

    let request_body = json!({ "id": id });
    let response = client.post("documents.archive", request_body).await?;

    Ok(create_mcp_success_response(
        "Document archived successfully",
        Some(response),
    ))
}

async fn move_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let collection_id = get_string_arg(&args, "collection_id")?;

    debug!("Moving document {} to collection {}", id, collection_id);

    let request_body = json!({
        "id": id,
        "collection_id": collection_id
    });
    let response = client.post("documents.move", request_body).await?;

    Ok(create_mcp_success_response(
        "Document moved successfully",
        Some(response),
    ))
}

async fn create_template_from_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let name = get_string_arg(&args, "name")?;

    debug!("Creating template from document {}: {}", id, name);

    let request_body = json!({
        "id": id,
        "name": name
    });
    let response = client
        .post("documents.create_template", request_body)
        .await?;

    Ok(create_mcp_success_response(
        "Template created successfully",
        Some(response),
    ))
}
