# Claude Skill Manager - Testing Strategy

## Document Info
- **Version**: 1.0
- **Status**: Draft
- **Last Updated**: 2026-01-22

---

## 1. Testing Philosophy

### 1.1 Core Principles

1. **Test Pyramid**: More unit tests, fewer E2E tests
2. **Fast Feedback**: Tests should run quickly (<30s for unit suite)
3. **Deterministic**: No flaky tests allowed
4. **Comprehensive**: High coverage of critical paths
5. **Maintainable**: Tests should be easy to understand and update

### 1.2 Coverage Targets

| Test Type | Coverage Target | Rationale |
|-----------|-----------------|-----------|
| Unit | 80%+ | Core business logic |
| Integration | 60%+ | Module interactions |
| E2E/Simulation | Critical paths | User workflows |

---

## 2. Test Categories

### 2.1 Unit Tests

**Scope**: Individual functions, methods, and modules in isolation.

**Location**: `tests/unit/` and inline `#[cfg(test)]` modules

**Framework**: Rust built-in test framework + `pretty_assertions`

**Characteristics**:
- No I/O (filesystem, network)
- No external dependencies
- Fast (<10ms per test)
- Deterministic

#### 2.1.1 Unit Test Categories

| Category | Module | Test Examples |
|----------|--------|---------------|
| Parsing | `src/parser/` | Parse skill source strings, validate CLAUDE.md format |
| Registry | `src/registry/` | CRUD operations, queries, constraints |
| Merging | `src/merge/` | Skill combination, priority ordering |
| Conflict | `src/conflict/` | Conflict detection algorithms |
| Config | `src/config/` | Config parsing, defaults, validation |
| CLI Args | `src/cli/` | Argument parsing, validation |
| Utils | `src/utils/` | Helper functions, string manipulation |

#### 2.1.2 Unit Test Examples

```rust
// tests/unit/parser_test.rs

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_github_source_full() {
        let input = "github:anthropics/claude-skills/typescript@v1.0.0";
        let result = parse_skill_source(input).unwrap();

        assert_eq!(result, SkillSource::GitHub {
            owner: "anthropics".to_string(),
            repo: "claude-skills".to_string(),
            path: Some("typescript".to_string()),
            ref_spec: Some("v1.0.0".to_string()),
        });
    }

    #[test]
    fn parse_github_source_minimal() {
        let input = "github:user/repo";
        let result = parse_skill_source(input).unwrap();

        assert_eq!(result, SkillSource::GitHub {
            owner: "user".to_string(),
            repo: "repo".to_string(),
            path: None,
            ref_spec: None,
        });
    }

    #[test]
    fn parse_github_source_invalid() {
        let input = "github:invalid";
        let result = parse_skill_source(input);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid GitHub source: expected 'owner/repo' format"
        );
    }

    #[test]
    fn parse_local_source() {
        let input = "/path/to/skill.md";
        let result = parse_skill_source(input).unwrap();

        assert_eq!(result, SkillSource::Local {
            path: PathBuf::from("/path/to/skill.md"),
        });
    }
}

// tests/unit/conflict_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_duplicate_instruction() {
        let skill_a = Skill {
            id: Uuid::new_v4(),
            name: "skill-a".to_string(),
            content: "Use 2-space indentation".to_string(),
            ..Default::default()
        };

        let skill_b = Skill {
            id: Uuid::new_v4(),
            name: "skill-b".to_string(),
            content: "Use 2-space indentation".to_string(),
            ..Default::default()
        };

        let conflicts = detect_conflicts(&[skill_a, skill_b]);

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::Duplicate);
    }

    #[test]
    fn detect_contradictory_instruction() {
        let skill_a = Skill {
            name: "strict".to_string(),
            content: "Always use strict null checks".to_string(),
            ..Default::default()
        };

        let skill_b = Skill {
            name: "flexible".to_string(),
            content: "Never use strict null checks".to_string(),
            ..Default::default()
        };

        let conflicts = detect_conflicts(&[skill_a, skill_b]);

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::Contradictory);
    }

    #[test]
    fn no_conflict_different_topics() {
        let skill_a = Skill {
            name: "typescript".to_string(),
            content: "Use TypeScript strict mode".to_string(),
            ..Default::default()
        };

        let skill_b = Skill {
            name: "python".to_string(),
            content: "Use Python type hints".to_string(),
            ..Default::default()
        };

        let conflicts = detect_conflicts(&[skill_a, skill_b]);

        assert!(conflicts.is_empty());
    }
}

// tests/unit/merge_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_skills_by_priority() {
        let skills = vec![
            Skill {
                name: "low".to_string(),
                content: "# Low Priority\nContent A".to_string(),
                priority: 10,
                ..Default::default()
            },
            Skill {
                name: "high".to_string(),
                content: "# High Priority\nContent B".to_string(),
                priority: 100,
                ..Default::default()
            },
        ];

        let merged = merge_skills(&skills);

        // Higher priority should come first
        assert!(merged.starts_with("# High Priority"));
        assert!(merged.contains("# Low Priority"));
    }

    #[test]
    fn merge_preserves_sections() {
        let skills = vec![
            Skill {
                content: "## Code Style\n- Use tabs".to_string(),
                ..Default::default()
            },
            Skill {
                content: "## Testing\n- Write tests".to_string(),
                ..Default::default()
            },
        ];

        let merged = merge_skills(&skills);

        assert!(merged.contains("## Code Style"));
        assert!(merged.contains("## Testing"));
    }
}
```

