//! Integration tests for the `csm list` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_list_empty() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No skills found"));
}

#[test]
fn test_list_shows_added_skill() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"))
        .stdout(predicate::str::contains("enabled"))
        .stdout(predicate::str::contains("Total: 1 skill"));
}

#[test]
fn test_list_with_json_output() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    let output = env
        .cmd()
        .arg("list")
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

    // Verify structure
    assert!(
        json.is_array() || json.is_object(),
        "JSON should be array or object"
    );
}

#[test]
fn test_list_filter_enabled() {
    let mut env = TestEnv::new();
    env.init();

    // Add two skills
    let skill1 = env.create_skill_file("skill-one", "# Skill One\n\nContent.");
    let skill2 = env.create_skill_file("skill-two", "# Skill Two\n\nContent.");

    env.add_skill(&skill1);
    env.add_skill(&skill2);

    // Disable one skill
    env.cmd().arg("disable").arg("skill-one").assert().success();

    // List only enabled skills
    env.cmd()
        .arg("list")
        .arg("--enabled")
        .assert()
        .success()
        .stdout(predicate::str::contains("skill-two"))
        .stdout(predicate::str::contains("skill-one").not());
}

#[test]
fn test_list_filter_disabled() {
    let mut env = TestEnv::new();
    env.init();

    // Add two skills
    let skill1 = env.create_skill_file("skill-one", "# Skill One\n\nContent.");
    let skill2 = env.create_skill_file("skill-two", "# Skill Two\n\nContent.");

    env.add_skill(&skill1);
    env.add_skill(&skill2);

    // Disable one skill
    env.cmd().arg("disable").arg("skill-one").assert().success();

    // List only disabled skills
    env.cmd()
        .arg("list")
        .arg("--disabled")
        .assert()
        .success()
        .stdout(predicate::str::contains("skill-one"))
        .stdout(predicate::str::contains("skill-two").not());
}

#[test]
fn test_list_multiple_skills() {
    let mut env = TestEnv::new();
    env.init();

    // Add multiple skills
    let skill1 = env.create_skill_file("alpha-skill", "# Alpha\n\nContent.");
    let skill2 = env.create_skill_file("beta-skill", "# Beta\n\nContent.");
    let skill3 = env.create_skill_file("gamma-skill", "# Gamma\n\nContent.");

    env.add_skill(&skill1);
    env.add_skill(&skill2);
    env.add_skill(&skill3);

    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha-skill"))
        .stdout(predicate::str::contains("beta-skill"))
        .stdout(predicate::str::contains("gamma-skill"))
        .stdout(predicate::str::contains("Total: 3 skill"));
}

#[test]
fn test_list_alias_ls() {
    let mut env = TestEnv::new();
    env.init();

    // The 'ls' alias should work the same as 'list'
    env.cmd()
        .arg("ls")
        .assert()
        .success()
        .stdout(predicate::str::contains("No skills found"));
}
