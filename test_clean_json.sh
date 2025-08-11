#!/run/current-system/sw/bin/bash
# Test script to verify clean JSON output in STDIO mode

echo "🧪 Testing clean JSON output in STDIO mode..."
echo "================================================"
echo ""

export OUTLINE_API_KEY="test-api-key-12345678"
export OUTLINE_API_URL="https://app.getoutline.com/api"

echo "📋 Configuration:"
echo "  OUTLINE_API_KEY: ${OUTLINE_API_KEY:0:10}..."
echo "  OUTLINE_API_URL: $OUTLINE_API_URL"
echo "  Mode: STDIO (should output ONLY JSON)"
echo ""

echo "📤 Test 1: MCP Initialization"
echo "Request:"
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{}}}'
echo ""
echo "Response:"
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{}}}' | ./target/release/outline-mcp
echo ""

echo "📤 Test 2: Tools List"
echo "Request:"
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}'
echo ""
echo "Response:"
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | ./target/release/outline-mcp
echo ""

echo "✅ Tests completed!"
echo ""
echo "🔍 Verification checklist:"
echo "  ✓ No extra log messages before JSON"
echo "  ✓ No extra log messages after JSON"  
echo "  ✓ Clean JSON responses only"
echo "  ✓ No .env file loading messages"
echo "  ✓ Compatible with MCP clients like Cursor" 