---

### 2.2 Integration Tests

**Scope**: Multiple modules working together, including filesystem and database.

**Location**: `tests/integration/`

**Framework**: Rust test framework + `tempfile` + `assert_fs`

**Characteristics**:
- Real filesystem operations (temp directories)
- Real SQLite database (in-memory or temp file)
- Mocked network calls
- Medium speed (<1s per test)

#### 2.2.1 Integration Test Categories

| Category | Description | Dependencies |
|----------|-------------|--------------|
| Registry CRUD | Full database operations | SQLite |
| File Operations | Skill file management | Filesystem |
| Symlink Management | Create/update/remove symlinks | Filesystem |
| Config Loading | Read and apply configuration | Filesystem |
| Merge Pipeline | Full skill merging workflow | All |
| CLI Commands | Command execution with real args | All |

#### 2.2.2 Integration Test Examples

```rust
// tests/integration/registry_test.rs

use tempfile::tempdir;
use csm::registry::Registry;
use csm::models::{Skill, SkillSource, SkillScope};

#[test]
fn test_registry_crud_operations() {
    let temp = tempdir().unwrap();
    let db_path = temp.path().join("test.db");
    let registry = Registry::new(&db_path).unwrap();

    // Create
    let skill = Skill {
        id: Uuid::new_v4(),
        name: "test-skill".to_string(),
        description: Some("Test description".to_string()),
        source: SkillSource::Local { path: PathBuf::from("/test") },
        scope: SkillScope::Global,
        enabled: true,
        content_hash: "abc123".to_string(),
        ..Default::default()
    };

    registry.create(&skill).unwrap();

    // Read
    let retrieved = registry.get_by_name("test-skill").unwrap();
    assert_eq!(retrieved.name, "test-skill");
    assert_eq!(retrieved.description, Some("Test description".to_string()));

    // Update
    let mut updated = retrieved.clone();
    updated.enabled = false;
    registry.update(&updated).unwrap();

    let after_update = registry.get_by_name("test-skill").unwrap();
    assert_eq!(after_update.enabled, false);

    // Delete
    registry.delete(&skill.id).unwrap();

    let after_delete = registry.get_by_name("test-skill");
    assert!(after_delete.is_err());
}

#[test]
fn test_registry_queries() {
    let temp = tempdir().unwrap();
    let db_path = temp.path().join("test.db");
    let registry = Registry::new(&db_path).unwrap();

    // Create multiple skills
    let skills = vec![
        Skill {
            name: "global-1".to_string(),
            scope: SkillScope::Global,
            enabled: true,
            ..Default::default()
        },
        Skill {
            name: "global-2".to_string(),
            scope: SkillScope::Global,
            enabled: false,
            ..Default::default()
        },
        Skill {
            name: "local-1".to_string(),
            scope: SkillScope::Project { path: PathBuf::from("/project") },
            enabled: true,
            ..Default::default()
        },
    ];

    for skill in &skills {
        registry.create(skill).unwrap();
    }

    // Query by scope
    let global_skills = registry.list_by_scope(SkillScope::Global).unwrap();
    assert_eq!(global_skills.len(), 2);

    // Query enabled only
    let enabled_skills = registry.list_enabled().unwrap();
    assert_eq!(enabled_skills.len(), 2);

    // Search
    let search_results = registry.search("global").unwrap();
    assert_eq!(search_results.len(), 2);
}

// tests/integration/filesystem_test.rs

use tempfile::tempdir;
use csm::storage::SkillStorage;

#[test]
fn test_skill_storage_operations() {
    let temp = tempdir().unwrap();
    let storage = SkillStorage::new(temp.path()).unwrap();

    // Store skill
    let content = "# My Skill\n\nSome instructions here.";
    storage.store("my-skill", content).unwrap();

    // Verify file exists
    let skill_path = temp.path().join("skills/my-skill/CLAUDE.md");
    assert!(skill_path.exists());

    // Read back
    let retrieved = storage.read("my-skill").unwrap();
    assert_eq!(retrieved, content);

    // Delete
    storage.delete("my-skill").unwrap();
    assert!(!skill_path.exists());
}

#[test]
fn test_symlink_creation() {
    let temp = tempdir().unwrap();
    let storage = SkillStorage::new(temp.path()).unwrap();

    // Store skill
    storage.store("my-skill", "# Content").unwrap();

    // Create symlink
    let project_path = temp.path().join("project");
    std::fs::create_dir_all(&project_path).unwrap();

    storage.create_symlink("my-skill", &project_path).unwrap();

    // Verify symlink
    let symlink_path = project_path.join(".csm/skills/my-skill");
    assert!(symlink_path.is_symlink());

    // Verify symlink target
    let target = std::fs::read_link(&symlink_path).unwrap();
    assert!(target.ends_with("skills/my-skill"));
}

// tests/integration/cli_test.rs

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_cli_init() {
    let temp = tempdir().unwrap();

    Command::cargo_bin("csm")
        .unwrap()
        .env("CSM_HOME", temp.path())
        .arg("init")
        .arg("--global")
        .assert()
        .success()
        .stdout(predicate::str::contains("initialized successfully"));

    // Verify config created
    assert!(temp.path().join("config.toml").exists());
    assert!(temp.path().join("registry.db").exists());
}

#[test]
fn test_cli_list_empty() {
    let temp = tempdir().unwrap();

    // Initialize first
    Command::cargo_bin("csm")
        .unwrap()
        .env("CSM_HOME", temp.path())
        .arg("init")
        .arg("--global")
        .assert()
        .success();

    // List should show no skills
    Command::cargo_bin("csm")
        .unwrap()
        .env("CSM_HOME", temp.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No skills installed"));
}

#[test]
fn test_cli_add_local() {
    let temp = tempdir().unwrap();
    let skill_file = temp.path().join("skill.md");
    std::fs::write(&skill_file, "# Test Skill\n\nContent here.").unwrap();

    // Initialize
    Command::cargo_bin("csm")
        .unwrap()
        .env("CSM_HOME", temp.path())
        .arg("init")
        .arg("--global")
        .assert()
        .success();

    // Add skill
    Command::cargo_bin("csm")
        .unwrap()
        .env("CSM_HOME", temp.path())
        .arg("add")
        .arg(skill_file.to_str().unwrap())
        .arg("--name")
        .arg("test-skill")
        .arg("--yes")
        .assert()
        .success()
        .stdout(predicate::str::contains("installed successfully"));

    // Verify in list
    Command::cargo_bin("csm")
        .unwrap()
        .env("CSM_HOME", temp.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-skill"));
}
```

