# Contributing

Thank you for your interest in contributing to Claude Skill Manager! This guide will help you get started.

## Getting Started

### Prerequisites

- **Rust 1.75+**: Install via [rustup](https://rustup.rs/)
- **Git**: For version control
- **SQLite**: Usually included with the OS

### Clone the Repository

```bash
git clone https://github.com/anthropics/claude-skill-manager.git
cd claude-skill-manager
```

### Development Environment

#### Using Nix (Recommended)

If you have Nix with flakes enabled:

```bash
nix develop
```

This provides a complete development environment with all dependencies.

#### Manual Setup

```bash
# Install Rust toolchain
rustup install stable
rustup default stable

# Install development tools
rustup component add clippy rustfmt

# Build the project
cargo build

# Run tests
cargo test
```

## Project Structure

```
skill-manager/
├── Cargo.toml           # Package manifest
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library exports
│   ├── domain/          # Domain models
│   ├── services/        # Business logic
│   ├── infra/           # Infrastructure
│   ├── cli/             # CLI interface
│   ├── tui/             # TUI interface
│   └── utils/           # Utilities
├── tests/               # Integration tests
├── design-docs/         # Design documentation
└── docs/                # User documentation
    └── sphinx/          # Sphinx docs
```

## Development Workflow

### 1. Create a Branch

```bash
# Create a feature branch
git checkout -b feature/my-feature

# Or a bugfix branch
git checkout -b fix/my-bugfix
```

### 2. Make Changes

Write your code following the project conventions (see below).

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for a specific module
cargo test services::
```

### 4. Check Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check for security issues
cargo audit
```

### 5. Commit Your Changes

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```bash
# Feature
git commit -m "feat: add skill versioning support"

# Bug fix
git commit -m "fix: resolve symlink creation on Windows"

# Documentation
git commit -m "docs: update installation guide"

# Refactoring
git commit -m "refactor: simplify conflict detection algorithm"

# Tests
git commit -m "test: add integration tests for GitHub client"

# Chores
git commit -m "chore: update dependencies"
```

### 6. Push and Create PR

```bash
git push origin feature/my-feature
```

Then create a Pull Request on GitHub.

## Code Conventions

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Address all `clippy` warnings

### Module Organization

```rust
// Good: Clear module structure
mod domain {
    mod skill;
    mod source;
    mod conflict;

    pub use skill::*;
    pub use source::*;
    pub use conflict::*;
}

// Good: Explicit imports
use crate::domain::{Skill, SkillSource};
use crate::services::SkillService;
```

### Error Handling

```rust
// Good: Use the custom Error type
use crate::utils::error::{Error, Result};

fn fetch_skill(source: &str) -> Result<Skill> {
    let content = fetch_content(source)
        .map_err(|e| Error::FetchFailed(e.to_string()))?;

    parse_skill(&content)
        .map_err(|e| Error::InvalidContent(e.to_string()))
}
```

### Documentation

```rust
/// Fetches a skill from the specified source.
///
/// # Arguments
///
/// * `source` - The source specification (e.g., "github:owner/repo")
///
/// # Returns
///
/// The fetched skill, or an error if fetching fails.
///
/// # Examples
///
/// ```rust
/// let skill = service.fetch("github:user/repo").await?;
/// println!("Fetched: {}", skill.name);
/// ```
pub async fn fetch(&self, source: &str) -> Result<Skill> {
    // Implementation
}
```

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_creation() {
        let skill = Skill::new("test-skill", SkillSource::Inline);
        assert_eq!(skill.name, "test-skill");
        assert!(skill.enabled);
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

## Adding Features

### Adding a New Command

1. **Define the command in `cli/mod.rs`**:

```rust
#[derive(Subcommand, Debug)]
pub enum Commands {
    // ... existing commands ...

    /// Your new command description
    NewCommand {
        #[arg(help = "Argument description")]
        arg: String,

        #[arg(long, help = "Optional flag")]
        flag: bool,
    },
}
```

2. **Create the command handler in `cli/commands/`**:

```rust
// cli/commands/new_command.rs

use crate::utils::error::Result;

pub async fn execute(arg: &str, flag: bool) -> Result<()> {
    // Implementation
    Ok(())
}
```

3. **Wire it up in `main.rs`**:

```rust
Commands::NewCommand { arg, flag } => {
    cli::commands::new_command::execute(&arg, flag).await
}
```

### Adding a New Source Type

1. **Extend `SkillSource` in `domain/source.rs`**:

```rust
pub enum SkillSource {
    // ... existing variants ...

    /// GitLab repository
    GitLab {
        host: String,
        owner: String,
        repo: String,
        path: Option<String>,
        ref_spec: Option<String>,
    },
}
```

2. **Add the parser in `domain/source.rs`**:

```rust
impl SkillSource {
    pub fn parse(source: &str) -> Result<Self> {
        if source.starts_with("gitlab:") {
            return Self::parse_gitlab(source);
        }
        // ... existing parsers ...
    }

    fn parse_gitlab(source: &str) -> Result<Self> {
        // Implementation
    }
}
```

3. **Create the client in `infra/`**:

```rust
// infra/gitlab.rs

pub struct GitLabClientImpl {
    client: Client,
    // ...
}

#[async_trait]
impl GitLabClient for GitLabClientImpl {
    async fn fetch(&self, ...) -> Result<FetchResult> {
        // Implementation
    }
}
```

### Adding Tests

#### Unit Tests

Add tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

#### Integration Tests

Add tests in the `tests/` directory:

```rust
// tests/integration_test.rs

use csm::prelude::*;

#[tokio::test]
async fn test_full_workflow() {
    // Setup
    let temp_dir = tempfile::tempdir().unwrap();

    // Test
    // ...

    // Cleanup happens automatically
}
```

## Documentation

### Code Documentation

- Document all public items with doc comments
- Include examples where helpful
- Keep comments up to date with code changes

### User Documentation

User documentation lives in `docs/sphinx/`:

- Write in Markdown (MyST flavor)
- Include practical examples
- Keep language clear and concise

#### Building Documentation

```bash
cd docs/sphinx
pip install -r requirements.txt
make html
```

View the built docs at `docs/sphinx/_build/html/index.html`.

## Pull Request Guidelines

### Before Submitting

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation is updated if needed
- [ ] Commit messages follow conventions

### PR Description

Include in your PR description:

1. **What**: Brief description of the change
2. **Why**: Motivation for the change
3. **How**: High-level approach taken
4. **Testing**: How you tested the change

### Review Process

1. Automated checks run (tests, linting)
2. Maintainer reviews the code
3. Address feedback if any
4. Merge once approved

## Reporting Issues

### Bug Reports

Include:

- CSM version (`csm --version`)
- Operating system and version
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs (with `--verbose`)

### Feature Requests

Include:

- Clear description of the feature
- Use case / motivation
- Proposed implementation (optional)
- Alternative solutions considered

## Community

### Getting Help

- Open a [Discussion](https://github.com/anthropics/claude-skill-manager/discussions) for questions
- Check existing [Issues](https://github.com/anthropics/claude-skill-manager/issues) for known problems

### Code of Conduct

Be respectful and constructive. We're all here to make CSM better together.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
