# Security Policy

## Secret Handling

AgentML **never** reads, logs, or exposes secrets:

- Environment files (`.env`, `.env.*`) are never read
- Secret-looking paths (`*/secret/*`, `*credential*`, `*key*`, `*token*`) are blocked
- The `validate_skill` tool rejects paths containing these patterns

## MCP Safety Model

The MCP server runs with these protections:

1. **No arbitrary shell execution**: Tools only run internal AgentML logic
2. **Path traversal blocked**: `validate_skill` rejects `..` and absolute paths
3. **Forbidden paths blocked**: Paths matching `.agent`, `secret`, `key`, `token` are rejected
4. **Write restrictions**: `generate_context` and `generate_brief` only write to `.agentml/`
5. **No file exposure**: Secret file contents are never returned in responses

## Reporting Vulnerabilities

If you discover a security vulnerability in AgentML:

1. **Do not** open a public issue
2. Email: security@nom-nom.ai (or open a private disclosure on GitHub)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will respond within 48 hours and provide a fix within 7 days for critical issues.

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |