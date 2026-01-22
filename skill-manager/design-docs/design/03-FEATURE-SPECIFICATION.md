# Claude Skill Manager - Feature Specification

## Document Info
- **Version**: 1.0
- **Status**: Draft
- **Last Updated**: 2026-01-22

---

## 1. Executive Summary

Claude Skill Manager (CSM) is a Rust-based CLI and TUI application for managing Claude AI skills. It provides a unified interface for skill discovery, installation, synchronization, and lifecycle management.

---

## 2. Feature Categories

### 2.1 Core Features (P0 - Must Have)

| ID | Feature | Description |
|----|---------|-------------|
| F001 | Skill Registry | Central database tracking all installed skills |
| F002 | Global Skill Management | Manage skills in `~/.claude/` |
| F003 | Local Skill Management | Manage skills in project `.claude/` directories |
| F004 | GitHub Installation | Install skills from GitHub repositories |
| F005 | Skill Enable/Disable | Toggle skills without deletion |
| F006 | Conflict Detection | Identify overlapping/conflicting skill definitions |
| F007 | CLI Interface | Complete command-line interface |
| F008 | Basic TUI | Interactive terminal interface |

### 2.2 Enhanced Features (P1 - Should Have)

| ID | Feature | Description |
|----|---------|-------------|
| F009 | Auto-Update | Automatic skill updates from sources |
| F010 | Symlink Architecture | Efficient storage via symlinks |
| F011 | Skill Composition | Combine multiple skills into one |
| F012 | Search & Filter | Find skills by name, tag, content |
| F013 | Skill Templates | Create skills from templates |
| F014 | Export/Import | Backup and restore skill configurations |
| F015 | Update Notifications | Alert users of available updates |

### 2.3 Advanced Features (P2 - Nice to Have)

| ID | Feature | Description |
|----|---------|-------------|
| F016 | Skill Marketplace | Browse community-published skills |
| F017 | Skill Publishing | Publish skills to marketplace |
| F018 | Team Sync | Synchronize skills across team members |
| F019 | Skill Analytics | Usage statistics and insights |
| F020 | GPG Signatures | Cryptographic skill verification |
| F021 | Skill Versioning | Pin specific skill versions |
| F022 | Dependency Management | Skills that depend on other skills |

---

## 3. Detailed Feature Specifications

### 3.1 F001: Skill Registry

**Purpose**: Maintain a central database of all skills known to CSM.

**Data Model**:
```rust
struct Skill {
    id: Uuid,
    name: String,
    description: Option<String>,
    source: SkillSource,
    scope: SkillScope,
    enabled: bool,
    content_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    tags: Vec<String>,
    priority: i32,
}

enum SkillSource {
    Local { path: PathBuf },
    GitHub {
        owner: String,
        repo: String,
        path: Option<String>,
        ref_spec: Option<String>,  // branch, tag, commit
    },
    Url { url: String },
    Inline { content: String },
}

enum SkillScope {
    Global,
    Project { path: PathBuf },
}
```

**Storage**: SQLite database at `~/.csm/registry.db`

**Operations**:
- Create skill entry
- Read skill by ID/name
- Update skill metadata
- Delete skill entry
- List all skills with filtering
- Search skills (FTS5)

---

### 3.2 F002: Global Skill Management

**Purpose**: Manage skills that apply to all Claude sessions.

**Location**: `~/.claude/CLAUDE.md` and `~/.csm/skills/`

**Behavior**:
- Global skills are loaded for every Claude session
- Lower precedence than project-local skills
- Stored in `~/.csm/skills/` with symlinks to `~/.claude/`

**Commands**:
```bash
csm global list              # List global skills
csm global add <skill>       # Add skill to global scope
csm global remove <skill>    # Remove from global scope
csm global enable <skill>    # Enable global skill
csm global disable <skill>   # Disable global skill
```

---

### 3.3 F003: Local Skill Management

**Purpose**: Manage skills specific to a project.

**Location**: `./CLAUDE.md` and `./.csm/skills/`

**Behavior**:
- Project skills override global skills
- Can be committed to version control
- Supports team sharing via git

**Commands**:
```bash
csm local list               # List project skills
csm local add <skill>        # Add skill to project
csm local remove <skill>     # Remove from project
csm local enable <skill>     # Enable project skill
csm local disable <skill>    # Disable project skill
csm local init               # Initialize project skill management
```

---

### 3.4 F004: GitHub Installation

**Purpose**: Install skills directly from GitHub repositories.

**Supported Formats**:
```bash
# Full repository (looks for CLAUDE.md or .claude/ directory)
csm add github:owner/repo

# Specific path within repo
csm add github:owner/repo/path/to/skill

# Specific ref (branch, tag, commit)
csm add github:owner/repo@main
csm add github:owner/repo@v1.2.0
csm add github:owner/repo@abc123

# Combined
csm add github:owner/repo/skills/typescript@v2.0.0
```

**Process**:
1. Parse GitHub URL/shorthand
2. Fetch repository metadata via GitHub API
3. Clone/fetch relevant files (sparse checkout)
4. Validate skill content
5. Register in skill registry
6. Create symlinks/copies as appropriate
7. Configure update tracking

**Authentication**:
- Uses `GITHUB_TOKEN` environment variable if set
- Falls back to git credential helpers
- Supports GitHub CLI (`gh`) auth

---

### 3.5 F005: Skill Enable/Disable

**Purpose**: Toggle skills without removing them.

**Behavior**:
- Disabled skills remain in registry but aren't loaded
- Symlinks are removed/restored on toggle
- State persisted in registry

**Commands**:
```bash
csm enable <skill>           # Enable a skill
csm disable <skill>          # Disable a skill
csm toggle <skill>           # Toggle skill state
```

---

