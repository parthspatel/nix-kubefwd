# Claude Skill Manager (CSM)

A powerful CLI and TUI application for managing Claude AI skills across projects and workstations.

## What is Claude Skill Manager?

Claude Skill Manager (CSM) is a unified tool that transforms how developers manage Claude AI customizations (skills). It eliminates the fragmented, error-prone experience of manually managing `CLAUDE.md` files and custom instructions.

### The Problem

Developers using Claude Code face significant friction when managing skills:

- **Fragmentation**: Skills are scattered across `~/.claude/CLAUDE.md` (global), project-level `CLAUDE.md`, and `.claude/` directories
- **No Version Control**: Skills aren't tracked, leading to lost customizations and inconsistent behavior
- **Manual Sync**: Keeping skills synchronized across projects requires tedious copy-paste operations
- **No Sharing**: Discovering and adopting community-created skills requires manual effort
- **Update Fatigue**: Skills from GitHub repos must be manually checked and updated

### The Solution

CSM provides a centralized registry and management system for all your Claude skills:

- **Single Source of Truth**: All skills tracked in one place with a SQLite registry
- **Automated Updates**: Skills sync with their sources automatically
- **Easy Sharing**: Install skills from GitHub with a single command
- **Visual Management**: Interactive TUI for browsing and configuring skills
- **Conflict Resolution**: Automatic detection and resolution of conflicting instructions

## Features & Functionality

### Core Features

| Feature | Description |
|---------|-------------|
| **Skill Registry** | Central SQLite database tracking all installed skills with metadata |
| **GitHub Integration** | Install skills directly from GitHub repositories with auto-updates |
| **Global/Local Management** | Define skills globally or per-project with clear precedence rules |
| **Symlink Architecture** | Efficient storage with symlinks preventing duplication |
| **Conflict Detection** | Automatic identification of overlapping or contradictory instructions |
| **CLI Interface** | Complete command-line interface with intuitive commands |
| **Interactive TUI** | Visual terminal interface for skill management |

### Skill Sources

CSM supports installing skills from multiple sources:

```bash
# GitHub repository
csm add github:anthropics/claude-skills/typescript

# Specific branch or tag
csm add github:user/repo@v1.0.0

# Local file
csm add ./my-skill.md

# Direct URL
csm add https://example.com/skill.md
```

### Update Modes

Control how skills stay synchronized with their sources:

| Mode | Behavior |
|------|----------|
| `auto` | Check and update automatically (default) |
| `notify` | Check and notify, user confirms update |
| `manual` | Only update on explicit request |

### Scope Management

Skills can be applied at different levels:

- **Global Skills** (`~/.claude/`): Apply to all Claude sessions
- **Local Skills** (`./.claude/`): Apply only to the current project
- Local skills take precedence over global skills

### Conflict Detection

CSM automatically detects:

- Duplicate definitions across skills
- Contradictory instructions
- Precedence ambiguity
- Syntax conflicts

## Installation

### Prerequisites

- macOS (x86_64, ARM64), Linux (x86_64, ARM64), or Windows 10+
- Git (for GitHub integration)
- Rust 1.75+ (for building from source)

### Using Cargo

```bash
cargo install claude-skill-manager
```

### Using Homebrew (macOS/Linux)

```bash
brew install csm
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/anthropics/claude-skill-manager.git
cd claude-skill-manager

# Build with cargo
cargo build --release

# Install locally
cargo install --path .
```

### Using Nix

If you have Nix with flakes enabled:

```bash
# Enter development shell
cd skill-manager
nix develop

# Build
cargo build --release
```

## Quick Start

### Initialize CSM

```bash
# Initialize with interactive prompts
csm init

# Initialize and import existing CLAUDE.md files
csm init --import-existing
```

### Add Skills

```bash
# Add a skill from GitHub
csm add github:anthropics/claude-skills/typescript-best-practices

# Add with custom name and global scope
csm add github:user/repo --name "my-ts-skill" --scope global
```

### List and Manage Skills

```bash
# List all skills
csm list

# Show skill details
csm show typescript-best

# Enable/disable skills
csm enable typescript-best
csm disable experimental-skill
```

### Update Skills

```bash
# Check for updates
csm update --check

# Update all skills
csm update

# Update specific skill
csm update typescript-best
```

### Launch TUI

```bash
# Open interactive interface
csm ui

# Open to specific section
csm ui --section skills
```

## CLI Reference

### Commands

| Command | Description |
|---------|-------------|
| `csm init` | Initialize CSM configuration |
| `csm add <source>` | Add a skill from various sources |
| `csm remove <skill>` | Remove a skill |
| `csm list` | List installed skills |
| `csm show <skill>` | Show skill details |
| `csm enable <skill>` | Enable a skill |
| `csm disable <skill>` | Disable a skill |
| `csm update [skill]` | Update skills from sources |
| `csm sync` | Synchronize skill state |
| `csm conflicts` | Detect and resolve conflicts |
| `csm search <query>` | Search for skills |
| `csm config` | Manage configuration |
| `csm create <name>` | Create a new skill |
| `csm edit <skill>` | Edit a skill |
| `csm export` | Export skills |
| `csm import <source>` | Import skills |
| `csm ui` | Launch interactive TUI |
| `csm doctor` | Diagnose and repair issues |

### Global Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Increase verbosity |
| `-q, --quiet` | Suppress non-essential output |
| `-y, --yes` | Auto-confirm all prompts |
| `--json` | Output in JSON format |
| `--config <path>` | Use custom config file |

## Configuration

CSM stores its configuration at `~/.csm/config.toml`:

```toml
[general]
default_scope = "local"
editor = "vim"
color = true

[updates]
mode = "auto"
schedule = "daily"
check_on_startup = true

[github]
default_ref = "main"

[ui]
theme = "dark"
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `CSM_CONFIG` | Path to config file |
| `CSM_HOME` | CSM data directory (default: `~/.csm`) |
| `GITHUB_TOKEN` | GitHub API token for private repos |
| `EDITOR` | Default editor for `csm edit` |

## Directory Structure

```
~/.csm/
├── config.toml          # Global configuration
├── registry.db          # SQLite skill database
├── skills/              # Master skill storage
│   └── <skill-name>/
│       ├── CLAUDE.md    # Skill content
│       └── .meta.toml   # Skill metadata
├── cache/               # Download cache
└── logs/                # Operation logs
```

## Documentation

For detailed documentation, see the [docs/](./docs/) directory:

- [Getting Started](./docs/sphinx/getting-started.md)
- [CLI Reference](./docs/sphinx/cli.md)
- [Configuration Guide](./docs/sphinx/configuration.md)
- [Architecture](./docs/sphinx/architecture.md)
- [API Reference](./docs/sphinx/api-reference.md)
- [Contributing](./docs/sphinx/contributing.md)

## Technology Stack

- **Language**: Rust (2021 edition)
- **CLI**: clap v4
- **TUI**: ratatui + crossterm
- **Database**: SQLite (rusqlite)
- **Git**: git2 (libgit2)
- **HTTP**: reqwest + tokio
- **Serialization**: serde + serde_json/toml

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./docs/sphinx/contributing.md) for guidelines.

```bash
# Clone and setup
git clone https://github.com/anthropics/claude-skill-manager.git
cd claude-skill-manager

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- list
```

## License

MIT License - See [LICENSE](./LICENSE) file for details.
