# Claude Skill Manager - Design Documentation

This directory contains the complete design documentation for Claude Skill Manager (CSM), a Rust-based CLI and TUI application for managing Claude AI skills.

## Documentation Index

### Amazon Working Backwards Documents

| Document | Description |
|----------|-------------|
| [01-PRESS-RELEASE.md](docs/01-PRESS-RELEASE.md) | Press release announcing CSM and its key features |
| [02-FAQ.md](docs/02-FAQ.md) | Frequently asked questions (customer & internal) |

### Design Specifications

| Document | Description |
|----------|-------------|
| [03-FEATURE-SPECIFICATION.md](docs/design/03-FEATURE-SPECIFICATION.md) | Complete feature set with data models and requirements |
| [04-CLI-SPECIFICATION.md](docs/design/04-CLI-SPECIFICATION.md) | CLI interface design with all commands and options |
| [05-TUI-SPECIFICATION.md](docs/design/05-TUI-SPECIFICATION.md) | TUI wireframes, storyboards, and interaction design |
| [06-USER-JOURNEYS.md](docs/design/06-USER-JOURNEYS.md) | User personas, journey maps, and interaction flows |

### Technical Documentation

| Document | Description |
|----------|-------------|
| [07-TESTING-STRATEGY.md](docs/testing/07-TESTING-STRATEGY.md) | Testing approach, coverage targets, and test examples |
| [08-TECHNICAL-ARCHITECTURE.md](docs/architecture/08-TECHNICAL-ARCHITECTURE.md) | System architecture, data models, and algorithms |

## Quick Summary

### What is CSM?

Claude Skill Manager is a unified tool for managing Claude AI customizations (skills) across projects:

- **Centralized Registry**: All skills tracked in one place
- **GitHub Integration**: Install skills from repositories with auto-updates
- **Global/Local Management**: Skills can be global or project-specific
- **Symlink Architecture**: Efficient storage, instant updates
- **Interactive TUI**: Visual interface for skill management
- **Conflict Detection**: Automatic identification of conflicting instructions

### Technology Stack

- **Language**: Rust (2021 edition)
- **CLI**: clap v4
- **TUI**: ratatui + crossterm
- **Database**: SQLite (rusqlite)
- **Git**: git2 (libgit2)
- **HTTP**: reqwest + tokio

### Key Features

| Feature | Priority | Status |
|---------|----------|--------|
| Skill Registry | P0 | Design Complete |
| GitHub Installation | P0 | Design Complete |
| Global/Local Skills | P0 | Design Complete |
| CLI Interface | P0 | Design Complete |
| TUI Interface | P0 | Design Complete |
| Auto-Update | P1 | Design Complete |
| Conflict Detection | P0 | Design Complete |
| Symlink Architecture | P1 | Design Complete |

## Directory Structure

```
skill-manager/
├── README.md                     # This file
├── docs/
│   ├── 01-PRESS-RELEASE.md       # PR/FAQ
│   ├── 02-FAQ.md
│   ├── design/
│   │   ├── 03-FEATURE-SPECIFICATION.md
│   │   ├── 04-CLI-SPECIFICATION.md
│   │   ├── 05-TUI-SPECIFICATION.md
│   │   └── 06-USER-JOURNEYS.md
│   ├── testing/
│   │   └── 07-TESTING-STRATEGY.md
│   └── architecture/
│       └── 08-TECHNICAL-ARCHITECTURE.md
└── wireframes/                   # (Future: visual assets)
```

## Next Steps

1. **Review & Feedback**: Gather feedback on design documents
2. **Prototype**: Build minimal CLI with core commands
3. **Database Layer**: Implement SQLite registry
4. **GitHub Client**: Build GitHub API integration
5. **TUI Development**: Implement ratatui-based interface
6. **Testing**: Build test suite following strategy document
7. **Documentation**: Write user documentation
8. **Release**: Package and distribute

## Getting Started (Post-Implementation)

```bash
# Install
cargo install claude-skill-manager

# Initialize
csm init

# Add a skill
csm add github:anthropics/claude-skills/typescript

# List skills
csm list

# Launch TUI
csm ui
```

## Contributing

See the testing strategy document for information on running tests and contributing to the project.

## License

MIT License - See LICENSE file for details.
