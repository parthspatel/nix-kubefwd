//! Integration tests for the `csm export` and `csm import` commands

mod common;

use common::TestEnv;
use predicates::prelude::*;

#[test]
fn test_export_all_skills() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    let export_path = env.export_path("export.json");

    env.cmd()
        .arg("export")
        .arg("--all")
        .arg("-o")
        .arg(&export_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Exported 1 skill"));

    // Verify file was created and contains valid JSON
    assert!(export_path.exists(), "Export file should exist");
    let content = std::fs::read_to_string(&export_path).expect("Failed to read export file");
    let json: serde_json::Value =
        serde_json::from_str(&content).expect("Export should be valid JSON");

    assert!(
        json.get("skills").is_some(),
        "Export should have 'skills' field"
    );
}

#[test]
fn test_export_specific_skill() {
    let mut env = TestEnv::new();
    env.init();

    // Add multiple skills
    let skill1 = env.create_hello_world();
    let skill2 = env.create_skill_file("other-skill", "# Other\n\nContent.");

    env.add_skill(&skill1);
    env.add_skill(&skill2);

    let export_path = env.export_path("single-export.json");

    env.cmd()
        .arg("export")
        .arg("--skill")
        .arg("hello-world")
        .arg("-o")
        .arg(&export_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Exported 1 skill"));

    // Verify only the specified skill was exported
    let content = std::fs::read_to_string(&export_path).expect("Failed to read export file");
    let json: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON");

    let skills = json.get("skills").expect("Missing skills field");
    assert_eq!(
        skills.as_array().unwrap().len(),
        1,
        "Should export exactly 1 skill"
    );
}

#[test]
fn test_export_toml_format() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    let export_path = env.export_path("export.toml");

    env.cmd()
        .arg("export")
        .arg("--all")
        .arg("-f")
        .arg("toml")
        .arg("-o")
        .arg(&export_path)
        .assert()
        .success();

    // Verify file contains TOML content
    let content = std::fs::read_to_string(&export_path).expect("Failed to read export file");
    assert!(
        content.contains("[") || content.contains("version"),
        "Should contain TOML syntax"
    );
}

#[test]
fn test_import_from_export() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    let export_path = env.export_path("roundtrip.json");

    // Export
    env.cmd()
        .arg("export")
        .arg("--all")
        .arg("-o")
        .arg(&export_path)
        .assert()
        .success();

    // Remove the skill
    env.cmd()
        .arg("remove")
        .arg("hello-world")
        .arg("--force")
        .assert()
        .success();

    // Verify it's gone
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No skills found"));

    // Import
    env.cmd()
        .arg("import")
        .arg(&export_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Imported: 1"));

    // Verify skill is back
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"));
}

#[test]
fn test_import_skip_existing() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    let export_path = env.export_path("duplicate.json");

    // Export
    env.cmd()
        .arg("export")
        .arg("--all")
        .arg("-o")
        .arg(&export_path)
        .assert()
        .success();

    // Import again (skill already exists)
    env.cmd()
        .arg("import")
        .arg(&export_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Skipped: 1"));
}

#[test]
fn test_import_dry_run() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    let export_path = env.export_path("dryrun.json");

    // Export
    env.cmd()
        .arg("export")
        .arg("--all")
        .arg("-o")
        .arg(&export_path)
        .assert()
        .success();

    // Remove skill
    env.cmd()
        .arg("remove")
        .arg("hello-world")
        .arg("--force")
        .assert()
        .success();

    // Dry run import
    env.cmd()
        .arg("import")
        .arg(&export_path)
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("dry run").or(predicate::str::contains("Would import")));

    // Skill should still not exist
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No skills found"));
}

#[test]
fn test_export_to_stdout() {
    let mut env = TestEnv::new();
    env.init();

    let skill_path = env.create_hello_world();
    env.add_skill(&skill_path);

    // Export without -o should output to stdout
    let output = env
        .cmd()
        .arg("export")
        .arg("--all")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).expect("Invalid UTF-8");

    // Should be valid JSON
    let _json: serde_json::Value =
        serde_json::from_str(&stdout).expect("Stdout should be valid JSON");
}

#[test]
fn test_import_nonexistent_file_fails() {
    let mut env = TestEnv::new();
    env.init();

    env.cmd()
        .arg("import")
        .arg("/nonexistent/path/to/export.json")
        .assert()
        .failure();
}
