//! Integration tests for the `csm remove` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_remove_skill_with_force() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Remove with --force (no confirmation)
    env.cmd()
        .arg("remove")
        .arg("hello-world")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed skill: hello-world"));
}

#[test]
fn test_remove_skill_with_yes_flag() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Remove with -y (global yes flag)
    env.cmd()
        .arg("-y")
        .arg("remove")
        .arg("hello-world")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed skill: hello-world"));
}

#[test]
fn test_remove_nonexistent_fails() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("remove")
        .arg("nonexistent-skill")
        .arg("--force")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Not found")));
}

#[test]
fn test_list_after_remove() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Verify skill exists
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"));

    // Remove it
    env.cmd()
        .arg("remove")
        .arg("hello-world")
        .arg("--force")
        .assert()
        .success();

    // Verify skill no longer appears
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No skills found"));
}

#[test]
fn test_remove_alias_rm() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // The 'rm' alias should work the same as 'remove'
    env.cmd()
        .arg("rm")
        .arg("hello-world")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed skill: hello-world"));
}

#[test]
fn test_remove_one_of_multiple() {
    let mut env = TestEnv::new();
    env.init();

    // Add multiple skills
    let skill1 = env.create_skill_file("skill-one", "# One\n\nContent.");
    let skill2 = env.create_skill_file("skill-two", "# Two\n\nContent.");

    env.add_skill(&skill1);
    env.add_skill(&skill2);

    // Remove only one
    env.cmd()
        .arg("remove")
        .arg("skill-one")
        .arg("--force")
        .assert()
        .success();

    // Verify only skill-two remains
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("skill-two"))
        .stdout(predicate::str::contains("skill-one").not())
        .stdout(predicate::str::contains("Total: 1 skill"));
}

#[test]
fn test_show_after_remove_fails() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Remove the skill
    env.cmd()
        .arg("remove")
        .arg("hello-world")
        .arg("--force")
        .assert()
        .success();

    // Show should now fail
    env.cmd().arg("show").arg("hello-world").assert().failure();
}
