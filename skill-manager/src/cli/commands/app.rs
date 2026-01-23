//! Application context for CLI commands
//!
//! Provides initialized services for command execution.

use std::sync::Arc;

use crate::domain::EventBus;
use crate::infra::{
    ConfigManagerImpl, FileOutputStorage, FileSkillStorage, GitHubClientImpl, SimpleUrlClient,
    SqliteConflictRepository, SqliteSkillRepository,
};
use crate::services::{
    ConfigManager, ConflictServiceImpl, MergeServiceImpl, SkillServiceImpl, UpdateServiceImpl,
};
use crate::utils::error::{Error, Result};

// Type aliases for complex service types
type MergeServiceType =
    MergeServiceImpl<SqliteSkillRepository, FileSkillStorage, FileOutputStorage>;
type SkillServiceType = SkillServiceImpl<
    SqliteSkillRepository,
    FileSkillStorage,
    GitHubClientImpl,
    SimpleUrlClient,
    MergeServiceType,
>;
type ConflictServiceType = ConflictServiceImpl<
    SqliteConflictRepository,
    SqliteSkillRepository,
    FileSkillStorage,
    MergeServiceType,
>;
type UpdateServiceType = UpdateServiceImpl<
    SqliteSkillRepository,
    FileSkillStorage,
    GitHubClientImpl,
    SimpleUrlClient,
    MergeServiceType,
>;

/// Application context with initialized services
pub struct AppContext {
    pub config: ConfigManagerImpl,
    pub skill_service: Arc<SkillServiceType>,
    pub merge_service: Arc<MergeServiceType>,
    pub conflict_service: Arc<ConflictServiceType>,
    pub update_service: Arc<UpdateServiceType>,
    pub skill_repo: Arc<SqliteSkillRepository>,
    pub conflict_repo: Arc<SqliteConflictRepository>,
    pub storage: Arc<FileSkillStorage>,
    pub output_storage: Arc<FileOutputStorage>,
}

impl AppContext {
    /// Create a new application context
    ///
    /// This initializes all services and repositories.
    pub fn new() -> Result<Self> {
        let csm_home = ConfigManagerImpl::detect_csm_home();

        // Check if initialized
        if !csm_home.exists() {
            return Err(Error::NotInitialized);
        }

        // Load configuration
        let mut config = ConfigManagerImpl::new(csm_home.clone());
        config.load()?;

        // Initialize repositories
        let db_path = config.database_path();
        let skill_repo = Arc::new(SqliteSkillRepository::new(&db_path)?);
        let conflict_repo = Arc::new(SqliteConflictRepository::new(&db_path)?);

        // Initialize storage
        let storage = Arc::new(FileSkillStorage::new(&csm_home));
        let output_storage = Arc::new(FileOutputStorage::new(&csm_home));

        // Initialize clients
        let github_token = std::env::var("GITHUB_TOKEN").ok();
        let github_client = Arc::new(GitHubClientImpl::new(github_token));
        let url_client = Arc::new(SimpleUrlClient::new());

        // Initialize event bus
        let event_bus = Arc::new(std::sync::RwLock::new(EventBus::new()));

        // Initialize merge service
        let merge_service = Arc::new(MergeServiceImpl::new(
            skill_repo.clone(),
            storage.clone(),
            output_storage.clone(),
            event_bus.clone(),
        ));

        // Initialize skill service
        let skill_service = Arc::new(SkillServiceImpl::new(
            skill_repo.clone(),
            storage.clone(),
            github_client.clone(),
            url_client.clone(),
            merge_service.clone(),
            event_bus.clone(),
        ));

        // Initialize conflict service
        let conflict_service = Arc::new(ConflictServiceImpl::new(
            conflict_repo.clone(),
            skill_repo.clone(),
            storage.clone(),
            merge_service.clone(),
            event_bus.clone(),
        ));

        // Initialize update service
        let update_service = Arc::new(UpdateServiceImpl::new(
            skill_repo.clone(),
            storage.clone(),
            github_client.clone(),
            url_client.clone(),
            merge_service.clone(),
            event_bus.clone(),
        ));

        Ok(Self {
            config,
            skill_service,
            merge_service,
            conflict_service,
            update_service,
            skill_repo,
            conflict_repo,
            storage,
            output_storage,
        })
    }
}
