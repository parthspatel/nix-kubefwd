//! Core traits (interfaces) for the service layer
//!
//! These traits define the contracts that infrastructure implementations must fulfill.
//! This enables dependency injection and testability.

use async_trait::async_trait;
use std::path::Path;
use uuid::Uuid;

use crate::domain::{Conflict, Skill, SkillScope, SkillSource};
use crate::utils::error::Result;

// =============================================================================
// Repository Traits (Data Access)
// =============================================================================

/// Repository for skill persistence
#[async_trait]
pub trait SkillRepository: Send + Sync {
    /// Create a new skill in the repository
    async fn create(&self, skill: &Skill) -> Result<()>;

    /// Get a skill by its ID
    async fn get(&self, id: Uuid) -> Result<Option<Skill>>;

    /// Get a skill by its name
    async fn get_by_name(&self, name: &str) -> Result<Option<Skill>>;

    /// Update an existing skill
    async fn update(&self, skill: &Skill) -> Result<()>;

    /// Delete a skill by ID
    async fn delete(&self, id: Uuid) -> Result<()>;

    /// List all skills
    async fn list(&self) -> Result<Vec<Skill>>;

    /// List skills by scope
    async fn list_by_scope(&self, scope: &SkillScope) -> Result<Vec<Skill>>;

    /// List only enabled skills
    async fn list_enabled(&self) -> Result<Vec<Skill>>;

    /// Search skills by query
    async fn search(&self, query: &str) -> Result<Vec<Skill>>;

    /// Check if a skill with the given name exists
    async fn exists(&self, name: &str) -> Result<bool>;
}

/// Repository for conflict persistence
#[async_trait]
pub trait ConflictRepository: Send + Sync {
    /// Create a new conflict
    async fn create(&self, conflict: &Conflict) -> Result<()>;

    /// Get a conflict by ID
    async fn get(&self, id: Uuid) -> Result<Option<Conflict>>;

    /// Update a conflict
    async fn update(&self, conflict: &Conflict) -> Result<()>;

    /// Delete a conflict
    async fn delete(&self, id: Uuid) -> Result<()>;

    /// List all conflicts
    async fn list(&self) -> Result<Vec<Conflict>>;

    /// List unresolved conflicts
    async fn list_unresolved(&self) -> Result<Vec<Conflict>>;

    /// List conflicts involving a specific skill
    async fn list_by_skill(&self, skill_id: Uuid) -> Result<Vec<Conflict>>;

    /// Delete all conflicts involving a skill
    async fn delete_by_skill(&self, skill_id: Uuid) -> Result<()>;
}

// =============================================================================
// Storage Traits (File System)
// =============================================================================

/// Storage for skill content files
#[async_trait]
pub trait SkillStorage: Send + Sync {
    /// Store skill content
    async fn store(&self, skill_id: Uuid, content: &str) -> Result<String>;

    /// Read skill content
    async fn read(&self, skill_id: Uuid) -> Result<String>;

    /// Delete skill content
    async fn delete(&self, skill_id: Uuid) -> Result<()>;

    /// Check if skill content exists
    async fn exists(&self, skill_id: Uuid) -> Result<bool>;

    /// Get the path to a skill's content
    fn get_path(&self, skill_id: Uuid) -> std::path::PathBuf;

    /// Calculate content hash
    fn hash_content(&self, content: &str) -> String;
}

/// Storage for merged CLAUDE.md output
#[async_trait]
pub trait OutputStorage: Send + Sync {
    /// Write merged content to CLAUDE.md
    async fn write_claude_md(&self, scope: &SkillScope, content: &str) -> Result<()>;

    /// Read current CLAUDE.md content
    async fn read_claude_md(&self, scope: &SkillScope) -> Result<Option<String>>;

    /// Get the path to CLAUDE.md for a scope
    fn get_claude_md_path(&self, scope: &SkillScope) -> std::path::PathBuf;

    /// Create symlinks for a project
    async fn create_symlinks(&self, project_path: &Path, skill_ids: &[Uuid]) -> Result<()>;

    /// Remove symlinks for a project
    async fn remove_symlinks(&self, project_path: &Path) -> Result<()>;
}

// =============================================================================
// External Service Traits
// =============================================================================

/// Client for fetching skills from GitHub
#[async_trait]
pub trait GitHubClient: Send + Sync {
    /// Fetch skill content from GitHub
    async fn fetch_content(
        &self,
        owner: &str,
        repo: &str,
        path: Option<&str>,
        ref_spec: Option<&str>,
    ) -> Result<FetchResult>;

