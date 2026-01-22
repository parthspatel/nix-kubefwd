# Claude Skill Manager - Frequently Asked Questions

## Customer FAQ

### General Questions

#### Q: What is a "skill" in the context of Claude?

**A:** A skill is a set of instructions, preferences, or behavioral guidelines that customize how Claude responds. Skills are typically stored in:
- `~/.claude/CLAUDE.md` - Global skills applied to all projects
- `./CLAUDE.md` - Project-level skills in repository root
- `./.claude/` directory - Additional project-specific configurations

Skills can define coding style preferences, domain expertise, response formats, tool usage patterns, and more.

---

#### Q: Why do I need a skill manager?

**A:** Without a skill manager, you face several challenges:
1. **Manual duplication** - Copying skills between projects
2. **Version drift** - Skills becoming outdated across projects
3. **No discovery** - Hard to find and adopt community skills
4. **Conflict blindness** - No visibility into conflicting skill definitions
5. **Update burden** - Manually checking for and applying updates

CSM automates all of this, saving time and ensuring consistency.

---

#### Q: Is CSM compatible with existing Claude skill files?

**A:** Yes. CSM is designed to work alongside existing CLAUDE.md files. Running `csm init` will:
1. Detect existing skill files
2. Import them into the CSM registry
3. Optionally convert them to managed skills
4. Never delete or corrupt existing configurations

---

### Installation & Setup

#### Q: What are the system requirements?

**A:**
- macOS (x86_64, ARM64), Linux (x86_64, ARM64), or Windows
- Git (for GitHub integration)
- Claude Code CLI installed

No runtime dependencies required - CSM is a single static binary.

---

#### Q: How do I install CSM?

**A:**
```bash
# Using cargo
cargo install claude-skill-manager

# Using Homebrew (macOS/Linux)
brew install csm

# Using the install script
curl -fsSL https://csm.dev/install.sh | sh

# Download pre-built binary
# See releases at github.com/anthropics/claude-skill-manager/releases
```

---

#### Q: What happens during `csm init`?

**A:** The initialization process:
1. Creates `~/.csm/` configuration directory
2. Scans for existing CLAUDE.md files
3. Creates a skill registry database
4. Sets up symlink infrastructure
5. Configures auto-update schedules (if enabled)

---

### Skill Management

#### Q: How do I add a skill from GitHub?

**A:**
```bash
# From a dedicated skill repo
csm add github:user/repo

# From a specific path within a repo
csm add github:user/repo/path/to/skill

# From a specific branch or tag
csm add github:user/repo@v1.2.0

# From a specific file
csm add github:user/repo/CLAUDE.md
```

---

#### Q: What's the difference between global and local skills?

**A:**
| Aspect | Global Skills | Local Skills |
|--------|---------------|--------------|
| Location | `~/.csm/skills/` | `./.csm/skills/` |
| Scope | All projects | Current project only |
| Precedence | Lower | Higher (overrides global) |
| Sharing | Personal only | Can be committed to repo |

---

#### Q: How does auto-update work?

**A:** CSM checks for updates based on your configuration:
- **auto** (default): Checks daily, updates automatically
- **notify**: Checks daily, notifies of available updates
- **manual**: Only updates when explicitly requested

Configure globally or per-skill:
```bash
# Global setting
csm config set update-mode notify

# Per-skill setting
csm update-mode typescript-best-practices manual
```

---

#### Q: How do I resolve skill conflicts?

**A:** When skills conflict, CSM provides tools:
```bash
# Detect conflicts
csm conflicts

# Interactive resolution
csm conflicts --resolve

# Force specific skill precedence
csm priority set skill-a --above skill-b
```

---

#### Q: Can I create my own skills?

**A:**
```bash
# Create a new skill interactively
csm create my-skill

# Create from existing CLAUDE.md
csm create my-skill --from ./CLAUDE.md

# Publish to GitHub (optional)
csm publish my-skill
```

---

### TUI Interface

#### Q: How do I access the TUI?

**A:**
```bash
# Launch TUI
csm ui

# Launch to specific section
csm ui --section skills
csm ui --section updates
csm ui --section conflicts
```

---

#### Q: What can I do in the TUI?

