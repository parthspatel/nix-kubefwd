# Claude Skill Manager - CLI Specification

## Document Info
- **Version**: 1.0
- **Status**: Draft
- **Last Updated**: 2026-01-22

---

## 1. Overview

The CLI is the primary interface for CSM. It follows modern CLI conventions and provides a consistent, intuitive command structure.

### 1.1 Design Principles

1. **Predictable**: Commands follow consistent patterns
2. **Discoverable**: Built-in help at every level
3. **Scriptable**: Machine-readable output options
4. **Safe**: Destructive operations require confirmation
5. **Fast**: Sub-second response for all local operations

### 1.2 Command Structure

```
csm [OPTIONS] <COMMAND> [SUBCOMMAND] [ARGS]
```

---

## 2. Global Options

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Show help information |
| `--version` | `-V` | Show version information |
| `--verbose` | `-v` | Increase verbosity (can stack: -vvv) |
| `--quiet` | `-q` | Suppress non-essential output |
| `--config <PATH>` | `-c` | Use custom config file |
| `--no-color` | | Disable colored output |
| `--json` | | Output in JSON format |
| `--yes` | `-y` | Auto-confirm all prompts |

---

## 3. Commands

### 3.1 `csm init`

Initialize CSM in the current environment.

```bash
csm init [OPTIONS]

Options:
  --global              Initialize global configuration only
  --local               Initialize local project only
  --force               Overwrite existing configuration
  --import-existing     Import existing CLAUDE.md files

Examples:
  csm init                    # Interactive initialization
  csm init --global           # Setup global config
  csm init --local            # Setup project config
  csm init --import-existing  # Import existing skills
```

**Behavior**:
1. Check for existing configuration
2. Create `~/.csm/` directory structure
3. Initialize SQLite registry
4. Scan for existing CLAUDE.md files
5. Offer to import discovered skills
6. Create default configuration

---

### 3.2 `csm add`

Add a skill from various sources.

```bash
csm add <SOURCE> [OPTIONS]

Arguments:
  <SOURCE>    Skill source (github:owner/repo, path, URL)

Options:
  --name <NAME>         Custom name for the skill
  --scope <SCOPE>       Scope: global, local (default: local)
  --enable              Enable immediately (default: true)
  --no-enable           Add but don't enable
  --update-mode <MODE>  Update mode: auto, notify, manual

Source Formats:
  github:owner/repo                    # Full repository
  github:owner/repo/path               # Specific path
  github:owner/repo@ref                # Specific ref (branch/tag/commit)
  /path/to/skill                       # Local path
  https://example.com/skill.md         # URL

Examples:
  csm add github:anthropics/skills/typescript
  csm add github:user/repo@v1.0.0 --scope global
  csm add ./my-skill.md --name "My Custom Skill"
  csm add https://gist.github.com/user/abc123/raw/skill.md
```

**Behavior**:
1. Parse source specification
2. Fetch skill content
3. Validate skill format
4. Check for conflicts
5. Register in database
6. Create symlinks/copies
7. Rebuild merged CLAUDE.md

---

### 3.3 `csm remove`

Remove a skill.

```bash
csm remove <SKILL> [OPTIONS]

Arguments:
  <SKILL>     Skill name or ID

Options:
  --force     Remove without confirmation
  --keep-files   Remove from registry but keep files

Examples:
  csm remove typescript-best
  csm remove --force old-skill
```

---

### 3.4 `csm list`

List skills.

```bash
csm list [OPTIONS]

Options:
  --scope <SCOPE>      Filter by scope: all, global, local (default: all)
  --enabled            Show only enabled skills
  --disabled           Show only disabled skills
  --source <SOURCE>    Filter by source type: github, local, url
  --format <FORMAT>    Output format: table, json, yaml (default: table)
  --verbose            Show additional details

Examples:
  csm list                        # List all skills
  csm list --scope global         # List global skills only
  csm list --enabled --json       # Enabled skills as JSON
  csm list -v                     # Verbose output with details
```

**Output (table format)**:
```
┌─────────────────────┬──────────┬─────────┬────────────────────────────┐
│ Name                │ Scope    │ Status  │ Source                     │
├─────────────────────┼──────────┼─────────┼────────────────────────────┤
│ typescript-best     │ global   │ enabled │ github:anthropics/skills   │
│ python-style        │ global   │ enabled │ github:user/python-skills  │
│ project-specific    │ local    │ enabled │ local:/path/to/skill       │
│ experimental        │ local    │ disabled│ github:user/experimental   │
└─────────────────────┴──────────┴─────────┴────────────────────────────┘
```