---

### 2.3 Simulation/E2E Tests

**Scope**: Full user workflows from start to finish.

**Location**: `tests/e2e/`

**Framework**: Custom harness + `expect` for TUI testing

**Characteristics**:
- Full application execution
- Mocked external services (GitHub API)
- Real filesystem and database
- Slower (<10s per test)

#### 2.3.1 E2E Test Scenarios

| ID | Scenario | Steps | Expected Outcome |
|----|----------|-------|------------------|
| E2E-001 | First-time setup | Install → init → import | Config created, skills imported |
| E2E-002 | Add GitHub skill | init → add github:x/y → list | Skill visible in list |
| E2E-003 | Update flow | add → modify source → update | Skill updated |
| E2E-004 | Conflict resolution | add conflicting → resolve | Conflict resolved |
| E2E-005 | Export/Import | add skills → export → reset → import | Skills restored |
| E2E-006 | TUI navigation | ui → navigate all screens | All screens accessible |
| E2E-007 | Offline mode | disconnect → operate | Graceful degradation |

#### 2.3.2 E2E Test Examples

```rust
// tests/e2e/workflow_test.rs

use std::process::{Command, Stdio};
use tempfile::tempdir;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_full_github_workflow() {
    // Setup mock GitHub API
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/repos/test/skills/contents/typescript/CLAUDE.md"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "content": base64::encode("# TypeScript Skill\n\nBe good at TypeScript."),
                "encoding": "base64"
            })))
        .mount(&mock_server)
        .await;

    let temp = tempdir().unwrap();

    // Initialize
    run_csm(&temp, &["init", "--global"]).await;

    // Add skill (using mock server)
    run_csm_with_env(
        &temp,
        &["add", "github:test/skills/typescript", "--yes"],
        &[("GITHUB_API_URL", &mock_server.uri())],
    ).await;

    // Verify skill added
    let output = run_csm(&temp, &["list", "--json"]).await;
    let skills: Vec<serde_json::Value> = serde_json::from_str(&output).unwrap();

    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0]["name"], "typescript");

    // Show skill details
    let output = run_csm(&temp, &["show", "typescript"]).await;
    assert!(output.contains("TypeScript Skill"));

    // Disable skill
    run_csm(&temp, &["disable", "typescript"]).await;

    let output = run_csm(&temp, &["list", "--json"]).await;
    let skills: Vec<serde_json::Value> = serde_json::from_str(&output).unwrap();
    assert_eq!(skills[0]["enabled"], false);

    // Re-enable
    run_csm(&temp, &["enable", "typescript"]).await;

    // Remove skill
    run_csm(&temp, &["remove", "typescript", "--force"]).await;

    let output = run_csm(&temp, &["list"]).await;
    assert!(output.contains("No skills installed"));
}

#[tokio::test]
async fn test_conflict_detection_and_resolution() {
    let temp = tempdir().unwrap();

    // Initialize
    run_csm(&temp, &["init", "--global"]).await;

    // Create two conflicting skills
    let skill_a = temp.path().join("skill-a.md");
    std::fs::write(&skill_a, "# Skill A\n\nAlways use tabs for indentation.").unwrap();

    let skill_b = temp.path().join("skill-b.md");
    std::fs::write(&skill_b, "# Skill B\n\nNever use tabs for indentation.").unwrap();

    // Add first skill
    run_csm(&temp, &["add", skill_a.to_str().unwrap(), "--name", "skill-a", "--yes"]).await;

    // Add second skill (should warn about conflict)
    let output = run_csm(&temp, &["add", skill_b.to_str().unwrap(), "--name", "skill-b", "--yes"]).await;
    assert!(output.contains("Conflict") || output.contains("conflict"));

    // Check conflicts
    let output = run_csm(&temp, &["conflicts", "--json"]).await;
    let conflicts: Vec<serde_json::Value> = serde_json::from_str(&output).unwrap();
    assert!(!conflicts.is_empty());
}

#[tokio::test]
async fn test_export_import_roundtrip() {
    let temp = tempdir().unwrap();
    let export_file = temp.path().join("export.json");

    // Initialize and add skills
    run_csm(&temp, &["init", "--global"]).await;

    let skill = temp.path().join("skill.md");
    std::fs::write(&skill, "# My Skill\n\nContent.").unwrap();
    run_csm(&temp, &["add", skill.to_str().unwrap(), "--name", "my-skill", "--yes"]).await;

    // Export
    let output = run_csm(&temp, &["export", "--all"]).await;
    std::fs::write(&export_file, &output).unwrap();

    // Reset (remove all skills)
    run_csm(&temp, &["remove", "my-skill", "--force"]).await;

    // Verify empty
    let output = run_csm(&temp, &["list"]).await;
    assert!(output.contains("No skills"));

    // Import
    run_csm(&temp, &["import", export_file.to_str().unwrap()]).await;

    // Verify restored
    let output = run_csm(&temp, &["list"]).await;
    assert!(output.contains("my-skill"));
}

async fn run_csm(temp: &tempfile::TempDir, args: &[&str]) -> String {
    run_csm_with_env(temp, args, &[]).await
}

async fn run_csm_with_env(
    temp: &tempfile::TempDir,
    args: &[&str],
    env: &[(&str, &str)],
) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_csm"));
    cmd.env("CSM_HOME", temp.path());

    for (key, value) in env {
        cmd.env(key, value);
    }

    cmd.args(args);

    let output = cmd.output().expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}
```

