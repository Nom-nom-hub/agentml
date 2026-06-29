# Agent Prompts

Copy-paste prompts to make AI agents respect AgentML contracts.

## Essential prompts

### 1. Load the contract at session start

```
Read AGENT.agent and .agentml/context.md before doing anything else.
Respect permissions, forbidden paths, forbidden actions, and validation commands.
```

### 2. Before editing any file

```
Check AGENT.agent permissions before editing files.
Only write to paths listed in permissions.write.
Do not touch paths listed in safety.forbidden_paths.
```

### 3. Before running commands

```
Check AGENT.agent before running any command.
Do not run commands listed in safety.forbidden_actions.
If a command is in safety.require_confirmation, stop and ask for approval.
```

### 4. After making changes

```
Run validation commands from AGENT.agent.validation after every change.
Do not report completion until validation passes.
```

### 5. For report format

```
When reporting results, use the format defined in AGENT.agent.output.
Include required sections: summary, files changed, tests run, risks.
```

## Prompt templates by agent

### Claude / Anthropic

Add to your system prompt or project instructions:

```
You are working on a project governed by AGENT.agent.
Before any action:
1. Read AGENT.agent and .agentml/context.md
2. Verify the action is allowed by permissions
3. Verify the target is not in safety.forbidden_paths
4. Run validation commands after changes
5. Report in the format defined in output
```

### Cursor

Add to `.cursorrules` or project-level instructions:

```
## AgentML Contract
- Read .agentml/context.md at session start
- Respect permissions and forbidden paths
- Run validation before reporting done
- Output markdown with: summary, files changed, tests run, risks
```

### GitHub Copilot

Add to `.github/copilot-instructions.md`:

```
## AgentML Rules
- Read .agentml/context.md before editing
- Only write to permitted paths
- Never edit forbidden paths
- Run cargo test / npm test / pytest after changes
- Report using the required output format
```

### VS Code Copilot Chat

Add to workspace settings or prompt:

```
Before editing or running commands, check AGENT.agent.
Respect permissions.write and safety.forbidden_paths.
Run validation commands after changes.
```

## Advanced: skill-aware prompts

If your project uses `.skill` files:

```
Read skills/*.skill in addition to AGENT.agent.
Each skill defines actions, rules, success criteria, and output format.
Follow the most specific skill when one applies.
```

## Troubleshooting

**Agent ignores the contract:**
- Make sure `.agentml/context.md` exists (run `agentml context`)
- Paste the contract content into the agent's system prompt
- Add reminders before every major action

**Agent edits wrong files:**
- Narrow `permissions.write` in `AGENT.agent`
- Add explicit `safety.forbidden_paths` for sensitive directories
- Use `agentml doctor` to verify structure

**Agent skips validation:**
- Ensure `validation` section has concrete commands
- Add a workflow step for validation
- Use `agentml run <task>` to show the plan before execution
