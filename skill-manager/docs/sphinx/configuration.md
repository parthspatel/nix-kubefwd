# Configuration

This guide covers all configuration options for Claude Skill Manager.

## Configuration File

CSM stores its configuration at `~/.csm/config.toml`. This file is created during initialization and can be edited manually or via the CLI.

### Default Configuration

```toml
# Claude Skill Manager Configuration

[general]
# Default scope for new skills: "global" or "local"
default_scope = "local"

# Preferred editor for editing skills
# Uses $EDITOR environment variable if not set
editor = ""

# Enable colored output
color = true

[updates]
# Default update mode: "auto", "notify", or "manual"
mode = "auto"

# Update check schedule: "hourly", "daily", or "weekly"
schedule = "daily"

# Check for updates when CSM starts
check_on_startup = true

[github]
# Default branch/ref for GitHub sources
default_ref = "main"

[ui]
# TUI theme: "dark" or "light"
theme = "dark"

# Show welcome screen on first TUI launch
show_welcome = true

[logging]
# Log level: "error", "warn", "info", "debug", "trace"
level = "info"

# Log file location
file = "~/.csm/logs/csm.log"

# Maximum log file size before rotation
max_size = "10MB"
```

## Configuration Sections

### General Settings

#### `default_scope`

The default scope for newly added skills.

| Value | Description |
|-------|-------------|
| `local` | Skills are added to the current project (default) |
| `global` | Skills are added globally |

```bash
# Set via CLI
csm config set default-scope global
```

#### `editor`

The text editor to use for `csm edit` and `csm create --edit`.

```toml
[general]
editor = "vim"         # Use vim
editor = "code"        # Use VS Code
editor = "nano"        # Use nano
editor = ""            # Use $EDITOR environment variable
```

```bash
# Set via CLI
csm config set editor "code --wait"
```

#### `color`

Enable or disable colored output in the terminal.

```toml
[general]
color = true   # Enable colors (default)
color = false  # Disable colors
```

You can also disable colors temporarily:

```bash
csm --no-color list
```

Or via environment variable:

```bash
NO_COLOR=1 csm list
```

### Update Settings

#### `mode`

Controls how skills are updated from their sources.

| Mode | Behavior |
|------|----------|
| `auto` | Automatically check and apply updates (default) |
| `notify` | Check for updates and notify, but don't auto-apply |
| `manual` | Only check/update when explicitly requested |

```bash
# Set globally
csm config set update-mode notify

# Set per-skill
csm update-mode my-skill manual
```

#### `schedule`

How often to check for updates (when mode is `auto` or `notify`).

| Schedule | Behavior |
|----------|----------|
| `hourly` | Check every hour |
| `daily` | Check once per day (default) |
| `weekly` | Check once per week |

```bash
csm config set update-schedule weekly
```

#### `check_on_startup`

Whether to check for updates when CSM starts.

```toml
[updates]
check_on_startup = true   # Check on startup (default)
check_on_startup = false  # Don't check on startup
```

### GitHub Settings

#### `default_ref`

The default branch or ref to use when no ref is specified in a GitHub source.

```toml
[github]
default_ref = "main"    # Use main branch (default)
default_ref = "master"  # Use master branch
```

#### Authentication

GitHub authentication is handled via environment variable or system keychain:

```bash
# Via environment variable
export GITHUB_TOKEN=ghp_xxxxxxxxxxxx

# Via CSM config (stored in system keychain if available)
csm config set github-token ghp_xxxxxxxxxxxx
```

### UI Settings

#### `theme`

The color theme for the TUI.

| Theme | Description |
|-------|-------------|
| `dark` | Dark background with light text (default) |
| `light` | Light background with dark text |

```bash
csm config set theme light
```

#### `show_welcome`

Whether to show the welcome screen on first TUI launch.

```toml
[ui]
show_welcome = true   # Show welcome (default)
show_welcome = false  # Skip welcome screen
```

### Logging Settings

#### `level`

The logging verbosity level.

| Level | Description |
|-------|-------------|
| `error` | Only errors |
| `warn` | Errors and warnings |
| `info` | Normal operation messages (default) |
| `debug` | Detailed debugging information |
| `trace` | Very verbose tracing |

```bash
# Set in config
csm config set log-level debug

# Or via environment variable
CSM_LOG_LEVEL=debug csm list
```

#### `file`

Location of the log file.

