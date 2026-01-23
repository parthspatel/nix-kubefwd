//! Integration tests for the `csm add` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_add_local_file() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_skill_file("test-skill", "# Test Skill\n\nA simple test.");

    env.cmd()
        .arg("add")
        .arg(&skill_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully added skill"));
}

#[test]
fn test_add_hello_world_skill() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();

    env.cmd()
        .arg("add")
        .arg(&skill_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Successfully added skill: hello-world",
        ));
}

#[test]
fn test_add_with_custom_name() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_skill_file("original-name", "# Skill\n\nContent.");

    env.cmd()
        .arg("add")
        .arg(&skill_path)
        .arg("--name")
        .arg("custom-name")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Successfully added skill: custom-name",
        ));

    // Verify the skill appears with custom name in list
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("custom-name"));
}

#[test]
fn test_add_global_scope() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_skill_file("global-skill", "# Global Skill\n\nContent.");

    env.cmd()
        .arg("add")
        .arg(&skill_path)
        .arg("--scope")
        .arg("global")
        .assert()
        .success()
        .stdout(predicate::str::contains("Scope: global"));
}

#[test]
fn test_add_nonexistent_file_fails() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("add")
        .arg("/nonexistent/path/to/skill.md")
        .assert()
        .failure();
}

#[test]
fn test_add_duplicate_name_fails() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_skill_file("duplicate", "# Duplicate\n\nFirst version.");

    // First add should succeed
    env.cmd().arg("add").arg(&skill_path).assert().success();

    // Create another file with same derived name
    let skill_path2 = env.create_skill_file("duplicate", "# Duplicate\n\nSecond version.");

    // Second add should fail (duplicate name)
    env.cmd()
        .arg("add")
        .arg(&skill_path2)
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("already exists")
                .or(predicate::str::contains("duplicate"))
                .or(predicate::str::contains("Duplicate")),
        );
}

#[test]
fn test_add_shows_in_list() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();

    // Add the skill
    env.cmd().arg("add").arg(&skill_path).assert().success();

    // Verify it appears in list
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"))
        .stdout(predicate::str::contains("enabled"));
}

#[test]
fn test_add_with_different_update_modes() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_skill_file("auto-skill", "# Auto\n\nContent.");

    // Test with auto update mode (default)
    env.cmd()
        .arg("add")
        .arg(&skill_path)
        .arg("--update-mode")
        .arg("auto")
        .assert()
        .success();
}