### 3.6 F006: Conflict Detection

**Purpose**: Identify and help resolve conflicting skill definitions.

**Conflict Types**:
1. **Duplicate definitions**: Same instruction in multiple skills
2. **Contradictory instructions**: Opposite directives
3. **Precedence ambiguity**: Unclear which skill takes priority
4. **Syntax conflicts**: Skills that can't be merged

**Detection Algorithm**:
```rust
struct Conflict {
    skill_a: SkillId,
    skill_b: SkillId,
    conflict_type: ConflictType,
    description: String,
    line_a: Option<usize>,
    line_b: Option<usize>,
    suggestion: Option<String>,
}

fn detect_conflicts(skills: &[Skill]) -> Vec<Conflict> {
    // 1. Parse each skill into semantic sections
    // 2. Compare sections across skills
    // 3. Identify overlaps and contradictions
    // 4. Generate conflict reports
}
```

**Commands**:
```bash
csm conflicts                # List all conflicts
csm conflicts --resolve      # Interactive resolution
csm conflicts --json         # Machine-readable output
```

---

### 3.7 F007: CLI Interface

See [04-CLI-SPECIFICATION.md](./04-CLI-SPECIFICATION.md) for complete CLI design.

---

### 3.8 F008: TUI Interface

See [05-TUI-SPECIFICATION.md](./05-TUI-SPECIFICATION.md) for complete TUI design.

---

### 3.9 F009: Auto-Update System

**Purpose**: Keep skills synchronized with their sources.

**Update Modes**:
| Mode | Behavior |
|------|----------|
| `auto` | Check and update automatically (default) |
| `notify` | Check and notify, user confirms update |
| `manual` | Only update on explicit request |

**Update Schedule**:
- Default: Check daily
- Configurable: hourly, daily, weekly, on-demand
- Respects rate limits (GitHub API)

**Update Process**:
1. Check source for changes (ETag, commit SHA)
2. If changed, fetch new content
3. Validate new content
4. Create backup of current version
5. Apply update
6. Log update in history

**Commands**:
```bash
csm update                   # Update all skills
csm update <skill>           # Update specific skill
csm update --check           # Check for updates without applying
csm config update-mode auto  # Set update mode
```

---

### 3.10 F010: Symlink Architecture

**Purpose**: Efficient storage and instant propagation of updates.

**Directory Structure**:
```
~/.csm/
├── config.toml              # Global configuration
├── registry.db              # SQLite database
├── skills/                  # Master skill storage
│   ├── typescript-best/
│   │   ├── CLAUDE.md
│   │   └── .meta.toml       # Skill metadata
│   └── python-style/
│       ├── CLAUDE.md
│       └── .meta.toml
├── cache/                   # Download cache
│   └── github/
└── logs/                    # Operation logs

~/.claude/
├── CLAUDE.md -> ~/.csm/skills/_merged_global.md  # Merged global skills

~/my-project/
├── CLAUDE.md -> ../.csm/skills/_merged_local.md
└── .csm/
    ├── config.toml          # Project-specific config
    └── skills/
        └── ts-best -> ~/.csm/skills/typescript-best  # Symlink
```

**Merging Strategy**:
- Skills are merged into single CLAUDE.md files
- Merge order determined by priority
- Section headers preserved
- Conflicts highlighted with comments

---

### 3.11 F011: Skill Composition

**Purpose**: Combine multiple skills into composite skills.

**Composition File**:
```toml
# ~/.csm/compositions/my-fullstack.toml
[composition]
name = "my-fullstack"
description = "Full-stack development setup"

[[skills]]
name = "typescript-best"
priority = 100

[[skills]]
name = "react-patterns"
priority = 90

[[skills]]
name = "nodejs-backend"
priority = 80

[overrides]
# Override specific settings
"code-style.indent" = 4
```

**Commands**:
```bash
csm compose create my-stack  # Create new composition
csm compose add my-stack typescript-best
csm compose remove my-stack react-patterns
csm compose apply my-stack   # Apply composition to project
```

---

### 3.12 F014: Export/Import

**Purpose**: Backup, share, and restore skill configurations.

**Export Formats**:
- JSON (machine-readable, includes metadata)
- TOML (human-readable configuration)
- Archive (tar.gz with all skill files)

**Commands**:
```bash
csm export --all > backup.json
csm export --skill typescript-best > ts.json
csm export --format archive -o skills.tar.gz

csm import backup.json
csm import skills.tar.gz
csm import https://example.com/skills.json
```

---

## 4. Non-Functional Requirements

### 4.1 Performance

| Metric | Target |
|--------|--------|
| CLI startup | <10ms |
| TUI startup | <50ms |
| Skill listing (100 skills) | <100ms |
| GitHub fetch | <2s (network dependent) |
| Memory usage | <50MB |
| Binary size | <20MB |

### 4.2 Reliability

- Atomic operations for all registry changes
- Automatic backup before destructive operations
- Graceful degradation on network failure
- Recovery from corrupted state

### 4.3 Security

- No execution of skill content
- Validation of all external input
- Optional GPG signature verification
- Secure credential storage
- Audit logging of all operations

### 4.4 Compatibility

- macOS 11+ (x86_64, ARM64)
- Linux (glibc 2.17+, x86_64, ARM64)
- Windows 10+ (x86_64)
- Claude Code CLI v1.0+

---

## 5. Future Considerations

### 5.1 Potential Extensions

1. **VS Code Extension**: GUI for skill management
2. **Web Dashboard**: Browser-based management
3. **Skill Marketplace API**: Public skill registry
4. **Enterprise Features**: SSO, audit logs, compliance

### 5.2 Out of Scope (v1.0)

1. Real-time collaboration
2. Skill execution/testing
3. AI-powered skill suggestions
4. Mobile applications