---

### 2.4 TUI Tests

**Scope**: Terminal UI interactions and rendering.

**Location**: `tests/tui/`

**Framework**: `insta` for snapshot testing + custom terminal emulator

**Characteristics**:
- Snapshot testing for visual regression
- Simulated keyboard input
- Terminal size variations

#### 2.4.1 TUI Test Examples

```rust
// tests/tui/snapshot_test.rs

use insta::assert_snapshot;
use csm::tui::{App, render};
use ratatui::backend::TestBackend;
use ratatui::Terminal;

#[test]
fn test_dashboard_render() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let app = App::new_with_test_data();

    terminal.draw(|frame| {
        render::dashboard(frame, &app);
    }).unwrap();

    let buffer = terminal.backend().buffer().clone();
    assert_snapshot!("dashboard", buffer_to_string(&buffer));
}

#[test]
fn test_skills_list_render() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new_with_test_data();
    app.skills = vec![
        TestSkill::new("typescript-best", true, "global"),
        TestSkill::new("python-style", true, "global"),
        TestSkill::new("experimental", false, "local"),
    ];

    terminal.draw(|frame| {
        render::skills_list(frame, &app);
    }).unwrap();

    let buffer = terminal.backend().buffer().clone();
    assert_snapshot!("skills_list", buffer_to_string(&buffer));
}

#[test]
fn test_skill_detail_render() {
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new_with_test_data();
    app.selected_skill = Some(TestSkill::detailed());

    terminal.draw(|frame| {
        render::skill_detail(frame, &app);
    }).unwrap();

    let buffer = terminal.backend().buffer().clone();
    assert_snapshot!("skill_detail", buffer_to_string(&buffer));
}

#[test]
fn test_help_overlay() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new_with_test_data();
    app.show_help = true;

    terminal.draw(|frame| {
        render::dashboard(frame, &app);
        render::help_overlay(frame, &app);
    }).unwrap();

    let buffer = terminal.backend().buffer().clone();
    assert_snapshot!("help_overlay", buffer_to_string(&buffer));
}

// tests/tui/interaction_test.rs

use csm::tui::{App, Event, handle_event};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn test_navigation_keys() {
    let mut app = App::new_with_test_data();
    app.screen = Screen::Skills;
    app.skills = vec![skill_1(), skill_2(), skill_3()];
    app.selected_index = 0;

    // Move down
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)));
    assert_eq!(app.selected_index, 1);

    // Move down again
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)));
    assert_eq!(app.selected_index, 2);

    // Move down at bottom (should not wrap)
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)));
    assert_eq!(app.selected_index, 2);

    // Move up
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)));
    assert_eq!(app.selected_index, 1);

    // Jump with 'g'
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)));
    assert_eq!(app.selected_index, 0);

    // Jump with 'G'
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::SHIFT)));
    assert_eq!(app.selected_index, 2);
}

#[test]
fn test_toggle_skill() {
    let mut app = App::new_with_test_data();
    app.screen = Screen::Skills;
    app.skills = vec![
        Skill { name: "test".into(), enabled: true, ..Default::default() },
    ];
    app.selected_index = 0;

    // Press 'e' to toggle
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)));

    assert_eq!(app.skills[0].enabled, false);

    // Toggle again
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)));

    assert_eq!(app.skills[0].enabled, true);
}

#[test]
fn test_screen_navigation() {
    let mut app = App::new();
    assert_eq!(app.screen, Screen::Dashboard);

    // Press '2' to go to Skills
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE)));
    assert_eq!(app.screen, Screen::Skills);

    // Press Esc to go back
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)));
    assert_eq!(app.screen, Screen::Dashboard);

    // Press '?' for help
    handle_event(&mut app, Event::Key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE)));
    assert!(app.show_help);
}
```