    /// Check if updates are available
    async fn check_updates(
        &self,
        owner: &str,
        repo: &str,
        current_sha: &str,
        ref_spec: Option<&str>,
    ) -> Result<Option<UpdateInfo>>;

    /// Get rate limit status
    async fn rate_limit(&self) -> Result<RateLimitInfo>;
}

/// Result of fetching content
#[derive(Debug, Clone)]
pub struct FetchResult {
    /// The content fetched
    pub content: String,
    /// SHA of the file
    pub sha: String,
    /// Commit SHA
    pub commit_sha: String,
}

/// Information about available updates
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    /// Current commit SHA
    pub current_sha: String,
    /// Latest commit SHA
    pub latest_sha: String,
    /// Number of commits behind
    pub commits_behind: usize,
    /// Commit messages
    pub commit_messages: Vec<String>,
}

/// Rate limit information
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    /// Maximum requests allowed
    pub limit: u32,
    /// Remaining requests
    pub remaining: u32,
    /// Reset timestamp (Unix)
    pub reset: u64,
}

/// Client for fetching skills from URLs
#[async_trait]
pub trait UrlClient: Send + Sync {
    /// Fetch content from a URL
    async fn fetch(&self, url: &str) -> Result<UrlFetchResult>;

    /// Check if content has changed (using ETag)
    async fn check_modified(&self, url: &str, etag: Option<&str>) -> Result<bool>;
}

/// Result of fetching URL content
#[derive(Debug, Clone)]
pub struct UrlFetchResult {
    /// The content fetched
    pub content: String,
    /// ETag for caching
    pub etag: Option<String>,
}

// =============================================================================
// Configuration Trait
// =============================================================================

/// Configuration management
pub trait ConfigManager: Send + Sync {
    /// Get a configuration value
    fn get(&self, key: &str) -> Option<String>;

    /// Set a configuration value
    fn set(&mut self, key: &str, value: &str) -> Result<()>;

    /// Get the CSM home directory
    fn csm_home(&self) -> &Path;

    /// Get the global skills directory
    fn global_skills_dir(&self) -> std::path::PathBuf;

    /// Get the cache directory
    fn cache_dir(&self) -> std::path::PathBuf;

    /// Get the database path
    fn database_path(&self) -> std::path::PathBuf;

    /// Check if CSM is initialized
    fn is_initialized(&self) -> bool;
}

// =============================================================================
// Service Traits
// =============================================================================

/// Skill management service interface
#[async_trait]
pub trait SkillService: Send + Sync {
    /// Add a new skill from a source
    async fn add(&self, source: &str, name: Option<&str>, scope: SkillScope) -> Result<Skill>;

    /// Remove a skill
    async fn remove(&self, name: &str) -> Result<()>;

    /// Enable a skill
    async fn enable(&self, name: &str) -> Result<()>;

    /// Disable a skill
    async fn disable(&self, name: &str) -> Result<()>;

    /// Get a skill by name
    async fn get(&self, name: &str) -> Result<Option<Skill>>;

    /// List all skills
    async fn list(&self, scope: Option<SkillScope>, enabled_only: bool) -> Result<Vec<Skill>>;

    /// Search skills
    async fn search(&self, query: &str) -> Result<Vec<Skill>>;

    /// Get skill content
    async fn get_content(&self, name: &str) -> Result<String>;
}

/// Update service interface
#[async_trait]
pub trait UpdateService: Send + Sync {
    /// Check for available updates
    async fn check(&self) -> Result<Vec<(Skill, UpdateInfo)>>;

    /// Update a specific skill
    async fn update_skill(&self, name: &str) -> Result<bool>;

    /// Update all skills
    async fn update_all(&self) -> Result<Vec<(String, bool)>>;
}

/// Conflict detection and resolution service interface
#[async_trait]
pub trait ConflictService: Send + Sync {
    /// Detect conflicts between enabled skills
    async fn detect(&self) -> Result<Vec<Conflict>>;

    /// Get all unresolved conflicts
    async fn list_unresolved(&self) -> Result<Vec<Conflict>>;

    /// Resolve a conflict with a strategy
    async fn resolve(&self, conflict_id: Uuid, strategy: crate::domain::ResolutionStrategy)
        -> Result<()>;

    /// Ignore a conflict
    async fn ignore(&self, conflict_id: Uuid) -> Result<()>;
}

