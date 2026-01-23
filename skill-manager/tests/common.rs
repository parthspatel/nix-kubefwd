//! Common test utilities for integration tests

use assert_cmd::Command;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Hello world skill content for testing
pub const HELLO_WORLD_CONTENT: &str = r#"# Hello World Skill

This is a test skill that demonstrates the basic structure of a Claude skill.

## Instructions

When the user says "hello", respond with a friendly greeting.

## Examples

User: Hello
Assistant: Hello! How can I help you today?

User: Hi there!
Assistant: Hi! It's great to meet you. What can I do for you?
"#;

/// Test environment that provides an isolated CSM_HOME directory
pub struct TestEnv {
    temp_dir: TempDir,
    initialized: bool,
}

impl TestEnv {
    /// Create a new test environment with a temporary directory
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        Self {
            temp_dir,
            initialized: false,
        }
    }

    /// Get the path to the CSM_HOME directory
    pub fn home(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Get the path as a string
    pub fn home_str(&self) -> &str {
        self.home().to_str().expect("Path is not valid UTF-8")
    }

    /// Initialize CSM in this test environment
    pub fn init(&mut self) -> &mut Self {
        if !self.initialized {
            self.cmd().arg("init").assert().success();
            self.initialized = true;
        }
        self
    }

    /// Initialize CSM with force flag
    pub fn init_force(&mut self) -> &mut Self {
        self.cmd().arg("init").arg("--force").assert().success();
        self.initialized = true;
        self
    }

    /// Get a Command configured for this test environment
    pub fn cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("csm").expect("Failed to find csm binary");
        cmd.env("CSM_HOME", self.home_str());
        // Set current directory to temp dir for local scope tests
        cmd.current_dir(self.home());
        cmd
    }

    /// Create a skill file in the test environment
    pub fn create_skill_file(&self, name: &str, content: &str) -> PathBuf {
        let path = self.temp_dir.path().join(format!("{}.md", name));
        std::fs::write(&path, content).expect("Failed to write skill file");
        path
    }

    /// Create the hello-world skill file
    pub fn create_hello_world(&self) -> PathBuf {
        self.create_skill_file("hello-world", HELLO_WORLD_CONTENT)
    }

    /// Add a skill and return the test env for chaining
    pub fn add_skill(&mut self, path: &Path) -> &mut Self {
        self.cmd().arg("add").arg(path).assert().success();
        self
    }

    /// Add a skill with a custom name
    pub fn add_skill_named(&mut self, path: &Path, name: &str) -> &mut Self {
        self.cmd()
            .arg("add")
            .arg(path)
            .arg("--name")
            .arg(name)
            .assert()
            .success();
        self
    }

    /// Check if the database file exists
    pub fn database_exists(&self) -> bool {
        self.home().join("registry.db").exists()
    }

    /// Check if the config file exists
    pub fn config_exists(&self) -> bool {
        self.home().join("config.toml").exists()
    }

    /// Check if the skills directory exists
    pub fn skills_dir_exists(&self) -> bool {
        self.home().join("skills").exists()
    }

    /// Get the path to an export file
    pub fn export_path(&self, name: &str) -> PathBuf {
        self.temp_dir.path().join(name)
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

/// Get a fresh Command for the csm binary (without test env)
pub fn csm_cmd() -> Command {
    Command::cargo_bin("csm").expect("Failed to find csm binary")
}