---

## 3. Test Infrastructure

### 3.1 Test Fixtures

```rust
// tests/fixtures/mod.rs

pub fn sample_skill_content() -> &'static str {
    include_str!("fixtures/sample_skill.md")
}

pub fn sample_config() -> &'static str {
    include_str!("fixtures/sample_config.toml")
}

pub fn create_test_registry(temp: &tempfile::TempDir) -> Registry {
    let db_path = temp.path().join("test.db");
    Registry::new(&db_path).unwrap()
}

pub fn create_test_skill(name: &str) -> Skill {
    Skill {
        id: Uuid::new_v4(),
        name: name.to_string(),
        description: Some(format!("Test skill: {}", name)),
        source: SkillSource::Local { path: PathBuf::from("/test") },
        scope: SkillScope::Global,
        enabled: true,
        content_hash: "test-hash".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        tags: vec!["test".to_string()],
        priority: 50,
    }
}
```

### 3.2 Mock Services

```rust
// tests/mocks/github.rs

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header};

pub async fn setup_github_mock() -> MockServer {
    let server = MockServer::start().await;

    // Mock repo contents endpoint
    Mock::given(method("GET"))
        .and(path_regex(r"/repos/.+/.+/contents/.+"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(mock_file_response()))
        .mount(&server)
        .await;

    // Mock rate limit endpoint
    Mock::given(method("GET"))
        .and(path("/rate_limit"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "rate": {
                    "limit": 5000,
                    "remaining": 4999,
                    "reset": 1234567890
                }
            })))
        .mount(&server)
        .await;

    server
}

fn mock_file_response() -> serde_json::Value {
    serde_json::json!({
        "name": "CLAUDE.md",
        "path": "CLAUDE.md",
        "sha": "abc123",
        "size": 1234,
        "content": base64::encode("# Mock Skill\n\nMock content."),
        "encoding": "base64"
    })
}
```

