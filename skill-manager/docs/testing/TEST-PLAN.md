# Claude Skill Manager - Test Plan

This document outlines the comprehensive test plan for the Claude Skill Manager (CSM) application. It covers unit tests, integration tests, and end-to-end tests for all major components.

## Test Coverage Goals

| Layer | Target Coverage | Priority |
|-------|----------------|----------|
| Domain Models | 95% | P0 |
| Service Layer | 90% | P0 |
| Repository Layer | 85% | P0 |
| Infrastructure | 80% | P1 |
| CLI Commands | 75% | P1 |
| TUI Components | 70% | P2 |

---

## 1. Domain Layer Tests

### 1.1 Skill Model (`src/domain/skill.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| D-SK-01 | `test_skill_new_creates_valid_skill` | Verify `Skill::new()` creates skill with correct defaults | P0 |
| D-SK-02 | `test_skill_builder_all_fields` | Verify builder sets all optional fields correctly | P0 |
| D-SK-03 | `test_skill_builder_defaults` | Verify builder uses correct defaults when fields omitted | P0 |
| D-SK-04 | `test_skill_scope_display` | Verify `SkillScope` Display trait formats correctly | P1 |
| D-SK-05 | `test_skill_scope_serialization` | Verify `SkillScope` serializes/deserializes to JSON | P0 |
| D-SK-06 | `test_update_mode_from_str` | Verify `UpdateMode::from_str` parses correctly | P0 |
| D-SK-07 | `test_update_mode_default` | Verify `UpdateMode::default()` returns `Auto` | P1 |

### 1.2 Source Model (`src/domain/source.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| D-SR-01 | `test_parse_source_github_full` | Parse `github:owner/repo/path@ref` format | P0 |
| D-SR-02 | `test_parse_source_github_minimal` | Parse `github:owner/repo` format (no path/ref) | P0 |
| D-SR-03 | `test_parse_source_url` | Parse `https://...` URL sources | P0 |
| D-SR-04 | `test_parse_source_local_absolute` | Parse absolute file paths | P0 |
| D-SR-05 | `test_parse_source_local_relative` | Parse relative file paths | P0 |
| D-SR-06 | `test_parse_source_invalid` | Return error for invalid source formats | P0 |
| D-SR-07 | `test_skill_source_display` | Verify Display trait formats all variants | P1 |
| D-SR-08 | `test_skill_source_serialization` | Verify JSON round-trip for all variants | P0 |

### 1.3 Conflict Model (`src/domain/conflict.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| D-CF-01 | `test_conflict_new` | Verify `Conflict::new()` creates with defaults | P0 |
| D-CF-02 | `test_conflict_type_display` | Verify `ConflictType` Display formatting | P1 |
| D-CF-03 | `test_conflict_status_display` | Verify `ConflictStatus` Display formatting | P1 |
| D-CF-04 | `test_resolution_strategy_variants` | Verify all resolution strategies exist | P1 |

### 1.4 Events Model (`src/domain/events.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| D-EV-01 | `test_event_bus_subscribe_publish` | Verify subscriber receives published events | P1 |
| D-EV-02 | `test_event_bus_multiple_subscribers` | Verify all subscribers receive events | P1 |
| D-EV-03 | `test_domain_event_variants` | Verify all event types can be created | P1 |

---

## 2. Service Layer Tests

### 2.1 SkillService (`src/services/skill_service.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| S-SK-01 | `test_add_skill_from_github` | Add skill from GitHub source, verify fetch & store | P0 |
| S-SK-02 | `test_add_skill_from_local` | Add skill from local file path | P0 |
| S-SK-03 | `test_add_skill_from_url` | Add skill from URL source | P0 |
| S-SK-04 | `test_add_skill_duplicate_error` | Return error when adding skill with existing name | P0 |
| S-SK-05 | `test_remove_skill_success` | Remove existing skill and its content | P0 |
| S-SK-06 | `test_remove_skill_not_found` | Return error when removing non-existent skill | P0 |
| S-SK-07 | `test_enable_skill` | Enable disabled skill, verify state change | P0 |
| S-SK-08 | `test_disable_skill` | Disable enabled skill, verify state change | P0 |
| S-SK-09 | `test_get_skill_by_name` | Retrieve skill by name | P0 |
| S-SK-10 | `test_list_skills_all` | List all skills ordered by priority | P0 |
| S-SK-11 | `test_list_skills_by_scope` | List skills filtered by scope | P1 |
| S-SK-12 | `test_list_skills_enabled_only` | List only enabled skills | P1 |

