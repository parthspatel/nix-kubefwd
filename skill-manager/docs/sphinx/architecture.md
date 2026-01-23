# Architecture

This document describes the architecture of Claude Skill Manager (CSM).

## Overview

CSM follows a layered architecture with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────┐
│                    Presentation Layer                    │
│  ┌─────────────────────┐  ┌─────────────────────────┐  │
│  │         CLI         │  │          TUI            │  │
│  │    (clap v4)        │  │   (ratatui + crossterm) │  │
│  └─────────────────────┘  └─────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│                    Application Layer                     │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   Services                       │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐ │   │
│  │  │  Skill   │ │  Update  │ │     Conflict     │ │   │
│  │  │ Service  │ │ Service  │ │     Service      │ │   │
│  │  └──────────┘ └──────────┘ └──────────────────┘ │   │
│  │  ┌──────────┐ ┌────────────────────────────────┐│   │
│  │  │  Merge   │ │         Event Bus              ││   │
│  │  │ Service  │ │                                ││   │
│  │  └──────────┘ └────────────────────────────────┘│   │
│  └─────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│                      Domain Layer                        │
│  ┌─────────────────────────────────────────────────┐   │
│  │                  Domain Models                   │   │
│  │  ┌───────┐ ┌────────┐ ┌──────────┐ ┌────────┐  │   │
│  │  │ Skill │ │ Source │ │ Conflict │ │ Events │  │   │
│  │  └───────┘ └────────┘ └──────────┘ └────────┘  │   │
│  └─────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│                  Infrastructure Layer                    │
│  ┌───────────────┐ ┌───────────────┐ ┌─────────────┐   │
│  │    SQLite     │ │  File System  │ │   GitHub    │   │
│  │  Repository   │ │    Storage    │ │   Client    │   │
│  └───────────────┘ └───────────────┘ └─────────────┘   │
│  ┌───────────────┐ ┌───────────────────────────────┐   │
│  │    Config     │ │        URL Client             │   │
│  │   Manager     │ │                               │   │
│  └───────────────┘ └───────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

## Design Principles

### 1. Clean Architecture

CSM follows Clean Architecture principles:

- **Domain Layer**: Contains business logic and domain models, independent of external concerns
- **Application Layer**: Orchestrates use cases through services
- **Infrastructure Layer**: Implements external integrations (database, file system, APIs)
- **Presentation Layer**: Handles user interaction (CLI, TUI)

### 2. Dependency Inversion

All dependencies point inward:

- Infrastructure depends on Domain (not vice versa)
- Services define traits that Infrastructure implements
- This enables easy testing and swapping implementations

### 3. Event-Driven Communication

Domain events propagate changes:

- Services publish events when state changes
- Other components can subscribe to relevant events
- Enables loose coupling and extensibility

## Module Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library exports
├── domain/              # Domain models
│   ├── mod.rs
│   ├── skill.rs         # Skill entity
│   ├── source.rs        # SkillSource types
│   ├── conflict.rs      # Conflict types
│   └── events.rs        # Domain events
├── services/            # Application services
│   ├── mod.rs
│   ├── traits.rs        # Service interfaces
│   ├── skill_service.rs
│   ├── update_service.rs
│   ├── conflict_service.rs
│   └── merge_service.rs
├── infra/               # Infrastructure
│   ├── mod.rs
│   ├── database.rs      # SQLite implementation
│   ├── storage.rs       # File storage
│   ├── github.rs        # GitHub API client
│   └── config.rs        # Configuration
├── cli/                 # CLI interface
│   ├── mod.rs
│   └── commands/
│       ├── mod.rs
│       ├── init.rs
│       ├── add.rs
│       └── list.rs
├── tui/                 # TUI interface
│   ├── mod.rs
│   ├── app.rs
│   ├── screens/
│   └── widgets/
└── utils/               # Utilities
    ├── mod.rs
    ├── error.rs
    └── hash.rs
