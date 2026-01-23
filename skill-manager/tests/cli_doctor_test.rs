//! Integration tests for the `csm doctor` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_doctor_healthy() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("CSM Doctor"))
        .stdout(predicate::str::contains("No issues found").or(predicate::str::contains("✓")));
}

#[test]
fn test_doctor_checks_directories() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("CSM home"))
        .stdout(predicate::str::contains("Database"))
        .stdout(predicate::str::contains("Config"));
}

#[test]
fn test_doctor_with_skills() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("1 skill"));
}

#[test]
fn test_doctor_with_fix_flag() {
    let mut env = TestEnv::new();
    env.init();

    // Doctor with --fix on healthy system should succeed
    env.cmd().arg("doctor").arg("--fix").assert().success();
}

#[test]
fn test_doctor_verifies_database() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Database").or(predicate::str::contains("registry.db")));
}

#[test]
fn test_doctor_verifies_skills_directory() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd().arg("doctor").assert().success().stdout(
        predicate::str::contains("Skills directory").or(predicate::str::contains("skills")),
    );
}