---

### 3.5 `csm show`

Show detailed information about a skill.

```bash
csm show <SKILL> [OPTIONS]

Arguments:
  <SKILL>     Skill name or ID

Options:
  --content           Show full skill content
  --metadata          Show metadata only
  --format <FORMAT>   Output format: text, json, yaml

Examples:
  csm show typescript-best
  csm show typescript-best --content
  csm show typescript-best --json
```

**Output**:
```
Skill: typescript-best
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ID:          a1b2c3d4-e5f6-7890-abcd-ef1234567890
  Scope:       global
  Status:      enabled
  Priority:    100

  Source:      github:anthropics/claude-skills/typescript
  Ref:         main (commit: abc1234)
  Update Mode: auto
  Last Update: 2026-01-20 14:32:00 UTC

  Description:
    Best practices for TypeScript development including
    strict typing, error handling, and code organization.

  Tags:        typescript, coding, best-practices

  File:        ~/.csm/skills/typescript-best/CLAUDE.md
  Size:        4.2 KB
  Hash:        sha256:abc123...

  Conflicts:   None detected
```

---

### 3.6 `csm enable` / `csm disable`

Toggle skill state.

```bash
csm enable <SKILL>
csm disable <SKILL>
csm toggle <SKILL>

Examples:
  csm enable typescript-best
  csm disable experimental
  csm toggle debugging-verbose
```

---

### 3.7 `csm update`

Update skills from their sources.

```bash
csm update [SKILL] [OPTIONS]

Arguments:
  [SKILL]     Specific skill to update (optional, updates all if omitted)

Options:
  --check             Check for updates without applying
  --force             Update even if no changes detected
  --dry-run           Show what would be updated

Examples:
  csm update                      # Update all skills
  csm update typescript-best      # Update specific skill
  csm update --check              # Check for available updates
  csm update --dry-run            # Preview updates
```

**Output**:
```
Checking for updates...

  typescript-best     main@abc1234 → main@def5678  (3 commits behind)
  python-style        v1.0.0 → v1.1.0              (new version)
  project-specific    (local - no updates)

Update 2 skills? [Y/n] y

Updating typescript-best... done
Updating python-style... done

✓ 2 skills updated successfully
```

---

### 3.8 `csm sync`

Synchronize skill state.

```bash
csm sync [OPTIONS]

Options:
  --rebuild           Force rebuild of merged CLAUDE.md files
  --verify            Verify symlink integrity
  --fix               Fix broken symlinks and references

Examples:
  csm sync                # Standard sync
  csm sync --rebuild      # Force rebuild
  csm sync --verify --fix # Verify and repair
```

---

### 3.9 `csm conflicts`

Detect and resolve skill conflicts.

```bash
csm conflicts [OPTIONS]

Options:
  --resolve           Interactive conflict resolution
  --format <FORMAT>   Output format: text, json

Examples:
  csm conflicts
  csm conflicts --resolve
  csm conflicts --json
```

**Output**:
```
Conflict Analysis
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠ 2 conflicts detected

1. Duplicate Definition
   Skills:  typescript-best (line 15) ↔ project-style (line 8)
   Content: "Use 2-space indentation"

   Suggestion: Remove from one skill or set explicit priority

2. Contradictory Instruction
   Skills:  strict-mode (line 22) ↔ flexible-style (line 45)
   Content: "Always use strict null checks" vs "Null checks optional"

   Suggestion: Disable one skill or create override

Run `csm conflicts --resolve` for interactive resolution.
```

---

### 3.10 `csm search`

Search for skills.

```bash
csm search <QUERY> [OPTIONS]

Arguments:
  <QUERY>     Search query

Options:
  --installed         Search installed skills only
  --marketplace       Search marketplace (future)
  --tag <TAG>         Filter by tag
  --limit <N>         Limit results (default: 20)

Examples:
  csm search typescript
  csm search "error handling" --installed
  csm search --tag python
```

---

### 3.11 `csm config`

Manage configuration.

```bash
csm config <SUBCOMMAND>

Subcommands:
  get <KEY>           Get configuration value
  set <KEY> <VALUE>   Set configuration value
  list                List all configuration
  edit                Open config in editor
  reset               Reset to defaults

Configuration Keys:
  update-mode         Default update mode (auto|notify|manual)
  update-schedule     Update check schedule (hourly|daily|weekly)
  default-scope       Default skill scope (global|local)
  editor              Preferred editor for editing
  color               Enable colored output (true|false)
  github-token        GitHub API token (stored securely)

Examples:
  csm config get update-mode
  csm config set update-mode notify
  csm config set default-scope global
  csm config list
  csm config edit
```

