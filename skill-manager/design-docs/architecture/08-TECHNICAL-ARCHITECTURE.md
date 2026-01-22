# Claude Skill Manager - Technical Architecture

## Document Info
- **Version**: 1.0
- **Status**: Draft
- **Last Updated**: 2026-01-22

---

## 1. System Overview

### 1.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Claude Skill Manager                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Presentation Layer                           │   │
│  │  ┌─────────────────┐                    ┌─────────────────┐         │   │
│  │  │    CLI Module    │                    │   TUI Module    │         │   │
│  │  │    (clap v4)     │                    │   (ratatui)     │         │   │
│  │  └────────┬─────────┘                    └────────┬────────┘         │   │
│  └───────────┼──────────────────────────────────────┼──────────────────┘   │
│              │                                       │                      │
│              └───────────────┬───────────────────────┘                      │
│                              │                                              │
│  ┌───────────────────────────▼─────────────────────────────────────────┐   │
│  │                         Application Layer                            │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │  Skill   │  │  Update  │  │ Conflict │  │  Merge   │            │   │
│  │  │ Service  │  │ Service  │  │ Service  │  │ Service  │            │   │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘            │   │
│  └───────┼─────────────┼─────────────┼─────────────┼───────────────────┘   │
│          │             │             │             │                        │
│  ┌───────▼─────────────▼─────────────▼─────────────▼───────────────────┐   │
│  │                          Domain Layer                                │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │  Skill   │  │  Source  │  │  Config  │  │  Event   │            │   │
│  │  │  Model   │  │  Model   │  │  Model   │  │  Model   │            │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                       Infrastructure Layer                           │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │ Registry │  │ Storage  │  │  GitHub  │  │  Cache   │            │   │
│  │  │ (SQLite) │  │ (Files)  │  │  Client  │  │ Manager  │            │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
              ┌─────────────────────────────────────────┐
              │           External Systems              │
              │  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
              │  │ GitHub  │  │  File   │  │ Claude  │ │
              │  │   API   │  │ System  │  │  Code   │ │
              │  └─────────┘  └─────────┘  └─────────┘ │
              └─────────────────────────────────────────┘
```

### 1.2 Technology Stack

| Component | Technology | Version | Rationale |
|-----------|------------|---------|-----------|
| Language | Rust | 2021 ed. | Performance, safety, single binary |
| CLI Framework | clap | 4.x | Best-in-class CLI parsing |
| TUI Framework | ratatui | 0.26+ | Modern, well-maintained TUI |
| Terminal Backend | crossterm | 0.27+ | Cross-platform terminal |
| Database | SQLite | 3.x | Embedded, zero-config |
| SQLite Bindings | rusqlite | 0.31+ | Best Rust SQLite bindings |
| Git Operations | git2 | 0.18+ | libgit2 bindings |
| HTTP Client | reqwest | 0.12+ | Async HTTP with rustls |
| Async Runtime | tokio | 1.x | Industry standard async |
| Serialization | serde | 1.x | De facto standard |
| Config Format | TOML | - | Human-readable config |
| Logging | tracing | 0.1+ | Structured logging |

---

## 2. Module Architecture

### 2.1 Crate Structure

```
claude-skill-manager/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   │
│   ├── cli/                 # CLI module
│   │   ├── mod.rs
│   │   ├── commands/        # Individual commands
│   │   │   ├── mod.rs
│   │   │   ├── init.rs
│   │   │   ├── add.rs
│   │   │   ├── remove.rs
│   │   │   ├── list.rs
│   │   │   ├── update.rs
│   │   │   ├── conflicts.rs
│   │   │   └── ...
│   │   └── output.rs        # Output formatting
│   │
│   ├── tui/                 # TUI module
│   │   ├── mod.rs
│   │   ├── app.rs           # Application state
│   │   ├── event.rs         # Event handling
│   │   ├── screens/         # Screen components
│   │   │   ├── mod.rs
│   │   │   ├── dashboard.rs
│   │   │   ├── skills.rs
│   │   │   ├── updates.rs
│   │   │   ├── conflicts.rs
│   │   │   └── settings.rs
│   │   ├── widgets/         # Reusable widgets
│   │   │   ├── mod.rs
│   │   │   ├── skill_list.rs
│   │   │   ├── skill_detail.rs
│   │   │   └── ...
│   │   └── theme.rs         # Theming
│   │
│   ├── services/            # Application services
│   │   ├── mod.rs
│   │   ├── skill_service.rs
│   │   ├── update_service.rs
│   │   ├── conflict_service.rs
│   │   └── merge_service.rs
│   │
│   ├── domain/              # Domain models
│   │   ├── mod.rs
│   │   ├── skill.rs
│   │   ├── source.rs
│   │   ├── conflict.rs
│   │   └── events.rs
│   │
│   ├── infra/               # Infrastructure
│   │   ├── mod.rs
│   │   ├── registry.rs      # SQLite registry
│   │   ├── storage.rs       # File storage
│   │   ├── github.rs        # GitHub client
│   │   ├── cache.rs         # Cache manager
│   │   └── config.rs        # Configuration
│   │
│   └── utils/               # Utilities
│       ├── mod.rs
│       ├── hash.rs
│       ├── path.rs
│       └── error.rs
│
├── tests/
│   ├── unit/
│   ├── integration/
│   └── e2e/
│
└── benches/
    └── benchmarks.rs
