//! Integration tests for the `csm create` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_create_skill() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("create")
        .arg("my-new-skill")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created skill: my-new-skill"));
}

#[test]
fn test_create_from_file() {
    let mut env = TestEnv::new();
    env.init();

    // Create a template file
    let template =
        env.create_skill_file("template", "# Template Skill\n\nThis is template content.");

    env.cmd()
        .arg("create")
        .arg("from-template")
        .arg("--from")
        .arg(&template)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created skill: from-template"));

    // Verify content was copied by showing it
    env.cmd()
        .arg("show")
        .arg("from-template")
        .arg("--content")
        .assert()
        .success()
        .stdout(predicate::str::contains("Template Skill"));
}

#[test]
fn test_create_shows_in_list() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("create")
        .arg("listed-skill")
        .assert()
        .success();

    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("listed-skill"));
}

#[test]
fn test_create_global_scope() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("create")
        .arg("global-skill")
        .arg("--scope")
        .arg("global")
        .assert()
        .success()
        .stdout(predicate::str::contains("Scope: global"));
}

#[test]
fn test_create_local_scope() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("create")
        .arg("local-skill")
        .arg("--scope")
        .arg("local")
        .assert()
        .success()
        .stdout(predicate::str::contains("project:").or(predicate::str::contains("local")));
}

#[test]
fn test_create_duplicate_name_fails() {
    let mut env = TestEnv::new();
    env.init();

    // Create first skill
    env.cmd()
        .arg("create")
        .arg("duplicate-skill")
        .assert()
        .success();

    // Try to create with same name
    env.cmd()
        .arg("create")
        .arg("duplicate-skill")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("already exists").or(predicate::str::contains("duplicate")),
        );
}

#[test]
fn test_create_can_be_shown() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("create")
        .arg("showable-skill")
        .assert()
        .success();

    env.cmd()
        .arg("show")
        .arg("showable-skill")
        .assert()
        .success()
        .stdout(predicate::str::contains("Skill: showable-skill"))
        .stdout(predicate::str::contains("enabled"));
}

#[test]
fn test_create_can_be_disabled() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("create")
        .arg("disableable-skill")
        .assert()
        .success();

    env.cmd()
        .arg("disable")
        .arg("disableable-skill")
        .assert()
        .success();

    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("disabled"));
}