---

### 3.12 `csm create`

Create a new skill.

```bash
csm create <NAME> [OPTIONS]

Arguments:
  <NAME>      Name for the new skill

Options:
  --from <PATH>       Create from existing file
  --template <NAME>   Use a template (basic, detailed, coding)
  --scope <SCOPE>     Scope: global, local (default: local)
  --edit              Open in editor after creation

Examples:
  csm create my-skill
  csm create my-skill --from ./existing.md
  csm create coding-style --template coding --scope global
  csm create quick-skill --edit
```

---

### 3.13 `csm edit`

Edit a skill.

```bash
csm edit <SKILL> [OPTIONS]

Arguments:
  <SKILL>     Skill name or ID

Options:
  --editor <EDITOR>   Use specific editor

Examples:
  csm edit typescript-best
  csm edit my-skill --editor code
```

---

### 3.14 `csm export` / `csm import`

Export and import skills.

```bash
csm export [OPTIONS]

Options:
  --all               Export all skills
  --skill <SKILL>     Export specific skill
  --scope <SCOPE>     Export by scope
  --format <FORMAT>   Format: json, toml, archive (default: json)
  --output <PATH>     Output file (default: stdout)

csm import <SOURCE> [OPTIONS]

Arguments:
  <SOURCE>    File path or URL

Options:
  --merge             Merge with existing skills
  --replace           Replace existing skills
  --dry-run           Preview import

Examples:
  csm export --all > backup.json
  csm export --skill typescript-best --format toml
  csm export --scope global --format archive -o global-skills.tar.gz

  csm import backup.json
  csm import skills.tar.gz --merge
  csm import https://example.com/skills.json --dry-run
```

---

### 3.15 `csm ui`

Launch the TUI.

```bash
csm ui [OPTIONS]

Options:
  --section <SECTION>   Start in section: dashboard, skills, updates, settings

Examples:
  csm ui
  csm ui --section skills
```

---

### 3.16 `csm doctor`

Diagnose and repair issues.

```bash
csm doctor [OPTIONS]

Options:
  --fix               Attempt to fix detected issues
  --verbose           Show detailed diagnostics

Examples:
  csm doctor
  csm doctor --fix
```

**Output**:
```
CSM Health Check
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Configuration
  ✓ Config file exists
  ✓ Config file valid

Registry
  ✓ Database accessible
  ✓ Schema version current
  ⚠ 1 orphaned entry found

File System
  ✓ Skills directory exists
  ✗ 2 broken symlinks detected
  ✓ Permissions correct

Network
  ✓ GitHub API accessible
  ✓ Rate limit OK (4832/5000)

Summary: 2 issues found

Run `csm doctor --fix` to attempt automatic repair.
```

---

### 3.17 `csm completions`

Generate shell completions.

```bash
csm completions <SHELL>

Arguments:
  <SHELL>     Shell: bash, zsh, fish, powershell

Examples:
  csm completions bash > ~/.bash_completion.d/csm
  csm completions zsh > ~/.zfunc/_csm
  csm completions fish > ~/.config/fish/completions/csm.fish
```

---

## 4. Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Configuration error |
| 4 | Network error |
| 5 | Conflict detected |
| 10 | User cancelled |

---

## 5. Environment Variables

| Variable | Description |
|----------|-------------|
| `CSM_CONFIG` | Path to config file |
| `CSM_HOME` | CSM data directory (default: `~/.csm`) |
| `CSM_LOG_LEVEL` | Log level: error, warn, info, debug, trace |
| `GITHUB_TOKEN` | GitHub API token |
| `NO_COLOR` | Disable colored output |
| `EDITOR` | Default editor for `csm edit` |

---

## 6. Configuration File

Location: `~/.csm/config.toml`

```toml
# CSM Configuration

[general]
default_scope = "local"
editor = "vim"
color = true

[updates]
mode = "auto"           # auto, notify, manual
schedule = "daily"      # hourly, daily, weekly
check_on_startup = true

[github]
# Token stored in system keychain, not in file
default_ref = "main"

[ui]
theme = "dark"          # dark, light
show_welcome = true

[logging]
level = "info"
file = "~/.csm/logs/csm.log"
max_size = "10MB"
```

---

## 7. Aliases and Shortcuts

Built-in aliases for common operations:

| Alias | Equivalent |
|-------|------------|
| `csm ls` | `csm list` |
| `csm rm` | `csm remove` |
| `csm up` | `csm update` |
| `csm s` | `csm search` |
| `csm i` | `csm add` (install) |