```

### 2.2 Module Dependencies

```
┌──────────────────────────────────────────────────────────────┐
│                        main.rs                               │
│                           │                                  │
│              ┌────────────┴────────────┐                    │
│              ▼                         ▼                    │
│         ┌────────┐               ┌────────┐                 │
│         │  cli   │               │  tui   │                 │
│         └───┬────┘               └───┬────┘                 │
│             │                        │                      │
│             └──────────┬─────────────┘                      │
│                        ▼                                    │
│                  ┌──────────┐                               │
│                  │ services │                               │
│                  └────┬─────┘                               │
│                       │                                     │
│         ┌─────────────┼─────────────┐                      │
│         ▼             ▼             ▼                      │
│    ┌────────┐    ┌────────┐    ┌────────┐                  │
│    │ domain │◄───│ infra  │    │ utils  │                  │
│    └────────┘    └────────┘    └───┬────┘                  │
│                       │            │                        │
│                       └────────────┘                        │
│                                                             │
│  Legend:                                                    │
│  ─────► depends on                                         │
│  ◄───── implements traits from                             │
└──────────────────────────────────────────────────────────────┘
```

---

## 3. Data Models

### 3.1 Core Domain Models

```rust
// src/domain/skill.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Represents a Claude skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Unique identifier
    pub id: Uuid,

    /// Human-readable name
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Where the skill came from
    pub source: SkillSource,

    /// Scope of the skill
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

/// Source of a skill
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    },

    /// Created inline (no external source)
    Inline,
}

/// Scope of a skill
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillScope {
    /// Applies to all projects
    Global,

    /// Applies to a specific project
    Project {
        path: PathBuf,
    },
}

/// How the skill should be updated
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum UpdateMode {
    /// Automatically update when changes detected
    #[default]
    Auto,

    /// Notify of updates but don't auto-apply
    Notify,

    /// Only update on explicit request
    Manual,
}

// src/domain/conflict.rs

/// Represents a conflict between skills
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Suggested resolution
    pub suggestion: Option<String>,

    /// Resolution status
    pub status: ConflictStatus,
}

/// Types of conflicts that can occur
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ConflictType {
    /// Same instruction appears in both skills
    Duplicate,

    /// Instructions contradict each other
    Contradictory,

    /// Overlapping scope with unclear precedence
    Overlap,

    /// Syntax/structure conflict
    Structural,
}

/// Resolution status of a conflict
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum ConflictStatus {
    /// Conflict not yet addressed
    #[default]
    Unresolved,

    /// Conflict has been resolved
    Resolved,

    /// User chose to ignore this conflict
    Ignored,
}
```

### 3.2 Database Schema

```sql
-- migrations/001_initial.sql

