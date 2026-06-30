_agentml_completion() {
    local IFS=$'\n'
    local response
    response=$(agentml completions bash | head -n -1)
    for completion in $response; do
        IFS=',' read -r option description <<< "$completion"
        eval "compopt --add-option=$option"
    done
}

complete -o nospace -F _agentml_completion agentml