//! Outline API client
//!
//! Simple HTTP client for Outline Knowledge Base API.

use reqwest::{header, Client as HttpClient};
use serde_json::Value;
use tracing::debug;
use url::Url;

use crate::config::ApiKey;
use crate::error::{Error, Result};

/// Outline API client
#[derive(Debug, Clone)]
pub struct Client {
    /// HTTP client instance
    http: HttpClient,
    /// API authentication key
    api_key: ApiKey,
    /// Base API URL
    base_url: Url,
}

impl Client {
    /// Create new Outline API client
    pub fn new(api_key: ApiKey, base_url: Url) -> Result<Self> {
        let http_client = HttpClient::builder()
            .user_agent(format!(
                "{}/{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            http: http_client,
            api_key,
            base_url,
        })
    }

    /// Execute POST request to Outline API
    pub async fn post(&self, endpoint: &str, body: Value) -> Result<Value> {
        // Ensure base_url ends with a slash for proper joining
        let mut url_string = self.base_url.to_string();
        if !url_string.ends_with('/') {
            url_string.push('/');
        }
        url_string.push_str(endpoint);
        let url = url_string.parse::<url::Url>().map_err(|e| Error::Config {
            message: format!("Invalid URL: {url_string}"),
            source: Some(Box::new(e)),
        })?;

        debug!("📤 POST request: {} | Body: {}", url, body);

        let response = self
            .http
            .post(url.clone())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.api_key.as_str()),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Execute GET request to Outline API
    #[allow(dead_code)]
    pub async fn get(&self, endpoint: &str) -> Result<Value> {
        // Ensure base_url ends with a slash for proper joining
        let mut url_string = self.base_url.to_string();
        if !url_string.ends_with('/') {
            url_string.push('/');
        }
        url_string.push_str(endpoint);
        let url = url_string.parse::<url::Url>().map_err(|e| Error::Config {
            message: format!("Invalid URL: {url_string}"),
            source: Some(Box::new(e)),
        })?;

        debug!("📥 GET request: {}", url);

        let response = self
            .http
            .get(url.clone())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.api_key.as_str()),
            )
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handle HTTP response and parse JSON
    async fn handle_response(&self, response: reqwest::Response) -> Result<Value> {
        let status = response.status();
        let status_code = status.as_u16();

        if status.is_success() {
            let text = response.text().await?;
            debug!("✅ Response: {}", text);

            serde_json::from_str(&text).map_err(|e| Error::Json {
                context: format!("Failed to parse API response: {text}"),
                source: e,
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            debug!("❌ Error response: {} - {}", status_code, error_text);

            Err(Error::Api {
                status: status_code,
                message: format!("API request failed with status {status_code}"),
                body: Some(error_text),
            })
        }
    }
}

// Helper functions for API operations

/// Create document request body
pub fn create_document_request(title: &str, text: &str, collection_id: Option<&str>) -> Value {
    let mut request = serde_json::json!({
        "title": title,
        "text": text
    });

    if let Some(cid) = collection_id {
        request["collectionId"] = serde_json::json!(cid);
    }

    request
}

/// Update document request body
pub fn update_document_request(id: &str, title: Option<&str>, text: Option<&str>) -> Value {
    let mut request = serde_json::json!({
        "id": id
    });

    if let Some(t) = title {
        request["title"] = serde_json::json!(t);
    }

    if let Some(txt) = text {
        request["text"] = serde_json::json!(txt);
    }

    request
}

/// Search documents request body
pub fn search_documents_request(query: &str, limit: Option<u32>) -> Value {
    let mut request = serde_json::json!({
        "query": query
    });

    if let Some(l) = limit {
        request["limit"] = serde_json::json!(l);
    }

    request
}

/// Create collection request body
pub fn create_collection_request(name: &str, description: Option<&str>) -> Value {
    let mut request = serde_json::json!({
        "name": name
    });

    if let Some(desc) = description {
        request["description"] = serde_json::json!(desc);
    }

    request
}

/// Create comment request body
pub fn create_comment_request(document_id: &str, data: &str) -> Value {
    serde_json::json!({
        "documentId": document_id,
        "data": data
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document_request() {
        let request = create_document_request("Test Title", "Test content", Some("collection-id"));
        assert_eq!(request["title"], "Test Title");
        assert_eq!(request["text"], "Test content");
        assert_eq!(request["collectionId"], "collection-id");
    }

    #[test]
    fn test_search_documents_request() {
        let request = search_documents_request("test query", Some(10));
        assert_eq!(request["query"], "test query");
        assert_eq!(request["limit"], 10);
    }
}