-- Skills table
CREATE TABLE skills (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    source_type TEXT NOT NULL,
    source_data TEXT NOT NULL,  -- JSON
    scope_type TEXT NOT NULL,
    scope_data TEXT,  -- JSON (null for global)
    enabled INTEGER NOT NULL DEFAULT 1,
    content_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    tags TEXT,  -- JSON array
    priority INTEGER NOT NULL DEFAULT 50,
    update_mode TEXT NOT NULL DEFAULT 'auto'
);

CREATE INDEX idx_skills_name ON skills(name);
CREATE INDEX idx_skills_scope ON skills(scope_type);
CREATE INDEX idx_skills_enabled ON skills(enabled);

-- Full-text search for skills
CREATE VIRTUAL TABLE skills_fts USING fts5(
    name,
    description,
    tags,
    content='skills',
    content_rowid='rowid'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER skills_ai AFTER INSERT ON skills BEGIN
    INSERT INTO skills_fts(rowid, name, description, tags)
    VALUES (new.rowid, new.name, new.description, new.tags);
END;

CREATE TRIGGER skills_ad AFTER DELETE ON skills BEGIN
    INSERT INTO skills_fts(skills_fts, rowid, name, description, tags)
    VALUES ('delete', old.rowid, old.name, old.description, old.tags);
END;

CREATE TRIGGER skills_au AFTER UPDATE ON skills BEGIN
    INSERT INTO skills_fts(skills_fts, rowid, name, description, tags)
    VALUES ('delete', old.rowid, old.name, old.description, old.tags);
    INSERT INTO skills_fts(rowid, name, description, tags)
    VALUES (new.rowid, new.name, new.description, new.tags);
END;

-- Conflicts table
CREATE TABLE conflicts (
    id TEXT PRIMARY KEY,
    skill_a_id TEXT NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    skill_b_id TEXT NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    conflict_type TEXT NOT NULL,
    description TEXT NOT NULL,
    line_a INTEGER,
    line_b INTEGER,
    suggestion TEXT,
    status TEXT NOT NULL DEFAULT 'unresolved',
    created_at TEXT NOT NULL,
    resolved_at TEXT
);

CREATE INDEX idx_conflicts_status ON conflicts(status);
CREATE INDEX idx_conflicts_skills ON conflicts(skill_a_id, skill_b_id);

-- Update history table
CREATE TABLE update_history (
    id TEXT PRIMARY KEY,
    skill_id TEXT NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    old_hash TEXT NOT NULL,
    new_hash TEXT NOT NULL,
    old_source_data TEXT,
    new_source_data TEXT,
    updated_at TEXT NOT NULL,
    success INTEGER NOT NULL,
    error_message TEXT
);

CREATE INDEX idx_update_history_skill ON update_history(skill_id);
CREATE INDEX idx_update_history_date ON update_history(updated_at);

-- Settings table
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

---

## 4. Key Algorithms

### 4.1 Skill Merging Algorithm

```rust
// src/services/merge_service.rs

use crate::domain::Skill;

pub struct MergeService;

impl MergeService {
    /// Merge multiple skills into a single CLAUDE.md content
    pub fn merge(skills: &[Skill], contents: &[(Uuid, String)]) -> String {
        // 1. Sort skills by priority (descending)
        let mut sorted: Vec<_> = skills.iter().collect();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));

        // 2. Parse each skill into sections
        let mut sections: Vec<MergedSection> = Vec::new();

        for skill in sorted {
            if !skill.enabled {
                continue;
            }

            let content = contents
                .iter()
                .find(|(id, _)| *id == skill.id)
                .map(|(_, c)| c.as_str())
                .unwrap_or("");

            let skill_sections = parse_sections(content);

            for section in skill_sections {
                // 3. Merge or append sections
                if let Some(existing) = sections.iter_mut().find(|s| s.heading == section.heading) {
                    // Merge into existing section
                    existing.content.push('\n');
                    existing.content.push_str(&section.content);
                    existing.sources.push(skill.name.clone());
                } else {
                    // New section
                    sections.push(MergedSection {
                        heading: section.heading,
                        content: section.content,
                        sources: vec![skill.name.clone()],
                    });
                }
            }
        }

        // 4. Build final output
        let mut output = String::new();
        output.push_str("# Claude Instructions\n\n");
        output.push_str("<!-- Generated by Claude Skill Manager -->\n");
        output.push_str("<!-- Do not edit directly - changes will be overwritten -->\n\n");

        for section in sections {
            if !section.heading.is_empty() {
                output.push_str(&section.heading);
                output.push_str("\n\n");
            }
            output.push_str(&section.content);
            output.push_str("\n\n");
        }

        output.trim().to_string()
    }
}

struct MergedSection {
    heading: String,
    content: String,
    sources: Vec<String>,
}

fn parse_sections(content: &str) -> Vec<ParsedSection> {
    let mut sections = Vec::new();
    let mut current_heading = String::new();
    let mut current_content = String::new();

    for line in content.lines() {
        if line.starts_with('#') {
            // Save previous section
            if !current_content.is_empty() {
                sections.push(ParsedSection {
                    heading: current_heading.clone(),
                    content: current_content.trim().to_string(),
                });
            }
            current_heading = line.to_string();
            current_content = String::new();
        } else {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Don't forget the last section
    if !current_content.is_empty() {
        sections.push(ParsedSection {
            heading: current_heading,
            content: current_content.trim().to_string(),
        });
    }

    sections
}

struct ParsedSection {
    heading: String,
    content: String,
}
```

### 4.2 Conflict Detection Algorithm

```rust
// src/services/conflict_service.rs

use crate::domain::{Conflict, ConflictType, Skill};
use std::collections::HashSet;

pub struct ConflictService;

impl ConflictService {
    /// Detect conflicts between a set of skills
    pub fn detect_conflicts(
        skills: &[Skill],
        contents: &[(Uuid, String)],
    ) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        // Compare each pair of skills
        for i in 0..skills.len() {
            for j in (i + 1)..skills.len() {
                let skill_a = &skills[i];
                let skill_b = &skills[j];

                // Only check enabled skills
                if !skill_a.enabled || !skill_b.enabled {
                    continue;
                }

                let content_a = contents
                    .iter()
                    .find(|(id, _)| *id == skill_a.id)
                    .map(|(_, c)| c.as_str())
                    .unwrap_or("");

                let content_b = contents
                    .iter()
                    .find(|(id, _)| *id == skill_b.id)
                    .map(|(_, c)| c.as_str())
                    .unwrap_or("");

                // Check for various conflict types
                conflicts.extend(Self::find_duplicates(skill_a, skill_b, content_a, content_b));
                conflicts.extend(Self::find_contradictions(skill_a, skill_b, content_a, content_b));
            }
        }

        conflicts
    }

    fn find_duplicates(
        skill_a: &Skill,
        skill_b: &Skill,
        content_a: &str,
        content_b: &str,
    ) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        // Extract instructions (lines starting with -)
        let instructions_a: Vec<(usize, &str)> = content_a
            .lines()
            .enumerate()
            .filter(|(_, line)| line.trim().starts_with('-'))
            .map(|(i, line)| (i + 1, line.trim()))
            .collect();

        let instructions_b: Vec<(usize, &str)> = content_b
            .lines()
            .enumerate()
            .filter(|(_, line)| line.trim().starts_with('-'))
            .map(|(i, line)| (i + 1, line.trim()))
            .collect();

        // Find exact duplicates
        for (line_a, inst_a) in &instructions_a {
            for (line_b, inst_b) in &instructions_b {
                if Self::normalize_instruction(inst_a) == Self::normalize_instruction(inst_b) {
                    conflicts.push(Conflict {
                        id: Uuid::new_v4(),
                        skill_a_id: skill_a.id,
                        skill_b_id: skill_b.id,
                        conflict_type: ConflictType::Duplicate,
                        description: format!("Duplicate instruction found"),
                        line_a: Some(*line_a),
                        line_b: Some(*line_b),
                        suggestion: Some("Remove from one skill or merge".to_string()),
                        status: ConflictStatus::Unresolved,
                    });
                }
            }
        }

        conflicts
    }

    fn find_contradictions(
        skill_a: &Skill,
        skill_b: &Skill,
        content_a: &str,
        content_b: &str,
    ) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        // Keywords that often indicate contradictions
        let contradiction_pairs = [
            ("always", "never"),
            ("must", "must not"),
            ("required", "optional"),
            ("enable", "disable"),
            ("use", "avoid"),
            ("prefer", "avoid"),
        ];

        let instructions_a: Vec<(usize, &str)> = content_a
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|(i, line)| (i + 1, line.trim().to_lowercase()))
            .map(|(i, line)| (i, line))
            .collect();

        let instructions_b: Vec<(usize, &str)> = content_b
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|(i, line)| (i + 1, line.trim().to_lowercase()))
            .collect();

        // This is a simplified heuristic - real implementation would use NLP
        for (line_a, inst_a) in &instructions_a {
            for (line_b, inst_b) in &instructions_b {
                for (word_a, word_b) in &contradiction_pairs {
                    if inst_a.contains(word_a) && inst_b.contains(word_b) {
                        // Check if they're about the same topic
                        if Self::same_topic(inst_a, inst_b) {
                            conflicts.push(Conflict {
                                id: Uuid::new_v4(),
                                skill_a_id: skill_a.id,
                                skill_b_id: skill_b.id,
                                conflict_type: ConflictType::Contradictory,
                                description: format!(
                                    "Contradictory instructions: '{}' vs '{}'",
                                    word_a, word_b
                                ),
                                line_a: Some(*line_a),
                                line_b: Some(*line_b),
                                suggestion: Some("Disable one skill or edit to resolve".to_string()),
                                status: ConflictStatus::Unresolved,
                            });
                        }
                    }
                }
            }
        }

        conflicts
    }

    fn normalize_instruction(inst: &str) -> String {
        inst.to_lowercase()
            .trim_start_matches('-')
            .trim()
            .to_string()
    }

    fn same_topic(a: &str, b: &str) -> bool {
        // Extract significant words and check overlap
        let words_a: HashSet<_> = a
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();
        let words_b: HashSet<_> = b
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();

        let intersection: HashSet<_> = words_a.intersection(&words_b).collect();

        // If more than 30% words overlap, consider same topic
        let min_len = words_a.len().min(words_b.len());
        if min_len == 0 {
            return false;
        }

        (intersection.len() as f64 / min_len as f64) > 0.3
    }
}
```

### 4.3 GitHub Sync Algorithm

```rust
// src/infra/github.rs

use reqwest::Client;
use serde::Deserialize;

pub struct GitHubClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.github.com".to_string(),
            token,
        }
    }

    /// Fetch a skill file from GitHub
    pub async fn fetch_skill(
        &self,
        owner: &str,
        repo: &str,
        path: Option<&str>,
        ref_spec: Option<&str>,
    ) -> Result<FetchResult, GitHubError> {
        let file_path = path.unwrap_or("CLAUDE.md");
        let ref_param = ref_spec.unwrap_or("HEAD");

        // First, get the file metadata
        let url = format!(
            "{}/repos/{}/{}/contents/{}?ref={}",
            self.base_url, owner, repo, file_path, ref_param
        );

        let mut request = self.client.get(&url);
        request = request.header("User-Agent", "claude-skill-manager");
        request = request.header("Accept", "application/vnd.github.v3+json");

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(GitHubError::NotFound(format!(
                "{}/{}/{}",
                owner, repo, file_path
            )));
        }

        let file_info: GitHubFile = response.json().await?;

        // Decode content
        let content = base64::decode(&file_info.content.replace('\n', ""))?;
        let content_str = String::from_utf8(content)?;

        // Get current commit SHA for tracking
        let commit_sha = self.get_commit_sha(owner, repo, ref_param).await?;

        Ok(FetchResult {
            content: content_str,
            sha: file_info.sha,
            commit_sha,
            etag: None,
        })
    }

    /// Check if a skill has updates available
    pub async fn check_for_updates(
        &self,
        owner: &str,
        repo: &str,
        current_commit: &str,
        ref_spec: Option<&str>,
    ) -> Result<Option<UpdateInfo>, GitHubError> {
        let ref_param = ref_spec.unwrap_or("HEAD");

        let latest_sha = self.get_commit_sha(owner, repo, ref_param).await?;

        if latest_sha == current_commit {
            return Ok(None);
        }

        // Get commits between current and latest
        let compare_url = format!(
            "{}/repos/{}/{}/compare/{}...{}",
            self.base_url, owner, repo, current_commit, latest_sha
        );

        let mut request = self.client.get(&compare_url);
        request = request.header("User-Agent", "claude-skill-manager");

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        let comparison: CompareResult = response.json().await?;

        Ok(Some(UpdateInfo {
            current_sha: current_commit.to_string(),
            latest_sha,
            commits_behind: comparison.ahead_by,
            commit_messages: comparison
                .commits
                .iter()
                .map(|c| c.commit.message.clone())
                .collect(),
        }))
    }

    async fn get_commit_sha(
        &self,
        owner: &str,
        repo: &str,
        ref_spec: &str,
    ) -> Result<String, GitHubError> {
        let url = format!(
            "{}/repos/{}/{}/commits/{}",
            self.base_url, owner, repo, ref_spec
        );

        let mut request = self.client.get(&url);
        request = request.header("User-Agent", "claude-skill-manager");

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        let commit: GitHubCommit = response.json().await?;

        Ok(commit.sha)
    }
}

#[derive(Debug, Deserialize)]
struct GitHubFile {
    sha: String,
    content: String,
    encoding: String,
}

#[derive(Debug, Deserialize)]
struct GitHubCommit {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct CompareResult {
    ahead_by: usize,
    commits: Vec<CommitInfo>,
}

#[derive(Debug, Deserialize)]
struct CommitInfo {
    commit: CommitDetail,
}

#[derive(Debug, Deserialize)]
struct CommitDetail {
    message: String,
}

pub struct FetchResult {
    pub content: String,
    pub sha: String,
    pub commit_sha: String,
    pub etag: Option<String>,
}

pub struct UpdateInfo {
    pub current_sha: String,
    pub latest_sha: String,
    pub commits_behind: usize,
    pub commit_messages: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum GitHubError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Rate limit exceeded")]
    RateLimited,
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Decode error: {0}")]
    Decode(String),
}
```

---

## 5. Error Handling

### 5.1 Error Types

```rust
// src/utils/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CsmError {
    // Registry errors
    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Skill already exists: {0}")]
    SkillExists(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    // Source errors
    #[error("Invalid source format: {0}")]
    InvalidSource(String),

    #[error("GitHub error: {0}")]
    GitHub(#[from] crate::infra::github::GitHubError),

    #[error("Source fetch failed: {0}")]
    FetchFailed(String),

    // File system errors
    #[error("File not found: {0}")]
    FileNotFound(std::path::PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(std::path::PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // Validation errors
    #[error("Invalid skill content: {0}")]
    InvalidContent(String),

    #[error("Skill validation failed: {0}")]
    ValidationFailed(String),

    // Conflict errors
    #[error("Unresolved conflicts exist")]
    UnresolvedConflicts,

    // Config errors
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Not initialized: run 'csm init' first")]
    NotInitialized,

    // Network errors
    #[error("Network error: {0}")]
    Network(String),

    #[error("Timeout")]
    Timeout,
}

impl CsmError {
    /// Get exit code for CLI
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::SkillNotFound(_) | Self::FileNotFound(_) => 1,
            Self::InvalidSource(_) | Self::InvalidContent(_) | Self::ValidationFailed(_) => 2,
            Self::Config(_) | Self::NotInitialized => 3,
            Self::Network(_) | Self::Timeout | Self::GitHub(_) | Self::FetchFailed(_) => 4,
            Self::UnresolvedConflicts => 5,
            _ => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, CsmError>;
```

---

## 6. Configuration

### 6.1 Configuration Structure

```rust
// src/infra/config.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub updates: UpdateConfig,

    #[serde(default)]
    pub github: GitHubConfig,

    #[serde(default)]
    pub ui: UiConfig,

    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_scope")]
    pub default_scope: String,

    #[serde(default)]
    pub editor: Option<String>,

    #[serde(default = "default_true")]
    pub color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    #[serde(default)]
    pub mode: UpdateMode,

    #[serde(default = "default_schedule")]
    pub schedule: String,

    #[serde(default = "default_true")]
    pub check_on_startup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    #[serde(default = "default_ref")]
    pub default_ref: String,

    // Token stored in system keychain, not in config
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_theme")]
    pub theme: String,

    #[serde(default = "default_true")]
    pub show_welcome: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,

    #[serde(default)]
    pub file: Option<PathBuf>,

    #[serde(default = "default_max_size")]
    pub max_size: String,
}

fn default_scope() -> String { "local".to_string() }
fn default_true() -> bool { true }
fn default_schedule() -> String { "daily".to_string() }
fn default_ref() -> String { "main".to_string() }
fn default_theme() -> String { "dark".to_string() }
fn default_log_level() -> String { "info".to_string() }
fn default_max_size() -> String { "10MB".to_string() }

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            updates: UpdateConfig::default(),
            github: GitHubConfig::default(),
            ui: UiConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}
```

---

## 7. Deployment

### 7.1 Build Configuration

```toml
# Cargo.toml

[package]
name = "claude-skill-manager"
version = "1.0.0"
edition = "2021"
authors = ["Anthropic"]
description = "CLI and TUI for managing Claude AI skills"
license = "MIT"
repository = "https://github.com/anthropics/claude-skill-manager"
keywords = ["claude", "ai", "skills", "cli", "tui"]
categories = ["command-line-utilities", "development-tools"]

[[bin]]
name = "csm"
path = "src/main.rs"

[dependencies]
# CLI
clap = { version = "4", features = ["derive", "env"] }

# TUI
ratatui = "0.26"
crossterm = "0.27"

# Async
tokio = { version = "1", features = ["full"] }

# Database
rusqlite = { version = "0.31", features = ["bundled", "blob"] }

# Git
git2 = "0.18"

# HTTP
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
directories = "5"
sha2 = "0.10"
base64 = "0.22"

[dev-dependencies]
tempfile = "3"
assert_cmd = "2"
predicates = "3"
wiremock = "0.6"
pretty_assertions = "1"
insta = "1"
criterion = "0.5"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.release-debug]
inherits = "release"
debug = true
strip = false
```

### 7.2 Release Process

```yaml
# .github/workflows/release.yml

name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            name: csm-linux-x86_64
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
            name: csm-linux-aarch64
          - target: x86_64-apple-darwin
            os: macos-latest
            name: csm-macos-x86_64
          - target: aarch64-apple-darwin
            os: macos-latest
            name: csm-macos-aarch64
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            name: csm-windows-x86_64.exe

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - uses: Swatinem/rust-cache@v2

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Package (Unix)
        if: matrix.os != 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          tar czvf ../../../${{ matrix.name }}.tar.gz csm

      - name: Package (Windows)
        if: matrix.os == 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          7z a ../../../${{ matrix.name }}.zip csm.exe

      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.name }}
          path: |
            *.tar.gz
            *.zip

  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            */*.tar.gz
            */*.zip
          generate_release_notes: true
```

---

## 8. Security Considerations

### 8.1 Threat Model

| Threat | Mitigation |
|--------|------------|
| Malicious skill content | Validate content, no code execution |
| Credential exposure | Use system keychain, never log tokens |
| MITM attacks | HTTPS only, certificate validation |
| Path traversal | Canonicalize all paths, sandbox operations |
| SQL injection | Parameterized queries only |
| Symlink attacks | Validate symlink targets |

### 8.2 Secure Defaults

- All network requests use HTTPS
- GitHub tokens stored in system keychain (keyring crate)
- Permissions: files 0600, directories 0700
- No execution of skill content
- Audit log of all operations
