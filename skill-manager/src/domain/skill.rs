//! Skill domain model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SkillSource;

/// Represents a Claude skill
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    /// Priority for merge ordering (higher = earlier in merged output)
    pub priority: i32,

    /// Update mode for this skill
    pub update_mode: UpdateMode,
}

impl Skill {
    /// Create a new skill with default values
    pub fn new(name: impl Into<String>, source: SkillSource, scope: SkillScope) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            source,
            scope,
            enabled: true,
            content_hash: String::new(),
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            priority: 50, // Default middle priority
            update_mode: UpdateMode::default(),
        }
    }

    /// Create a builder for constructing a skill
    pub fn builder(name: impl Into<String>) -> SkillBuilder {
        SkillBuilder::new(name)
    }

    /// Check if this skill is from a remote source (can be updated)
    pub fn is_remote(&self) -> bool {
        matches!(self.source, SkillSource::GitHub { .. } | SkillSource::Url { .. })
    }

    /// Check if this skill is global scope
    pub fn is_global(&self) -> bool {
        matches!(self.scope, SkillScope::Global)
    }
}

impl Default for Skill {
    fn default() -> Self {
        Self::new("unnamed", SkillSource::Inline, SkillScope::Global)
    }
}

/// Builder for constructing Skill instances
#[derive(Debug, Clone)]
pub struct SkillBuilder {
    name: String,
    description: Option<String>,
    source: SkillSource,
    scope: SkillScope,
    enabled: bool,
    tags: Vec<String>,
    priority: i32,
    update_mode: UpdateMode,
}

impl SkillBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            source: SkillSource::Inline,
            scope: SkillScope::Global,
            enabled: true,
            tags: Vec::new(),
            priority: 50,
            update_mode: UpdateMode::default(),
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn source(mut self, source: SkillSource) -> Self {
        self.source = source;
        self
    }

    pub fn scope(mut self, scope: SkillScope) -> Self {
        self.scope = scope;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn update_mode(mut self, mode: UpdateMode) -> Self {
        self.update_mode = mode;
        self
    }

    pub fn build(self) -> Skill {
        let now = Utc::now();
        Skill {
            id: Uuid::new_v4(),
            name: self.name,
            description: self.description,
            source: self.source,
            scope: self.scope,
            enabled: self.enabled,
            content_hash: String::new(),
            created_at: now,
            updated_at: now,
            tags: self.tags,
            priority: self.priority,
            update_mode: self.update_mode,
        }
    }
}

/// Scope of a skill - where it applies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SkillScope {
    /// Applies to all projects
    Global,

    /// Applies to a specific project
    Project {
        /// Path to the project root
        path: std::path::PathBuf,
    },
}

impl SkillScope {
    /// Create a project scope from a path
    pub fn project(path: impl Into<std::path::PathBuf>) -> Self {
        Self::Project { path: path.into() }
    }

    /// Check if this is a global scope
    pub fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Get the project path if this is a project scope
    pub fn project_path(&self) -> Option<&std::path::Path> {
        match self {
            Self::Project { path } => Some(path),
            Self::Global => None,
        }
    }
}

impl Default for SkillScope {
    fn default() -> Self {
        Self::Global
    }
}

impl std::fmt::Display for SkillScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Global => write!(f, "global"),
            Self::Project { path } => write!(f, "project:{}", path.display()),
        }
    }
}

/// How the skill should be updated
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum UpdateMode {
    /// Automatically update when changes detected
    #[default]
    Auto,

    /// Notify of updates but don't auto-apply
    Notify,

    /// Only update on explicit request
    Manual,
}

impl std::fmt::Display for UpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Notify => write!(f, "notify"),
            Self::Manual => write!(f, "manual"),
        }
    }
}

impl std::str::FromStr for UpdateMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "notify" => Ok(Self::Notify),
            "manual" => Ok(Self::Manual),
            _ => Err(format!("Invalid update mode: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_new() {
        let skill = Skill::new("test-skill", SkillSource::Inline, SkillScope::Global);
        assert_eq!(skill.name, "test-skill");
        assert!(skill.enabled);
        assert_eq!(skill.priority, 50);
    }

    #[test]
    fn test_skill_builder() {
        let skill = Skill::builder("my-skill")
            .description("A test skill")
            .priority(100)
            .enabled(false)
            .build();

        assert_eq!(skill.name, "my-skill");
        assert_eq!(skill.description, Some("A test skill".to_string()));
        assert_eq!(skill.priority, 100);
        assert!(!skill.enabled);
    }

    #[test]
    fn test_skill_is_remote() {
        let local = Skill::new("local", SkillSource::Inline, SkillScope::Global);
        assert!(!local.is_remote());

        let github = Skill::new(
            "github",
            SkillSource::GitHub {
                owner: "test".to_string(),
                repo: "repo".to_string(),
                path: None,
                ref_spec: None,
                commit_sha: None,
            },
            SkillScope::Global,
        );
        assert!(github.is_remote());
    }

    #[test]
    fn test_update_mode_from_str() {
        assert_eq!("auto".parse::<UpdateMode>().unwrap(), UpdateMode::Auto);
        assert_eq!("notify".parse::<UpdateMode>().unwrap(), UpdateMode::Notify);
        assert_eq!("manual".parse::<UpdateMode>().unwrap(), UpdateMode::Manual);
        assert!("invalid".parse::<UpdateMode>().is_err());
    }
}
