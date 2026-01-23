//! Migrate command implementation
//!
//! Migrates CSM data from legacy ~/.csm to XDG-compliant ~/.config/csm

use std::path::PathBuf;

use crate::infra::ConfigManagerImpl;
use crate::utils::error::{Error, Result};

/// Execute the migrate command
pub async fn execute(dry_run: bool, force: bool) -> Result<()> {
    // Check if CSM_HOME is set - migration doesn't apply
    if std::env::var("CSM_HOME").is_ok() {
        println!("CSM_HOME environment variable is set.");
        println!("Migration is not needed when using a custom home directory.");
        return Ok(());
    }

    // Detect legacy and new paths
    let legacy_home = ConfigManagerImpl::detect_legacy_home();
    let new_home = ConfigManagerImpl::detect_csm_home();

    // Check if legacy exists
    let legacy_path = match legacy_home {
        Some(path) => path,
        None => {
            println!("No legacy ~/.csm directory found.");
            println!("Nothing to migrate.");
            return Ok(());
        }
    };

    // Check if new location already exists
    if new_home.exists() && !force {
        println!("Target directory already exists: {}", new_home.display());
        println!("Use --force to overwrite (existing data will be lost).");
        return Err(Error::Config(
            "Target directory already exists. Use --force to overwrite.".to_string(),
        ));
    }

    println!("Migration plan:");
    println!("  From: {}", legacy_path.display());
    println!("  To:   {}", new_home.display());
    println!();

    if dry_run {
        println!("[dry-run] Would migrate the following:");
        print_directory_contents(&legacy_path, "  ")?;
        println!();
        println!("Run without --dry-run to perform the migration.");
        return Ok(());
    }

    // Perform the migration
    println!("Migrating...");

    // Create parent directory if needed
    if let Some(parent) = new_home.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| Error::Config(format!("Failed to create parent directory: {}", e)))?;
    }

    // If force and target exists, remove it first
    if new_home.exists() && force {
        std::fs::remove_dir_all(&new_home)
            .map_err(|e| Error::Config(format!("Failed to remove existing directory: {}", e)))?;
    }

    // Move the directory
    if let Err(e) = std::fs::rename(&legacy_path, &new_home) {
        // If rename fails (cross-device), fall back to copy + delete
        if e.raw_os_error() == Some(18) {
            // EXDEV - cross-device link
            copy_dir_recursive(&legacy_path, &new_home)?;
            std::fs::remove_dir_all(&legacy_path).map_err(|e| {
                Error::Config(format!(
                    "Failed to remove legacy directory after copy: {}",
                    e
                ))
            })?;
        } else {
            return Err(Error::Config(format!("Failed to migrate: {}", e)));
        }
    }

    println!();
    println!("Migration complete!");
    println!("CSM now uses: {}", new_home.display());

    Ok(())
}

/// Print the contents of a directory (for dry-run preview)
fn print_directory_contents(path: &PathBuf, indent: &str) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    for entry in std::fs::read_dir(path).map_err(|e| Error::Io(e))? {
        let entry = entry.map_err(|e| Error::Io(e))?;
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        if path.is_dir() {
            println!("{}{}/", indent, name);
        } else {
            let size = path.metadata().map(|m| m.len()).unwrap_or(0);
            println!("{}{} ({} bytes)", indent, name, size);
        }
    }

    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    std::fs::create_dir_all(dst).map_err(|e| {
        Error::Config(format!(
            "Failed to create directory {}: {}",
            dst.display(),
            e
        ))
    })?;

    for entry in std::fs::read_dir(src).map_err(|e| Error::Io(e))? {
        let entry = entry.map_err(|e| Error::Io(e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path).map_err(|e| {
                Error::Config(format!(
                    "Failed to copy {} to {}: {}",
                    src_path.display(),
                    dst_path.display(),
                    e
                ))
            })?;
        }
    }

    Ok(())
}
