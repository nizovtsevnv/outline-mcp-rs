//! Configuration management
//!
//! Environment variable parsing and validation.

use crate::error::{Error, Result};
use std::net::IpAddr;
use url::Url;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// API key for Outline access
    pub outline_api_key: ApiKey,
    /// Outline API URL
    pub outline_api_url: Url,
    /// HTTP server IP address
    pub http_host: IpAddr,
    /// HTTP server port
    pub http_port: Port,
    /// Log level (unused but may be useful in future)
    #[allow(dead_code)]
    pub log_level: LogLevel,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are missing or invalid.
    pub fn from_env() -> Result<Self> {
        let outline_api_key = std::env::var("OUTLINE_API_KEY").map_err(|_| Error::Config {
            message: "OUTLINE_API_KEY environment variable required".to_string(),
            source: None,
        })?;

        let outline_api_url = std::env::var("OUTLINE_API_URL")
            .unwrap_or_else(|_| "https://app.getoutline.com/api".to_string());

        let http_host = std::env::var("HTTP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let http_port = std::env::var("HTTP_PORT").unwrap_or_else(|_| "3000".to_string());

        let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            outline_api_key: ApiKey::new(outline_api_key)?,
            outline_api_url: outline_api_url
                .parse()
                .map_err(|e| Error::config_with_source("Invalid OUTLINE_API_URL", e))?,
            http_host: http_host
                .parse()
                .map_err(|e| Error::config_with_source("Invalid HTTP_HOST", e))?,
            http_port: Port::new(
                http_port
                    .parse()
                    .map_err(|e| Error::config_with_source("Invalid HTTP_PORT", e))?,
            )?,
            log_level: LogLevel::new(&log_level)?,
        })
    }

    /// Simple configuration validation
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid (e.g., unsupported URL scheme).
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<()> {
        if self.outline_api_url.scheme() != "https" && self.outline_api_url.scheme() != "http" {
            return Err(Error::Config {
                message: "API URL must use HTTP or HTTPS".to_string(),
                source: None,
            });
        }

        Ok(())
    }

    /// Create configuration for tests
    ///
    /// # Panics
    ///
    /// Panics if test configuration values are invalid (should not happen in tests).
    #[cfg(test)]
    #[must_use]
    #[allow(dead_code)]
    pub fn for_testing() -> Self {
        Self {
            outline_api_key: ApiKey::new("test-api-key-12345".to_string()).unwrap(),
            outline_api_url: "https://test.example.com/api".parse().unwrap(),
            http_host: "127.0.0.1".parse().unwrap(),
            http_port: Port::new(3000).unwrap(),
            log_level: LogLevel::new("info").unwrap(),
        }
    }
}

/// Secure API key wrapper
#[derive(Debug, Clone)]
pub struct ApiKey {
    key: String,
}

impl ApiKey {
    /// Create new API key with validation
    ///
    /// # Errors
    ///
    /// Returns error if key is empty or too short.
    pub fn new(key: String) -> Result<Self> {
        if key.trim().is_empty() {
            return Err(Error::Config {
                message: "API key cannot be empty".to_string(),
                source: None,
            });
        }
        if key.len() < 10 {
            return Err(Error::Config {
                message: "API key too short (minimum 10 characters)".to_string(),
                source: None,
            });
        }
        Ok(Self { key })
    }

    /// Get API key as string reference
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.key
    }
}

/// Safe port number wrapper
#[derive(Debug, Clone, Copy)]
pub struct Port {
    port: u16,
}

impl Port {
    /// Create new port with validation
    ///
    /// # Errors
    ///
    /// Returns error if port is below 1024.
    pub fn new(port: u16) -> Result<Self> {
        if port < 1024 {
            return Err(Error::Config {
                message: "Port must be >= 1024".to_string(),
                source: None,
            });
        }
        Ok(Self { port })
    }

    /// Get port as u16 value
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.port
    }
}

/// Log level configuration
#[derive(Debug, Clone)]
pub struct LogLevel {
    level: String,
}

impl LogLevel {
    /// Create new log level with validation
    ///
    /// # Errors
    ///
    /// Returns error if log level is not one of: error, warn, info, debug, trace.
    pub fn new(level: &str) -> Result<Self> {
        let normalized = level.to_lowercase();
        if !["error", "warn", "info", "debug", "trace"].contains(&normalized.as_str()) {
            return Err(Error::Config {
                message: format!("Invalid log level: {level}"),
                source: None,
            });
        }
        Ok(Self { level: normalized })
    }

    /// Get log level as string reference
    #[allow(dead_code)]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_validation() {
        assert!(ApiKey::new(String::new()).is_err());
        assert!(ApiKey::new("short".to_string()).is_err());
        assert!(ApiKey::new("valid-api-key-123".to_string()).is_ok());
    }

    #[test]
    fn test_port_validation() {
        assert!(Port::new(80).is_err());
        assert!(Port::new(1024).is_ok());
        assert!(Port::new(8080).is_ok());
    }

    #[test]
    fn test_log_level_validation() {
        assert!(LogLevel::new("invalid").is_err());
        assert!(LogLevel::new("info").is_ok());
        assert!(LogLevel::new("DEBUG").is_ok());
    }
}