### 3.3 CI Configuration

```yaml
# .github/workflows/test.yml

name: Tests

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  unit:
    name: Unit Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Run unit tests
        run: cargo test --lib -- --test-threads=4

  integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Run integration tests
        run: cargo test --test '*' -- --test-threads=2

  e2e:
    name: E2E Tests
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Run E2E tests
        run: cargo test --test e2e_* -- --test-threads=1

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: llvm-tools-preview
      - uses: Swatinem/rust-cache@v2
      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov
      - name: Generate coverage
        run: cargo llvm-cov --all-features --lcov --output-path lcov.info
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info

  tui-snapshots:
    name: TUI Snapshot Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Run snapshot tests
        run: cargo insta test --review
      - name: Check for snapshot changes
        run: |
          if [[ -n $(git status --porcelain) ]]; then
            echo "Snapshot changes detected!"
            git diff
            exit 1
          fi
```

---

## 4. Test Matrix

### 4.1 Platform Coverage

| Platform | Unit | Integration | E2E | Notes |
|----------|------|-------------|-----|-------|
| Linux x86_64 | ✓ | ✓ | ✓ | Primary CI platform |
| Linux ARM64 | ✓ | ✓ | ✓ | CI via QEMU |
| macOS x86_64 | ✓ | ✓ | ✓ | CI native |
| macOS ARM64 | ✓ | ✓ | ✓ | CI native |
| Windows x86_64 | ✓ | ✓ | ✓ | CI native |

