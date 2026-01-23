# Claude Skill Manager (CSM)

A powerful CLI and TUI application for managing Claude AI skills across global and project-level configurations.

## The Problem

Managing Claude skills today is a frustrating experience:

- **Scattered Configuration**: Skills are split between global (`~/.claude/CLAUDE.md`) and local project files with no unified management
- **Manual Updates**: When a skill you're using gets updated on GitHub, you have to manually check and copy changes
- **No Version Control**: Skills from external sources aren't tracked - you don't know what version you have or when it changed
- **Conflict Blindness**: When multiple skills give contradictory instructions, there's no way to detect or resolve conflicts
- **Copy-Paste Workflow**: Adding new skills means manually finding, copying, and pasting markdown content
- **No Organization**: As your skill collection grows, there's no way to search, categorize, or prioritize them

## The Solution

Claude Skill Manager (CSM) provides a unified interface to:

- **Centralize Management**: One tool to manage all your skills, both global and project-specific
- **Auto-Update Skills**: Skills from GitHub or URLs can automatically update when their source changes
- **Track Sources**: Know exactly where each skill came from and what version you have
- **Detect Conflicts**: Automatically identify when skills contradict each other
- **Simple Installation**: Add skills with a single command from GitHub, URLs, or local files
- **Smart Organization**: Search, filter, enable/disable, and prioritize your skills

## Features

### Skill Sources

| Source | Example | Auto-Update |
|--------|---------|-------------|
| GitHub | `github:anthropics/claude-skills/typescript` | Yes |
| URL | `https://example.com/skills/rust.md` | Yes |
| Local File | `./my-skills/coding-standards.md` | No |
| Inline | Created directly in CSM | No |

### Core Functionality

- **Add Skills** - Install skills from GitHub repos, URLs, or local files
- **Remove Skills** - Clean removal with automatic CLAUDE.md regeneration
- **Enable/Disable** - Toggle skills without removing them
- **List & Search** - Find skills by name, tags, or description
- **Update Management** - Check for and apply updates (auto, notify, or manual modes)
- **Conflict Detection** - Identify duplicate or contradictory instructions
- **Merge Output** - Automatically combine enabled skills into CLAUDE.md

### Scope Management

```
Global Skills (~/.claude/CLAUDE.md)
├── Applied to all Claude sessions
└── Managed in ~/.csm/

Project Skills (.claude/CLAUDE.md)
├── Applied only to specific project
└── Can override or extend global skills
```

### Update Modes

| Mode | Behavior |
|------|----------|
| `auto` | Automatically apply updates (default) |
| `notify` | Alert when updates available, don't auto-apply |
| `manual` | Never check for updates automatically |

## Installation

### Prerequisites

- Rust 1.70+ and Cargo
- SQLite3 development libraries
- OpenSSL development libraries (for HTTPS)

#### Ubuntu/Debian

```bash
sudo apt-get install build-essential libsqlite3-dev libssl-dev pkg-config
```

#### macOS

```bash
brew install sqlite openssl
```

#### Fedora/RHEL

```bash
sudo dnf install sqlite-devel openssl-devel
```

### From Source

```bash
# Clone the repository
git clone https://github.com/anthropics/claude-skills.git
cd claude-skills/skill-manager

# Build and install
cargo install --path .

# Verify installation
csm --version
```

### From Cargo (Coming Soon)

```bash
cargo install claude-skill-manager
```

### From Homebrew (Coming Soon)

```bash
brew install claude-skill-manager
```

## Quick Start

```bash
# Initialize CSM (creates ~/.csm directory)
csm init

# Add a skill from GitHub
csm add github:anthropics/claude-skills/typescript

# Add a skill from URL
csm add https://example.com/skills/rust-best-practices.md

# Add a local skill
csm add ./my-team-standards.md --name team-standards

# List all skills
csm list

# Disable a skill temporarily
csm disable typescript

# Check for updates
csm update --check

# Apply all updates
csm update

# Launch interactive TUI
csm tui
```

## CLI Reference

```
csm <COMMAND>

Commands:
  init      Initialize CSM in current directory or globally
  add       Add a skill from a source
  remove    Remove a skill
  list      List all skills
  enable    Enable a disabled skill
  disable   Disable a skill
  update    Check for and apply updates
  merge     Rebuild CLAUDE.md from enabled skills
  conflicts Detect and manage skill conflicts
  config    View or modify configuration
  tui       Launch interactive terminal UI

Options:
  -h, --help     Print help
  -V, --version  Print version
  --json         Output in JSON format
```

### Common Examples

```bash
# Add skill with custom name and priority
csm add github:owner/repo/path --name my-skill --priority 100

# Add as project-local skill (not global)
csm add ./skill.md --local

# List only enabled skills
csm list --enabled

# List skills in JSON format
csm list --json

# Search for skills
csm list --search "typescript"

# Update specific skill
csm update typescript-best

# Set skill to manual update mode
csm config set skills.typescript-best.update_mode manual

# Detect conflicts between skills
csm conflicts detect

# View unresolved conflicts
csm conflicts list
```

## Configuration

CSM stores configuration in `~/.csm/config.toml`:

```toml
[general]
auto_update = true
update_interval = "24h"
default_scope = "global"

[github]
# Optional: GitHub token for higher rate limits
token = "ghp_..."

[output]
# Include source comments in merged output
include_sources = true
# Include generation timestamp
include_timestamp = true
```

## File Structure

```
~/.csm/
├── config.toml      # Global configuration
├── skills.db        # SQLite database of skills
├── skills/          # Cached skill content
│   ├── {uuid}.md
│   └── ...
└── cache/           # Temporary files

~/.claude/
└── CLAUDE.md        # Generated global skills file

./project/
└── .claude/
    └── CLAUDE.md    # Generated project skills file
```

## Design Documentation

For detailed design specifications, see the [docs/](docs/) directory:

| Document | Description |
|----------|-------------|
| [Press Release](docs/01-PRESS-RELEASE.md) | Product announcement and vision |
| [FAQ](docs/02-FAQ.md) | Frequently asked questions |
| [Feature Spec](docs/design/03-FEATURE-SPECIFICATION.md) | Complete feature specifications |
| [CLI Spec](docs/design/04-CLI-SPECIFICATION.md) | CLI interface design |
| [TUI Spec](docs/design/05-TUI-SPECIFICATION.md) | TUI wireframes and flows |
| [Use Cases](docs/design/09-USE-CASES.md) | Use case diagrams |
| [Architecture](docs/architecture/08-TECHNICAL-ARCHITECTURE.md) | Technical architecture |
| [Test Plan](docs/testing/TEST-PLAN.md) | Testing strategy |

## Development

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run with debug logging
RUST_LOG=debug cargo run -- list

# Build release binary
cargo build --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Technology Stack

- **Language**: Rust 2021 edition
- **CLI**: clap v4
- **TUI**: ratatui + crossterm
- **Database**: SQLite (rusqlite)
- **Git**: git2 (libgit2)
- **HTTP**: reqwest + tokio
- **Serialization**: serde (JSON, TOML)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass (`cargo test`)
5. Submit a pull request

See the [testing documentation](docs/testing/TEST-PLAN.md) for testing guidelines.
