# Claude Skill Manager

**Unified skill management for Claude AI**

Claude Skill Manager (CSM) is a powerful CLI and TUI application that transforms how developers manage Claude AI customizations across projects and workstations.

```{image} https://img.shields.io/badge/version-0.1.0-blue
:alt: Version
```

```{image} https://img.shields.io/badge/license-MIT-green
:alt: License
```

```{image} https://img.shields.io/badge/rust-1.75+-orange
:alt: Rust Version
```

## Why Claude Skill Manager?

Managing Claude skills manually is fragmented and error-prone:

- Skills scattered across multiple locations (`~/.claude/`, project directories)
- No version control or update tracking
- Manual synchronization between projects
- Difficult to share and discover community skills

CSM solves these problems with a **centralized registry**, **automated updates**, and **unified management interface**.

## Key Features

::::{grid} 2
:gutter: 3

:::{grid-item-card} Centralized Registry
:class-card: sd-border-0

SQLite-backed database tracking all skills with full metadata, enabling powerful search and management capabilities.
:::

:::{grid-item-card} GitHub Integration
:class-card: sd-border-0

Install skills directly from GitHub repositories with automatic update tracking and configurable sync schedules.
:::

:::{grid-item-card} Global & Local Scopes
:class-card: sd-border-0

Define skills globally for all projects or locally for specific workspaces, with clear precedence rules.
:::

:::{grid-item-card} Conflict Detection
:class-card: sd-border-0

Automatic identification of overlapping or contradictory skill definitions with interactive resolution tools.
:::

::::

## Quick Example

```bash
# Initialize CSM
csm init

# Add a skill from GitHub
csm add github:anthropics/claude-skills/typescript-best-practices

# List installed skills
csm list

# Launch interactive TUI
csm ui
```

## Documentation

```{toctree}
:maxdepth: 2
:caption: User Guide

getting-started
configuration
cli
```

```{toctree}
:maxdepth: 2
:caption: Reference

api-reference
architecture
```

```{toctree}
:maxdepth: 2
:caption: Development

contributing
changelog
```

## Installation

### Using Cargo

```bash
cargo install claude-skill-manager
```

### Using Homebrew

```bash
brew install csm
```

### From Source

```bash
git clone https://github.com/anthropics/claude-skill-manager.git
cd claude-skill-manager
cargo install --path .
```

## Project Status

CSM is currently in active development. Core features are implemented:

| Feature | Status |
|---------|--------|
| Skill Registry | Implemented |
| GitHub Installation | Implemented |
| Global/Local Skills | Implemented |
| CLI Interface | Implemented |
| TUI Interface | In Progress |
| Auto-Update | Implemented |
| Conflict Detection | Implemented |

## Getting Help

- [GitHub Issues](https://github.com/anthropics/claude-skill-manager/issues) - Report bugs and request features
- [Discussions](https://github.com/anthropics/claude-skill-manager/discussions) - Ask questions and share ideas

## License

Claude Skill Manager is released under the MIT License. See the [LICENSE](https://github.com/anthropics/claude-skill-manager/blob/main/LICENSE) file for details.