### 4.2 Feature Coverage Matrix

| Feature | Unit | Integration | E2E | Priority |
|---------|------|-------------|-----|----------|
| Skill parsing | ✓ | - | - | P0 |
| Registry CRUD | ✓ | ✓ | - | P0 |
| GitHub fetch | ✓ | ✓ | ✓ | P0 |
| Symlink management | - | ✓ | ✓ | P0 |
| Skill merging | ✓ | ✓ | - | P0 |
| Conflict detection | ✓ | ✓ | ✓ | P0 |
| CLI commands | ✓ | ✓ | ✓ | P0 |
| TUI rendering | - | - | ✓ | P1 |
| Auto-update | ✓ | ✓ | - | P1 |
| Export/Import | ✓ | ✓ | ✓ | P1 |
| Config management | ✓ | ✓ | - | P1 |

---

## 5. Performance Testing

### 5.1 Benchmarks

```rust
// benches/registry_bench.rs

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use tempfile::tempdir;

fn bench_registry_operations(c: &mut Criterion) {
    let temp = tempdir().unwrap();
    let registry = create_test_registry(&temp);

    c.bench_function("registry_create", |b| {
        b.iter(|| {
            let skill = create_test_skill("bench-skill");
            registry.create(&skill).unwrap();
            registry.delete(&skill.id).unwrap();
        });
    });

    // Pre-populate for query benchmarks
    for i in 0..1000 {
        let skill = create_test_skill(&format!("skill-{}", i));
        registry.create(&skill).unwrap();
    }

    c.bench_function("registry_list_1000", |b| {
        b.iter(|| {
            registry.list_all().unwrap();
        });
    });

    c.bench_function("registry_search_1000", |b| {
        b.iter(|| {
            registry.search("skill-500").unwrap();
        });
    });
}

fn bench_merge_operations(c: &mut Criterion) {
    let skills: Vec<Skill> = (0..100)
        .map(|i| Skill {
            name: format!("skill-{}", i),
            content: format!("# Skill {}\n\nContent for skill {}.\n\n", i, i).repeat(10),
            ..Default::default()
        })
        .collect();

    let mut group = c.benchmark_group("merge");

    for size in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let subset: Vec<_> = skills.iter().take(size).cloned().collect();
            b.iter(|| merge_skills(&subset));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_registry_operations, bench_merge_operations);
criterion_main!(benches);
```

### 5.2 Performance Targets

| Operation | Target | Measured |
|-----------|--------|----------|
| CLI startup | <10ms | TBD |
| List 100 skills | <50ms | TBD |
| Search 1000 skills | <100ms | TBD |
| Merge 10 skills | <10ms | TBD |
| TUI frame render | <16ms | TBD |

---

## 6. Test Documentation

### 6.1 Running Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test '*'

# Run specific test
cargo test test_registry_crud

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench

# Generate coverage report
cargo llvm-cov --html
open target/llvm-cov/html/index.html

# Update TUI snapshots
cargo insta test
cargo insta review
```

### 6.2 Writing New Tests

1. **Unit tests**: Add to relevant module's `#[cfg(test)]` block
2. **Integration tests**: Create file in `tests/integration/`
3. **E2E tests**: Create file in `tests/e2e/`
4. **Fixtures**: Add to `tests/fixtures/`
5. **Mocks**: Add to `tests/mocks/`
