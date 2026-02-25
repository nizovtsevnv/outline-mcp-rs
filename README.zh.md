# Outline MCP 服务器

[![CI](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/CI/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/ci.yml)
[![Release Build](https://github.com/nizovtsevnv/outline-mcp-rs/workflows/Release%20Build/badge.svg)](https://github.com/nizovtsevnv/outline-mcp-rs/actions/workflows/release.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/nizovtsevnv/outline-mcp-rs?sort=semver)](https://github.com/nizovtsevnv/outline-mcp-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

用于与 Outline API 交互的 MCP（Model Context Protocol）服务器，专注于**简洁性**、**性能**和**可靠性**。

支持两种传输模式：
- **STDIO** — 单用户模式，用于与 MCP 客户端（Cursor IDE、Claude Desktop 等）直接集成
- **HTTP** — 多用户 Streamable HTTP 传输（MCP 2025-03-26 规范），支持认证、速率限制、会话管理、SSE 和 CORS

## 快速开始

### 1. 获取 Outline API 密钥
- **Outline.com**: https://app.outline.com/settings/api-and-apps
- **自托管**: https://your-instance.com/settings/api-and-apps

### 2. 下载和安装

选择以下安装方法之一：

#### 选项 1：下载预构建二进制文件（推荐）
从 [GitHub Releases](https://github.com/nizovtsevnv/outline-mcp-rs/releases) 下载

**解压后：**
- **Linux/macOS**: 如需要，设置可执行权限：`chmod +x outline-mcp`
- **Windows**: 由于发布版本未经代码签名，Windows Defender 可能会阻止执行。您需要：
  1. 通过 Windows Defender/防病毒软件允许该可执行文件
  2. 将文件夹添加到 Windows Defender 排除列表，或
  3. 右键单击文件 > 属性 > 如果从互联网下载，点击"解除阻止"

#### 选项 2：从 crates.io 安装
```bash
cargo install outline-mcp-rs
```
*需要 Rust 工具链。二进制文件将安装到 `~/.cargo/bin/outline-mcp`*

#### 选项 3：从源码构建
```bash
git clone https://github.com/nizovtsevnv/outline-mcp-rs.git
cd outline-mcp-rs
cargo build --release
# 二进制文件位于 target/release/outline-mcp
```

#### 选项 4：Nix（可重现环境）
```bash
nix run github:nizovtsevnv/outline-mcp-rs
```

### 3. 配置您的 AI 代理（STDIO 模式）

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

> **路径说明：**
> - **cargo install**: 使用 `"outline-mcp"`（自动添加到 PATH）
> - **下载的二进制文件**: 使用完整路径如 `"/path/to/outline-mcp"`
> - **从源码构建**: 使用 `"/path/to/outline-mcp-rs/target/release/outline-mcp"`

**路径要求：**
- **使用绝对路径** — 相对路径可能无法正常工作
- **路径中不要有空格**（使用下划线或连字符代替）
- **仅使用 ASCII 字符** — 避免在路径中使用中文或其他非拉丁字符
- **Windows 用户**：在路径中使用双反斜杠 `\\`（例如 `"C:\\tools\\outline-mcp.exe"`）

## HTTP 模式（多用户）

HTTP 模式启动一个 Streamable HTTP 传输服务器，支持多个用户连接，每个用户使用自己的 Outline API 密钥。服务器本身通过 MCP 访问令牌进行保护。

### 架构

```
                    +--------------------------+
  客户端 A -------->|                          |-------> Outline API
  [X-MCP-Token]     |    MCP HTTP 服务器        |  （用户 A 的密钥）
  [Authorization]   |                          |
                    |  - 认证（X-MCP-Token）     |
  客户端 B -------->|  - IP 速率限制             |-------> Outline API
  [X-MCP-Token]     |  - 会话管理（UUID v4）     |  （用户 B 的密钥）
  [Authorization]   |  - SSE 流式传输           |
                    |  - CORS                   |
  客户端 C -------->|                          |-------> Outline API
                    +--------------------------+  （用户 C 的密钥）
```

### 设置

```bash
# 必需：设置允许的 MCP 访问令牌（逗号分隔）
export MCP_AUTH_TOKENS="alice-token,bob-token"

# 可选：Outline API URL（默认：https://app.getoutline.com/api）
export OUTLINE_API_URL="https://your-outline.example.com/api"

# 启动服务器
./outline-mcp --http
```

### 客户端请求

每个请求必须包含两种凭证：
- **`X-MCP-Token`** 头 — 服务器访问令牌（`MCP_AUTH_TOKENS` 中的某个值）
- **`Authorization: Bearer <key>`** 头 — 用户自己的 Outline API 密钥

```bash
# 健康检查（无需认证）
curl http://127.0.0.1:3000/health

# 初始化会话
curl -X POST http://127.0.0.1:3000/mcp \
  -H "Content-Type: application/json" \
  -H "X-MCP-Token: alice-token" \
  -H "Authorization: Bearer ol_api_xxxxx" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}'
# 响应包含 Mcp-Session-Id 头

# 列出工具（带会话）
curl -X POST http://127.0.0.1:3000/mcp \
  -H "Content-Type: application/json" \
  -H "X-MCP-Token: alice-token" \
  -H "Authorization: Bearer ol_api_xxxxx" \
  -H "Mcp-Session-Id: <初始化返回的id>" \
  -d '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":2}'

# 删除会话
curl -X DELETE http://127.0.0.1:3000/mcp \
  -H "X-MCP-Token: alice-token" \
  -H "Mcp-Session-Id: <初始化返回的id>"
```

### HTTP 端点

| 方法 | 路径 | 认证 | 描述 |
|------|------|------|------|
| GET | /health | 否 | 健康检查（`{"status":"ok","version":"..."}`） |
| POST | /mcp | 是 | 处理 MCP JSON-RPC 请求 |
| GET | /mcp | 是 | 打开 SSE 流（需要 `Mcp-Session-Id`） |
| DELETE | /mcp | 是 | 终止会话（需要 `Mcp-Session-Id`） |
| OPTIONS | * | 否 | CORS 预检 |

### HTTP 错误响应

| 状态码 | 含义 |
|--------|------|
| 401 | 缺少或无效的 `X-MCP-Token` / `Authorization` 头 |
| 413 | 请求体超过 `HTTP_MAX_BODY_SIZE` |
| 415 | `Content-Type` 不是 `application/json` |
| 429 | 该 IP 超过速率限制 |

## 配置

### 环境变量

| 变量 | 模式 | 必需 | 默认值 | 描述 |
|------|------|------|--------|------|
| `OUTLINE_API_KEY` | STDIO | 是 | — | Outline API 密钥 |
| `OUTLINE_API_URL` | 两者 | 否 | `https://app.getoutline.com/api` | Outline 实例 URL |
| `MCP_AUTH_TOKENS` | HTTP | 是 | — | 逗号分隔的 MCP 访问令牌 |
| `HTTP_HOST` | HTTP | 否 | `127.0.0.1` | 绑定地址 |
| `HTTP_PORT` | HTTP | 否 | `3000` | 端口号（>= 1024） |
| `HTTP_RATE_LIMIT` | HTTP | 否 | `60` | 每个 IP 每分钟最大请求数 |
| `HTTP_SESSION_TIMEOUT` | HTTP | 否 | `1800` | 会话 TTL（秒）（30 分钟） |
| `HTTP_MAX_BODY_SIZE` | HTTP | 否 | `1048576` | 最大请求体大小（字节）（1 MB） |
| `RUST_LOG` | 两者 | 否 | `error`（STDIO）/ `info`（HTTP） | 日志级别 |

### STDIO 模式（默认）
```bash
export OUTLINE_API_KEY="您的密钥"
./outline-mcp
```

### HTTP 模式
```bash
export MCP_AUTH_TOKENS="secret-token"
./outline-mcp --http
```

## 支持的工具（25）

完整覆盖 Outline API 功能：

### 文档操作（12）
- `create_document` — 创建新文档
- `get_document` — 通过 ID 获取文档
- `update_document` — 更新现有文档
- `delete_document` — 删除文档
- `list_documents` — 列出文档并进行筛选
- `search_documents` — 按查询搜索文档
- `archive_document` — 归档文档
- `restore_document` — 从回收站恢复文档
- `unarchive_document` — 取消归档文档
- `move_document` — 在集合之间移动文档
- `list_drafts` — 列出草稿文档
- `create_template_from_document` — 从文档创建模板

### 集合管理（6）
- `create_collection` — 创建新集合
- `get_collection` — 获取集合详情
- `update_collection` — 更新集合元数据
- `delete_collection` — 删除集合
- `list_collections` — 列出所有集合
- `get_collection_documents` — 获取集合的文档结构

### 评论和协作（5）
- `create_comment` — 为文档添加评论
- `get_comment` — 通过 ID 获取评论
- `update_comment` — 修改现有评论
- `delete_comment` — 删除评论
- `list_document_comments` — 列出文档的评论

### 用户管理（2）
- `list_users` — 列出团队成员
- `get_user` — 通过 ID 获取用户

## 架构

```
┌─────────────────┐    ┌──────────────────────────────┐    ┌─────────────────┐
│   MCP 客户端     │────│         传输层                │────│  Outline API    │
│ (Claude/Cursor) │    │  STDIO | Streamable HTTP      │    │   (REST/JSON)   │
└─────────────────┘    └──────────────────────────────┘    └─────────────────┘
```

### 源码结构

```
src/
├── main.rs          # 入口点，日志初始化
├── lib.rs           # run_stdio(), run_http()
├── cli.rs           # 命令行参数解析
├── config.rs        # 环境变量配置
├── error.rs         # 集中式错误类型
├── mcp.rs           # MCP JSON-RPC 2.0 协议处理器
├── outline.rs       # Outline API HTTP 客户端
├── tools/           # MCP 工具实现
│   ├── mod.rs       # 工具注册表和调度器
│   ├── common.rs    # 共享工具实用程序
│   ├── documents.rs # 文档操作（12 个工具）
│   ├── collections.rs # 集合操作（6 个工具）
│   ├── comments.rs  # 评论操作（5 个工具）
│   └── users.rs     # 用户操作（2 个工具）
└── http/            # Streamable HTTP 传输
    ├── mod.rs       # HttpBody 枚举，模块声明
    ├── server.rs    # HttpServer、AppState、优雅关闭
    ├── router.rs    # 按方法和路径进行请求路由
    ├── handler.rs   # MCP POST/GET/DELETE 处理器
    ├── auth.rs      # 令牌验证 + 速率限制
    ├── session.rs   # UUID v4 会话管理（带 TTL）
    ├── sse.rs       # SSE body + keepalive 编码
    ├── cors.rs      # CORS 头管理
    ├── request.rs   # 请求验证（Content-Type、大小）
    ├── response.rs  # HTTP 响应构建器（状态码）
    └── health.rs    # GET /health 端点
```

## 项目原则

### 性能
- **使用 musl/glibc 的静态构建** — 无依赖的单文件
- **< 5MB 二进制文件** 具有完整功能
- **< 10ms 启动时间** 达到就绪状态
- **< 10MB 内存** 使用量

### 可靠性
- **运行时零依赖**（静态链接）
- **显式错误处理** — 生产环境中无 panic
- **类型安全** — 利用 Rust 的所有权系统
- **全面测试** — 单元测试和集成测试

### 简洁性
- **最小代码** — 仅包含基本功能
- **清晰架构** — 易于理解和修改
- **单一二进制文件** — 简单部署
- **环境变量配置** — 无配置文件

## 开发

### 要求
- **Nix**（推荐）— 自动处理所有依赖项
- **或手动**：Rust 1.75+、OpenSSL 开发库

### 构建命令

```bash
# 开发环境
nix develop              # 带工具的本机开发
nix develop .#musl       # musl 静态构建环境
nix develop .#windows    # Windows 交叉编译
nix develop .#macos      # macOS 开发（仅 Darwin）

# 包构建
nix build                # 本机构建（Linux/macOS 自动检测）
nix build .#musl         # 静态 musl 构建（可移植 Linux）
nix build .#glibc-optimized # 优化的 glibc 构建
nix build .#windows      # Windows 交叉编译
nix build .#macos-x86_64 # macOS Intel
nix build .#macos-arm64  # macOS Apple Silicon
```

### 测试

```bash
# 运行所有测试
nix develop -c cargo test

# 集成测试
nix develop -c cargo test --test http_transport_tests
nix develop -c cargo test --test mcp_tests
```

### 开发工作流

```bash
nix develop
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## 贡献

1. **Fork** 仓库
2. **创建** 功能分支（`git checkout -b feature/amazing-feature`）
3. **进行** 更改并添加测试
4. **确保** 所有检查通过：`cargo fmt && cargo clippy -- -D warnings && cargo test`
5. **提交** pull request

## 许可证

MIT License — 详情请查看 [LICENSE](LICENSE) 文件。

## 致谢

- **Outline** 团队提供的优秀 API 文档
- **Anthropic** 提供的 MCP 协议规范
- **Rust** 社区提供的出色工具和库
