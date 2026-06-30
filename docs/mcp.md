# AgentML MCP Server

The AgentML MCP server exposes project contracts and rules to AI agents through the Model Context Protocol (MCP).

## What it does

The MCP server reads `AGENT.agent` from the current working directory and exposes tools for agents to query project rules, validate changes, and generate context.

## How to run it

```bash
cargo run -- mcp
```

Or with the binary:

```bash
agentml mcp
```

The server runs over stdio and communicates using the MCP JSON-RPC protocol.

## Available tools

| Tool | Description |
|------|-------------|
| `get_project_contract` | Returns the parsed AGENT.agent contract |
| `get_agent_brief` | Returns the agent operating brief (md or json format) |
| `get_allowed_paths` | Returns read/write/forbidden paths |
| `get_validation_commands` | Returns commands to run before completion |
| `audit_diff` | Runs git diff and returns risk assessment |
| `validate_contract` | Validates AGENT.agent |
| `validate_skill` | Validates a .skill file (path required) |
| `generate_context` | Generates .agentml/context.md |
| `generate_brief` | Generates .agentml/brief.md |

## Security model

- **No arbitrary code execution**: Tools only run internal AgentML logic
- **Path traversal blocked**: `validate_skill` rejects `..` and absolute paths
- **Secret detection**: Paths containing `.agent`, `secret`, `key`, or `token` are blocked
- **Write restrictions**: `generate_context` and `generate_brief` only write to `.agentml/`

## Example client config

### Claude Code

```json
{
  "mcpServers": {
    "agentml": {
      "command": "cargo",
      "args": ["run", "--", "mcp"],
      "cwd": "/path/to/project"
    }
  }
}
```

### Cursor

Add to settings.json:

```json
{
  "cursor.mcp": {
    "servers": {
      "agentml": {
        "command": "cargo run --package agentml -- mcp"
      }
    }
  }
}
```

## Example agent workflow

1. Agent calls `get_agent_brief` to understand project rules
2. Agent edits only allowed files
3. Agent calls `audit_diff` to check risk
4. Agent runs validation commands
5. Agent returns required final report