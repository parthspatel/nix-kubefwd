//! Error types for Claude Skill Manager

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for CSM operations
#[derive(Error, Debug)]
pub enum Error {
    // =========================================================================
    // Skill Errors
    // =========================================================================
    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Skill already exists: {0}")]
    SkillExists(String),

    #[error("Invalid skill name: {0}")]
    InvalidSkillName(String),

    #[error("Invalid skill content: {0}")]
    InvalidContent(String),

    // =========================================================================
    // Source Errors
    // =========================================================================
    #[error("Invalid source format: {0}")]
    InvalidSource(String),

    #[error("Source not accessible: {0}")]
    SourceNotAccessible(String),

    #[error("Failed to fetch from source: {0}")]
    FetchFailed(String),

    // =========================================================================
    // GitHub Errors
    // =========================================================================
    #[error("GitHub error: {0}")]
    GitHub(String),

    #[error("GitHub rate limit exceeded")]
    RateLimited,

    #[error("GitHub repository not found: {owner}/{repo}")]
    RepoNotFound { owner: String, repo: String },

    // =========================================================================
    // File System Errors
    // =========================================================================
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // =========================================================================
    // Database Errors
    // =========================================================================
    #[error("Database error: {0}")]
    Database(String),

    // =========================================================================
    // Configuration Errors
    // =========================================================================
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Not initialized: run 'csm init' first")]
    NotInitialized,

    #[error("Already initialized")]
    AlreadyInitialized,

    // =========================================================================
    // Conflict Errors
    // =========================================================================
    #[error("Conflict not found: {0}")]
    ConflictNotFound(String),

    #[error("Unresolved conflicts exist")]
    UnresolvedConflicts,

    // =========================================================================
    // Network Errors
    // =========================================================================
    #[error("Network error: {0}")]
    Network(String),

    #[error("Request timeout")]
    Timeout,

    // =========================================================================
    // Validation Errors
    // =========================================================================
    #[error("Validation error: {0}")]
    Validation(String),

    // =========================================================================
    // Generic Errors
    // =========================================================================
    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    /// Get the exit code for this error (for CLI)
    pub fn exit_code(&self) -> i32 {
        match self {
            // General errors
            Self::Other(_) | Self::Anyhow(_) => 1,

            // Not found errors
            Self::SkillNotFound(_) | Self::FileNotFound(_) | Self::ConflictNotFound(_) => 1,

            // Validation/argument errors
            Self::InvalidSource(_)
            | Self::InvalidSkillName(_)
            | Self::InvalidContent(_)
            | Self::Validation(_) => 2,

            // Configuration errors
            Self::Config(_) | Self::NotInitialized | Self::AlreadyInitialized => 3,

            // Network errors
            Self::Network(_)
            | Self::Timeout
            | Self::GitHub(_)
            | Self::RateLimited
            | Self::FetchFailed(_)
            | Self::SourceNotAccessible(_)
            | Self::RepoNotFound { .. } => 4,

            // Conflict errors
            Self::UnresolvedConflicts => 5,

            // Permission/IO errors
            Self::PermissionDenied(_) | Self::Io(_) => 6,

            // Database errors
            Self::Database(_) => 7,

            // Existing resource errors
            Self::SkillExists(_) => 8,
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Network(_) | Self::Timeout | Self::RateLimited | Self::FetchFailed(_)
        )
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a database error
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    /// Create a GitHub error
    pub fn github(msg: impl Into<String>) -> Self {
        Self::GitHub(msg.into())
    }

    /// Create a network error
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }
}

// Convert from rusqlite errors
impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database(err.to_string())
    }
}

// Convert from reqwest errors
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Self::Timeout
        } else if err.is_connect() {
            Self::Network(format!("Connection failed: {}", err))
        } else {
            Self::Network(err.to_string())
        }
    }
}

// Convert from URL parse errors
impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::InvalidSource(format!("Invalid URL: {}", err))
    }
}

// Convert from serde_json errors
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(format!("JSON error: {}", err))
    }
}

// Convert from toml errors
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Config(format!("TOML parse error: {}", err))
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::Config(format!("TOML serialize error: {}", err))
    }
}

/// Result type alias for CSM operations
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_codes() {
        assert_eq!(Error::SkillNotFound("test".to_string()).exit_code(), 1);
        assert_eq!(Error::InvalidSource("test".to_string()).exit_code(), 2);
        assert_eq!(Error::NotInitialized.exit_code(), 3);
        assert_eq!(Error::Network("test".to_string()).exit_code(), 4);
        assert_eq!(Error::UnresolvedConflicts.exit_code(), 5);
    }

    #[test]
    fn test_is_retryable() {
        assert!(Error::Network("test".to_string()).is_retryable());
        assert!(Error::Timeout.is_retryable());
        assert!(Error::RateLimited.is_retryable());
        assert!(!Error::SkillNotFound("test".to_string()).is_retryable());
        assert!(!Error::NotInitialized.is_retryable());
    }

    #[test]
    fn test_error_display() {
        let err = Error::SkillNotFound("my-skill".to_string());
        assert_eq!(err.to_string(), "Skill not found: my-skill");

        let err = Error::RepoNotFound {
            owner: "user".to_string(),
            repo: "repo".to_string(),
        };
        assert_eq!(err.to_string(), "GitHub repository not found: user/repo");
    }
}