### 2.2 UpdateService (`src/services/update_service.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| S-UP-01 | `test_check_update_github_new_commit` | Detect update when GitHub has new commit | P0 |
| S-UP-02 | `test_check_update_github_no_change` | No update when commit SHA matches | P0 |
| S-UP-03 | `test_check_update_url_etag_changed` | Detect update when URL ETag changes | P0 |
| S-UP-04 | `test_check_update_local_hash_changed` | Detect update when local file hash changes | P0 |
| S-UP-05 | `test_apply_update_success` | Apply update, verify content and metadata updated | P0 |
| S-UP-06 | `test_check_all_updates` | Check updates for multiple skills | P1 |
| S-UP-07 | `test_update_mode_manual_skipped` | Manual mode skills not auto-updated | P0 |

### 2.3 ConflictService (`src/services/conflict_service.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| S-CF-01 | `test_detect_duplicate_skills` | Detect skills with identical content | P0 |
| S-CF-02 | `test_detect_contradictory_instructions` | Detect "always X" vs "never X" conflicts | P0 |
| S-CF-03 | `test_detect_overlapping_sections` | Detect skills with overlapping sections | P1 |
| S-CF-04 | `test_no_conflicts_unique_skills` | No conflicts for unique, non-overlapping skills | P0 |
| S-CF-05 | `test_resolve_conflict_keep_a` | Resolve by keeping skill A | P0 |
| S-CF-06 | `test_resolve_conflict_keep_b` | Resolve by keeping skill B | P0 |
| S-CF-07 | `test_resolve_conflict_merge` | Resolve by merging both skills | P1 |
| S-CF-08 | `test_resolve_conflict_ignore` | Mark conflict as ignored | P0 |
| S-CF-09 | `test_list_unresolved_conflicts` | List only unresolved conflicts | P0 |

### 2.4 MergeService (`src/services/merge_service.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| S-MG-01 | `test_merge_single_skill` | Output with single skill matches input | P0 |
| S-MG-02 | `test_merge_multiple_skills_priority` | Skills merged in priority order | P0 |
| S-MG-03 | `test_merge_disabled_skills_excluded` | Disabled skills not included in merge | P0 |
| S-MG-04 | `test_merge_preserves_sections` | Section headers preserved in output | P0 |
| S-MG-05 | `test_merge_global_and_local` | Global skills before local in output | P1 |
| S-MG-06 | `test_merge_empty_list` | Empty skill list produces empty output | P0 |

---

## 3. Repository Layer Tests

### 3.1 SqliteSkillRepository (`src/infra/database.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| R-SK-01 | `test_skill_repository_create` | Insert skill into database | P0 |
| R-SK-02 | `test_skill_repository_get_by_id` | Retrieve skill by UUID | P0 |
| R-SK-03 | `test_skill_repository_get_by_name` | Retrieve skill by name | P0 |
| R-SK-04 | `test_skill_repository_update` | Update skill fields | P0 |
| R-SK-05 | `test_skill_repository_delete` | Delete skill from database | P0 |
| R-SK-06 | `test_skill_repository_list` | List all skills ordered | P0 |
| R-SK-07 | `test_skill_repository_list_by_scope` | Filter by scope | P0 |
| R-SK-08 | `test_skill_repository_list_enabled` | Filter enabled only | P0 |
| R-SK-09 | `test_skill_repository_search` | Search by name/description | P0 |
| R-SK-10 | `test_skill_repository_exists` | Check name existence | P0 |
| R-SK-11 | `test_skill_repository_duplicate_name_error` | Error on duplicate name insert | P0 |

