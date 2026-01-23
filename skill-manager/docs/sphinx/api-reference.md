# API Reference

This document provides the API reference for the Claude Skill Manager library (`csm`).

## Overview

The CSM library is organized into several modules:

- **`domain`**: Core domain models (Skill, Source, Conflict)
- **`services`**: Application services with business logic
- **`infra`**: Infrastructure implementations (database, storage, API clients)
- **`cli`**: Command-line interface
- **`tui`**: Terminal user interface
- **`utils`**: Utility functions and error handling

## Domain Models

### Skill

Represents a Claude skill.

```rust
pub struct Skill {
    /// Unique identifier
    pub id: Uuid,

    /// Human-readable name (must be unique)
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Where the skill came from
    pub source: SkillSource,

    /// Scope of the skill (global or project-specific)
    pub scope: SkillScope,

    /// Whether the skill is currently active
    pub enabled: bool,

    /// SHA-256 hash of content for change detection
    pub content_hash: String,

    /// When the skill was first added
    pub created_at: DateTime<Utc>,

    /// When the skill was last modified
    pub updated_at: DateTime<Utc>,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Priority for merge ordering (higher = earlier)
    pub priority: i32,

    /// Update mode for this skill
    pub update_mode: UpdateMode,
}
```

#### Methods

```rust
impl Skill {
    /// Create a new skill with default values
    pub fn new(name: impl Into<String>, source: SkillSource) -> Self;

    /// Create a builder for constructing a skill
    pub fn builder(name: impl Into<String>) -> SkillBuilder;

    /// Check if the skill is from a remote source
    pub fn is_remote(&self) -> bool;

    /// Check if the skill can be updated
    pub fn can_update(&self) -> bool;
}
```

### SkillSource

Represents where a skill originated from.

```rust
pub enum SkillSource {
    /// Local file on disk
    Local {
        path: PathBuf,
    },

    /// GitHub repository
    GitHub {
        owner: String,
        repo: String,
        path: Option<String>,
        ref_spec: Option<String>,
        commit_sha: Option<String>,
    },

    /// Direct URL
    Url {
        url: String,
        etag: Option<String>,
    },

    /// Created inline (no external source)
    Inline,
}
```

#### Methods

```rust
impl SkillSource {
    /// Create a local source from a path
    pub fn local(path: impl Into<PathBuf>) -> Self;

    /// Create a GitHub source
    pub fn github(owner: impl Into<String>, repo: impl Into<String>) -> Self;

    /// Create a URL source
    pub fn url(url: impl Into<String>) -> Self;

    /// Parse a source string (e.g., "github:owner/repo")
    pub fn parse(source: &str) -> Result<Self>;

    /// Get a display string for the source
    pub fn display(&self) -> String;
}
```

### SkillScope

Defines the scope of a skill.

```rust
pub enum SkillScope {
    /// Global scope - applies to all projects
    Global,

    /// Project scope - applies to specific project
    Project {
        path: PathBuf,
    },
}
```

### UpdateMode

Controls how a skill receives updates.

```rust
pub enum UpdateMode {
    /// Automatically update when changes are detected
    Auto,

    /// Notify user of updates, require confirmation
    Notify,

    /// Only update when explicitly requested
    Manual,
}
```

### Conflict

Represents a conflict between two skills.

```rust
pub struct Conflict {
    /// Unique identifier
    pub id: Uuid,

    /// First skill involved
    pub skill_a_id: Uuid,

    /// Second skill involved
    pub skill_b_id: Uuid,

    /// Type of conflict
    pub conflict_type: ConflictType,

    /// Human-readable description
    pub description: String,

    /// Line number in skill A (if applicable)
    pub line_a: Option<usize>,

    /// Line number in skill B (if applicable)
    pub line_b: Option<usize>,

    /// Content snippet from skill A
    pub content_a: Option<String>,

    /// Content snippet from skill B
    pub content_b: Option<String>,

    /// Suggested resolution
    pub suggestion: Option<String>,

    /// Resolution status
    pub status: ConflictStatus,

    /// When the conflict was detected
    pub detected_at: DateTime<Utc>,

    /// When the conflict was resolved
    pub resolved_at: Option<DateTime<Utc>>,
}
```

### ConflictType

Types of conflicts that can occur.

```rust
pub enum ConflictType {
    /// Same instruction appears in multiple skills
    Duplicate,

    /// Skills contain contradictory instructions
    Contradictory,

    /// Unclear which skill should take precedence
    PrecedenceAmbiguity,

    /// Skills cannot be merged due to syntax issues
    SyntaxConflict,
}
```

### ConflictStatus

Status of a conflict.

```rust
pub enum ConflictStatus {
    /// Conflict is pending resolution
    Pending,

    /// Conflict has been resolved
    Resolved,

    /// Conflict was dismissed/ignored
    Dismissed,
}
```

## Service Traits

### SkillRepository

Interface for skill persistence.

```rust
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
```

### SkillStorage

Interface for skill content storage.

