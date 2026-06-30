use anyhow::Result;

pub fn run(shell: &str) -> Result<()> {
    let completions = match shell {
        "bash" => include_str!("completions.bash"),
        "zsh" => include_str!("completions.zsh"),
        "fish" => include_str!("completions.fish"),
        _ => {
            eprintln!("Unknown shell: {}. Supported: bash, zsh, fish", shell);
            std::process::exit(1);
        }
    };

    print!("{}", completions);
    Ok(())
}
