//! Tools for working with Outline API
//!
//! Simple implementation of all 18 tools as separate async functions without complex abstractions.

use serde_json::{json, Value};
use tracing::{debug, info};

use crate::error::{Error, Result};
use crate::outline::{
    create_collection_request, create_comment_request, create_document_request,
    search_documents_request, update_document_request, Client as OutlineClient,
};

/// Get list of all available tools
#[allow(clippy::too_many_lines)]
pub fn get_tools_list() -> Vec<Value> {
    vec![
        // Документы (10 инструментов)
        tool_definition(
            "create_document",
            "Создание нового документа",
            &[
                ("title", "string", "Название документа"),
                ("text", "string", "Содержимое документа"),
                ("collection_id", "string", "ID коллекции (опционально)"),
            ],
        ),
        tool_definition(
            "get_document",
            "Получение документа по ID",
            &[("id", "string", "ID документа")],
        ),
        tool_definition(
            "update_document",
            "Обновление документа",
            &[
                ("id", "string", "ID документа"),
                ("title", "string", "Новое название (опционально)"),
                ("text", "string", "Новое содержимое (опционально)"),
            ],
        ),
        tool_definition(
            "delete_document",
            "Удаление документа",
            &[("id", "string", "ID документа")],
        ),
        tool_definition(
            "list_documents",
            "Список документов",
            &[
                ("collection_id", "string", "ID коллекции (опционально)"),
                ("limit", "number", "Количество документов (опционально)"),
            ],
        ),
        tool_definition(
            "search_documents",
            "Поиск документов",
            &[
                ("query", "string", "Поисковый запрос"),
                ("limit", "number", "Количество результатов (опционально)"),
            ],
        ),
        tool_definition(
            "ask_documents",
            "AI запрос к документам",
            &[
                ("query", "string", "Вопрос к документам"),
                (
                    "document_ids",
                    "array",
                    "Список ID документов (опционально)",
                ),
            ],
        ),
        tool_definition(
            "archive_document",
            "Архивирование документа",
            &[("id", "string", "ID документа")],
        ),
        tool_definition(
            "move_document",
            "Перемещение документа",
            &[
                ("id", "string", "ID документа"),
                ("collection_id", "string", "ID целевой коллекции"),
            ],
        ),
        tool_definition(
            "create_template_from_document",
            "Создание шаблона из документа",
            &[
                ("id", "string", "ID документа"),
                ("name", "string", "Название шаблона"),
            ],
        ),
        // Коллекции (4 инструмента)
        tool_definition(
            "create_collection",
            "Создание коллекции",
            &[
                ("name", "string", "Название коллекции"),
                ("description", "string", "Описание (опционально)"),
            ],
        ),
        tool_definition(
            "get_collection",
            "Получение коллекции",
            &[("id", "string", "ID коллекции")],
        ),
        tool_definition(
            "update_collection",
            "Обновление коллекции",
            &[
                ("id", "string", "ID коллекции"),
                ("name", "string", "Новое название (опционально)"),
                ("description", "string", "Новое описание (опционально)"),
            ],
        ),
        tool_definition(
            "list_collections",
            "Список коллекций",
            &[("limit", "number", "Количество коллекций (опционально)")],
        ),
        // Комментарии (3 инструмента)
        tool_definition(
            "create_comment",
            "Создание комментария",
            &[
                ("document_id", "string", "ID документа"),
                ("data", "string", "Содержимое комментария"),
            ],
        ),
        tool_definition(
            "update_comment",
            "Обновление комментария",
            &[
                ("id", "string", "ID комментария"),
                ("data", "string", "Новое содержимое"),
            ],
        ),
        tool_definition(
            "delete_comment",
            "Удаление комментария",
            &[("id", "string", "ID комментария")],
        ),
        // Пользователи (1 инструмент)
        tool_definition(
            "list_users",
            "Список пользователей",
            &[("limit", "number", "Количество пользователей (опционально)")],
        ),
    ]
}

/// Создание определения инструмента
fn tool_definition(name: &str, description: &str, parameters: &[(&str, &str, &str)]) -> Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for (param_name, param_type, param_desc) in parameters {
        properties.insert(
            (*param_name).to_string(),
            json!({
                "type": param_type,
                "description": param_desc
            }),
        );

        // Все параметры обязательные, кроме опциональных
        if !param_desc.contains("опционально") {
            required.push((*param_name).to_string());
        }
    }

    json!({
        "name": name,
        "description": description,
        "inputSchema": {
            "type": "object",
            "properties": properties,
            "required": required
        }
    })
}

/// Вызов инструмента по имени
pub async fn call_tool(name: &str, arguments: Value, client: &OutlineClient) -> Result<Value> {
    info!("🔨 Вызов инструмента: {}", name);
    debug!("📊 Аргументы: {}", arguments);

    match name {
        // Документы
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

        // Коллекции
        "create_collection" => create_collection(arguments, client).await,
        "get_collection" => get_collection(arguments, client).await,
        "update_collection" => update_collection(arguments, client).await,
        "list_collections" => list_collections(arguments, client).await,

        // Комментарии
        "create_comment" => create_comment(arguments, client).await,
        "update_comment" => update_comment(arguments, client).await,
        "delete_comment" => delete_comment(arguments, client).await,

        // Пользователи
        "list_users" => list_users(arguments, client).await,

        _ => Err(Error::Tool {
            tool_name: name.to_string(),
            message: "Неизвестный инструмент".to_string(),
            source: None,
        }),
    }
}

