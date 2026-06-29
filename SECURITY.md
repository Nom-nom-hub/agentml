# Security Policy

AgentML is a tool that governs AI agent behavior. Security is central to its design.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security vulnerabilities seriously. Please report them privately.

**Do not open public issues for security vulnerabilities.**

Email: security@example.com (replace with actual contact)

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Security Model

AgentML protects projects through:

1. **Permission boundaries** — explicit read/write/execute policies
2. **Forbidden paths** — sensitive files are never writable
3. **Forbidden actions** — destructive commands are blocked
4. **Confirmation requirements** — high-risk actions need approval
5. **Risk scoring** — contracts are rated 0-100 for safety

## Known Limitations

- AgentML is a contract layer, not a sandbox
- It does not scan file contents for secrets
- It does not prevent all forms of code injection
- It relies on agents respecting the contract

## Best Practices

- Keep contracts under version control
- Review risk scores regularly
- Run `self-check` in CI
- Update contracts when project structure changes
