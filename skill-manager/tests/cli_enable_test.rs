//! Integration tests for the `csm enable` and `csm disable` commands

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_disable_skill() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Disable the skill
    env.cmd()
        .arg("disable")
        .arg("hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("Disabled skill: hello-world"));

    // Verify it's disabled in list
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("disabled"));
}

#[test]
fn test_enable_skill() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // First disable it
    env.cmd()
        .arg("disable")
        .arg("hello-world")
        .assert()
        .success();

    // Then enable it
    env.cmd()
        .arg("enable")
        .arg("hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("Enabled skill: hello-world"));

    // Verify it's enabled in list
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("enabled"));
}

#[test]
fn test_enable_already_enabled() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Skills are enabled by default, enabling again should be idempotent
    env.cmd()
        .arg("enable")
        .arg("hello-world")
        .assert()
        .success();

    // Verify still enabled
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("enabled"));
}

#[test]
fn test_disable_already_disabled() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Disable twice - should be idempotent
    env.cmd()
        .arg("disable")
        .arg("hello-world")
        .assert()
        .success();

    env.cmd()
        .arg("disable")
        .arg("hello-world")
        .assert()
        .success();

    // Verify still disabled
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("disabled"));
}

#[test]
fn test_enable_nonexistent_fails() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("enable")
        .arg("nonexistent-skill")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Not found")));
}

#[test]
fn test_disable_nonexistent_fails() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("disable")
        .arg("nonexistent-skill")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Not found")));
}

#[test]
fn test_show_reflects_enable_disable() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Initially enabled
    env.cmd()
        .arg("show")
        .arg("hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("enabled"));

    // Disable
    env.cmd()
        .arg("disable")
        .arg("hello-world")
        .assert()
        .success();

    // Show should reflect disabled status
    env.cmd()
        .arg("show")
        .arg("hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("disabled"));
}
