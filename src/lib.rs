//! # Outline MCP Server
//!
//! MCP (Model Context Protocol) server for Outline knowledge base interaction
//! with focus on simplicity and performance.
//!
//! ## Design Principles
//!
//! - **Simplicity**: Direct functions instead of complex abstractions
//! - **Performance**: Static builds and minimal dependencies
//! - **Elegance**: One file for each area of responsibility
//!
//! ## Usage Example
//!
//! ```no_run
//! use outline_mcp_rs::{Config, run_stdio, run_http};
//!
//! #[tokio::main]
//! async fn main() -> outline_mcp_rs::Result<()> {
//!     let config = Config::from_env()?;
//!     
//!     // STDIO mode
//!     run_stdio(config.clone()).await?;
//!     
//!     // Or HTTP mode
//!     run_http(config).await
//! }
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

// Публичные экспорты
pub use config::Config;
pub use error::{Error, Result};

// Модули
pub mod config;
pub mod error;
mod mcp;
mod outline;
mod tools;

/// Run server in STDIO mode
///
/// Used for integration with MCP clients through standard input/output streams.
///
/// # Errors
///
/// Returns error on initialization or request processing problems.
pub async fn run_stdio(config: Config) -> Result<()> {
    use std::io::{self, Write};
    use tracing::{error, info};

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Initialize Outline API client
    let outline_client = outline::Client::new(config.outline_api_key, config.outline_api_url)?;

    info!("✅ STDIO server ready");

    // Main STDIO processing loop
    loop {
        let input = {
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => line.trim_end().to_string(),
                Err(e) => {
                    error!("Ошибка чтения STDIN: {}", e);
                    break;
                }
            }
        };

        if input.trim().is_empty() {
            continue;
        }

        // Обработка JSON-RPC запроса
        match mcp::handle_request(&input, &outline_client).await {
            Ok(response) => {
                writeln!(stdout, "{response}")?;
                stdout.flush()?;
            }
            Err(e) => {
                error!("Ошибка обработки запроса: {}", e);
                let error_response = mcp::create_error_response(&e);
                writeln!(stdout, "{error_response}")?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}

/// Запуск сервера в режиме HTTP
///
/// Создает веб-сервер на указанном в конфигурации хосте и порту.
///
/// # Errors
///
/// Возвращает ошибку при проблемах с привязкой к порту или HTTP transport.
pub async fn run_http(config: Config) -> Result<()> {
    use tokio::net::TcpListener;
    use tracing::{error, info};

    let addr = format!("{}:{}", config.http_host, config.http_port.as_u16());
    let listener = TcpListener::bind(&addr).await?;

    info!("🌐 HTTP сервер запущен на {}", addr);
    info!("📡 Доступен по /mcp для MCP запросов");

    // Инициализация клиента Outline API
    let outline_client = outline::Client::new(config.outline_api_key, config.outline_api_url)?;

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                info!("🔗 Новое подключение: {}", addr);
                let client = outline_client.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_http_connection(stream, client).await {
                        error!("Ошибка обработки HTTP соединения: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Ошибка принятия соединения: {}", e);
            }
        }
    }
}

/// Обработка HTTP соединения
async fn handle_http_connection(
    mut stream: tokio::net::TcpStream,
    outline_client: outline::Client,
) -> Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).await?;

    // Простая HTTP обработка
    if request_line.starts_with("POST /mcp") {
        // Читаем заголовки
        let mut content_length = 0;
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).await?;

            if line.trim().is_empty() {
                break;
            }

            if line.to_lowercase().starts_with("content-length:") {
                if let Some(len_str) = line.split(':').nth(1) {
                    content_length = len_str.trim().parse().unwrap_or(0);
                }
            }
        }

        // Читаем тело запроса
        if content_length > 0 {
            let mut buffer = vec![0; content_length];
            tokio::io::AsyncReadExt::read_exact(&mut reader, &mut buffer).await?;
            let body = String::from_utf8(buffer)?;

            // Обработка MCP запроса
            match mcp::handle_request(&body, &outline_client).await {
                Ok(response) => {
                    let http_response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        response.len(),
                        response
                    );
                    stream.write_all(http_response.as_bytes()).await?;
                }
                Err(e) => {
                    let error_response = mcp::create_error_response(&e);
                    let http_response = format!(
                        "HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        error_response.len(),
                        error_response
                    );
                    stream.write_all(http_response.as_bytes()).await?;
                }
            }
        }
    } else {
        // 404 для других путей
        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::for_testing();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_error_types() {
        let _error = Error::Config {
            message: "test error".to_string(),
            source: None,
        };

        // Тест что error типы работают корректно
        // Test passes if error creation doesn't panic
    }
}
