# Outline MCP Server

[![CI](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/CI/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/ci.yml)
[![Release Build](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/Release%20Build/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/release.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/nizovtsevnv/outline-mcp-rs?sort=semver)](https://github.com/nizovtsevnv/outline-mcp-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

MCP (Model Context Protocol) server for Outline API interaction with focus on **simplicity**, **performance**, and **reliability**.

Supports two transport modes:
- **STDIO** — single-user, for direct integration with MCP clients (Cursor IDE, Claude Desktop, etc.)
- **HTTP** — multi-user Streamable HTTP transport (MCP 2025-03-26 spec) with authentication, rate limiting, sessions, SSE, and CORS

## Quick Start

### 1. Get Your Outline API Key
- **Outline.com**: https://app.outline.com/settings/api-and-apps
- **Self-hosted**: https://your-instance.com/settings/api-and-apps

### 2. Download & Install

Choose one of the installation methods:

#### Option 1: Download pre-built binary (Recommended)
Download from [GitHub Releases](https://github.com/nizovtsevnv/outline-mcp-rs/releases)

**After extracting:**
- **Linux/macOS**: If needed, make executable: `chmod +x outline-mcp`
- **Windows**: Since the release is not code-signed, Windows Defender may block execution. You'll need to:
  1. Allow the executable through Windows Defender/antivirus
  2. Add the folder to Windows Defender exclusions, or
  3. Right-click the file > Properties > "Unblock" if downloaded from internet

#### Option 2: Install from crates.io
```bash
cargo install outline-mcp-rs
```
*Requires Rust toolchain. The binary will be installed to `~/.cargo/bin/outline-mcp`*

#### Option 3: Build from source
```bash
git clone https://github.com/nizovtsevnv/outline-mcp-rs.git
cd outline-mcp-rs
cargo build --release
# Binary will be in target/release/outline-mcp
```

#### Option 4: Nix (with reproducible environment)
```bash
nix run github:nizovtsevnv/outline-mcp-rs
```

### 3. Configure your AI agent (STDIO mode)

JSON configuration for Cursor IDE, Gemini CLI:
```json
{
  "mcpServers": {
    "Outline knowledge base": {
      "command": "outline-mcp",
      "env": {
        "OUTLINE_API_KEY": "your-api-key-here",
        "OUTLINE_API_URL": "https://app.getoutline.com/api"
      }
    }
  }
}
```

> **Path Notes:**
> - **cargo install**: Use `"outline-mcp"` (added to PATH automatically)
> - **Downloaded binary**: Use full path like `"/path/to/outline-mcp"`
> - **Built from source**: Use `"/path/to/outline-mcp-rs/target/release/outline-mcp"`

**Path Requirements:**
- **Use absolute paths** - relative paths may not work correctly
- **No spaces** in the executable file path (use underscores or hyphens instead)
- **ASCII characters only** - avoid non-Latin characters in paths
- **Windows users**: Use double backslashes `\\` in paths (e.g., `"C:\\tools\\outline-mcp.exe"`)

## HTTP Mode (Multi-User)

HTTP mode enables a Streamable HTTP transport server where multiple users can connect, each with their own Outline API key. The server itself is protected by MCP access tokens.

### Architecture

```
                    +--------------------------+
  Client A ------->|                          |-------> Outline API
  [X-MCP-Token]    |    MCP HTTP Server       |  (user A's key)
  [Authorization]  |                          |
                   |  - Auth (X-MCP-Token)    |
  Client B ------->|  - Rate limiting per IP  |-------> Outline API
  [X-MCP-Token]    |  - Sessions (UUID v4)    |  (user B's key)
  [Authorization]  |  - SSE streaming         |
                   |  - CORS                  |
  Client C ------->|                          |-------> Outline API
                   +--------------------------+  (user C's key)
```

### Setup

```bash
# Required: set allowed MCP access tokens (comma-separated)
export MCP_AUTH_TOKENS="token-for-alice,token-for-bob"

# Optional: configure Outline API URL (default: https://app.getoutline.com/api)
export OUTLINE_API_URL="https://your-outline.example.com/api"

# Start the server
./outline-mcp --http
```

### Client Requests

Each request must include two credentials:
- **`X-MCP-Token`** header — server access token (one of the values from `MCP_AUTH_TOKENS`)
- **`Authorization: Bearer <key>`** header — the user's own Outline API key

```bash
# Health check (no auth required)
curl http://127.0.0.1:3000/health

# Initialize session
curl -X POST http://127.0.0.1:3000/mcp \
  -H "Content-Type: application/json" \
  -H "X-MCP-Token: token-for-alice" \
  -H "Authorization: Bearer ol_api_xxxxx" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}'
# Response includes Mcp-Session-Id header

# List tools (with session)
curl -X POST http://127.0.0.1:3000/mcp \
  -H "Content-Type: application/json" \
  -H "X-MCP-Token: token-for-alice" \
  -H "Authorization: Bearer ol_api_xxxxx" \
  -H "Mcp-Session-Id: <id-from-init>" \
  -d '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":2}'

# Delete session
curl -X DELETE http://127.0.0.1:3000/mcp \
  -H "X-MCP-Token: token-for-alice" \
  -H "Mcp-Session-Id: <id-from-init>"
```

### HTTP Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | /health | No | Health check (`{"status":"ok","version":"..."}`) |
| POST | /mcp | Yes | Process MCP JSON-RPC request |
| GET | /mcp | Yes | Open SSE stream (requires `Mcp-Session-Id`) |
| DELETE | /mcp | Yes | Terminate session (requires `Mcp-Session-Id`) |
| OPTIONS | * | No | CORS preflight |

### HTTP Error Responses

| Status | Meaning |
|--------|---------|
| 401 | Missing or invalid `X-MCP-Token` / `Authorization` header |
| 413 | Request body exceeds `HTTP_MAX_BODY_SIZE` |
| 415 | `Content-Type` is not `application/json` |
| 429 | Rate limit exceeded for this IP |

## Configuration

### Environment Variables

| Variable | Mode | Required | Default | Description |
|----------|------|----------|---------|-------------|
| `OUTLINE_API_KEY` | STDIO | Yes | — | Outline API key |
| `OUTLINE_API_URL` | Both | No | `https://app.getoutline.com/api` | Outline instance URL |
| `MCP_AUTH_TOKENS` | HTTP | Yes | — | Comma-separated MCP access tokens |
| `HTTP_HOST` | HTTP | No | `127.0.0.1` | Bind address |
| `HTTP_PORT` | HTTP | No | `3000` | Port number (>= 1024) |
| `HTTP_RATE_LIMIT` | HTTP | No | `60` | Max requests/min per IP |
| `HTTP_SESSION_TIMEOUT` | HTTP | No | `1800` | Session TTL in seconds (30 min) |
| `HTTP_MAX_BODY_SIZE` | HTTP | No | `1048576` | Max request body in bytes (1 MB) |
| `RUST_LOG` | Both | No | `error` (STDIO) / `info` (HTTP) | Log level |

### STDIO Mode (Default)
```bash
export OUTLINE_API_KEY="your-key-here"
./outline-mcp
```

### HTTP Mode
```bash
export MCP_AUTH_TOKENS="my-secret-token"
./outline-mcp --http
```

## Supported Tools (25)

Complete coverage of Outline API functionality:

### Document Operations (12)
- `create_document` - Create new document
- `get_document` - Retrieve document by ID
- `update_document` - Update existing document
- `delete_document` - Delete document
- `list_documents` - List documents with filtering
- `search_documents` - Search documents by query
- `archive_document` - Archive document
- `restore_document` - Restore document from trash
- `unarchive_document` - Unarchive document
- `move_document` - Move document between collections
- `list_drafts` - List draft documents
- `create_template_from_document` - Create reusable templates

### Collection Management (6)
- `create_collection` - Create new collection
- `get_collection` - Retrieve collection details
- `update_collection` - Update collection metadata
- `delete_collection` - Delete collection
- `list_collections` - List all collections
- `get_collection_documents` - Get document structure of a collection

### Comments & Collaboration (5)
- `create_comment` - Add comment to document
- `get_comment` - Get comment by ID
- `update_comment` - Modify existing comment
- `delete_comment` - Remove comment
- `list_document_comments` - List comments for a document

### User Management (2)
- `list_users` - List team members
- `get_user` - Get user by ID

## Architecture

```
┌─────────────────┐    ┌──────────────────────────────┐    ┌─────────────────┐
│   MCP Client    │────│      Transport Layer          │────│  Outline API    │
│ (Claude/Cursor) │    │  STDIO | Streamable HTTP      │    │   (REST/JSON)   │
└─────────────────┘    └──────────────────────────────┘    └─────────────────┘
```

### Source Layout

```
src/
├── main.rs          # Entry point, logging init
├── lib.rs           # run_stdio(), run_http()
├── cli.rs           # CLI argument parsing
├── config.rs        # Environment variable configuration
├── error.rs         # Centralized error types
├── mcp.rs           # MCP JSON-RPC 2.0 protocol handler
├── outline.rs       # Outline API HTTP client
├── tools/           # MCP tool implementations
│   ├── mod.rs       # Tool registry & dispatcher
│   ├── common.rs    # Shared tool utilities
│   ├── documents.rs # Document operations (12 tools)
│   ├── collections.rs # Collection operations (6 tools)
│   ├── comments.rs  # Comment operations (5 tools)
│   └── users.rs     # User operations (2 tools)
└── http/            # Streamable HTTP transport
    ├── mod.rs       # HttpBody enum, module declarations
    ├── server.rs    # HttpServer, AppState, graceful shutdown
    ├── router.rs    # Request routing by method + path
    ├── handler.rs   # MCP POST/GET/DELETE handlers
    ├── auth.rs      # Token validation + rate limiting
    ├── session.rs   # UUID v4 session management with TTL
    ├── sse.rs       # SSE body + keepalive encoding
    ├── cors.rs      # CORS header management
    ├── request.rs   # Request validation (Content-Type, size)
    ├── response.rs  # HTTP response builders (status codes)
    └── health.rs    # GET /health endpoint
```

## Project Principles

### Performance
- **Static builds** with musl/glibc - single file without dependencies
- **< 5MB binary** with full functionality
- **< 10ms startup time** to ready state
- **< 10MB memory** usage

### Reliability
- **Zero dependencies** at runtime (static linking)
- **Explicit error handling** - no panics in production
- **Type safety** - leveraging Rust's ownership system
- **Comprehensive testing** - unit and integration tests

### Simplicity
- **Minimal code** - only essential functionality
- **Clear architecture** - easy to understand and modify
- **Single binary** - simple deployment
- **Environment configuration** - no config files

## Development

### Requirements
- **Nix** (recommended) - handles all dependencies automatically
- **OR manually**: Rust 1.75+, OpenSSL development libraries

### Build Commands

```bash
# Development environments
nix develop              # Native development with tools
nix develop .#musl       # musl static build environment
nix develop .#windows    # Windows cross-compilation
nix develop .#macos      # macOS development (Darwin only)

# Package building
nix build                # Native build (Linux/macOS auto-detect)
nix build .#musl         # Static musl build (portable Linux)
nix build .#glibc-optimized # Optimized glibc build
nix build .#windows      # Windows cross-compilation
nix build .#macos-x86_64 # macOS Intel
nix build .#macos-arm64  # macOS Apple Silicon
```

### Testing

```bash
# Run all tests
nix develop -c cargo test

# Integration tests
nix develop -c cargo test --test http_transport_tests
nix develop -c cargo test --test mcp_tests
```

### Development Workflow

```bash
nix develop
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## Contributing

1. **Fork** the repository
2. **Create** feature branch (`git checkout -b feature/amazing-feature`)
3. **Make** changes with tests
4. **Ensure** all checks pass: `cargo fmt && cargo clippy -- -D warnings && cargo test`
5. **Submit** pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Outline** team for excellent API documentation
- **Anthropic** for MCP protocol specification
- **Rust** community for outstanding tooling and libraries