**A:** The TUI provides:
- **Dashboard**: Overview of skill status, updates, and conflicts
- **Browse**: Visual skill browser with search and filters
- **Edit**: Inline skill editing with syntax highlighting
- **Sync**: Manage sync relationships and update schedules
- **Settings**: Configure CSM preferences

---

### Advanced Usage

#### Q: How does the symlink architecture work?

**A:** CSM uses symlinks to prevent duplication:
```
~/.csm/
├── skills/           # Master copies of all skills
│   ├── ts-best/
│   └── python-style/
├── registry.json     # Skill metadata and relationships
└── config.yaml       # CSM configuration

~/my-project/.csm/
└── skills/
    └── ts-best -> ~/.csm/skills/ts-best  # Symlink to master
```

This ensures:
- Single source of truth for each skill
- Instant updates across all projects
- Minimal disk usage

---

#### Q: Can I use CSM in a CI/CD environment?

**A:** Yes. CSM supports headless operation:
```bash
# Install skills without prompts
csm add github:user/repo --yes

# Verify skill integrity
csm verify --strict

# Export skill configuration
csm export --format json > skills.json
```

---

#### Q: How do I backup my skills?

**A:**
```bash
# Export all skills
csm export --all > backup.json

# Export specific skill
csm export my-skill > my-skill.json

# Restore from backup
csm import backup.json
```

---

## Internal FAQ (Technical)

### Architecture

#### Q: What language/framework is CSM built with?

**A:** CSM is built entirely in Rust:
- **Language**: Rust (2021 edition)
- **CLI Framework**: clap v4
- **TUI Framework**: ratatui + crossterm
- **Database**: SQLite (via rusqlite)
- **Git Operations**: git2 (libgit2 bindings)
- **Async Runtime**: tokio
- **HTTP Client**: reqwest
- **Serialization**: serde + serde_json/serde_yaml

---

#### Q: Why Rust?

**A:** Rust provides:
- **Single binary distribution** - No runtime dependencies
- **Cross-platform** - Native builds for all major platforms
- **Performance** - Fast startup, low memory usage
- **Safety** - Memory safety without garbage collection
- **Excellent CLI/TUI ecosystem** - clap, ratatui are best-in-class

---

#### Q: Why SQLite over plain files?

**A:** SQLite provides:
- Atomic transactions for registry updates
- Efficient querying for large skill sets
- ACID compliance for data integrity
- Single-file backup and portability
- Excellent Rust support via rusqlite

---

#### Q: How are GitHub skills cached?

**A:** GitHub skills use a layered cache:
1. **Local cache**: `~/.csm/cache/` with TTL
2. **ETag tracking**: Efficient update checking via GitHub API
3. **Shallow clones**: Only fetch necessary commits
4. **Sparse checkout**: Only fetch necessary files
5. **Compression**: Cached skills are zstd compressed

---

### Security

#### Q: How are skills validated before installation?

**A:** CSM performs multi-layer validation:
1. **Schema validation**: Skill file structure
2. **Content scanning**: No executable code in unexpected places
3. **Signature verification**: Optional GPG signatures for trusted sources
4. **Sandbox preview**: Option to preview skill effects before enabling

---

#### Q: Can skills execute arbitrary code?

**A:** No. Skills are strictly configuration/instruction files. CSM:
- Rejects files with executable patterns
- Sandboxes parsing operations
- Provides audit logs of all skill changes

---

### Performance

#### Q: What's the startup overhead of CSM?

**A:** CSM is optimized for speed (native Rust binary):
- Cold start: <10ms
- Warm start: <5ms
- TUI render: <16ms first paint (60fps capable)
- Memory usage: <10MB typical

---

#### Q: How does CSM handle large skill collections?

**A:** CSM is designed to scale:
- Lazy loading of skill content
- Indexed search in SQLite with FTS5
- Virtualized lists in TUI (ratatui)
- Streaming exports for large datasets
- Parallel processing with rayon where applicable

---

### Integration

#### Q: Does CSM integrate with Claude Code?

**A:** Yes. CSM is designed to complement Claude Code:
- Respects Claude Code's skill file conventions
- Can be invoked from within Claude Code sessions
- Provides skill suggestions based on project context

---

#### Q: Can CSM manage skills for other AI assistants?

**A:** The architecture is extensible. Future versions may support:
- Cursor rules
- GitHub Copilot instructions
- Other AI assistant configurations

Initial release focuses on Claude to ensure quality.
