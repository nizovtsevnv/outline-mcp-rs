//! Error handling
//!
//! Centralized error types for the application.

/// Application result type
pub type Result<T> = std::result::Result<T, Error>;

/// Application error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config {
        /// Error description
        message: String,
        /// Optional source error
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Network and HTTP errors
    #[error("Network error: {context}")]
    Network {
        /// Operation context
        context: String,
        /// Source error
        #[source]
        source: reqwest::Error,
    },

    /// API response errors
    #[error("API error: {status} - {message}")]
    Api {
        /// HTTP status code
        status: u16,
        /// Error message
        message: String,
        /// Response body (if available)
        body: Option<String>,
    },

    /// Protocol errors (MCP, JSON-RPC)
    #[error("Protocol error: {protocol} - {message}")]
    Protocol {
        /// Protocol type
        protocol: String,
        /// Error message
        message: String,
        /// JSON-RPC error code (if applicable)
        code: Option<i32>,
    },

    /// Tool execution errors
    #[error("Tool error '{tool_name}': {message}")]
    Tool {
        /// Tool name
        tool_name: String,
        /// Error message
        message: String,
        /// Source error
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Transport layer errors (STDIO, HTTP)
    #[error("Transport error: {transport_type}")]
    Transport {
        /// Transport type (STDIO, HTTP, etc.)
        transport_type: String,
        /// Source error
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// JSON parsing and validation errors
    #[error("JSON error: {context}")]
    Json {
        /// Operation context
        context: String,
        /// Source error
        #[source]
        source: serde_json::Error,
    },

    /// Input validation errors
    #[allow(dead_code)]
    #[error("Validation error: {field} - {message}")]
    Validation {
        /// Field with error
        field: String,
        /// Validation error description
        message: String,
    },

    /// Serialization/deserialization errors
    #[allow(dead_code)]
    #[error("Serialization error: {context}")]
    Serialization {
        /// Operation context
        context: String,
        /// Source error
        #[source]
        source: serde_json::Error,
    },

    /// I/O operation errors
    #[error("I/O error: {operation}")]
    Io {
        /// Operation description
        operation: String,
        /// Source error
        #[source]
        source: std::io::Error,
    },

    /// Internal application errors
    #[allow(dead_code)]
    #[error("Internal error: {message}")]
    Internal {
        /// Error description
        message: String,
        /// Optional context
        context: Option<String>,
    },
}

impl Error {
    /// Create configuration error
    #[allow(dead_code)]
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            source: None,
        }
    }

    /// Create configuration error with source
    pub fn config_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Config {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Create transport error
    #[allow(dead_code)]
    pub fn transport(
        transport_type: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Transport {
            transport_type: transport_type.into(),
            source: Box::new(source),
        }
    }

    /// Create network error
    #[allow(dead_code)]
    pub fn network(context: impl Into<String>, source: reqwest::Error) -> Self {
        Self::Network {
            context: context.into(),
            source,
        }
    }

    /// Create API error
    #[allow(dead_code)]
    pub fn api(status: u16, message: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
            body: None,
        }
    }

    /// Create API error with response body
    #[allow(dead_code)]
    pub fn api_with_body(status: u16, message: impl Into<String>, body: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
            body: Some(body.into()),
        }
    }

    /// Create JSON error
    #[allow(dead_code)]
    pub fn json(context: impl Into<String>, source: serde_json::Error) -> Self {
        Self::Json {
            context: context.into(),
            source,
        }
    }

    /// Create I/O error
    #[allow(dead_code)]
    pub fn io(operation: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            operation: operation.into(),
            source,
        }
    }
}

// Automatic conversions for common error types
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Network {
            context: "HTTP request failed".to_string(),
            source: err,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json {
            context: "JSON operation failed".to_string(),
            source: err,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io {
            operation: "I/O operation failed".to_string(),
            source: err,
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::Config {
            message: format!("Invalid URL format: {err}"),
            source: Some(Box::new(err)),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Transport {
            transport_type: "String conversion".to_string(),
            source: Box::new(err),
        }
    }
}