/// Merge service interface
#[async_trait]
pub trait MergeService: Send + Sync {
    /// Merge enabled skills into CLAUDE.md
    async fn merge(&self, scope: &SkillScope) -> Result<String>;

    /// Rebuild all CLAUDE.md files
    async fn rebuild_all(&self) -> Result<()>;
}

// =============================================================================
// Mock implementations for testing
// =============================================================================

#[cfg(test)]
pub mod mocks {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Simple mock for SkillRepository
    #[derive(Default)]
    pub struct MockSkillRepository {
        pub skills: Arc<Mutex<Vec<Skill>>>,
        pub exists_result: Arc<Mutex<Option<bool>>>,
    }

    impl MockSkillRepository {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_skills(skills: Vec<Skill>) -> Self {
            Self {
                skills: Arc::new(Mutex::new(skills)),
                exists_result: Arc::new(Mutex::new(None)),
            }
        }
    }

    #[async_trait]
    impl SkillRepository for MockSkillRepository {
        async fn create(&self, skill: &Skill) -> Result<()> {
            self.skills.lock().unwrap().push(skill.clone());
            Ok(())
        }

        async fn get(&self, id: Uuid) -> Result<Option<Skill>> {
            Ok(self.skills.lock().unwrap().iter().find(|s| s.id == id).cloned())
        }

        async fn get_by_name(&self, name: &str) -> Result<Option<Skill>> {
            Ok(self.skills.lock().unwrap().iter().find(|s| s.name == name).cloned())
        }

        async fn update(&self, skill: &Skill) -> Result<()> {
            let mut skills = self.skills.lock().unwrap();
            if let Some(existing) = skills.iter_mut().find(|s| s.id == skill.id) {
                *existing = skill.clone();
            }
            Ok(())
        }

        async fn delete(&self, id: Uuid) -> Result<()> {
            self.skills.lock().unwrap().retain(|s| s.id != id);
            Ok(())
        }

        async fn list(&self) -> Result<Vec<Skill>> {
            Ok(self.skills.lock().unwrap().clone())
        }

        async fn list_by_scope(&self, scope: &SkillScope) -> Result<Vec<Skill>> {
            Ok(self.skills.lock().unwrap().iter().filter(|s| &s.scope == scope).cloned().collect())
        }

        async fn list_enabled(&self) -> Result<Vec<Skill>> {
            Ok(self.skills.lock().unwrap().iter().filter(|s| s.enabled).cloned().collect())
        }

        async fn search(&self, query: &str) -> Result<Vec<Skill>> {
            Ok(self.skills.lock().unwrap().iter()
                .filter(|s| s.name.contains(query) || s.description.as_ref().is_some_and(|d| d.contains(query)))
                .cloned()
                .collect())
        }

        async fn exists(&self, name: &str) -> Result<bool> {
            if let Some(result) = *self.exists_result.lock().unwrap() {
                return Ok(result);
            }
            Ok(self.skills.lock().unwrap().iter().any(|s| s.name == name))
        }
    }

    /// Simple mock for SkillStorage
    #[derive(Default)]
    pub struct MockSkillStorage {
        pub content: Arc<Mutex<std::collections::HashMap<Uuid, String>>>,
    }

    impl MockSkillStorage {
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[async_trait]
    impl SkillStorage for MockSkillStorage {
        async fn store(&self, skill_id: Uuid, content: &str) -> Result<String> {
            let hash = self.hash_content(content);
            self.content.lock().unwrap().insert(skill_id, content.to_string());
            Ok(hash)
        }

        async fn read(&self, skill_id: Uuid) -> Result<String> {
            self.content.lock().unwrap()
                .get(&skill_id)
                .cloned()
                .ok_or_else(|| crate::utils::error::Error::not_found("Skill content"))
        }

        async fn delete(&self, skill_id: Uuid) -> Result<()> {
            self.content.lock().unwrap().remove(&skill_id);
            Ok(())
        }

        async fn exists(&self, skill_id: Uuid) -> Result<bool> {
            Ok(self.content.lock().unwrap().contains_key(&skill_id))
        }

        fn get_path(&self, skill_id: Uuid) -> std::path::PathBuf {
            std::path::PathBuf::from(format!("/mock/skills/{}.md", skill_id))
        }

        fn hash_content(&self, content: &str) -> String {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            format!("{:x}", hasher.finalize())
        }
    }