```toml
[logging]
file = "~/.csm/logs/csm.log"
```

#### `max_size`

Maximum size of the log file before rotation.

```toml
[logging]
max_size = "10MB"   # Default
max_size = "100MB"  # For extensive debugging
```

## Environment Variables

Environment variables override configuration file settings:

| Variable | Description | Example |
|----------|-------------|---------|
| `CSM_CONFIG` | Path to config file | `~/.csm/custom-config.toml` |
| `CSM_HOME` | CSM data directory | `~/.csm` |
| `CSM_LOG_LEVEL` | Log level | `debug` |
| `GITHUB_TOKEN` | GitHub API token | `ghp_xxx` |
| `NO_COLOR` | Disable colored output | `1` |
| `EDITOR` | Default editor | `vim` |

### Precedence

Configuration values are resolved in this order (highest to lowest priority):

1. Command-line flags (e.g., `--verbose`)
2. Environment variables
3. Configuration file
4. Default values

## Project-Level Configuration

Projects can have their own configuration at `.csm/config.toml`:

```
my-project/
├── .csm/
│   ├── config.toml    # Project-specific config
│   └── skills/        # Project skills
└── CLAUDE.md          # Merged output
```

### Project Configuration File

```toml
# .csm/config.toml

[general]
# Override default scope for this project
default_scope = "local"

[skills]
# Skills specific to this project
required = ["typescript-best", "react-patterns"]

[updates]
# Project might want different update behavior
mode = "manual"  # Don't auto-update in this project
```

### Configuration Inheritance

Project configuration inherits from global configuration:

```
Global (~/.csm/config.toml)
    └── Project (.csm/config.toml)
        └── Command-line flags
```

## Managing Configuration

### View Configuration

```bash
# List all configuration
csm config list

# Get specific value
csm config get update-mode
csm config get general.editor
```

### Set Configuration

```bash
# Set a value
csm config set update-mode notify
csm config set editor "code --wait"

# Set nested values
csm config set logging.level debug
```

### Edit Configuration

```bash
# Open config in editor
csm config edit
```

### Reset Configuration

```bash
# Reset to defaults
csm config reset

# Reset specific section
csm config reset updates
```

## Skill-Level Configuration

Individual skills can have their own settings in `.meta.toml`:

```toml
# ~/.csm/skills/<uuid>/.meta.toml

[skill]
priority = 100
update_mode = "notify"
tags = ["typescript", "coding"]

[source]
# Source-specific metadata
last_check = "2026-01-20T14:32:00Z"
commit_sha = "abc1234"
```

### Setting Skill Configuration

```bash
# Set skill priority
csm priority set my-skill 100

# Set skill update mode
csm update-mode my-skill manual

# Add tags
csm tag add my-skill typescript coding
```

## Configuration Examples

### Development Environment

For active development with frequent changes:

```toml
[general]
default_scope = "local"
editor = "code --wait"
color = true

[updates]
mode = "notify"
schedule = "hourly"
check_on_startup = true

[logging]
level = "debug"
```

### Production/Stable Environment

For stable environments with controlled updates:

```toml
[general]
default_scope = "global"
editor = "vim"
color = true

[updates]
mode = "manual"
schedule = "weekly"
check_on_startup = false

[logging]
level = "warn"
```

### CI/CD Environment

For automated environments:

```toml
[general]
default_scope = "local"
color = false  # Disable for cleaner logs

[updates]
mode = "manual"  # Only explicit updates
check_on_startup = false

[logging]
level = "info"
file = "/var/log/csm.log"
```

## Troubleshooting

### Configuration Not Loading

Check the configuration file path:

```bash
csm config list
# Shows: Config file: /path/to/.csm/config.toml
```

Verify the file is valid TOML:

```bash
cat ~/.csm/config.toml
```

### Environment Variables Not Working

Verify the variable is set:

```bash
echo $GITHUB_TOKEN
echo $CSM_LOG_LEVEL
```

Check for typos in variable names (they are case-sensitive).

### Permissions Issues

Ensure you have write access:

```bash
ls -la ~/.csm/config.toml
```

### Reset Corrupted Configuration

If the configuration file becomes corrupted:

```bash
# Backup current config
cp ~/.csm/config.toml ~/.csm/config.toml.bak

# Reset to defaults
csm config reset

# Or manually recreate
rm ~/.csm/config.toml
csm init
```
