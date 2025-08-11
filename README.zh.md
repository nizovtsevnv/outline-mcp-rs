# Outline MCP 服务器

[![CI](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/CI/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/ci.yml)
[![Release Build](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/Release%20Build/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/release.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/nizovtsevnv/outline-mcp-rs?sort=semver)](https://github.com/nizovtsevnv/outline-mcp-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

用于与 Outline API 交互的 MCP（Model Context Protocol）服务器，专注于**简洁性**、**性能**和**可靠性**。

## 🚀 快速开始

### 1. 获取您的 Outline API 密钥
- **Outline.com**: https://app.outline.com/settings/api-and-apps
- **自托管**: https://your-instance.com/settings/api-and-apps

### 2. 下载和安装

选择其中一种安装方法：

#### 🔄 选项 1：下载预构建二进制文件（推荐）
从 [GitHub Releases](https://github.com/nizovtsevnv/outline-mcp-rs/releases) 下载

**解压后：**
- **Linux/macOS**: 如需要，设置可执行权限：`chmod +x outline-mcp`
- **Windows**: 由于发布版本未经代码签名，🛡️ Windows Defender 可能会阻止执行。您需要：
  1. 通过 Windows Defender/防病毒软件允许该可执行文件
  2. 将文件夹添加到 Windows Defender 排除列表，或
  3. 右键单击文件 → 属性 → 如果从互联网下载，点击"解除阻止"

#### 📦 选项 2：从 crates.io 安装
```bash
cargo install outline-mcp-rs
```
*需要 Rust 工具链。二进制文件将安装到 `~/.cargo/bin/outline-mcp`*

#### 🔨 选项 3：从源码构建
```bash
git clone https://github.com/nizovtsevnv/outline-mcp-rs.git
cd outline-mcp-rs
cargo build --release
# 二进制文件位于 target/release/outline-mcp
```

#### ❄️ 选项 4：Nix（可重现环境）
```bash
nix run github:nizovtsevnv/outline-mcp-rs
```

### 3. 配置您的 AI 代理

Cursor IDE、Gemini CLI 的 JSON 配置：
```json
{
  "mcpServers": {
    "Outline knowledge base": {
      "command": "outline-mcp",
      "env": {
        "OUTLINE_API_KEY": "您的API密钥",
        "OUTLINE_API_URL": "https://app.getoutline.com/api"
      }
    }
  }
}
```

> **💡 路径说明：**
> - **cargo install**: 使用 `"outline-mcp"`（自动添加到 PATH）  
> - **下载的二进制文件**: 使用完整路径如 `"/path/to/outline-mcp"`
> - **从源码构建**: 使用 `"/path/to/outline-mcp-rs/target/release/outline-mcp"`

**⚠️ 重要路径要求：**
- **使用绝对路径** - 相对路径可能无法正常工作
- **路径中不要有空格**（使用下划线或连字符代替）
- **仅使用 ASCII 字符** - 避免在路径中使用中文或其他非拉丁字符
- **Windows 用户**：在路径中使用双反斜杠 `\\`（例如，`"C:\\tools\\outline-mcp.exe"`）

**✅ 良好示例：**
- Linux/macOS: `"/usr/local/bin/outline-mcp"` 或 `"/home/user/bin/outline-mcp"`
- Windows: `"C:\\tools\\outline-mcp.exe"` 或 `"C:\\Users\\YourName\\bin\\outline-mcp.exe"`

**❌ 避免：**
- `"./outline-mcp"`（相对路径）
- `"/path with spaces/outline-mcp"`（路径中有空格）
- `"/路径/outline-mcp"`（非拉丁字符）
- `"C:\tools\outline-mcp.exe"`（Windows 中的单反斜杠）

## 🛠️ 支持的工具

完整覆盖 Outline API 功能：

### 📄 文档操作
- `create_document` - 创建新文档
- `get_document` - 通过 ID 获取文档
- `update_document` - 更新现有文档
- `delete_document` - 删除文档
- `list_documents` - 列出文档并进行筛选
- `search_documents` - 按查询搜索文档
- `archive_document` - 归档文档
- `move_document` - 在集合之间移动文档

### 📁 集合管理
- `create_collection` - 创建新集合
- `get_collection` - 获取集合详情
- `update_collection` - 更新集合元数据
- `list_collections` - 列出所有集合

### 💬 评论和协作
- `create_comment` - 为文档添加评论
- `update_comment` - 修改现有评论
- `delete_comment` - 删除评论

### 🔍 高级功能
- `ask_documents` - AI 驱动的文档查询
- `create_template_from_document` - 创建可重用模板
- `list_users` - 用户管理

## 🎯 项目原则

### ⚡ 性能
- **使用 musl 的静态构建** - 无依赖的单文件
- **< 5MB 二进制文件** 具有完整功能
- **< 10ms 启动时间** 达到就绪状态
- **< 10MB 内存** 使用量

### 🛡️ 可靠性
- **运行时零依赖**（静态链接）
- **显式错误处理** - 生产环境中无 panic
- **类型安全** - 利用 Rust 的所有权系统
- **全面测试** - 单元测试和集成测试

### 🔧 简洁性
- **最小代码** - 仅包含基本功能
- **清晰架构** - 易于理解和修改
- **单一二进制文件** - 简单部署
- **环境变量配置** - 无配置文件

## 📋 开发要求
- **Nix**（推荐）- 自动处理所有依赖项
- **或手动**：Rust 1.75+、OpenSSL 开发库

## 🏗️ 架构

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   MCP 客户端    │────│    传输层        │────│  Outline API    │
│   (Claude/等)   │    │  (STDIO/HTTP)    │    │   (REST/JSON)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### 核心组件
- **传输层**：STDIO 和 HTTP 适配器
- **MCP 协议**：JSON-RPC 2.0 实现
- **Outline 客户端**：HTTP API 包装器
- **工具注册表**：动态工具发现和执行

#### 快速构建命令：
```bash
# Linux/Unix 系统
nix build                # Linux 本机构建
nix build .#musl         # Linux 静态构建（可移植）
nix build .#windows      # Windows 交叉编译

# macOS 系统（需要在 macOS 上安装 Nix）  
nix build                # 自动检测 Intel/ARM
nix build .#macos-x86_64 # Intel 目标
nix build .#macos-arm64  # ARM 目标
```

#### macOS 开发环境设置：
```bash
# 在 macOS 上安装 Nix
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install

# 启用 flakes
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf

# 克隆和构建
git clone https://github.com/nizovtsevnv/outline-mcp-rs
cd outline-mcp-rs
nix build
```

## 🧪 测试

```bash
# 运行所有测试
nix develop -c cargo test

# 运行覆盖率测试
nix develop -c cargo test --coverage

# 使用实时 API 的集成测试（设置 OUTLINE_API_KEY）
nix develop -c cargo test --test integration
```

## 🔧 配置

### STDIO 模式（默认）
```bash
export OUTLINE_API_KEY="您的密钥"
./outline-mcp
```

### HTTP 模式
```bash
export OUTLINE_API_KEY="您的密钥"
export HTTP_HOST="0.0.0.0"
export HTTP_PORT="8080"
./outline-mcp --http
```

## 🔧 优化的 Nix 配置

我们的 `flake.nix` 经过精心优化，消除重复并提高可维护性：

### 🏗️ 架构改进

- **📦 元数据同步**：包信息引用 `Cargo.toml` 值并带有注释
- **🔄 可重用 Shell 构建器**：`mkDevShell` 函数消除代码重复
- **🎯 一致的 Shell 钩子**：所有环境的统一 `mkShellHook` 函数  
- **⚡ 基础构建输入**：所有开发 shell 的共享依赖项
- **🧪 自动化检查**：内置格式化、linting 和测试工作流

### 📋 可用命令

```bash
# 开发环境
nix develop              # 带工具的本机开发
nix develop .#musl       # musl 静态构建环境  
nix develop .#windows    # Windows 交叉编译
nix develop .#macos      # macOS 开发（仅 Darwin）

# 包构建
nix build                # 本机构建（Linux/macOS 自动检测）
nix build .#musl         # 静态 musl 构建（可移植 Linux）
nix build .#windows      # Windows 交叉编译
nix build .#macos-x86_64 # macOS Intel（需要 macOS 或 CI）
nix build .#macos-arm64  # macOS Apple Silicon（需要 macOS 或 CI）

# 替代方案：使用开发环境进行构建
nix develop -c cargo build --release                              # 本机
nix develop .#musl -c cargo build --target x86_64-unknown-linux-musl --release    # musl
nix develop .#windows -c cargo build --target x86_64-pc-windows-gnu --release     # Windows

# macOS 目标（仅 macOS）
nix develop -c cargo build --target x86_64-apple-darwin --release   # Intel Mac
nix develop -c cargo build --target aarch64-apple-darwin --release  # Apple Silicon
```

## 🤝 贡献

1. **Fork** 仓库
2. **创建** 功能分支 (`git checkout -b feature/amazing-feature`)
3. **进行** 更改并添加测试
4. **确保** 所有检查通过：`cargo test && cargo clippy`
5. **提交** pull request

### 开发工作流
```bash
# 设置开发环境
nix develop

# 代码格式化
cargo fmt

# Linting
cargo clippy

# 测试
cargo test

# 跨平台测试
nix develop .#musl --command cargo test --target x86_64-unknown-linux-musl
nix develop .#windows --command cargo check --target x86_64-pc-windows-gnu
```

## 📄 许可证

MIT License - 详情请查看 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- **Outline** 团队提供的优秀 API 文档
- **Anthropic** 提供的 MCP 协议规范
- **Rust** 社区提供的出色工具和库 