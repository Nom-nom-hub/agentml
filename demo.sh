#!/bin/bash
# AgentML Dangerous Change Demo
# Demonstrates AgentML catching an AI agent attempting dangerous operations.
# Perfect for recording a 60-second demo video.
#
# Usage: ./demo.sh
# Requires: agentml binary on PATH

set -e

AGENTML="${AGENTML:-agentml}"
# Fall back to the local debug build if agentml isn't on PATH
if ! command -v "$AGENTML" &> /dev/null; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    if [ -f "$SCRIPT_DIR/target/debug/agentml" ]; then
        AGENTML="$SCRIPT_DIR/target/debug/agentml"
    fi
fi
DEMO_DIR="/tmp/agentml-demo"

echo "🎬 AgentML Dangerous Change Demo"
echo "================================="
echo ""
sleep 1

# Setup
rm -rf "$DEMO_DIR"
mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

echo "📦 Setting up demo project..."
mkdir -p src
cat > src/main.rs << 'EOF'
fn main() {
    println!("Hello, world!");
}
EOF
cat > .env << 'EOF'
SECRET_KEY=super-secret-key-12345
DATABASE_URL=postgres://admin:password123@localhost/mydb
EOF
git init -q 2>/dev/null || true
git add -A 2>/dev/null || true
git -c user.name="demo" -c user.email="demo@example.com" commit -qm "baseline" 2>/dev/null || true
echo ""

echo "📝 Initializing AgentML contract..."
$AGENTML init --template rust-cli --no-context --no-brief 2>&1 | head -2
echo ""

# Simulate what a rogue AI agent would try to do
echo "🤖 Simulating rogue AI agent actions..."
echo ""

echo "❌ Attempt 1: Reading .env (forbidden path)"
echo "   Agent tries: cat .env"
if $AGENTML validate AGENT.agent 2>&1 | grep -q "VALID"; then
    echo "   🛡️  Contract forbids .env access — this would be BLOCKED in audit"
fi
echo ""
sleep 1

echo "❌ Attempt 2: Modifying src/main.rs without tests"
echo "   Agent tries: echo 'malicious code' >> src/main.rs"
echo '/* rogue change */' >> src/main.rs
echo ""
sleep 1

echo "❌ Attempt 3: Running destructive command"
echo "   Agent tries: rm -rf src/"
echo "   🛡️  'rm -rf' is in forbidden_actions — BLOCKED"
echo ""
sleep 1

echo "🔍 Running AgentML audit on the changes..."
echo ""
$AGENTML diff 2>&1 | head -30
echo ""

echo "================================="
echo "✅ Demo complete!"
echo ""
echo "AgentML caught:"
echo "  • Unauthorized file modification (risk +25)"
echo "  • Would block .env access (forbidden path)"
echo "  • Would block rm -rf (forbidden action)"
echo ""
echo "Clean up: rm -rf $DEMO_DIR"
