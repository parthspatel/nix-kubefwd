//! Integration tests for the `csm search` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_search_by_name() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("search")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"))
        .stdout(predicate::str::contains("Found 1 skill"));
}

#[test]
fn test_search_no_results() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("search")
        .arg("nonexistent")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("No skills found")
                .or(predicate::str::contains("Found 0 skill")),
        );
}

#[test]
fn test_search_json_output() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    let output = env
        .cmd()
        .arg("search")
        .arg("hello")
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).expect("Invalid UTF-8");

    // Verify it's valid JSON
    let _json: serde_json::Value =
        serde_json::from_str(&stdout).expect("Output should be valid JSON");
}

#[test]
fn test_search_partial_match() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_skill_file("my-awesome-skill", "# Awesome\n\nContent.");
    env.add_skill(&skill_path);

    // Search for partial name
    env.cmd()
        .arg("search")
        .arg("awesome")
        .assert()
        .success()
        .stdout(predicate::str::contains("my-awesome-skill"));
}

#[test]
fn test_search_multiple_results() {
    let mut env = TestEnv::new();
    env.init();

    // Add skills with similar names
    let skill1 = env.create_skill_file("test-alpha", "# Alpha\n\nContent.");
    let skill2 = env.create_skill_file("test-beta", "# Beta\n\nContent.");
    let skill3 = env.create_skill_file("other-skill", "# Other\n\nContent.");

    env.add_skill(&skill1);
    env.add_skill(&skill2);
    env.add_skill(&skill3);

    // Search for "test" should find two skills
    env.cmd()
        .arg("search")
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-alpha"))
        .stdout(predicate::str::contains("test-beta"))
        .stdout(predicate::str::contains("other-skill").not())
        .stdout(predicate::str::contains("Found 2 skill"));
}

#[test]
fn test_search_case_insensitive() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Search with different case
    env.cmd()
        .arg("search")
        .arg("HELLO")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"));
}

#[test]
fn test_search_alias_s() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // The 's' alias should work the same as 'search'
    env.cmd()
        .arg("s")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"));
}
