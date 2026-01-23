# Changelog

All notable changes to Claude Skill Manager will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project structure and architecture
- Core domain models (Skill, Source, Conflict)
- SQLite-based skill registry
- File system skill storage with symlink support
- GitHub API client for remote skill fetching
- CLI interface with clap v4
- Basic TUI framework with ratatui
- Configuration management system
- Skill add/remove/list/show commands
- Enable/disable skill functionality
- Update checking and application
- Conflict detection system
- Doctor command for diagnostics
- Shell completion generation
- Export/import functionality
- Comprehensive documentation

### Changed

- N/A (initial release)

### Deprecated

- N/A (initial release)

### Removed

- N/A (initial release)

### Fixed

- N/A (initial release)

### Security

- N/A (initial release)

## [0.1.0] - Unreleased

Initial release of Claude Skill Manager.

### Features

- **Skill Registry**: Central SQLite database for tracking all skills
- **Multiple Sources**: Support for GitHub, local files, and URLs
- **Scope Management**: Global and project-level skills
- **Update System**: Automatic, notify, and manual update modes
- **Conflict Detection**: Identify overlapping skill definitions
- **CLI**: Full-featured command-line interface
- **TUI**: Interactive terminal user interface (basic)

### Supported Platforms

- macOS (x86_64, ARM64)
- Linux (x86_64, ARM64)
- Windows 10+ (x86_64)

---

[Unreleased]: https://github.com/anthropics/claude-skill-manager/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/anthropics/claude-skill-manager/releases/tag/v0.1.0
