//! Integration tests for the `csm init` command

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_init_creates_config_directory() {
    let mut env = TestEnv::new();

    // Verify directory doesn't exist yet
    assert!(!env.config_exists());
    assert!(!env.database_exists());
    assert!(!env.skills_dir_exists());

    // Initialize
    env.init();

    // Verify directories and files were created
    assert!(env.config_exists(), "config.toml should exist");
    assert!(env.database_exists(), "registry.db should exist");
    assert!(env.skills_dir_exists(), "skills directory should exist");
}

#[test]
fn test_init_creates_database() {
    let mut env = TestEnv::new();
    env.init();

    let db_path = env.home().join("registry.db");
    assert!(db_path.exists(), "Database file should exist");
    assert!(
        db_path.metadata().unwrap().len() > 0,
        "Database should not be empty"
    );
}

#[test]
fn test_init_idempotent_with_force() {
    let mut env = TestEnv::new();

    // First init
    env.init();
    assert!(env.config_exists());

    // Second init with --force should succeed
    env.init_force();
    assert!(env.config_exists());
}

#[test]
fn test_init_fails_if_already_initialized() {
    let mut env = TestEnv::new();

    // First init should succeed
    env.init();

    // Second init without --force should fail
    env.cmd().arg("init").assert().failure().stderr(
        predicate::str::contains("Already initialized")
            .or(predicate::str::contains("already initialized")),
    );
}

#[test]
fn test_init_creates_subdirectories() {
    let mut env = TestEnv::new();
    env.init();

    // Check that all expected subdirectories exist
    assert!(
        env.home().join("skills").exists(),
        "skills directory should exist"
    );
    assert!(
        env.home().join("cache").exists(),
        "cache directory should exist"
    );
    assert!(
        env.home().join("logs").exists(),
        "logs directory should exist"
    );
}

#[test]
fn test_init_output_message() {
    let mut env = TestEnv::new();

    env.cmd()
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("CSM initialized successfully"));
}
