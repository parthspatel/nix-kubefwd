# CLI Reference

Complete reference for all Claude Skill Manager commands.

## Command Structure

```
csm [OPTIONS] <COMMAND> [SUBCOMMAND] [ARGS]
```

## Global Options

These options can be used with any command:

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Show help information |
| `--version` | `-V` | Show version information |
| `--verbose` | `-v` | Increase verbosity (stackable: `-vvv`) |
| `--quiet` | `-q` | Suppress non-essential output |
| `--config <PATH>` | `-c` | Use custom config file |
| `--no-color` | | Disable colored output |
| `--json` | | Output in JSON format |
| `--yes` | `-y` | Auto-confirm all prompts |

## Commands

### csm init

Initialize CSM configuration.

```bash
csm init [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--global` | Initialize global configuration only |
| `--local` | Initialize local project only |
| `--force` | Overwrite existing configuration |
| `--import-existing` | Import existing CLAUDE.md files |

**Examples:**

```bash
# Interactive initialization
csm init

# Setup global config only
csm init --global

# Setup project config only
csm init --local

# Import existing skills
csm init --import-existing
```

---

### csm add

Add a skill from various sources.

```bash
csm add <SOURCE> [OPTIONS]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<SOURCE>` | Skill source (github:owner/repo, path, URL) |

**Options:**

| Option | Description |
|--------|-------------|
| `--name <NAME>` | Custom name for the skill |
| `--scope <SCOPE>` | Scope: `global`, `local` (default: local) |
| `--enable` | Enable immediately (default: true) |
| `--no-enable` | Add but don't enable |
| `--update-mode <MODE>` | Update mode: `auto`, `notify`, `manual` |

**Source Formats:**

```bash
# GitHub repository
github:owner/repo

# Specific path within repo
github:owner/repo/path/to/skill

# Specific ref (branch/tag/commit)
github:owner/repo@main
github:owner/repo@v1.0.0
github:owner/repo@abc1234

# Local file path
/path/to/skill.md
./relative/path/skill.md

# URL
https://example.com/skill.md
```

**Examples:**

```bash
# Add from GitHub
csm add github:anthropics/claude-skills/typescript

# Add with specific version
csm add github:user/repo@v1.0.0 --scope global

# Add local file with custom name
csm add ./my-skill.md --name "My Custom Skill"

# Add from URL
csm add https://gist.github.com/user/abc123/raw/skill.md
```

---

### csm remove

Remove a skill from the registry.

```bash
csm remove <SKILL> [OPTIONS]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<SKILL>` | Skill name or ID |

**Options:**

| Option | Description |
|--------|-------------|
| `--force` | Remove without confirmation |
| `--keep-files` | Remove from registry but keep files |

**Examples:**

```bash
csm remove typescript-best
csm remove --force old-skill
csm remove my-skill --keep-files
```

---

### csm list

List installed skills.

```bash
csm list [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--scope <SCOPE>` | Filter by scope: `all`, `global`, `local` (default: all) |
| `--enabled` | Show only enabled skills |
| `--disabled` | Show only disabled skills |
| `--source <SOURCE>` | Filter by source type: `github`, `local`, `url` |
| `--format <FORMAT>` | Output format: `table`, `json`, `yaml` (default: table) |
| `--verbose` | Show additional details |

**Examples:**

```bash
# List all skills
csm list

# List global skills only
csm list --scope global

# List enabled skills as JSON
csm list --enabled --json

# Verbose output
csm list -v
```

---

### csm show

Show detailed information about a skill.

```bash
csm show <SKILL> [OPTIONS]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<SKILL>` | Skill name or ID |

**Options:**

| Option | Description |
|--------|-------------|
| `--content` | Show full skill content |
| `--metadata` | Show metadata only |
| `--format <FORMAT>` | Output format: `text`, `json`, `yaml` |

**Examples:**

```bash
csm show typescript-best
csm show typescript-best --content
csm show typescript-best --json
```

---

### csm enable / csm disable / csm toggle

Toggle skill state.

```bash
csm enable <SKILL>
csm disable <SKILL>
csm toggle <SKILL>
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<SKILL>` | Skill name or ID |

**Examples:**

```bash
csm enable typescript-best
csm disable experimental
csm toggle debugging-verbose
```

---

### csm update

Update skills from their sources.

```bash
csm update [SKILL] [OPTIONS]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `[SKILL]` | Specific skill to update (optional, updates all if omitted) |

**Options:**

| Option | Description |
|--------|-------------|
| `--check` | Check for updates without applying |
| `--force` | Update even if no changes detected |
| `--dry-run` | Show what would be updated |

**Examples:**

```bash
# Update all skills
csm update

# Update specific skill
csm update typescript-best

# Check for available updates
csm update --check

