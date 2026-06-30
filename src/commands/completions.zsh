#compdef agentml

_agentml() {
    local -a commands
    commands=(
        "init:Initialize a new project"
        "validate:Validate an AGENT.agent file"
        "inspect:Inspect project information"
        "run:Run a task"
        "context:Generate context for LLMs"
        "brief:Generate agent operating brief"
        "mcp:Start MCP server"
        "self-check:Run self-check"
        "diff:Audit git diff"
        "doctor:Check repo health"
        "completions:Generate shell completions"
        "version:Show version"
        "skill:Skill commands"
    )

    _arguments -C \
        '1:command:->cmds' \
        '*: :->args' && \
    case $cmds in
        init)
            _arguments \
                '1:path:->paths' \
                '*: :->args' && \
            case $paths in
                --template)
                    _arguments \
                        '*:template:(generic rust-cli nextjs-app python-package)' \
                        '*: :->args'
                    ;;
            esac
            ;;
        brief)
            _arguments \
                '--format:md or json:(md json)' \
                '--write:Write to file' \
                '--max-lines:Maximum lines' \
                '--include-diff:Include diff' \
                '--no-diff:Exclude diff'
            ;;
        completions)
            _arguments '1:shell:(bash zsh fish)'
            ;;
        validate)
            _arguments '1:file:_files'
            ;;
        context)
            _arguments '1:file:_files' '--output:output file'
            ;;
    esac
}

_compdef _agentml agentml