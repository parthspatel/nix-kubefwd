//! Integration tests for the `csm sync` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_sync_completes() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("sync")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sync complete"));
}

#[test]
fn test_sync_with_verify() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("sync")
        .arg("--verify")
        .assert()
        .success()
        .stdout(predicate::str::contains("verified").or(predicate::str::contains("Verified")));
}

#[test]
fn test_sync_with_skills() {
    let mut env = TestEnv::new();
    env.init();

    // Add a skill first
    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("sync")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sync complete"));
}

#[test]
fn test_sync_rebuild() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    env.cmd()
        .arg("sync")
        .arg("--rebuild")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sync complete").or(predicate::str::contains("Rebuilt")));
}

#[test]
fn test_sync_multiple_skills() {
    let mut env = TestEnv::new();
    env.init();

    // Add multiple skills
    let skill1 = env.create_skill_file("skill-one", "# One\n\nContent.");
    let skill2 = env.create_skill_file("skill-two", "# Two\n\nContent.");

    env.add_skill(&skill1);
    env.add_skill(&skill2);

    env.cmd()
        .arg("sync")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sync complete"));
}
