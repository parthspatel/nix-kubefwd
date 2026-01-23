# Getting Started

This guide walks you through installing and setting up Claude Skill Manager (CSM) for the first time.

## Prerequisites

Before installing CSM, ensure you have:

- **Operating System**: macOS 11+, Linux (glibc 2.17+), or Windows 10+
- **Git**: Required for GitHub integration
- **Rust 1.75+**: Required if building from source

## Installation

### Using Cargo (Recommended)

The easiest way to install CSM is via Cargo:

```bash
cargo install claude-skill-manager
```

This installs the `csm` binary to your Cargo bin directory (typically `~/.cargo/bin/`).

### Using Homebrew (macOS/Linux)

```bash
brew install csm
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/anthropics/claude-skill-manager.git
cd claude-skill-manager

# Build in release mode
cargo build --release

# Install to your path
cargo install --path .
```

### Using Nix

If you use Nix with flakes:

```bash
cd skill-manager
nix develop
cargo build --release
```

### Verify Installation

Confirm CSM is installed correctly:

```bash
csm --version
# Output: csm 0.1.0
```

## Initial Setup

### Initialize CSM

Run the initialization command to set up CSM:

```bash
csm init
```

This creates the following directory structure:

```
~/.csm/
├── config.toml      # Global configuration
├── registry.db      # SQLite skill database
├── skills/          # Master skill storage
├── cache/           # Download cache
└── logs/            # Operation logs
```

### Import Existing Skills

If you have existing `CLAUDE.md` files, import them during initialization:

```bash
csm init --import-existing
```

CSM will scan for:
- `~/.claude/CLAUDE.md` (global skills)
- `./CLAUDE.md` in your current directory (local skills)
- `./.claude/` directories

### Initialize a Project

To set up skill management for a specific project:

```bash
cd /path/to/your/project
csm init --local
```

This creates a `.csm/` directory in your project for local skill management.

## Adding Your First Skill

### From GitHub

Install a skill from a GitHub repository:

```bash
csm add github:anthropics/claude-skills/typescript-best-practices
```

CSM will:
1. Fetch the skill from GitHub
2. Validate the content
3. Register it in the database
4. Create appropriate symlinks

### From a Local File

Add a skill from a local file:

```bash
csm add ./my-custom-skill.md --name "my-skill"
```

### From a URL

Install a skill from any URL:

```bash
csm add https://example.com/path/to/skill.md
```

## Viewing Skills

### List All Skills

```bash
csm list
```

Output:
```
┌─────────────────────┬──────────┬─────────┬────────────────────────────┐
│ Name                │ Scope    │ Status  │ Source                     │
├─────────────────────┼──────────┼─────────┼────────────────────────────┤
│ typescript-best     │ global   │ enabled │ github:anthropics/skills   │
│ my-skill            │ local    │ enabled │ local:/path/to/skill       │
└─────────────────────┴──────────┴─────────┴────────────────────────────┘
```

### View Skill Details

```bash
csm show typescript-best
```

Output:
```
Skill: typescript-best
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ID:          a1b2c3d4-e5f6-7890-abcd-ef1234567890
  Scope:       global
  Status:      enabled
  Priority:    100

  Source:      github:anthropics/claude-skills/typescript
  Update Mode: auto
  Last Update: 2026-01-20 14:32:00 UTC
```

## Managing Skills

### Enable/Disable Skills

Toggle skills without removing them:

```bash
# Disable a skill
csm disable typescript-best

# Enable a skill
csm enable typescript-best

# Toggle state
csm toggle typescript-best
```

### Update Skills

Check for and apply updates:

```bash
# Check for available updates
csm update --check

# Update all skills
csm update

# Update specific skill
csm update typescript-best
```

### Remove Skills

```bash
csm remove old-skill
```

## Using the TUI

Launch the interactive terminal interface:

```bash
csm ui
```

Navigate using:
- Arrow keys to move between items
- Enter to select
- `q` to quit
- `?` for help

Jump to specific sections:

```bash
csm ui --section skills
csm ui --section updates
csm ui --section conflicts
```

## Understanding Scopes

CSM supports two skill scopes:

### Global Skills

- Stored in `~/.csm/skills/`
- Apply to all Claude sessions
- Lower precedence than local skills

```bash
csm add github:user/repo --scope global
```

### Local Skills

- Stored in `./.csm/skills/`
- Apply only to the current project
- Override global skills with same name
- Can be committed to version control

```bash
csm add github:user/repo --scope local
```

## Conflict Detection

CSM automatically detects conflicting skill definitions:

```bash
csm conflicts
```

For interactive resolution:

```bash
csm conflicts --resolve
```

## Setting Up GitHub Authentication

For private repositories or to avoid rate limits:

```bash
# Using environment variable
export GITHUB_TOKEN=your_token_here

# Or configure in CSM
csm config set github-token your_token_here
```

## Shell Completions

Generate shell completions for better CLI experience:

```bash
# Bash
csm completions bash > ~/.bash_completion.d/csm

# Zsh
csm completions zsh > ~/.zfunc/_csm

# Fish
csm completions fish > ~/.config/fish/completions/csm.fish
```

## Next Steps

- Read the [Configuration Guide](configuration.md) for customization options
- Explore the [CLI Reference](cli.md) for all available commands
- Learn about the [Architecture](architecture.md) to understand how CSM works
- Check [Contributing](contributing.md) if you want to help improve CSM

## Troubleshooting

### CSM Not Found After Installation

Ensure your Cargo bin directory is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add this to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.).

### Permission Denied Errors

Ensure you have write access to the CSM directories:

```bash
ls -la ~/.csm
```

### GitHub Rate Limiting

If you encounter rate limit errors, set up GitHub authentication as described above.

### Database Corruption

If the registry database becomes corrupted:

```bash
csm doctor --fix
```

This will attempt to repair common issues automatically.
