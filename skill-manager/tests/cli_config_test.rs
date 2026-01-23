//! Integration tests for the `csm config` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_config_list() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("config")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("CSM Configuration"))
        .stdout(predicate::str::contains("general.default_scope"))
        .stdout(predicate::str::contains("updates.mode"));
}

#[test]
fn test_config_get() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("config")
        .arg("get")
        .arg("general.default_scope")
        .assert()
        .success()
        .stdout(predicate::str::contains("local").or(predicate::str::contains("global")));
}

#[test]
fn test_config_set() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("config")
        .arg("set")
        .arg("general.default_scope")
        .arg("global")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Set general.default_scope = global",
        ));
}

#[test]
fn test_config_get_after_set() {
    let mut env = TestEnv::new();
    env.init();

    // Set a value
    env.cmd()
        .arg("config")
        .arg("set")
        .arg("general.default_scope")
        .arg("global")
        .assert()
        .success();

    // Get should return the new value
    env.cmd()
        .arg("config")
        .arg("get")
        .arg("general.default_scope")
        .assert()
        .success()
        .stdout(predicate::str::contains("global"));
}

#[test]
fn test_config_list_json() {
    let mut env = TestEnv::new();
    env.init();

    let output = env
        .cmd()
        .arg("config")
        .arg("list")
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
fn test_config_set_updates_mode() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("config")
        .arg("set")
        .arg("updates.mode")
        .arg("manual")
        .assert()
        .success();

    env.cmd()
        .arg("config")
        .arg("get")
        .arg("updates.mode")
        .assert()
        .success()
        .stdout(predicate::str::contains("manual"));
}

#[test]
fn test_config_set_boolean() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("config")
        .arg("set")
        .arg("general.color")
        .arg("false")
        .assert()
        .success();

    env.cmd()
        .arg("config")
        .arg("get")
        .arg("general.color")
        .assert()
        .success()
        .stdout(predicate::str::contains("false"));
}

#[test]
fn test_config_get_nonexistent_key() {
    let mut env = TestEnv::new();
    env.init();

    // Getting a nonexistent key should fail with an error
    env.cmd()
        .arg("config")
        .arg("get")
        .arg("nonexistent.key")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Unknown config key")
                .or(predicate::str::contains("not found")),
        );
}

#[test]
fn test_config_reset_with_force() {
    let mut env = TestEnv::new();
    env.init();

    // Change a setting
    env.cmd()
        .arg("config")
        .arg("set")
        .arg("general.default_scope")
        .arg("global")
        .assert()
        .success();

    // Reset with force
    env.cmd()
        .arg("config")
        .arg("reset")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("reset").or(predicate::str::contains("Reset")));

    // Verify default value is restored
    env.cmd()
        .arg("config")
        .arg("get")
        .arg("general.default_scope")
        .assert()
        .success()
        .stdout(predicate::str::contains("local"));
}
