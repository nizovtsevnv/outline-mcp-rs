Outline MCP Server - Knowledge base integration for AI assistants

USAGE:
    outline-mcp [OPTIONS]

OPTIONS:
    --http              Run HTTP server mode (default: STDIO mode)
    --help, -h          Show this help message
    --version, -V       Show version information

TRANSPORT MODES:

  STDIO Mode (default):
    Communicates via standard input/output. Used for direct integration
    with MCP clients (Cursor IDE, Claude Desktop, etc.).

  HTTP Mode (--http):
    Streamable HTTP transport (MCP 2025-03-26 spec) with multi-user
    support, authentication, rate limiting, sessions, SSE streaming,
    and CORS. Each user provides their own Outline API key.

ENVIRONMENT VARIABLES:

  Common:
    OUTLINE_API_URL     Outline instance URL (default: https://app.getoutline.com/api)
    RUST_LOG            Log level: error|warn|info|debug|trace
                        Default: 'error' for STDIO mode, 'info' for HTTP mode
                        Note: STDIO logs go to stderr to avoid JSON pollution

  STDIO mode:
    OUTLINE_API_KEY     Outline API key (required)

  HTTP mode:
    MCP_AUTH_TOKENS     Comma-separated list of allowed MCP access tokens (required)
                        Clients must send a valid token in the X-MCP-Token header
    HTTP_HOST           Bind address (default: 127.0.0.1)
    HTTP_PORT           Port number (default: 3000)
    HTTP_RATE_LIMIT     Max requests per minute per IP (default: 60)
    HTTP_SESSION_TIMEOUT  Session TTL in seconds (default: 1800 = 30 min)
    HTTP_MAX_BODY_SIZE  Max request body in bytes (default: 1048576 = 1 MB)

QUICK SETUP FOR CURSOR IDE (STDIO mode):

1. Get your Outline API key:
   - Outline.com: https://app.outline.com/settings/api-and-apps
   - Self-hosted: https://your-instance.com/settings/api-and-apps

2. Configure Cursor IDE via settings window or `mcp.json` configuration file:
   {
     "mcpServers": {
       "Outline knowledge base": {
         "command": "outline-mcp",
         "env": {
           "OUTLINE_API_KEY": "your-api-key-here"
         }
       }
     }
   }

3. Restart Cursor IDE to activate the MCP server

HTTP MODE SETUP (multi-user):

1. Set required environment variables:
   export MCP_AUTH_TOKENS="token-for-user-a,token-for-user-b"
   export OUTLINE_API_URL="https://app.getoutline.com/api"

2. Start the server:
   ./outline-mcp --http

3. Clients connect with two credentials:
   - X-MCP-Token header: server access token (from MCP_AUTH_TOKENS)
   - Authorization header: user's own Outline API key (Bearer token)

4. Example request:
   curl -X POST http://127.0.0.1:3000/mcp \
     -H "Content-Type: application/json" \
     -H "X-MCP-Token: token-for-user-a" \
     -H "Authorization: Bearer ol_api_xxxxx" \
     -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}'

HTTP ENDPOINTS:
    GET    /health     Health check (no auth required)
    POST   /mcp        Process MCP JSON-RPC request
    GET    /mcp        Open SSE stream (requires Mcp-Session-Id)
    DELETE /mcp        Terminate session (requires Mcp-Session-Id)
    OPTIONS *          CORS preflight

AVAILABLE ACTIONS:
    - Documents: create, read, update, delete, search, archive, restore, unarchive, move, list drafts
    - Collections: create, read, update, delete, list, view document structure
    - Comments: create, read, update, delete, list by document
    - Users: list team members, get user details
    - Templates: create templates from documents
    - Search: full-text search across content

EXAMPLES:
    outline-mcp                    # Run in STDIO mode (default)
    outline-mcp --http             # Run HTTP server on localhost:3000
    RUST_LOG=debug outline-mcp     # Run with debug logging

For more information: https://github.com/nizovtsevnv/outline-mcp-rs
