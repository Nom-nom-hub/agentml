complete -c agentml -f -a init -d 'Initialize a new project'
complete -c agentml -f -a validate -d 'Validate an AGENT.agent file'
complete -c agentml -f -a inspect -d 'Inspect project information'
complete -c agentml -f -a run -d 'Run a task'
complete -c agentml -f -a context -d 'Generate context for LLMs'
complete -c agentml -f -a brief -d 'Generate agent operating brief'
complete -c agentml -f -a mcp -d 'Start MCP server'
complete -c agentml -f -a self-check -d 'Run self-check'
complete -c agentml -f -a diff -d 'Audit git diff'
complete -c agentml -f -a doctor -d 'Check repo health'
complete -c agentml -f -a completions -d 'Generate shell completions'
complete -c agentml -f -a version -d 'Show version'
complete -c agentml -f -a skill -d 'Skill commands'

complete -c agentml -n '__fish_use_subcommand' -f -a init -d 'Initialize a new project'
complete -c agentml -n 'complete -c agentml --arguments init' -f -a --template -d 'Template type' -x -k
complete -c agentml -n 'complete -c agentml --arguments init' -f --template -d 'Template type' -x -k
complete -c agentml -n '__fish_complete_unique' -f -a generic rust-cli nextjs-app python-package

complete -c agentml -n 'complete -c agentml --arguments brief' -l format -d 'Output format' -x -k
complete -c agentml -n 'complete -c agentml --arguments brief' -l write -d 'Write to file'
complete -c agentml -n 'complete -c agentml --arguments brief' -l max-lines -d 'Maximum lines' -x -k
complete -c agentml -n 'complete -c agentml --arguments brief' -l include-diff -d 'Include diff'
complete -c agentml -n 'complete -c agentml --arguments brief' -l no-diff -d 'Exclude diff'

complete -c agentml -n 'complete -c agentml --arguments completions' -f -a bash zsh fish