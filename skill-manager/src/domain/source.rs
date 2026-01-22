//! Skill source models

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Source of a skill - where it came from
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SkillSource {
    /// Local file on disk
    Local {
        /// Path to the skill file
        path: PathBuf,
    },

    /// GitHub repository
    GitHub {
        /// Repository owner
        owner: String,
        /// Repository name
        repo: String,
        /// Optional path within repository
        path: Option<String>,
        /// Optional ref (branch, tag, commit)
        ref_spec: Option<String>,
        /// Tracked commit SHA for updates
        commit_sha: Option<String>,
    },

    /// Direct URL
    Url {
        /// URL to the skill file
        url: String,
        /// ETag for cache validation
        etag: Option<String>,
    },

    /// Created inline (no external source)
    Inline,
}

impl SkillSource {
    /// Create a local source from a path
    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::Local { path: path.into() }
    }

    /// Create a GitHub source
    pub fn github(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self::GitHub {
            owner: owner.into(),
            repo: repo.into(),
            path: None,
            ref_spec: None,
            commit_sha: None,
        }
    }

    /// Create a GitHub source with path
    pub fn github_path(
        owner: impl Into<String>,
        repo: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self::GitHub {
            owner: owner.into(),
            repo: repo.into(),
            path: Some(path.into()),
            ref_spec: None,
            commit_sha: None,
        }
    }

    /// Create a URL source
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url {
            url: url.into(),
            etag: None,
        }
    }

    /// Check if this source can be updated
    pub fn is_updatable(&self) -> bool {
        matches!(self, Self::GitHub { .. } | Self::Url { .. })
    }

    /// Check if this source is local
    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local { .. } | Self::Inline)
    }

    /// Get a display string for this source
    pub fn display_string(&self) -> String {
        match self {
            Self::Local { path } => format!("local:{}", path.display()),
            Self::GitHub {
                owner,
                repo,
                path,
                ref_spec,
                ..
            } => {
                let mut s = format!("github:{}/{}", owner, repo);
                if let Some(p) = path {
                    s.push('/');
                    s.push_str(p);
                }
                if let Some(r) = ref_spec {
                    s.push('@');
                    s.push_str(r);
                }
                s
            }
            Self::Url { url, .. } => url.clone(),
            Self::Inline => "inline".to_string(),
        }
    }
}

impl Default for SkillSource {
    fn default() -> Self {
        Self::Inline
    }
}

impl std::fmt::Display for SkillSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_string())
    }
}

/// Parsed skill source from a string input
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedSource {
    pub source: SkillSource,
    pub suggested_name: String,
}

/// Parse a source string into a SkillSource
///
/// Supported formats:
/// - `github:owner/repo`
/// - `github:owner/repo/path`
/// - `github:owner/repo@ref`
/// - `github:owner/repo/path@ref`
/// - `/path/to/file` or `./path/to/file`
/// - `https://...` or `http://...`
pub fn parse_source(input: &str) -> Result<ParsedSource, SourceParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(SourceParseError::Empty);
    }

    // GitHub source
    if let Some(rest) = input.strip_prefix("github:") {
        return parse_github_source(rest);
    }

    // URL source
    if input.starts_with("https://") || input.starts_with("http://") {
        return parse_url_source(input);
    }

    // Local file path
    if input.starts_with('/') || input.starts_with("./") || input.starts_with("../") {
        return parse_local_source(input);
    }

    // Try to detect if it looks like a GitHub shorthand (owner/repo)
    if input.contains('/') && !input.contains(':') && !input.contains(' ') {
        // Might be shorthand github reference
        return parse_github_source(input);
    }

    Err(SourceParseError::UnknownFormat(input.to_string()))
}

fn parse_github_source(input: &str) -> Result<ParsedSource, SourceParseError> {
    // Split off ref if present (e.g., @main, @v1.0.0)
    let (path_part, ref_spec) = if let Some(idx) = input.rfind('@') {
        let (p, r) = input.split_at(idx);
        (p, Some(r[1..].to_string()))
    } else {
        (input, None)
    };

    // Parse owner/repo/path
    let parts: Vec<&str> = path_part.split('/').collect();
    if parts.len() < 2 {
        return Err(SourceParseError::InvalidGitHub(
            "Expected format: owner/repo[/path][@ref]".to_string(),
        ));
    }

    let owner = parts[0].to_string();
    let repo = parts[1].to_string();

    if owner.is_empty() || repo.is_empty() {
        return Err(SourceParseError::InvalidGitHub(
            "Owner and repo cannot be empty".to_string(),
        ));
    }

    let path = if parts.len() > 2 {
        Some(parts[2..].join("/"))
    } else {
        None
    };

    // Derive suggested name from repo or path
    let suggested_name = if let Some(ref p) = path {
        p.split('/').last().unwrap_or(&repo).to_string()
    } else {
        repo.clone()
    };

    Ok(ParsedSource {
        source: SkillSource::GitHub {
            owner,
            repo,
            path,
            ref_spec,
            commit_sha: None,
        },
        suggested_name,
    })
}

