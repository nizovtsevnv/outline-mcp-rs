# Outline MCP Server

[![CI](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/CI/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/ci.yml)
[![Release Build](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/Release%20Build/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/release.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/nizovtsevnv/outline-mcp-rs?sort=semver)](https://github.com/nizovtsevnv/outline-mcp-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

MCP (Model Context Protocol) server for Outline API interaction with focus on **simplicity**, **performance**, and **reliability**.

## 🚀 Quick Start

### 1. Get Your Outline API Key
- **Outline.com**: https://app.outline.com/settings/api-and-apps
- **Self-hosted**: https://your-instance.com/settings/api-and-apps

### 2. Download & Install
Download pre-built binary from [GitHub Releases](https://github.com/nizovtsevnv/outline-mcp-rs/releases) or build from source.

**📦 After extracting:**
- **Linux/macOS**: If needed, make executable: `chmod +x outline-mcp`
- **Windows** Since the release is not code-signed, 🛡️ Windows Defender may block execution. You'll need to:
1. Allow the executable through Windows Defender/antivirus
2. Add the folder to Windows Defender exclusions, or
3. Right-click the file → Properties → "Unblock" if downloaded from internet

### 3. Configure your AI agent

JSON configuration for Cursor IDE, Gemini CLI:
```json
{
  "mcpServers": {
    "Outline knowledge base": {
      "command": "full-location-of-outline-mcp-executable-file",
      "env": {
        "OUTLINE_API_KEY": "your-api-key-here",
        "OUTLINE_API_URL": "https://app.getoutline.com/api"
      }
    }
  }
}
```

**⚠️ Important Path Requirements:**
- **Use absolute paths** - relative paths may not work correctly
- **No spaces** in the executable file path (use underscores or hyphens instead)
- **ASCII characters only** - avoid non-Latin characters in paths
- **Windows users**: Use double backslashes `\\` in paths (e.g., `"C:\\tools\\outline-mcp.exe"`)

**✅ Good examples:**
- Linux/macOS: `"/usr/local/bin/outline-mcp"` or `"/home/user/bin/outline-mcp"`
- Windows: `"C:\\tools\\outline-mcp.exe"` or `"C:\\Users\\YourName\\bin\\outline-mcp.exe"`

**❌ Avoid:**
- `"./outline-mcp"` (relative path)
- `"/path with spaces/outline-mcp"` (spaces in path)
- `"/путь/outline-mcp"` (non-Latin characters)
- `"C:\tools\outline-mcp.exe"` (single backslash on Windows)

## 🛠️ Supported Tools

Complete coverage of Outline API functionality:

### 📄 Document Operations
- `create_document` - Create new document
- `get_document` - Retrieve document by ID
- `update_document` - Update existing document
- `delete_document` - Delete document
- `list_documents` - List documents with filtering
- `search_documents` - Search documents by query
- `archive_document` - Archive document
- `move_document` - Move document between collections

### 📁 Collection Management
- `create_collection` - Create new collection
- `get_collection` - Retrieve collection details
- `update_collection` - Update collection metadata
- `list_collections` - List all collections

### 💬 Comments & Collaboration
- `create_comment` - Add comment to document
- `update_comment` - Modify existing comment
- `delete_comment` - Remove comment

### 🔍 Advanced Features
- `ask_documents` - AI-powered document queries
- `create_template_from_document` - Create reusable templates
- `list_users` - User management

## 🎯 Project Principles

### ⚡ Performance
- **Static builds** with musl - single file without dependencies
- **< 5MB binary** with full functionality
- **< 10ms startup time** to ready state
- **< 10MB memory** usage

### 🛡️ Reliability
- **Zero dependencies** at runtime (static linking)
- **Explicit error handling** - no panics in production
- **Type safety** - leveraging Rust's ownership system
- **Comprehensive testing** - unit and integration tests

### 🔧 Simplicity
- **Minimal code** - only essential functionality
- **Clear architecture** - easy to understand and modify
- **Single binary** - simple deployment
- **Environment configuration** - no config files

## 📋 Development Requirements
- **Nix** (recommended) - handles all dependencies automatically
- **OR manually**: Rust 1.75+, OpenSSL development libraries

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   MCP Client    │────│  Transport Layer │────│  Outline API    │
│   (Claude/etc)  │    │  (STDIO/HTTP)    │    │   (REST/JSON)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Core Components
- **Transport Layer**: STDIO and HTTP adapters
- **MCP Protocol**: JSON-RPC 2.0 implementation
- **Outline Client**: HTTP API wrapper
- **Tools Registry**: Dynamic tool discovery and execution

#### Quick Build Commands:
```bash
# Linux/Unix systems
nix build                # Linux native
nix build .#musl         # Linux static (portable)
nix build .#windows      # Windows cross-compile

# macOS systems (requires Nix on macOS)  
nix build                # Auto-detects Intel/ARM
nix build .#macos-x86_64 # Intel target
nix build .#macos-arm64  # ARM target
```

#### macOS Development Setup:
```bash
# Install Nix on macOS
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install

# Enable flakes
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf

# Clone and build
git clone https://github.com/nizovtsevnv/outline-mcp-rs
cd outline-mcp-rs
nix build
```

📖 **For detailed macOS development instructions, see [docs/MACOS.md](docs/MACOS.md)**  
🔐 **For Windows code signing setup, see [docs/WINDOWS_SIGNING.md](docs/WINDOWS_SIGNING.md)**

## 🧪 Testing

```bash
# Run all tests
nix develop -c cargo test

# Run with coverage
nix develop -c cargo test --coverage

# Integration tests with live API (set OUTLINE_API_KEY)
nix develop -c cargo test --test integration
```

## 🔧 Configuration

### STDIO Mode (Default)
```bash
export OUTLINE_API_KEY="your-key-here"
./outline-mcp
```

### HTTP Mode
```bash
export OUTLINE_API_KEY="your-key-here"
export HTTP_HOST="0.0.0.0"
export HTTP_PORT="8080"
./outline-mcp --http
```

## 🔧 Optimized Nix Configuration

Our `flake.nix` has been carefully optimized to eliminate duplication and improve maintainability:

### 🏗️ Architecture Improvements

- **📦 Metadata Sync**: Package information references `Cargo.toml` values with comments
- **🔄 Reusable Shell Builder**: `mkDevShell` function eliminates code duplication
- **🎯 Consistent Shell Hooks**: Unified `mkShellHook` function for all environments  
- **⚡ Base Build Inputs**: Shared dependencies across all development shells
- **🧪 Automated Checks**: Built-in formatting, linting, and testing workflows

### 📋 Available Commands

```bash
# Development environments
nix develop              # Native development with tools
nix develop .#musl       # musl static build environment  
nix develop .#windows    # Windows cross-compilation
nix develop .#macos      # macOS development (Darwin only)

# Package building
nix build                # Native build (Linux/macOS auto-detect)
nix build .#musl         # Static musl build (portable Linux)
nix build .#windows      # Windows cross-compilation
nix build .#macos-x86_64 # macOS Intel (requires macOS or CI)
nix build .#macos-arm64  # macOS Apple Silicon (requires macOS or CI)

# Alternative: Use dev environment for building
nix develop -c cargo build --release                              # Native
nix develop .#musl -c cargo build --target x86_64-unknown-linux-musl --release    # musl
nix develop .#windows -c cargo build --target x86_64-pc-windows-gnu --release     # Windows

# macOS targets (macOS only)
nix develop -c cargo build --target x86_64-apple-darwin --release   # Intel Mac
nix develop -c cargo build --target aarch64-apple-darwin --release  # Apple Silicon
```

## 🤝 Contributing

1. **Fork** the repository
2. **Create** feature branch (`git checkout -b feature/amazing-feature`)
3. **Make** changes with tests
4. **Ensure** all checks pass: `cargo test && cargo clippy`
5. **Submit** pull request

### Development Workflow
```bash
# Setup development environment
nix develop

# Code formatting
cargo fmt

# Linting
cargo clippy

# Testing
cargo test

# Cross-platform testing
nix develop .#musl --command cargo test --target x86_64-unknown-linux-musl
nix develop .#windows --command cargo check --target x86_64-pc-windows-gnu
```

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Outline** team for excellent API documentation
- **Anthropic** for MCP protocol specification
- **Rust** community for outstanding tooling and libraries 