    /// Simple mock for GitHubClient
    #[derive(Default)]
    pub struct MockGitHubClient {
        pub fetch_result: Arc<Mutex<Option<FetchResult>>>,
        pub update_info: Arc<Mutex<Option<UpdateInfo>>>,
    }

    impl MockGitHubClient {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_content(content: String, sha: String, commit_sha: String) -> Self {
            Self {
                fetch_result: Arc::new(Mutex::new(Some(FetchResult { content, sha, commit_sha }))),
                update_info: Arc::new(Mutex::new(None)),
            }
        }
    }

    #[async_trait]
    impl GitHubClient for MockGitHubClient {
        async fn fetch_content(
            &self,
            _owner: &str,
            _repo: &str,
            _path: Option<&str>,
            _ref_spec: Option<&str>,
        ) -> Result<FetchResult> {
            self.fetch_result.lock().unwrap()
                .clone()
                .ok_or_else(|| crate::utils::error::Error::github("No mock result configured"))
        }

        async fn check_updates(
            &self,
            _owner: &str,
            _repo: &str,
            _current_sha: &str,
            _ref_spec: Option<&str>,
        ) -> Result<Option<UpdateInfo>> {
            Ok(self.update_info.lock().unwrap().clone())
        }

        async fn rate_limit(&self) -> Result<RateLimitInfo> {
            Ok(RateLimitInfo {
                limit: 60,
                remaining: 60,
                reset: 0,
            })
        }
    }

    /// Simple mock for UrlClient
    #[derive(Default)]
    pub struct MockUrlClient {
        pub fetch_result: Arc<Mutex<Option<UrlFetchResult>>>,
        pub modified: Arc<Mutex<bool>>,
    }

    impl MockUrlClient {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_content(content: String) -> Self {
            Self {
                fetch_result: Arc::new(Mutex::new(Some(UrlFetchResult { content, etag: None }))),
                modified: Arc::new(Mutex::new(false)),
            }
        }
    }

    #[async_trait]
    impl UrlClient for MockUrlClient {
        async fn fetch(&self, _url: &str) -> Result<UrlFetchResult> {
            self.fetch_result.lock().unwrap()
                .clone()
                .ok_or_else(|| crate::utils::error::Error::network("No mock result configured"))
        }

        async fn check_modified(&self, _url: &str, _etag: Option<&str>) -> Result<bool> {
            Ok(*self.modified.lock().unwrap())
        }
    }

    /// Simple mock for ConflictRepository
    #[derive(Default)]
    pub struct MockConflictRepository {
        pub conflicts: Arc<Mutex<Vec<Conflict>>>,
    }

    impl MockConflictRepository {
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[async_trait]
    impl ConflictRepository for MockConflictRepository {
        async fn create(&self, conflict: &Conflict) -> Result<()> {
            self.conflicts.lock().unwrap().push(conflict.clone());
            Ok(())
        }

        async fn get(&self, id: Uuid) -> Result<Option<Conflict>> {
            Ok(self.conflicts.lock().unwrap().iter().find(|c| c.id == id).cloned())
        }

        async fn update(&self, conflict: &Conflict) -> Result<()> {
            let mut conflicts = self.conflicts.lock().unwrap();
            if let Some(existing) = conflicts.iter_mut().find(|c| c.id == conflict.id) {
                *existing = conflict.clone();
            }
            Ok(())
        }

        async fn delete(&self, id: Uuid) -> Result<()> {
            self.conflicts.lock().unwrap().retain(|c| c.id != id);
            Ok(())
        }

        async fn list(&self) -> Result<Vec<Conflict>> {
            Ok(self.conflicts.lock().unwrap().clone())
        }

        async fn list_unresolved(&self) -> Result<Vec<Conflict>> {
            Ok(self.conflicts.lock().unwrap().iter()
                .filter(|c| !c.is_resolved())
                .cloned()
                .collect())
        }

        async fn list_by_skill(&self, skill_id: Uuid) -> Result<Vec<Conflict>> {
            Ok(self.conflicts.lock().unwrap().iter()
                .filter(|c| c.skill_a_id == skill_id || c.skill_b_id == skill_id)
                .cloned()
                .collect())
        }

        async fn delete_by_skill(&self, skill_id: Uuid) -> Result<()> {
            self.conflicts.lock().unwrap().retain(|c| c.skill_a_id != skill_id && c.skill_b_id != skill_id);
            Ok(())
        }
    }
}
