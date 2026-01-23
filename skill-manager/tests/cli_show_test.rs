//! Integration tests for the `csm show` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_show_skill_details() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("show")
        .arg("hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("Skill: hello-world"))
        .stdout(predicate::str::contains("ID:"))
        .stdout(predicate::str::contains("Source:"))
        .stdout(predicate::str::contains("Status:"))
        .stdout(predicate::str::contains("enabled"));
}

#[test]
fn test_show_with_content() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("show")
        .arg("hello-world")
        .arg("--content")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World Skill"))
        .stdout(predicate::str::contains("When the user says"));
}

#[test]
fn test_show_json_output() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    let output = env
        .cmd()
        .arg("show")
        .arg("hello-world")
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).expect("Invalid UTF-8");

    // Verify it's valid JSON
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("Output should be valid JSON");

    // Verify expected fields
    assert!(json.get("name").is_some(), "JSON should have 'name' field");
    assert!(
        json.get("enabled").is_some(),
        "JSON should have 'enabled' field"
    );
}

#[test]
fn test_show_nonexistent_fails() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("show")
        .arg("nonexistent-skill")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("not found")
                .or(predicate::str::contains("Not found"))
                .or(predicate::str::contains("does not exist")),
        );
}

#[test]
fn test_show_displays_scope() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_skill_file("scoped-skill", "# Scoped\n\nContent.");

    // Add with global scope
    env.cmd()
        .arg("add")
        .arg(&skill_path)
        .arg("--scope")
        .arg("global")
        .assert()
        .success();

    env.cmd()
        .arg("show")
        .arg("scoped-skill")
        .assert()
        .success()
        .stdout(predicate::str::contains("Scope:"))
        .stdout(predicate::str::contains("global"));
}

#[test]
fn test_show_displays_priority() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("show")
        .arg("hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("Priority:"));
}

#[test]
fn test_show_displays_timestamps() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("show")
        .arg("hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created:"))
        .stdout(predicate::str::contains("Updated:"));
}