```

## Data Flow

### Adding a Skill

```
┌──────┐    ┌────────────┐    ┌─────────────┐    ┌──────────┐
│ CLI  │───▶│   Skill    │───▶│   GitHub    │───▶│ Storage  │
│      │    │  Service   │    │   Client    │    │          │
└──────┘    └────────────┘    └─────────────┘    └──────────┘
                 │                                     │
                 │            ┌─────────────┐          │
                 └───────────▶│  Repository │◀─────────┘
                              │  (SQLite)   │
                              └─────────────┘
                                     │
                              ┌─────────────┐
                              │  Event Bus  │
                              └─────────────┘
```

1. CLI parses command and calls SkillService
2. SkillService parses the source specification
3. If GitHub source, GitHubClient fetches content
4. Storage writes content to disk
5. Repository stores metadata in SQLite
6. Event bus publishes SkillAdded event

### Merging Skills

```
┌─────────┐    ┌─────────────┐    ┌────────────┐
│  Merge  │───▶│  Repository │───▶│  Storage   │
│ Service │    │             │    │            │
└─────────┘    └─────────────┘    └────────────┘
     │                                   │
     │         ┌─────────────┐           │
     └────────▶│   Output    │◀──────────┘
               │   Storage   │
               └─────────────┘
```

1. MergeService queries enabled skills from Repository
2. For each skill, retrieves content from Storage
3. Merges content by priority order
4. Writes merged output to appropriate location

## Storage Architecture

### SQLite Registry

The registry database stores skill metadata:

```sql
CREATE TABLE skills (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    source_json TEXT NOT NULL,
    scope_json TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    content_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    tags_json TEXT,
    priority INTEGER DEFAULT 0,
    update_mode TEXT DEFAULT 'auto'
);

CREATE TABLE conflicts (
    id TEXT PRIMARY KEY,
    skill_a_id TEXT NOT NULL,
    skill_b_id TEXT NOT NULL,
    conflict_type TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    detected_at TEXT NOT NULL,
    resolved_at TEXT,
    FOREIGN KEY (skill_a_id) REFERENCES skills(id),
    FOREIGN KEY (skill_b_id) REFERENCES skills(id)
);
```

### File Storage

Skills are stored on disk:

```
~/.csm/
├── config.toml              # Global configuration
├── registry.db              # SQLite database
├── skills/                  # Master skill storage
│   └── <skill-uuid>/
│       ├── CLAUDE.md        # Skill content
│       └── .meta.toml       # Local metadata
├── output/                  # Merged outputs
│   ├── _global.md           # Merged global skills
│   └── _local_<hash>.md     # Merged local skills
├── cache/                   # Download cache
│   └── github/
│       └── <owner>_<repo>/
└── logs/                    # Operation logs
    └── csm.log
```

### Symlink Architecture

CSM uses symlinks for efficient storage:

```
~/.claude/
└── CLAUDE.md -> ~/.csm/output/_global.md

~/my-project/
├── CLAUDE.md -> .csm/output/_merged.md
└── .csm/
    ├── config.toml
    └── skills/
        └── ts-best -> ~/.csm/skills/<uuid>  # Symlink to master
```

Benefits:
- Single source of truth for each skill
- Instant propagation of updates
- Minimal disk usage
- Clear separation of concerns

## Conflict Detection

### Detection Algorithm

```rust
fn detect_conflicts(skills: &[Skill]) -> Vec<Conflict> {
    let mut conflicts = Vec::new();

    for i in 0..skills.len() {
        for j in (i + 1)..skills.len() {
            let skill_a = &skills[i];
            let skill_b = &skills[j];

            // Parse skills into semantic sections
            let sections_a = parse_sections(content_a);
            let sections_b = parse_sections(content_b);

            // Check for duplicates
            for (key, value_a) in &sections_a {
                if let Some(value_b) = sections_b.get(key) {
                    if value_a == value_b {
                        conflicts.push(Conflict::duplicate(skill_a, skill_b, key));
                    } else if is_contradictory(value_a, value_b) {
                        conflicts.push(Conflict::contradictory(skill_a, skill_b, key));
                    }
                }
            }
        }
    }

    conflicts
}
```

### Conflict Types

1. **Duplicate**: Same instruction in multiple skills
2. **Contradictory**: Opposing instructions (e.g., "use tabs" vs "use spaces")
3. **Precedence Ambiguity**: Equal priority skills with overlapping scope
4. **Syntax Conflict**: Skills that cannot be merged structurally

## Update System

### Update Check Flow

```
┌────────────┐    ┌───────────────┐    ┌────────────┐
│   Update   │───▶│    GitHub     │───▶│   Check    │
│  Service   │    │    Client     │    │   ETag/    │
│            │    │               │    │   SHA      │
└────────────┘    └───────────────┘    └────────────┘
      │                                      │
      │           ┌───────────────┐          │
      └──────────▶│    Compare    │◀─────────┘
                  │    Versions   │
                  └───────────────┘
                         │
                  ┌──────▼──────┐
                  │   Update    │
                  │  Available? │
                  └─────────────┘