// === ДОКУМЕНТЫ ===

async fn create_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let title = get_string_arg(&args, "title")?;
    let text = get_string_arg(&args, "text")?;
    let collection_id = get_optional_string_arg(&args, "collection_id");

    let request = create_document_request(&title, &text, collection_id.as_deref());
    client.post("documents.create", request).await
}

async fn get_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let request = json!({ "id": id });
    client.post("documents.info", request).await
}

async fn update_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let title = get_optional_string_arg(&args, "title");
    let text = get_optional_string_arg(&args, "text");

    let request = update_document_request(&id, title.as_deref(), text.as_deref());
    client.post("documents.update", request).await
}

async fn delete_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let request = json!({ "id": id });
    client.post("documents.delete", request).await
}

async fn list_documents(args: Value, client: &OutlineClient) -> Result<Value> {
    let mut request = json!({});

    if let Some(collection_id) = get_optional_string_arg(&args, "collection_id") {
        request["collectionId"] = json!(collection_id);
    }

    if let Some(limit) = get_optional_number_arg(&args, "limit") {
        request["limit"] = json!(limit);
    }

    client.post("documents.list", request).await
}

async fn search_documents(args: Value, client: &OutlineClient) -> Result<Value> {
    let query = get_string_arg(&args, "query")?;
    let limit = get_optional_number_arg(&args, "limit");

    let request = search_documents_request(&query, limit.and_then(|l| u32::try_from(l).ok()));
    client.post("documents.search", request).await
}

async fn ask_documents(args: Value, client: &OutlineClient) -> Result<Value> {
    let query = get_string_arg(&args, "query")?;
    let mut request = json!({ "query": query });

    if let Some(doc_ids) = args.get("document_ids").and_then(|v| v.as_array()) {
        request["documentIds"] = json!(doc_ids);
    }

    client.post("documents.answerQuestion", request).await
}

async fn archive_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let request = json!({ "id": id });
    client.post("documents.archive", request).await
}

async fn move_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let collection_id = get_string_arg(&args, "collection_id")?;
    let request = json!({
        "id": id,
        "collectionId": collection_id
    });
    client.post("documents.move", request).await
}

async fn create_template_from_document(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let name = get_string_arg(&args, "name")?;
    let request = json!({
        "id": id,
        "name": name
    });
    client.post("documents.templatize", request).await
}

// === КОЛЛЕКЦИИ ===

async fn create_collection(args: Value, client: &OutlineClient) -> Result<Value> {
    let name = get_string_arg(&args, "name")?;
    let description = get_optional_string_arg(&args, "description");

    let request = create_collection_request(&name, description.as_deref());
    client.post("collections.create", request).await
}

async fn get_collection(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let request = json!({ "id": id });
    client.post("collections.info", request).await
}

async fn update_collection(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let mut request = json!({ "id": id });

    if let Some(name) = get_optional_string_arg(&args, "name") {
        request["name"] = json!(name);
    }

    if let Some(description) = get_optional_string_arg(&args, "description") {
        request["description"] = json!(description);
    }

    client.post("collections.update", request).await
}

async fn list_collections(args: Value, client: &OutlineClient) -> Result<Value> {
    let mut request = json!({});

    if let Some(limit) = get_optional_number_arg(&args, "limit") {
        request["limit"] = json!(limit);
    }

    client.post("collections.list", request).await
}

// === КОММЕНТАРИИ ===

async fn create_comment(args: Value, client: &OutlineClient) -> Result<Value> {
    let document_id = get_string_arg(&args, "document_id")?;
    let data = get_string_arg(&args, "data")?;

    let request = create_comment_request(&document_id, &data);
    client.post("comments.create", request).await
}

async fn update_comment(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let data = get_string_arg(&args, "data")?;
    let request = json!({
        "id": id,
        "data": data
    });
    client.post("comments.update", request).await
}

async fn delete_comment(args: Value, client: &OutlineClient) -> Result<Value> {
    let id = get_string_arg(&args, "id")?;
    let request = json!({ "id": id });
    client.post("comments.delete", request).await
}

// === ПОЛЬЗОВАТЕЛИ ===

async fn list_users(args: Value, client: &OutlineClient) -> Result<Value> {
    let mut request = json!({});

    if let Some(limit) = get_optional_number_arg(&args, "limit") {
        request["limit"] = json!(limit);
    }

    client.post("users.list", request).await
}

// === ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ ===

fn get_string_arg(args: &Value, name: &str) -> Result<String> {
    args.get(name)
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string)
        .ok_or_else(|| Error::Tool {
            tool_name: "argument_parser".to_string(),
            message: format!("Отсутствует обязательный параметр '{name}'"),
            source: None,
        })
}

fn get_optional_string_arg(args: &Value, name: &str) -> Option<String> {
    args.get(name)
        .and_then(|v| v.as_str())
        .map(std::string::ToString::to_string)
}

fn get_optional_number_arg(args: &Value, name: &str) -> Option<i64> {
    args.get(name).and_then(serde_json::Value::as_i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tools_list() {
        let tools = get_tools_list();
        assert_eq!(tools.len(), 18);

        // Проверяем первый инструмент
        let first_tool = &tools[0];
        assert_eq!(first_tool["name"], "create_document");
        assert!(first_tool["description"]
            .as_str()
            .unwrap()
            .contains("Создание"));
    }

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
}
