# Outline MCP Server

MCP (Model Context Protocol) server for Outline API interaction with focus on **simplicity**, **performance**, and **reliability**.

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

## 🚀 Quick Start

### Static Build (musl)
```bash
nix develop .#musl -с сargo build --target x86_64-unknown-linux-musl --release
```

### Static build (windows)
```bash
nix develop .#windows -с cargo build --target x86_64-pc-windows-gnu --release
```

## 📋 Requirements

### Development Requirements
- **Nix** (recommended) - handles all dependencies automatically
- **OR manually**: Rust 1.75+, OpenSSL development libraries

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

## 📥 Installation

### Download Pre-built Binaries

Download the latest release from [GitHub Releases](https://github.com/nizovtsevnv/outline-mcp-rs/releases):

- **Linux x86_64**: `outline-mcp-linux-x86_64.tar.gz` (contains `outline-mcp`)
- **Linux x86_64 (musl)**: `outline-mcp-linux-x86_64-musl.tar.gz` (static, portable)
- **Windows x86_64**: `outline-mcp-windows-x86_64.zip` (contains `outline-mcp.exe`)
- **macOS Intel**: `outline-mcp-macos-x86_64.tar.gz` (contains `outline-mcp`)
- **macOS Apple Silicon**: `outline-mcp-macos-arm64.tar.gz` (contains `outline-mcp`)

Each archive includes the binary, SHA256 checksums, and usage instructions.

### Build from Source

See [Nix Configuration](#-optimized-nix-configuration) section below.

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

### 🔧 Configuration

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