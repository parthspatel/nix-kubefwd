//! Init command implementation

use crate::infra::ConfigManagerImpl;
use crate::utils::error::{Error, Result};

/// Execute the init command
pub async fn execute(global: bool, local: bool, force: bool, import_existing: bool) -> Result<()> {
    let csm_home = ConfigManagerImpl::detect_csm_home();

    // Check if already initialized
    if csm_home.exists() && !force {
        return Err(Error::AlreadyInitialized);
    }

    // Create CSM home directory
    std::fs::create_dir_all(&csm_home)
        .map_err(|e| Error::Io(e))?;

    // Create subdirectories
    std::fs::create_dir_all(csm_home.join("skills"))
        .map_err(|e| Error::Io(e))?;
    std::fs::create_dir_all(csm_home.join("cache"))
        .map_err(|e| Error::Io(e))?;
    std::fs::create_dir_all(csm_home.join("logs"))
        .map_err(|e| Error::Io(e))?;

    // Initialize config
    let mut config_manager = ConfigManagerImpl::new(csm_home.clone());
    config_manager.save()?;

    // Initialize database
    let db_path = csm_home.join("registry.db");
    let _skill_repo = crate::infra::SqliteSkillRepository::new(&db_path)?;
    let _conflict_repo = crate::infra::SqliteConflictRepository::new(&db_path)?;

    println!("CSM initialized successfully at {}", csm_home.display());

    if import_existing {
        println!("Scanning for existing CLAUDE.md files...");
        // TODO: Implement import
    }

    Ok(())
}