```rust
#[async_trait]
pub trait SkillStorage: Send + Sync {
    /// Store skill content and return the content hash
    async fn store(&self, skill_id: Uuid, content: &str) -> Result<String>;

    /// Retrieve skill content
    async fn retrieve(&self, skill_id: Uuid) -> Result<String>;

    /// Delete skill content
    async fn delete(&self, skill_id: Uuid) -> Result<()>;

    /// Check if skill content exists
    async fn exists(&self, skill_id: Uuid) -> Result<bool>;

    /// Get the path to a skill's content file
    fn get_path(&self, skill_id: Uuid) -> PathBuf;
}
```

### SkillService

Main service for skill management operations.

```rust
#[async_trait]
pub trait SkillService: Send + Sync {
    /// Add a skill from a source
    async fn add(
        &self,
        source: &str,
        name: Option<&str>,
        scope: SkillScope,
    ) -> Result<Skill>;

    /// Remove a skill
    async fn remove(&self, skill_id: Uuid) -> Result<()>;

    /// Enable a skill
    async fn enable(&self, skill_id: Uuid) -> Result<()>;

    /// Disable a skill
    async fn disable(&self, skill_id: Uuid) -> Result<()>;

    /// Get a skill by name or ID
    async fn get(&self, identifier: &str) -> Result<Option<Skill>>;

    /// List all skills
    async fn list(&self, scope: Option<&SkillScope>) -> Result<Vec<Skill>>;

    /// Search skills
    async fn search(&self, query: &str) -> Result<Vec<Skill>>;
}
```

### UpdateService

Service for managing skill updates.

```rust
#[async_trait]
pub trait UpdateService: Send + Sync {
    /// Check for available updates
    async fn check(&self) -> Result<Vec<UpdateInfo>>;

    /// Check a specific skill for updates
    async fn check_skill(&self, skill_id: Uuid) -> Result<Option<UpdateInfo>>;

    /// Apply all available updates
    async fn update_all(&self) -> Result<Vec<UpdateResult>>;

    /// Update a specific skill
    async fn update(&self, skill_id: Uuid) -> Result<UpdateResult>;
}
```

### ConflictService

Service for conflict detection and resolution.

```rust
#[async_trait]
pub trait ConflictService: Send + Sync {
    /// Detect conflicts across all skills
    async fn detect(&self) -> Result<Vec<Conflict>>;

    /// Detect conflicts for a specific skill
    async fn detect_for(&self, skill_id: Uuid) -> Result<Vec<Conflict>>;

    /// Resolve a conflict with a given strategy
    async fn resolve(
        &self,
        conflict_id: Uuid,
        strategy: ResolutionStrategy,
    ) -> Result<()>;

    /// Dismiss a conflict
    async fn dismiss(&self, conflict_id: Uuid) -> Result<()>;

    /// List pending conflicts
    async fn list_pending(&self) -> Result<Vec<Conflict>>;
}
```

### MergeService

Service for merging skills into output files.

```rust
#[async_trait]
pub trait MergeService: Send + Sync {
    /// Merge all enabled skills for a scope
    async fn merge(&self, scope: &SkillScope) -> Result<String>;

    /// Rebuild all merged output files
    async fn rebuild_all(&self) -> Result<()>;

    /// Get the merged output path for a scope
    fn output_path(&self, scope: &SkillScope) -> PathBuf;
}
```

### GitHubClient

Interface for GitHub API operations.

```rust
#[async_trait]
pub trait GitHubClient: Send + Sync {
    /// Fetch skill content from GitHub
    async fn fetch(
        &self,
        owner: &str,
        repo: &str,
        path: Option<&str>,
        ref_spec: Option<&str>,
    ) -> Result<FetchResult>;

    /// Check for updates (returns new commit SHA if available)
    async fn check_update(
        &self,
        owner: &str,
        repo: &str,
        current_sha: &str,
        ref_spec: Option<&str>,
    ) -> Result<Option<String>>;

    /// Get rate limit information
    async fn rate_limit(&self) -> Result<RateLimitInfo>;
}
```

## Infrastructure Implementations

### SqliteSkillRepository

SQLite-based implementation of `SkillRepository`.

```rust
impl SqliteSkillRepository {
    /// Create a new repository with the given database path
    pub fn new(db_path: &Path) -> Result<Self>;

    /// Create an in-memory repository (for testing)
    pub fn in_memory() -> Result<Self>;
}
```

### FileSkillStorage

File system based implementation of `SkillStorage`.

```rust
impl FileSkillStorage {
    /// Create a new skill storage with the given base path
    pub fn new(base_path: impl Into<PathBuf>) -> Self;
}
```

### GitHubClientImpl

Implementation of `GitHubClient` using the GitHub API.

```rust
impl GitHubClientImpl {
    /// Create a new GitHub client
    pub fn new(token: Option<String>) -> Self;

    /// Create with custom base URL (for testing)
    pub fn with_base_url(base_url: impl Into<String>, token: Option<String>) -> Self;
}
```