# Preview updates
csm update --dry-run
```

---

### csm sync

Synchronize skill state.

```bash
csm sync [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--rebuild` | Force rebuild of merged CLAUDE.md files |
| `--verify` | Verify symlink integrity |
| `--fix` | Fix broken symlinks and references |

**Examples:**

```bash
csm sync
csm sync --rebuild
csm sync --verify --fix
```

---

### csm conflicts

Detect and resolve skill conflicts.

```bash
csm conflicts [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--resolve` | Interactive conflict resolution |
| `--format <FORMAT>` | Output format: `text`, `json` |

**Examples:**

```bash
csm conflicts
csm conflicts --resolve
csm conflicts --json
```

---

### csm search

Search for skills.

```bash
csm search <QUERY> [OPTIONS]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<QUERY>` | Search query |

**Options:**

| Option | Description |
|--------|-------------|
| `--installed` | Search installed skills only |
| `--tag <TAG>` | Filter by tag |
| `--limit <N>` | Limit results (default: 20) |

**Examples:**

```bash
csm search typescript
csm search "error handling" --installed
csm search --tag python
```

---

### csm config

Manage configuration.

```bash
csm config <SUBCOMMAND>
```

**Subcommands:**

| Subcommand | Description |
|------------|-------------|
| `get <KEY>` | Get configuration value |
| `set <KEY> <VALUE>` | Set configuration value |
| `list` | List all configuration |
| `edit` | Open config in editor |
| `reset` | Reset to defaults |

**Configuration Keys:**

| Key | Description | Values |
|-----|-------------|--------|
| `update-mode` | Default update mode | `auto`, `notify`, `manual` |
| `update-schedule` | Update check schedule | `hourly`, `daily`, `weekly` |
| `default-scope` | Default skill scope | `global`, `local` |
| `editor` | Preferred editor | Any editor command |
| `color` | Enable colored output | `true`, `false` |
| `github-token` | GitHub API token | Token string |

**Examples:**

```bash
csm config get update-mode
csm config set update-mode notify
csm config set default-scope global
csm config list
csm config edit
```

---

### csm create

Create a new skill.

```bash
csm create <NAME> [OPTIONS]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<NAME>` | Name for the new skill |

**Options:**

| Option | Description |
|--------|-------------|
| `--from <PATH>` | Create from existing file |
| `--template <NAME>` | Use a template: `basic`, `detailed`, `coding` |
| `--scope <SCOPE>` | Scope: `global`, `local` (default: local) |
| `--edit` | Open in editor after creation |

**Examples:**

```bash
csm create my-skill
csm create my-skill --from ./existing.md
csm create coding-style --template coding --scope global
csm create quick-skill --edit
```

---

### csm edit

Edit a skill.

```bash
csm edit <SKILL> [OPTIONS]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<SKILL>` | Skill name or ID |

**Options:**

| Option | Description |
|--------|-------------|
| `--editor <EDITOR>` | Use specific editor |

**Examples:**

```bash
csm edit typescript-best
csm edit my-skill --editor code
```

---

### csm export

Export skills.

```bash
csm export [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--all` | Export all skills |
| `--skill <SKILL>` | Export specific skill |
| `--scope <SCOPE>` | Export by scope |
| `--format <FORMAT>` | Format: `json`, `toml`, `archive` (default: json) |
| `--output <PATH>` | Output file (default: stdout) |

**Examples:**

```bash
csm export --all > backup.json
csm export --skill typescript-best --format toml
csm export --scope global --format archive -o global-skills.tar.gz
```

---

### csm import

Import skills.

```bash
csm import <SOURCE> [OPTIONS]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<SOURCE>` | File path or URL |

**Options:**

| Option | Description |
|--------|-------------|
| `--merge` | Merge with existing skills |
| `--replace` | Replace existing skills |
| `--dry-run` | Preview import |

**Examples:**

```bash
csm import backup.json
csm import skills.tar.gz --merge
csm import https://example.com/skills.json --dry-run
```

---

### csm ui

Launch the interactive TUI.

```bash
csm ui [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--section <SECTION>` | Start in section: `dashboard`, `skills`, `updates`, `settings` |

**Examples:**

```bash
csm ui
csm ui --section skills
```

---

### csm doctor

Diagnose and repair issues.

```bash
csm doctor [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--fix` | Attempt to fix detected issues |
| `--verbose` | Show detailed diagnostics |

**Examples:**

```bash
csm doctor
csm doctor --fix
csm doctor --verbose
```

---

### csm completions

Generate shell completions.

```bash
csm completions <SHELL>
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<SHELL>` | Shell: `bash`, `zsh`, `fish`, `powershell` |

**Examples:**

```bash
csm completions bash > ~/.bash_completion.d/csm
csm completions zsh > ~/.zfunc/_csm
csm completions fish > ~/.config/fish/completions/csm.fish
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Configuration error |
| 4 | Network error |
| 5 | Conflict detected |
| 10 | User cancelled |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `CSM_CONFIG` | Path to config file |
| `CSM_HOME` | CSM data directory (default: `~/.csm`) |
| `CSM_LOG_LEVEL` | Log level: `error`, `warn`, `info`, `debug`, `trace` |
| `GITHUB_TOKEN` | GitHub API token |
| `NO_COLOR` | Disable colored output |
| `EDITOR` | Default editor for `csm edit` |

## Aliases

Built-in aliases for common operations:

| Alias | Equivalent |
|-------|------------|
| `csm ls` | `csm list` |
| `csm rm` | `csm remove` |
| `csm up` | `csm update` |
| `csm s` | `csm search` |
| `csm i` | `csm add` |