fn parse_url_source(input: &str) -> Result<ParsedSource, SourceParseError> {
    // Validate URL
    let url = url::Url::parse(input).map_err(|e| SourceParseError::InvalidUrl(e.to_string()))?;

    // Derive name from last path segment
    let suggested_name = url
        .path_segments()
        .and_then(|s| s.last())
        .map(|s| s.trim_end_matches(".md"))
        .unwrap_or("skill")
        .to_string();

    Ok(ParsedSource {
        source: SkillSource::Url {
            url: input.to_string(),
            etag: None,
        },
        suggested_name,
    })
}

fn parse_local_source(input: &str) -> Result<ParsedSource, SourceParseError> {
    let path = PathBuf::from(input);

    // Derive name from filename
    let suggested_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("skill")
        .to_string();

    Ok(ParsedSource {
        source: SkillSource::Local { path },
        suggested_name,
    })
}

/// Errors that can occur when parsing a source string
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum SourceParseError {
    #[error("Source string is empty")]
    Empty,

    #[error("Invalid GitHub source: {0}")]
    InvalidGitHub(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Unknown source format: {0}")]
    UnknownFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_basic() {
        let result = parse_source("github:owner/repo").unwrap();
        assert_eq!(
            result.source,
            SkillSource::GitHub {
                owner: "owner".to_string(),
                repo: "repo".to_string(),
                path: None,
                ref_spec: None,
                commit_sha: None,
            }
        );
        assert_eq!(result.suggested_name, "repo");
    }

    #[test]
    fn test_parse_github_with_path() {
        let result = parse_source("github:owner/repo/skills/typescript").unwrap();
        assert_eq!(
            result.source,
            SkillSource::GitHub {
                owner: "owner".to_string(),
                repo: "repo".to_string(),
                path: Some("skills/typescript".to_string()),
                ref_spec: None,
                commit_sha: None,
            }
        );
        assert_eq!(result.suggested_name, "typescript");
    }

    #[test]
    fn test_parse_github_with_ref() {
        let result = parse_source("github:owner/repo@v1.0.0").unwrap();
        assert_eq!(
            result.source,
            SkillSource::GitHub {
                owner: "owner".to_string(),
                repo: "repo".to_string(),
                path: None,
                ref_spec: Some("v1.0.0".to_string()),
                commit_sha: None,
            }
        );
    }

    #[test]
    fn test_parse_github_full() {
        let result = parse_source("github:anthropics/claude-skills/typescript@main").unwrap();
        assert_eq!(
            result.source,
            SkillSource::GitHub {
                owner: "anthropics".to_string(),
                repo: "claude-skills".to_string(),
                path: Some("typescript".to_string()),
                ref_spec: Some("main".to_string()),
                commit_sha: None,
            }
        );
    }

    #[test]
    fn test_parse_github_shorthand() {
        let result = parse_source("owner/repo").unwrap();
        assert!(matches!(result.source, SkillSource::GitHub { .. }));
    }

    #[test]
    fn test_parse_local_absolute() {
        let result = parse_source("/path/to/skill.md").unwrap();
        assert_eq!(
            result.source,
            SkillSource::Local {
                path: PathBuf::from("/path/to/skill.md")
            }
        );
        assert_eq!(result.suggested_name, "skill");
    }

    #[test]
    fn test_parse_local_relative() {
        let result = parse_source("./my-skill.md").unwrap();
        assert!(matches!(result.source, SkillSource::Local { .. }));
        assert_eq!(result.suggested_name, "my-skill");
    }

    #[test]
    fn test_parse_url() {
        let result = parse_source("https://example.com/skills/typescript.md").unwrap();
        assert!(matches!(result.source, SkillSource::Url { .. }));
        assert_eq!(result.suggested_name, "typescript");
    }

    #[test]
    fn test_parse_empty() {
        assert!(matches!(
            parse_source(""),
            Err(SourceParseError::Empty)
        ));
    }

    #[test]
    fn test_parse_invalid_github() {
        assert!(matches!(
            parse_source("github:invalid"),
            Err(SourceParseError::InvalidGitHub(_))
        ));
    }

    #[test]
    fn test_source_display() {
        let github = SkillSource::github_path("owner", "repo", "path");
        assert_eq!(github.display_string(), "github:owner/repo/path");

        let local = SkillSource::local("/tmp/skill.md");
        assert_eq!(local.display_string(), "local:/tmp/skill.md");
    }
}