## Configuration

### Config

Application configuration structure.

```rust
pub struct Config {
    pub general: GeneralConfig,
    pub updates: UpdateConfig,
    pub github: GitHubConfig,
    pub ui: UiConfig,
}

pub struct GeneralConfig {
    /// Default scope for new skills
    pub default_scope: String,

    /// Preferred editor
    pub editor: Option<String>,

    /// Enable colored output
    pub color: bool,
}

pub struct UpdateConfig {
    /// Update mode (auto, notify, manual)
    pub mode: String,

    /// Update schedule (hourly, daily, weekly)
    pub schedule: String,

    /// Check for updates on startup
    pub check_on_startup: bool,
}

pub struct GitHubConfig {
    /// Default ref for GitHub sources
    pub default_ref: String,
}

pub struct UiConfig {
    /// UI theme (dark, light)
    pub theme: String,
}
```

### ConfigManager

Interface for configuration management.

```rust
pub trait ConfigManager {
    /// Load configuration
    fn load(&mut self) -> Result<()>;

    /// Save configuration
    fn save(&self) -> Result<()>;

    /// Get a configuration value
    fn get(&self, key: &str) -> Option<String>;

    /// Set a configuration value
    fn set(&mut self, key: &str, value: &str) -> Result<()>;

    /// Get the full configuration
    fn config(&self) -> &Config;
}
```

## Error Handling

### Error

Main error type for CSM operations.

```rust
pub enum Error {
    // Skill Errors
    SkillNotFound(String),
    SkillExists(String),
    InvalidSkillName(String),
    InvalidContent(String),

    // Source Errors
    InvalidSource(String),
    SourceNotAccessible(String),
    FetchFailed(String),

    // GitHub Errors
    GitHub(String),
    RateLimited,
    RepoNotFound { owner: String, repo: String },

    // File System Errors
    Io(std::io::Error),
    PathNotFound(PathBuf),

    // Database Errors
    Database(String),

    // Configuration Errors
    ConfigNotFound,
    InvalidConfig(String),
    AlreadyInitialized,

    // Other
    Internal(String),
}
```

### Result

Type alias for CSM results.

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

## Events

### DomainEvent

Events emitted during skill operations.

```rust
pub enum DomainEvent {
    SkillAdded {
        skill_id: Uuid,
        name: String,
        source: SkillSource,
        scope: SkillScope,
        timestamp: DateTime<Utc>,
    },

    SkillRemoved {
        skill_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },

    SkillEnabled {
        skill_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },

    SkillDisabled {
        skill_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },

    SkillUpdated {
        skill_id: Uuid,
        name: String,
        old_hash: String,
        new_hash: String,
        timestamp: DateTime<Utc>,
    },

    ConflictDetected {
        conflict_id: Uuid,
        skill_a_id: Uuid,
        skill_b_id: Uuid,
        conflict_type: ConflictType,
        timestamp: DateTime<Utc>,
    },

    ConflictResolved {
        conflict_id: Uuid,
        timestamp: DateTime<Utc>,
    },

    MergeCompleted {
        scope: SkillScope,
        skill_count: usize,
        timestamp: DateTime<Utc>,
    },
}
```

### EventBus

Event bus for publishing and subscribing to domain events.

```rust
impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self;

    /// Publish an event
    pub fn publish(&self, event: &DomainEvent);

    /// Subscribe to events
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&DomainEvent) + Send + Sync + 'static;
}
```

## Utilities

### Hash Functions

```rust
/// Calculate SHA-256 hash of content
pub fn sha256(content: &str) -> String;

/// Calculate SHA-256 hash and return first 12 characters
pub fn sha256_short(content: &str) -> String;
```

## Example Usage

### Adding a Skill Programmatically

```rust
use csm::domain::{SkillScope, SkillSource};
use csm::services::SkillService;
use csm::infra::{
    SqliteSkillRepository,
    FileSkillStorage,
    GitHubClientImpl,
};

async fn add_skill() -> csm::Result<()> {
    // Initialize components
    let repo = SqliteSkillRepository::new(&db_path)?;
    let storage = FileSkillStorage::new(&skills_path);
    let github = GitHubClientImpl::new(github_token);

    // Create service
    let service = SkillServiceImpl::new(
        Arc::new(repo),
        Arc::new(storage),
        Arc::new(github),
        // ... other dependencies
    );

    // Add a skill
    let skill = service.add(
        "github:anthropics/claude-skills/typescript",
        None,
        SkillScope::Global,
    ).await?;

    println!("Added skill: {}", skill.name);
    Ok(())
}
```

### Detecting Conflicts

```rust
use csm::services::ConflictService;

async fn check_conflicts(service: &impl ConflictService) -> csm::Result<()> {
    let conflicts = service.detect().await?;

    for conflict in conflicts {
        println!(
            "Conflict: {} ({:?})",
            conflict.description,
            conflict.conflict_type
        );
    }

    Ok(())
}
```