### 3.2 SqliteConflictRepository (`src/infra/database.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| R-CF-01 | `test_conflict_repository_create` | Insert conflict | P0 |
| R-CF-02 | `test_conflict_repository_get` | Retrieve conflict by ID | P0 |
| R-CF-03 | `test_conflict_repository_update` | Update conflict status | P0 |
| R-CF-04 | `test_conflict_repository_delete` | Delete conflict | P0 |
| R-CF-05 | `test_conflict_repository_list` | List all conflicts | P0 |
| R-CF-06 | `test_conflict_repository_list_unresolved` | Filter unresolved only | P0 |
| R-CF-07 | `test_conflict_repository_list_by_skill` | Filter by skill ID | P0 |
| R-CF-08 | `test_conflict_repository_delete_by_skill` | Delete all conflicts for skill | P0 |

---

## 4. Infrastructure Layer Tests

### 4.1 Storage (`src/infra/storage.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| I-ST-01 | `test_storage_read_file` | Read content from file path | P0 |
| I-ST-02 | `test_storage_write_file` | Write content to file path | P0 |
| I-ST-03 | `test_storage_delete_file` | Delete file from path | P0 |
| I-ST-04 | `test_storage_exists` | Check file existence | P0 |
| I-ST-05 | `test_storage_read_nonexistent` | Error on missing file | P0 |
| I-ST-06 | `test_output_storage_write_merged` | Write merged CLAUDE.md | P0 |
| I-ST-07 | `test_output_storage_create_symlink` | Create symlink to skill | P1 |

### 4.2 GitHub Client (`src/infra/github.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| I-GH-01 | `test_github_fetch_file_success` | Fetch file content from repo | P0 |
| I-GH-02 | `test_github_fetch_file_not_found` | Error on missing file | P0 |
| I-GH-03 | `test_github_get_latest_commit` | Get HEAD commit SHA | P0 |
| I-GH-04 | `test_github_rate_limit_handling` | Handle rate limit response | P1 |
| I-GH-05 | `test_url_client_fetch` | Fetch content from URL | P0 |
| I-GH-06 | `test_url_client_etag` | Return ETag header | P1 |

### 4.3 Config Manager (`src/infra/config.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| I-CF-01 | `test_config_load_defaults` | Load with default values | P0 |
| I-CF-02 | `test_config_load_from_file` | Load from TOML file | P0 |
| I-CF-03 | `test_config_save` | Save config to file | P0 |
| I-CF-04 | `test_config_get_set_values` | Get/set individual values | P0 |
| I-CF-05 | `test_config_csm_home_path` | Correct CSM home directory | P0 |

---

## 5. CLI Command Tests

### 5.1 Init Command (`src/cli/commands/init.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| C-IN-01 | `test_init_creates_directories` | Create .csm directory structure | P0 |
| C-IN-02 | `test_init_creates_database` | Initialize SQLite database | P0 |
| C-IN-03 | `test_init_creates_config` | Create default config file | P0 |
| C-IN-04 | `test_init_idempotent` | Re-init doesn't overwrite | P0 |
| C-IN-05 | `test_init_force_overwrites` | Force flag overwrites existing | P1 |
| C-IN-06 | `test_init_import_existing` | Import existing CLAUDE.md | P1 |

### 5.2 Add Command (`src/cli/commands/add.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| C-AD-01 | `test_add_github_skill` | Add skill from GitHub | P0 |
| C-AD-02 | `test_add_local_skill` | Add skill from local path | P0 |
| C-AD-03 | `test_add_url_skill` | Add skill from URL | P0 |
| C-AD-04 | `test_add_with_name` | Custom name option | P0 |
| C-AD-05 | `test_add_global_scope` | Add as global skill | P0 |
| C-AD-06 | `test_add_local_scope` | Add as project-local skill | P0 |
| C-AD-07 | `test_add_disabled` | Add in disabled state | P1 |
| C-AD-08 | `test_add_with_priority` | Set custom priority | P1 |

