//! Shell completions command implementation

use crate::cli::Cli;
use crate::utils::error::{Error, Result};
use clap::CommandFactory;
use clap_complete::{generate, Shell};

/// Execute the completions command
pub async fn execute(shell: &str) -> Result<()> {
    let shell = match shell.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" | "ps" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => {
            return Err(Error::Validation(format!(
                "Unknown shell: {}. Supported: bash, zsh, fish, powershell, elvish",
                shell
            )));
        }
    };

    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();

    generate(shell, &mut cmd, name, &mut std::io::stdout());

    Ok(())
}