```

### Update Modes

| Mode | Behavior |
|------|----------|
| Auto | Check periodically, update automatically |
| Notify | Check periodically, prompt before updating |
| Manual | Only check/update when explicitly requested |

### Update Scheduling

- **Hourly**: Check every hour (for frequently updated skills)
- **Daily**: Check once per day (default)
- **Weekly**: Check once per week
- **On-demand**: Only when `csm update` is run

## Technology Stack

### Core Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `ratatui` | TUI framework |
| `crossterm` | Terminal manipulation |
| `tokio` | Async runtime |
| `rusqlite` | SQLite database |
| `git2` | Git operations |
| `reqwest` | HTTP client |
| `serde` | Serialization |

### Why Rust?

1. **Single Binary**: No runtime dependencies, easy distribution
2. **Performance**: Fast startup, low memory usage
3. **Safety**: Memory safety without garbage collection
4. **Ecosystem**: Excellent CLI/TUI libraries (clap, ratatui)
5. **Cross-Platform**: Native builds for all major platforms

### Why SQLite?

1. **Atomic Transactions**: Safe concurrent access
2. **Single File**: Easy backup and portability
3. **No Server**: Embedded, no external dependencies
4. **FTS5**: Full-text search for skill content
5. **Mature**: Battle-tested, well-documented

## Security Considerations

### Input Validation

All external input is validated:

- Source URLs are parsed and sanitized
- Skill content is checked for structure
- File paths are normalized and bounded

### Credential Storage

- GitHub tokens stored in system keychain (when available)
- Fallback to environment variables
- Never stored in plain text configuration

### Content Safety

- Skills are treated as data, not code
- No execution of skill content
- Content scanning for suspicious patterns

## Extensibility

### Adding New Source Types

1. Implement the source parser in `domain/source.rs`
2. Add fetcher in `infra/` (e.g., `gitlab.rs`)
3. Register in SkillService's source handler

### Adding New Commands

1. Define command in `cli/mod.rs`
2. Implement handler in `cli/commands/`
3. Wire up in main command dispatcher

### Custom Validators

The conflict detection system is extensible:

```rust
pub trait ConflictDetector: Send + Sync {
    fn detect(&self, skills: &[SkillContent]) -> Vec<Conflict>;
}
```

## Performance Characteristics

| Operation | Target | Notes |
|-----------|--------|-------|
| CLI startup | <10ms | Native binary, minimal initialization |
| TUI startup | <50ms | Including initial data load |
| List 100 skills | <100ms | Indexed SQLite query |
| GitHub fetch | <2s | Network dependent |
| Memory usage | <50MB | Lazy loading of content |
| Binary size | <20MB | Release build with LTO |

## Future Considerations

### Potential Extensions

1. **Plugin System**: Dynamic loading of extensions
2. **Remote Sync**: Synchronize across machines
3. **Skill Marketplace**: Community skill discovery
4. **VS Code Extension**: GUI integration
5. **AI Suggestions**: Recommend skills based on project

### Scalability

The current architecture supports:

- Hundreds of skills per installation
- Large skill files (tested up to 1MB)
- Concurrent CLI/TUI access
- Background update checking