### 5.3 List Command (`src/cli/commands/list.rs`)

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| C-LS-01 | `test_list_all_skills` | List all skills | P0 |
| C-LS-02 | `test_list_global_only` | Filter global skills | P0 |
| C-LS-03 | `test_list_local_only` | Filter local skills | P0 |
| C-LS-04 | `test_list_enabled_only` | Filter enabled skills | P0 |
| C-LS-05 | `test_list_disabled_only` | Filter disabled skills | P0 |
| C-LS-06 | `test_list_json_output` | JSON format output | P0 |
| C-LS-07 | `test_list_empty` | Handle no skills | P0 |

---

## 6. Integration Tests

### 6.1 End-to-End Workflows

| Test ID | Test Name | Description | Priority |
|---------|-----------|-------------|----------|
| E2E-01 | `test_full_workflow_github_skill` | Init -> Add GitHub -> List -> Merge | P0 |
| E2E-02 | `test_full_workflow_local_skill` | Init -> Add Local -> Enable -> Merge | P0 |
| E2E-03 | `test_conflict_detection_workflow` | Add conflicting skills -> Detect -> Resolve | P0 |
| E2E-04 | `test_update_workflow` | Add -> Modify source -> Check -> Apply | P1 |
| E2E-05 | `test_multi_project_workflow` | Global + Project-local skills merge | P1 |

---

## 7. Test Implementation Notes

### 7.1 Mocking Strategy

Using `mockall` for creating mock implementations:

```rust
// Example mock usage
#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::MockSkillRepository;

    #[tokio::test]
    async fn test_add_skill_from_github() {
        let mut mock_repo = MockSkillRepository::new();
        mock_repo
            .expect_exists()
            .returning(|_| Ok(false));
        mock_repo
            .expect_create()
            .returning(|_| Ok(()));

        let mut mock_github = MockGitHubClient::new();
        mock_github
            .expect_fetch_file()
            .returning(|_, _, _, _| Ok("# Skill content".to_string()));

        // ... test implementation
    }
}
```

### 7.2 Test Fixtures

Create test fixtures in `tests/fixtures/`:

```
tests/
├── fixtures/
│   ├── skills/
│   │   ├── typescript.md
│   │   ├── rust-best.md
│   │   └── conflicting-a.md
│   ├── configs/
│   │   └── test-config.toml
│   └── expected/
│       └── merged-output.md
├── integration/
│   ├── mod.rs
│   ├── workflow_tests.rs
│   └── conflict_tests.rs
└── common/
    └── mod.rs (shared test utilities)
```

### 7.3 Test Database

Use in-memory SQLite for repository tests:

```rust
let repo = SqliteSkillRepository::in_memory()?;
```

### 7.4 Async Test Runtime

All async tests use `#[tokio::test]`:

```rust
#[tokio::test]
async fn test_async_operation() {
    // test body
}
```

---

## 8. Test Execution

### 8.1 Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_skill_repository_crud

# Run tests for module
cargo test domain::

# Run integration tests only
cargo test --test integration
```

### 8.2 Coverage

```bash
# Using cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## 9. Summary

| Category | Test Count | Priority Breakdown |
|----------|------------|-------------------|
| Domain Models | 18 | P0: 12, P1: 6 |
| Services | 31 | P0: 24, P1: 7 |
| Repositories | 19 | P0: 19 |
| Infrastructure | 17 | P0: 12, P1: 5 |
| CLI Commands | 21 | P0: 15, P1: 6 |
| Integration | 5 | P0: 3, P1: 2 |
| **Total** | **111** | **P0: 85, P1: 26** |

---

## 10. Approval Required

Before implementing these tests, please review:

1. **Test Coverage**: Are there any critical paths missing?
2. **Priorities**: Should any P1 tests be elevated to P0?
3. **Mock Strategy**: Is the mockall-based approach acceptable?
4. **Test Fixtures**: Any additional fixture files needed?
5. **Integration Tests**: Any additional E2E scenarios?

**Please provide explicit approval to proceed with test implementation.**
