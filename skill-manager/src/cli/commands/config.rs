//! Config command implementations

use crate::infra::ConfigManagerImpl;
use crate::services::ConfigManager;
use crate::utils::error::{Error, Result};

/// Execute config get command
pub async fn execute_get(key: &str) -> Result<()> {
    let csm_home = ConfigManagerImpl::detect_csm_home();
    let mut config = ConfigManagerImpl::new(csm_home);
    config.load()?;

    match config.get(key) {
        Some(value) => println!("{}", value),
        None => {
            return Err(Error::Config(format!("Unknown config key: {}", key)));
        }
    }

    Ok(())
}

/// Execute config set command
pub async fn execute_set(key: &str, value: &str) -> Result<()> {
    let csm_home = ConfigManagerImpl::detect_csm_home();
    let mut config = ConfigManagerImpl::new(csm_home);
    config.load()?;

    config.set(key, value)?;
    println!("✓ Set {} = {}", key, value);

    Ok(())
}

/// Execute config list command
pub async fn execute_list(json: bool) -> Result<()> {
    let csm_home = ConfigManagerImpl::detect_csm_home();
    let mut config = ConfigManagerImpl::new(csm_home);
    config.load()?;

    let keys = [
        "general.default_scope",
        "general.editor",
        "general.color",
        "updates.mode",
        "updates.schedule",
        "updates.check_on_startup",
        "github.default_ref",
        "ui.theme",
        "ui.show_welcome",
    ];

    if json {
        let mut map = serde_json::Map::new();
        for key in &keys {
            if let Some(value) = config.get(key) {
                map.insert(key.to_string(), serde_json::Value::String(value));
            }
        }
        println!("{}", serde_json::to_string_pretty(&map)?);
    } else {
        println!("CSM Configuration");
        println!("{}", "=".repeat(40));
        println!();

        for key in &keys {
            let value = config.get(key).unwrap_or_else(|| "(not set)".to_string());
            println!("{:<30} = {}", key, value);
        }

        println!();
        println!(
            "Config file: {}",
            config.csm_home().join("config.toml").display()
        );
    }

    Ok(())
}

/// Execute config edit command
pub async fn execute_edit() -> Result<()> {
    let csm_home = ConfigManagerImpl::detect_csm_home();
    let config_path = csm_home.join("config.toml");

    // Get editor from environment or config
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string());

    println!("Opening {} in {}...", config_path.display(), editor);

    let status = std::process::Command::new(&editor)
        .arg(&config_path)
        .status()?;

    if !status.success() {
        return Err(Error::Other(format!(
            "Editor exited with status: {}",
            status
        )));
    }

    println!("✓ Configuration updated");
    Ok(())
}

/// Execute config reset command
pub async fn execute_reset(force: bool) -> Result<()> {
    if !force {
        print!("Are you sure you want to reset configuration to defaults? [y/N] ");

        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    let csm_home = ConfigManagerImpl::detect_csm_home();
    let config = ConfigManagerImpl::new(csm_home);
    config.save()?;

    println!("✓ Configuration reset to defaults");
    Ok(())
}
