//! Conflict domain models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a conflict between two skills
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conflict {
    /// Unique identifier
    pub id: Uuid,

    /// First skill involved
    pub skill_a_id: Uuid,

    /// Second skill involved
    pub skill_b_id: Uuid,

    /// Type of conflict
    pub conflict_type: ConflictType,

    /// Human-readable description of the conflict
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

    /// When the conflict was resolved (if resolved)
    pub resolved_at: Option<DateTime<Utc>>,
}

impl Conflict {
    /// Create a new conflict
    pub fn new(
        skill_a_id: Uuid,
        skill_b_id: Uuid,
        conflict_type: ConflictType,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            skill_a_id,
            skill_b_id,
            conflict_type,
            description: description.into(),
            line_a: None,
            line_b: None,
            content_a: None,
            content_b: None,
            suggestion: None,
            status: ConflictStatus::default(),
            detected_at: Utc::now(),
            resolved_at: None,
        }
    }

    /// Create a builder for constructing conflicts
    pub fn builder(
        skill_a_id: Uuid,
        skill_b_id: Uuid,
        conflict_type: ConflictType,
    ) -> ConflictBuilder {
        ConflictBuilder::new(skill_a_id, skill_b_id, conflict_type)
    }

    /// Check if this conflict is resolved
    pub fn is_resolved(&self) -> bool {
        matches!(
            self.status,
            ConflictStatus::Resolved | ConflictStatus::Ignored
        )
    }

    /// Mark this conflict as resolved
    pub fn resolve(&mut self) {
        self.status = ConflictStatus::Resolved;
        self.resolved_at = Some(Utc::now());
    }

    /// Mark this conflict as ignored
    pub fn ignore(&mut self) {
        self.status = ConflictStatus::Ignored;
        self.resolved_at = Some(Utc::now());
    }
}

/// Builder for constructing Conflict instances
#[derive(Debug)]
pub struct ConflictBuilder {
    skill_a_id: Uuid,
    skill_b_id: Uuid,
    conflict_type: ConflictType,
    description: String,
    line_a: Option<usize>,
    line_b: Option<usize>,
    content_a: Option<String>,
    content_b: Option<String>,
    suggestion: Option<String>,
}

impl ConflictBuilder {
    pub fn new(skill_a_id: Uuid, skill_b_id: Uuid, conflict_type: ConflictType) -> Self {
        Self {
            skill_a_id,
            skill_b_id,
            conflict_type,
            description: String::new(),
            line_a: None,
            line_b: None,
            content_a: None,
            content_b: None,
            suggestion: None,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn lines(mut self, line_a: usize, line_b: usize) -> Self {
        self.line_a = Some(line_a);
        self.line_b = Some(line_b);
        self
    }

    pub fn content(mut self, content_a: impl Into<String>, content_b: impl Into<String>) -> Self {
        self.content_a = Some(content_a.into());
        self.content_b = Some(content_b.into());
        self
    }

    pub fn suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    pub fn build(self) -> Conflict {
        Conflict {
            id: Uuid::new_v4(),
            skill_a_id: self.skill_a_id,
            skill_b_id: self.skill_b_id,
            conflict_type: self.conflict_type,
            description: self.description,
            line_a: self.line_a,
            line_b: self.line_b,
            content_a: self.content_a,
            content_b: self.content_b,
            suggestion: self.suggestion,
            status: ConflictStatus::default(),
            detected_at: Utc::now(),
            resolved_at: None,
        }
    }
}

/// Types of conflicts that can occur between skills
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ConflictType {
    /// Same instruction appears in both skills
    Duplicate,

    /// Instructions contradict each other
    Contradictory,

    /// Overlapping scope with unclear precedence
    Overlap,

    /// Incompatible section structures or formats
    Structural,
}

impl ConflictType {
    /// Get a human-readable label for this conflict type
    pub fn label(&self) -> &'static str {
        match self {
            Self::Duplicate => "Duplicate",
            Self::Contradictory => "Contradictory",
            Self::Overlap => "Overlap",
            Self::Structural => "Structural",
        }
    }

    /// Get a description of this conflict type
    pub fn description(&self) -> &'static str {
        match self {
            Self::Duplicate => "Same instruction appears in multiple skills",
            Self::Contradictory => "Instructions contradict each other",
            Self::Overlap => "Skills have overlapping scope with unclear precedence",
            Self::Structural => "Incompatible section structures or formats",
        }
    }
}

impl std::fmt::Display for ConflictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Resolution status of a conflict
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ConflictStatus {
    /// Conflict not yet addressed
    #[default]
    Unresolved,

    /// Conflict has been resolved
    Resolved,

    /// User chose to ignore this conflict
    Ignored,
}

impl std::fmt::Display for ConflictStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unresolved => write!(f, "unresolved"),
            Self::Resolved => write!(f, "resolved"),
            Self::Ignored => write!(f, "ignored"),
        }
    }
}

/// Resolution strategy for a conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionStrategy {
    /// Disable skill A, keep skill B
    DisableSkillA,

    /// Disable skill B, keep skill A
    DisableSkillB,

    /// Set priority so A takes precedence
    PrioritizeA,

    /// Set priority so B takes precedence
    PrioritizeB,

    /// Ignore the conflict (accept undefined behavior)
    Ignore,
}

impl std::fmt::Display for ResolutionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DisableSkillA => write!(f, "Disable first skill"),
            Self::DisableSkillB => write!(f, "Disable second skill"),
            Self::PrioritizeA => write!(f, "Prioritize first skill"),
            Self::PrioritizeB => write!(f, "Prioritize second skill"),
            Self::Ignore => write!(f, "Ignore conflict"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_new() {
        let skill_a = Uuid::new_v4();
        let skill_b = Uuid::new_v4();

        let conflict = Conflict::new(
            skill_a,
            skill_b,
            ConflictType::Duplicate,
            "Test conflict",
        );

        assert_eq!(conflict.skill_a_id, skill_a);
        assert_eq!(conflict.skill_b_id, skill_b);
        assert_eq!(conflict.conflict_type, ConflictType::Duplicate);
        assert!(!conflict.is_resolved());
    }

    #[test]
    fn test_conflict_builder() {
        let skill_a = Uuid::new_v4();
        let skill_b = Uuid::new_v4();

        let conflict = Conflict::builder(skill_a, skill_b, ConflictType::Contradictory)
            .description("Use tabs vs use spaces")
            .lines(10, 20)
            .content("Use tabs", "Use spaces")
            .suggestion("Choose one style")
            .build();

        assert_eq!(conflict.line_a, Some(10));
        assert_eq!(conflict.line_b, Some(20));
        assert_eq!(conflict.content_a, Some("Use tabs".to_string()));
        assert!(conflict.suggestion.is_some());
    }

    #[test]
    fn test_conflict_resolve() {
        let mut conflict = Conflict::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            ConflictType::Duplicate,
            "Test",
        );

        assert!(!conflict.is_resolved());

        conflict.resolve();

        assert!(conflict.is_resolved());
        assert_eq!(conflict.status, ConflictStatus::Resolved);
        assert!(conflict.resolved_at.is_some());
    }

    #[test]
    fn test_conflict_ignore() {
        let mut conflict = Conflict::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            ConflictType::Overlap,
            "Test",
        );

        conflict.ignore();

        assert!(conflict.is_resolved());
        assert_eq!(conflict.status, ConflictStatus::Ignored);
    }
}
