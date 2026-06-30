use crate::parser::parse_agent_file;
use crate::types::AgentFile;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run(write: bool, force: bool) -> Result<()> {
    let agent_path = Path::new("AGENT.agent");
    if !agent_path.exists() {
        anyhow::bail!("AGENT.agent not found in current directory");
    }
    let agent = parse_agent_file(agent_path)?;
    let markdown = generate(&agent);

    if write {
        let md_path = Path::new("AGENTS.md");
        if md_path.exists() && !force {
            println!(
                "{}",
                "AGENTS.md already exists. Use --force to overwrite.".yellow()
            );
            return Ok(());
        }
        fs::write(md_path, &markdown)?;
        println!("{}", "Written to AGENTS.md".green());
    } else {
        println!("{}", markdown);
    }

    Ok(())
}

pub fn generate(agent: &AgentFile) -> String {
    let mut output = String::new();

    output.push_str("# AGENTS.md\n\n");
    output.push_str("## Purpose\n\n");
    output.push_str(
        "This project uses AgentML to define how AI coding agents should safely work in this repository.\n\n",
    );
    output.push_str("The machine-readable source of truth is `AGENT.agent`.\n\n");
    output.push_str("## Required first steps\n\n");
    output.push_str("Before editing files, agents should read:\n\n");
    output.push_str("1. `AGENT.agent`\n");
    output.push_str("2. `.agentml/brief.md`\n");
    output.push_str("3. `.agentml/context.md`\n\n");
    output.push_str("Recommended command:\n\n```bash\nagentml brief\n```\n\n");
    output.push_str("If AgentML is not installed, read AGENT.agent directly.\n\n");

    output.push_str("## Project context\n\n");
    output.push_str("### Stack\n\n");

    let mut stack = Vec::new();
    if let Some(ctx) = &agent.context {
        if let Some(pt) = &ctx.project_type {
            stack.push(pt.clone());
        }
        if let Some(langs) = &ctx.languages {
            for lang in langs {
                stack.push(lang.clone());
            }
        }
        if let Some(fw) = &ctx.frameworks {
            for f in fw {
                stack.push(f.clone());
            }
        }
    }
    if stack.is_empty() {
        stack.push("Generic".to_string());
    }
    for s in &stack {
        output.push_str(&format!("- {}\n", s));
    }
    output.push('\n');

    output.push_str("### Important files\n\n");
    if let Some(perms) = &agent.permissions
        && let Some(read) = &perms.read
    {
        for p in read {
            output.push_str(&format!("- {}\n", p));
        }
    }
    output.push('\n');

    output.push_str("## Allowed work areas\n\n");
    output.push_str("Agents may usually modify:\n\n");
    if let Some(perms) = &agent.permissions
        && let Some(write) = &perms.write
    {
        for p in write {
            output.push_str(&format!("- {}\n", p));
        }
    }
    output.push('\n');
    output.push_str("Always check AGENT.agent for the authoritative list.\n\n");

    output.push_str("## Forbidden areas\n\n");
    output.push_str("Agents must not modify or expose:\n\n");
    if let Some(safety) = &agent.safety
        && let Some(obj) = safety.as_mapping()
    {
        if let Some(paths) = obj
            .get(serde_yaml::Value::String("forbidden_paths".to_string()))
            .and_then(|v| v.as_sequence())
        {
            for p in paths {
                if let Some(s) = p.as_str() {
                    output.push_str(&format!("- {}\n", s));
                }
            }
        }
        if let Some(sp) = obj.get(serde_yaml::Value::String("secrets_policy".to_string()))
            && let Some(sp_map) = sp.as_mapping()
            && let Some(nr) = sp_map
                .get(serde_yaml::Value::String("never_read".to_string()))
                .and_then(|v| v.as_sequence())
        {
            output.push_str("\nNever read:\n");
            for p in nr {
                if let Some(s) = p.as_str() {
                    output.push_str(&format!("- {}\n", s));
                }
            }
        }
    }

    output.push('\n');
    output.push_str("## Validation commands\n\n");
    output.push_str("Before reporting completion, run:\n\n");
    if let Some(validation) = &agent.validation {
        for v in validation {
            output.push_str(&format!("- `{}` — {}\n", v.command, v.name));
        }
    }
    output.push('\n');

    output.push_str("## Diff audit\n\n");
    output.push_str("After making changes, run:\n\n```bash\nagentml diff\n```\n\n");
    output.push_str("Include the risk score in the final report.\n\n");
    output.push_str(
        "Never report `Risk score: N/A` if `agentml diff` was successfully run. Include the actual score and status.\n\n",
    );

    output.push_str("## Final report format\n\n");
    output.push_str("Every agent task should end with:\n\n");
    output.push_str(
        "```\nSummary:\nFiles changed:\nCommands run:\nValidation result:\nRisk score:\nCommit:\nRisks:\nNext steps:\n```\n\n",
    );
    output.push_str("The `Commit:` field should contain the commit hash and short message. If not committed, state why:\n\n");

    output.push_str("## Source of truth\n\n");
    output.push_str("If files disagree, follow this order:\n\n");
    output.push_str("1. `AGENT.agent`\n");
    output.push_str("2. `.agentml/brief.md`\n");
    output.push_str("3. `.agentml/context.md`\n");
    output.push_str("4. `AGENTS.md`\n");
    output.push_str("5. `README.md`\n\n");

    output.push_str("## Maintenance Intelligence\n\n");
    output.push_str(
        "Agents must keep the project synchronized. When behavior changes, update every surface affected by that behavior.\n\n",
    );
    output.push_str("Before reporting completion, ask:\n\n");
    output.push_str("1. Did CLI behavior change?\n");
    output.push_str("2. Did validation behavior change?\n");
    output.push_str("3. Did MCP behavior change?\n");
    output.push_str("4. Did generated file behavior change?\n");
    output.push_str("5. Did project templates change?\n");
    output.push_str("6. Did public documentation become outdated?\n");
    output.push_str("7. Did the website need a matching update?\n");
    output.push_str("8. Did examples need to be updated?\n");
    output.push_str("9. Did tests cover the new behavior?\n");
    output.push_str("10. Did CHANGELOG.md need an entry?\n\n");
    output.push_str("If yes, update the matching files before reporting completion.\n\n");

    output.push_str("## Documentation Sync Rule\n\n");
    output.push_str("Code changes are incomplete if user-facing documentation is stale.\n\n");
    output
        .push_str("When user-facing behavior changes, update documentation in the same task.\n\n");
    output.push_str("User-facing behavior includes:\n\n");
    output.push_str("- CLI commands\n");
    output.push_str("- command flags\n");
    output.push_str("- command output\n");
    output.push_str("- validation errors\n");
    output.push_str("- generated files\n");
    output.push_str("- templates\n");
    output.push_str("- MCP tools\n");
    output.push_str("- install flow\n");
    output.push_str("- examples\n");
    output.push_str("- website copy\n");
    output.push_str("- security behavior\n");
    output.push_str("- release process\n\n");
    output.push_str("Do not report completion if docs are knowingly outdated.\n\n");

    output.push_str("## Agent Self-Update Rule\n\n");
    output.push_str("Agents may update `AGENTS.md` when project workflow guidance changes.\n\n");
    output.push_str("Update `AGENTS.md` when:\n\n");
    output.push_str("- new commands are added\n");
    output.push_str("- agent workflow changes\n");
    output.push_str("- validation requirements change\n");
    output.push_str("- MCP usage changes\n");
    output.push_str("- final report expectations change\n");
    output.push_str("- source-of-truth order changes\n");
    output.push_str("- docs synchronization rules change\n");
    output.push_str("- release process changes\n");
    output.push_str("- project maintenance expectations change\n\n");
    output.push_str("Do not update `AGENTS.md` for unrelated code changes.\n\n");
    output.push_str(
        "When updating `AGENTS.md`, keep it concise, accurate, and aligned with `AGENT.agent`.\n\n",
    );

    output.push_str("## Pre-Final Checklist\n\n");
    output.push_str("Before reporting completion, verify:\n\n");
    output.push_str("- Code changes are complete.\n");
    output.push_str("- Tests were added or updated when needed.\n");
    output.push_str("- Validation commands were run.\n");
    output.push_str("- `agentml diff` was run.\n");
    output.push_str("- Risk score is included in the final report.\n");
    output.push_str("- `README.md` is updated if public behavior changed.\n");
    output.push_str("- `docs/` are updated if behavior or architecture changed.\n");
    output.push_str("- `CHANGELOG.md` is updated if user-facing behavior changed.\n");
    output.push_str("- Website content is updated if public messaging changed.\n");
    output.push_str("- Examples are updated if templates or expected usage changed.\n");
    output.push_str("- `AGENT.agent` is updated if contract rules changed.\n");
    output.push_str("- `AGENTS.md` is updated if agent workflow changed.\n");
    output.push_str("- `git status --short` was checked.\n");
    output.push_str("- Unrelated user changes were not included.\n");
    output.push_str(
        "- All intended changes were committed unless the user explicitly said not to commit.\n",
    );
    output.push_str("- Final report includes the commit hash.\n\n");

    output.push_str("## Git Workflow\n\n");
    output.push_str("Before finishing, check:\n\n");
    output.push_str("```bash\ngit status --short\nagentml diff\n```\n\n");
    output.push_str("If the task is complete and validation passes, commit the intended changes unless the human asked you not to.\n\n");
    output.push_str("Do not commit secrets, build artifacts, dependency folders, or unrelated user changes.\n\n");
    output.push_str("Final reports should include the commit hash when a commit is created.\n\n");

    output.push_str("## Task Closure Rule\n\n");
    output.push_str(
        "A task is not complete until the repository is left in a clear final state.\n\n",
    );
    output.push_str("For normal implementation tasks, that means:\n\n");
    output.push_str("1. Code/docs/tests are updated.\n");
    output.push_str("2. Required validation commands pass.\n");
    output.push_str("3. `agentml diff` has been run.\n");
    output.push_str("4. Changes are committed.\n");
    output.push_str("5. Final report includes commit hash and risk score.\n\n");
    output.push_str(
        "If changes are intentionally left uncommitted, the final report must clearly say:\n\n",
    );
    output.push_str("- why they were not committed\n");
    output.push_str("- which files remain modified\n");
    output.push_str("- what command the user should run next\n");

    output
}
