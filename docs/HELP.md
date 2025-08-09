Outline MCP Server - Knowledge base integration for AI assistants

USAGE:
    outline-mcp [OPTIONS]

OPTIONS:
    --http              Run HTTP server mode (default: STDIO mode)
    --help, -h          Show this help message
    --version, -V       Show version information

ENVIRONMENT VARIABLES:
    OUTLINE_API_KEY     Outline API key (required)
    OUTLINE_API_URL     Outline instance URL (default: https://app.getoutline.com/api)
    HTTP_HOST           HTTP server host (default: 127.0.0.1, HTTP mode only)
    HTTP_PORT           HTTP server port (default: 3000, HTTP mode only)
    RUST_LOG            Log level: error|warn|info|debug|trace (default: info)

QUICK SETUP FOR CURSOR IDE:

1. Get your Outline API key:
   • Outline.com: https://app.outline.com/settings/api-and-apps
   • Self-hosted: https://your-instance.com/settings/api-and-apps

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

AVAILABLE ACTIONS:
    • Documents: create, read, update, delete, search
    • Collections: manage document collections  
    • Comments: add and manage comments
    • Users: list team members
    • Search: full-text search across content

EXAMPLES:
    outline-mcp                    # Run in STDIO mode (default)
    outline-mcp --http             # Run HTTP server on localhost:3000
    RUST_LOG=debug outline-mcp     # Run with debug logging

For more information: https://github.com/nizovtsevnv/outline-mcp-rs 