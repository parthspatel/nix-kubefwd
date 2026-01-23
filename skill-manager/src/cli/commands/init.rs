//! Init command implementation

use crate::infra::ConfigManagerImpl;
use crate::utils::error::{Error, Result};

/// Execute the init command
pub async fn execute(
    _global: bool,
    _local: bool,
    force: bool,
    import_existing: bool,
) -> Result<()> {
    let csm_home = ConfigManagerImpl::detect_csm_home();

    // Check if migration from legacy ~/.csm is needed
    if ConfigManagerImpl::needs_migration() {
        if let Some(legacy_path) = ConfigManagerImpl::detect_legacy_home() {
            println!(
                "Found existing CSM data at legacy location: {}",
                legacy_path.display()
            );
            println!("New location: {}", csm_home.display());
            println!();
            println!("Run `csm migrate` to migrate your data to the new location.");
            println!("Or run `csm migrate --dry-run` to preview the migration.");
            println!();
            return Err(Error::Config(
                "Migration needed. Run `csm migrate` first.".to_string(),
            ));
        }
    }

    // Check if already initialized (has config.toml or registry.db)
    let config_path = csm_home.join("config.toml");
    let db_path = csm_home.join("registry.db");
    if (config_path.exists() || db_path.exists()) && !force {
        return Err(Error::AlreadyInitialized);
    }

    // Create CSM home directory
    std::fs::create_dir_all(&csm_home).map_err(|e| Error::Io(e))?;

    // Create subdirectories
    std::fs::create_dir_all(csm_home.join("skills")).map_err(|e| Error::Io(e))?;
    std::fs::create_dir_all(csm_home.join("cache")).map_err(|e| Error::Io(e))?;
    std::fs::create_dir_all(csm_home.join("logs")).map_err(|e| Error::Io(e))?;

    // Initialize config
    let config_manager = ConfigManagerImpl::new(csm_home.clone());
    config_manager.save()?;

    // Initialize database
    let _skill_repo = crate::infra::SqliteSkillRepository::new(&db_path)?;
    let _conflict_repo = crate::infra::SqliteConflictRepository::new(&db_path)?;

    println!("CSM initialized successfully at {}", csm_home.display());

    if import_existing {
        todo!("import existing CLAUDE.md files")
    }

    Ok(())
}
