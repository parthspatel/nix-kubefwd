# Claude Skill Manager - Use Cases

## Document Info
- **Version**: 1.0
- **Status**: Draft
- **Last Updated**: 2026-01-22

---

## 1. Actors

### 1.1 Actor Definitions

| Actor | Description | Examples |
|-------|-------------|----------|
| **Developer** | Primary user who manages Claude skills for personal or project use | Solo developer, team member |
| **Team Lead** | User who creates and distributes skills across a team | Tech lead, architect |
| **System (CSM)** | The Claude Skill Manager application itself | Background processes, schedulers |
| **GitHub** | External system providing skill repositories | GitHub API |
| **Claude Code** | External system that consumes the generated CLAUDE.md files | Claude Code CLI |
| **File System** | Local storage for skills, config, and database | OS filesystem |

### 1.2 Actor Hierarchy

```
                    ┌─────────────┐
                    │    User     │
                    └──────┬──────┘
                           │
              ┌────────────┴────────────┐
              │                         │
       ┌──────▼──────┐          ┌───────▼───────┐
       │  Developer  │          │   Team Lead   │
       └─────────────┘          └───────────────┘
                                       │
                                       │ extends
                                       ▼
                                ┌─────────────┐
                                │  Developer  │
                                └─────────────┘
```

---

## 2. Use Case Diagram - Overview

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                     │
│                          Claude Skill Manager System                                │
│                                                                                     │
│  ┌─────────────────────────────────────────────────────────────────────────────┐   │
│  │                                                                             │   │
│  │                         Skill Management                                    │   │
│  │                                                                             │   │
│  │    ┌───────────────┐      ┌───────────────┐      ┌───────────────┐        │   │
│  │    │  UC-01: Init  │      │ UC-02: Add    │      │ UC-03: Remove │        │   │
│  │    │   System      │      │   Skill       │      │   Skill       │        │   │
│  │    └───────┬───────┘      └───────┬───────┘      └───────────────┘        │   │
│  │            │                      │                                        │   │
│  │            │              ┌───────┴───────┐                                │   │
│  │            │              │   «include»   │                                │   │
│  │            │              ▼               ▼                                │   │
│  │            │      ┌─────────────┐ ┌─────────────┐                         │   │
│  │            │      │ UC-04: Fetch│ │ UC-05:      │                         │   │
│ ┌┴┐           │      │ from GitHub │ │ Validate    │                         │   │
│ │D│           │      └─────────────┘ └─────────────┘                         │   │
│ │e│           │                                                               │   │
│ │v│    ┌──────┴──────────────────────────────────────────────┐               │   │
│ │e│    │                                                      │               │   │
│ │l│    │    ┌───────────────┐      ┌───────────────┐         │               │   │
│ │o│    │    │ UC-06: List   │      │ UC-07: Enable │         │               │   │
│ │p│    │    │   Skills      │      │   /Disable    │         │               │   │
│ │e│    │    └───────────────┘      └───────────────┘         │               │   │
│ │r│    │                                                      │               │   │
│ └┬┘    │    ┌───────────────┐      ┌───────────────┐         │               │   │
│  │     │    │ UC-08: Update │      │ UC-09: Search │         │               │   │
│  │     │    │   Skills      │      │   Skills      │         │               │   │
│  │     │    └───────┬───────┘      └───────────────┘         │               │   │
│  │     │            │                                         │               │   │
│  │     └────────────┼─────────────────────────────────────────┘               │   │
│  │                  │ «extend»                                                │   │
│  │                  ▼                                                         │   │
│  │          ┌─────────────┐                                                   │   │
│  │          │ UC-10: Auto │◄──────────┐                                      │   │
│  │          │   Update    │           │                                      │   │
│  │          └─────────────┘          ┌┴┐                                     │   │
│  │                                   │S│                                     │   │
│  │                                   │y│                                     │   │
│  │                                   │s│                                     │   │
│  └───────────────────────────────────┤t├─────────────────────────────────────┘   │
│                                      │e│                                         │
│  ┌───────────────────────────────────┤m├─────────────────────────────────────┐   │
│  │                                   └┬┘                                     │   │
│  │                      Conflict Management                                  │   │
│  │                                    │                                      │   │
│  │    ┌───────────────┐      ┌───────┴───────┐      ┌───────────────┐       │   │
│  │    │ UC-11: Detect │      │ UC-12: Resolve│      │ UC-13: Set    │       │   │
│ ┌┴┐   │   Conflicts   │      │   Conflicts   │      │   Priority    │       │   │
│ │D│   └───────────────┘      └───────────────┘      └───────────────┘       │   │
│ │e│                                                                          │   │
│ │v│                                                                          │   │
│ └┬┘                                                                          │   │
│  │   └───────────────────────────────────────────────────────────────────────┘   │
│  │                                                                               │
│  │   ┌───────────────────────────────────────────────────────────────────────┐   │
│  │   │                                                                       │   │
│  │   │                      Sync & Export                                    │   │
│  │   │                                                                       │   │
│  │   │    ┌───────────────┐      ┌───────────────┐      ┌───────────────┐   │   │
│ ┌┴┐  │    │ UC-14: Merge  │      │ UC-15: Export │      │ UC-16: Import │   │   │
│ │D│  │    │   Skills      │      │   Skills      │      │   Skills      │   │   │
│ │e│  │    └───────────────┘      └───────────────┘      └───────────────┘   │   │
│ │v│  │                                                                       │   │
│ └─┘  │    ┌───────────────┐      ┌───────────────┐                          │   │
│      │    │ UC-17: Sync   │      │ UC-18: Create │                          │   │
│ ┌─┐  │    │   to Project  │      │   Skill       │                          │   │
│ │T│  │    └───────────────┘      └───────────────┘                          │   │
│ │e│  │                                                                       │   │
│ │a│  └───────────────────────────────────────────────────────────────────────┘   │
│ │m│                                                                               │
│ │ │  ┌───────────────────────────────────────────────────────────────────────┐   │
│ │L│  │                                                                       │   │
│ │e│  │                      Team Collaboration                               │   │
│ │a│  │                                                                       │   │
│ │d│  │    ┌───────────────┐      ┌───────────────┐                          │   │
│ └┬┘  │    │ UC-19: Share  │      │ UC-20: Publish│                          │   │
│  │   │    │   with Team   │      │   Skill       │                          │   │
│  │   │    └───────────────┘      └───────────────┘                          │   │
│  │   │                                                                       │   │
│  │   └───────────────────────────────────────────────────────────────────────┘   │
│  │                                                                               │
└──┴───────────────────────────────────────────────────────────────────────────────┘

                              External Systems
                   ┌──────────────────────────────────────┐
                   │                                      │
                   │    ┌────────┐      ┌────────────┐   │
                   │    │ GitHub │      │ Claude Code│   │
                   │    │  API   │      │            │   │
                   │    └────────┘      └────────────┘   │
                   │                                      │
                   └──────────────────────────────────────┘
```

---

## 3. Detailed Use Case Specifications

### UC-01: Initialize System

#### UML Diagram
```
┌─────────────────────────────────────────────────────────────┐
│                    UC-01: Initialize System                 │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ┌───┐                    ┌─────────────────┐             │
│   │   │                    │                 │             │
│   │ D │───────────────────►│   Initialize    │             │
│   │ e │                    │     System      │             │
│   │ v │                    │                 │             │
│   │   │                    └────────┬────────┘             │
│   └───┘                             │                      │
│                                     │ «include»            │
│                           ┌─────────┴─────────┐            │
│                           │                   │            │
│                           ▼                   ▼            │
│                   ┌───────────────┐   ┌───────────────┐    │
│                   │ Create Config │   │ Import        │    │
│                   │ Directory     │   │ Existing      │    │
│                   └───────────────┘   └───────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### Textual Specification

| Field | Description |
|-------|-------------|
| **Use Case ID** | UC-01 |
| **Use Case Name** | Initialize System |
| **Actor(s)** | Developer |
| **Description** | Set up CSM for first-time use, creating necessary directories, database, and configuration |
| **Trigger** | User runs `csm init` or launches TUI for the first time |
| **Preconditions** | CSM binary is installed; User has write access to home directory |
| **Postconditions** | CSM configuration directory exists; Database is initialized; Existing skills are optionally imported |

**Main Flow:**
1. System checks if CSM is already initialized
2. System creates `~/.csm/` directory structure
3. System initializes SQLite database with schema
4. System creates default `config.toml`
5. System scans for existing CLAUDE.md files
6. System prompts user to import discovered skills
7. User selects skills to import (or skips)
8. System imports selected skills to registry
9. System displays success message

**Alternative Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| 1a | Already initialized | System displays warning; Asks for `--force` to reinitialize |
| 5a | No existing skills found | System skips import prompt; Proceeds to success |
| 6a | User runs with `--yes` flag | System auto-imports all discovered skills |
| 6b | User runs with `--skip-import` | System skips import step entirely |

**Exception Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| E1 | No write permission | System displays error; Suggests running with proper permissions |
| E2 | Database creation fails | System displays error; Cleans up partial state |

---

### UC-02: Add Skill

#### UML Diagram
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           UC-02: Add Skill                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌───┐                        ┌─────────────────┐                         │
│   │   │                        │                 │                         │
│   │ D │───────────────────────►│    Add Skill    │                         │
│   │ e │                        │                 │                         │
│   │ v │                        └────────┬────────┘                         │
│   │   │                                 │                                  │
│   └───┘                    ┌────────────┼────────────┐                     │
│                            │            │            │                     │
│                   «include»│   «include»│   «include»│                     │
│                            ▼            ▼            ▼                     │
│                    ┌────────────┐┌────────────┐┌────────────┐              │
│                    │Parse Source││  Validate  ││  Register  │              │
│                    │    URL     ││  Content   ││  in DB     │              │
│                    └─────┬──────┘└────────────┘└────────────┘              │
│                          │                                                  │
│            ┌─────────────┼─────────────┐                                   │
│            │             │             │                                   │
│   «extend» │    «extend» │    «extend» │                                   │
│            ▼             ▼             ▼                                   │
│     ┌────────────┐┌────────────┐┌────────────┐                             │
│     │Fetch GitHub││ Read Local ││ Fetch URL  │                             │
│     │    Repo    ││   File     ││  Content   │                             │
│     └─────┬──────┘└────────────┘└────────────┘                             │
│           │                                                                 │
│           │                                            ┌────────┐          │
│           └───────────────────────────────────────────►│ GitHub │          │
│                                                        │  API   │          │
│                                                        └────────┘          │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Textual Specification

| Field | Description |
|-------|-------------|
| **Use Case ID** | UC-02 |
| **Use Case Name** | Add Skill |
| **Actor(s)** | Developer |
| **Description** | Add a new skill from various sources (GitHub, local file, URL) |
| **Trigger** | User runs `csm add <source>` or uses TUI "Add Skill" action |
| **Preconditions** | CSM is initialized; Source is accessible |
| **Postconditions** | Skill is registered in database; Skill content is stored; CLAUDE.md is regenerated |

**Main Flow:**
1. User provides skill source specification
2. System parses source type (GitHub, local, URL)
3. System fetches skill content from source
4. System validates skill content format
5. System checks for conflicts with existing skills
6. System prompts for scope (global/local)
7. System registers skill in database
8. System stores skill content in `~/.csm/skills/`
9. System creates symlinks as needed
10. System regenerates merged CLAUDE.md
11. System displays success with skill details

**Alternative Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| 5a | Conflicts detected | System displays conflicts; Prompts for resolution; Continues after resolution |
| 6a | `--scope` flag provided | System uses specified scope; Skips prompt |
| 6b | `--global` flag | System sets scope to global |
| 9a | Skill already exists in project | System updates symlink if needed |

**Exception Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| E1 | Invalid source format | System displays parse error with examples |
| E2 | Network error (GitHub/URL) | System displays network error; Suggests retry |
| E3 | Skill not found at source | System displays "not found" error |
| E4 | Validation fails | System displays validation errors; Does not add skill |
| E5 | Skill name already exists | System prompts for new name or `--force` to replace |

**Business Rules:**
- GitHub sources must be in format `github:owner/repo[/path][@ref]`
- Local files must exist and be readable
- Skill content must be valid UTF-8 markdown
- Skill names must be unique within scope

---

### UC-08: Update Skills

#### UML Diagram
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          UC-08: Update Skills                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌───┐                        ┌─────────────────┐                         │
│   │   │                        │                 │                         │
│   │ D │───────────────────────►│  Update Skills  │◄─ ─ ─ ─ ─┐              │
│   │ e │                        │                 │          │              │
│   │ v │                        └────────┬────────┘          │ «trigger»    │
│   │   │                                 │                   │              │
│   └───┘                                 │              ┌────┴────┐         │
│                            ┌────────────┼────────────┐ │ System  │         │
│                            │            │            │ │Scheduler│         │
│                   «include»│   «include»│   «include»│ └─────────┘         │
│                            ▼            ▼            ▼                     │
│                    ┌────────────┐┌────────────┐┌────────────┐              │
│                    │   Check    ││   Fetch    ││   Apply    │              │
│                    │  Updates   ││    New     ││  Updates   │              │
│                    └─────┬──────┘└────────────┘└────────────┘              │
│                          │                                                  │
│                          │ «extend»                                        │
│                          ▼                                                  │
│                    ┌────────────┐                                          │
│                    │  Notify    │                                          │
│                    │   User     │                                          │
│                    └────────────┘                                          │
│                                                        ┌────────┐          │
│                          ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─►│ GitHub │          │
│                                                        │  API   │          │
│                                                        └────────┘          │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Textual Specification

| Field | Description |
|-------|-------------|
| **Use Case ID** | UC-08 |
| **Use Case Name** | Update Skills |
| **Actor(s)** | Developer, System (Scheduler) |
| **Description** | Check for and apply updates to skills from their sources |
| **Trigger** | User runs `csm update`, TUI update action, or scheduled auto-update |
| **Preconditions** | CSM is initialized; At least one skill with remote source exists |
| **Postconditions** | Skills are updated to latest versions; Update history is recorded |

**Main Flow:**
1. System identifies skills to check (all or specified)
2. For each skill with remote source:
   - System queries source for current version (commit SHA, ETag)
   - System compares with stored version
3. System compiles list of available updates
4. System displays update summary to user
5. User confirms updates to apply
6. For each update to apply:
   - System creates backup of current content
   - System fetches new content
   - System validates new content
   - System replaces stored content
   - System updates database record
7. System regenerates merged CLAUDE.md
8. System displays success summary

**Alternative Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| 4a | No updates available | System displays "all skills up to date" |
| 5a | `--yes` flag provided | System applies all updates without confirmation |
| 5b | `--check` flag provided | System displays available updates; Does not apply |
| 5c | Auto-update mode is "auto" | System applies updates without prompting |
| 5d | Auto-update mode is "notify" | System sends notification; Waits for manual update |
| 6a | Update fails validation | System restores backup; Reports error; Continues with other updates |

**Exception Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| E1 | Network error | System displays error; Retries with backoff; Skips unreachable sources |
| E2 | Rate limited (GitHub) | System displays rate limit warning; Suggests waiting or using token |
| E3 | Source deleted/moved | System marks skill as "orphaned"; Suggests removal |

**Business Rules:**
- Updates only apply to skills with remote sources (GitHub, URL)
- Local-only skills are never updated via this flow
- Backup is always created before update
- Failed updates do not affect other skills

---

### UC-11: Detect Conflicts

#### UML Diagram
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        UC-11: Detect Conflicts                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌───┐                        ┌─────────────────┐                         │
│   │   │                        │                 │                         │
│   │ D │───────────────────────►│    Detect       │                         │
│   │ e │                        │   Conflicts     │                         │
│   │ v │                        │                 │                         │
│   │   │                        └────────┬────────┘                         │
│   └───┘                                 │                                  │
│                                         │ «include»                        │
│                            ┌────────────┼────────────┐                     │
│                            │            │            │                     │
│                            ▼            ▼            ▼                     │
│                    ┌────────────┐┌────────────┐┌────────────┐              │
│                    │   Find     ││   Find     ││   Find     │              │
│                    │ Duplicates ││Contradicts ││  Overlaps  │              │
│                    └────────────┘└────────────┘└────────────┘              │
│                                                                             │
│                                         │                                  │
│                                         ▼                                  │
│                                 ┌───────────────┐                          │
│                                 │    Report     │                          │
│                                 │   Conflicts   │                          │
│                                 └───────────────┘                          │
│                                                                             │
│   Automatic Triggers:                                                       │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                                                                     │  │
│   │   ┌─────────┐      ┌─────────┐      ┌─────────┐                    │  │
│   │   │Add Skill│─────►│ Detect  │◄─────│ Update  │                    │  │
│   │   │ (UC-02) │      │Conflicts│      │ (UC-08) │                    │  │
│   │   └─────────┘      └─────────┘      └─────────┘                    │  │
│   │                          ▲                                          │  │
│   │                          │                                          │  │
│   │                    ┌─────────┐                                      │  │
│   │                    │ Enable  │                                      │  │
│   │                    │ (UC-07) │                                      │  │
│   │                    └─────────┘                                      │  │
│   │                                                                     │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Textual Specification

| Field | Description |
|-------|-------------|
| **Use Case ID** | UC-11 |
| **Use Case Name** | Detect Conflicts |
| **Actor(s)** | Developer, System |
| **Description** | Analyze enabled skills to identify conflicting instructions |
| **Trigger** | User runs `csm conflicts`, skill is added/enabled, or skill is updated |
| **Preconditions** | CSM is initialized; At least two enabled skills exist |
| **Postconditions** | Conflicts are identified and stored; Report is generated |

**Main Flow:**
1. System loads all enabled skills
2. System loads skill content from storage
3. For each pair of skills:
   - System parses instructions from content
   - System compares for exact duplicates
   - System compares for semantic contradictions
   - System checks for scope overlaps
4. System stores detected conflicts in database
5. System generates conflict report
6. System displays report to user

**Alternative Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| 3a | No conflicts found | System displays "no conflicts detected" |
| 5a | `--json` flag provided | System outputs JSON format |
| 5b | Called from UC-02 (Add) | System returns conflicts to caller; No direct display |

**Conflict Types:**

| Type | Detection Logic |
|------|-----------------|
| **Duplicate** | Exact or near-exact match of instruction text |
| **Contradictory** | Opposing keywords (always/never, must/must not) on same topic |
| **Overlap** | Same scope targeting same aspect with different values |
| **Structural** | Incompatible section structures or formats |

---

### UC-12: Resolve Conflicts

#### UML Diagram
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        UC-12: Resolve Conflicts                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌───┐                        ┌─────────────────┐                         │
│   │   │                        │                 │                         │
│   │ D │───────────────────────►│    Resolve      │                         │
│   │ e │                        │   Conflicts     │                         │
│   │ v │                        │                 │                         │
│   │   │                        └────────┬────────┘                         │
│   └───┘                                 │                                  │
│                            ┌────────────┴────────────┐                     │
│                            │                         │                     │
│                   «extend» │                «extend» │                     │
│                            ▼                         ▼                     │
│                    ┌────────────────┐        ┌────────────────┐            │
│                    │   Interactive  │        │     Batch      │            │
│                    │   Resolution   │        │   Resolution   │            │
│                    └───────┬────────┘        └────────────────┘            │
│                            │                                               │
│           ┌────────────────┼────────────────┬────────────────┐            │
│           │                │                │                │            │
│           ▼                ▼                ▼                ▼            │
│   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐      │
│   │  Disable    │  │    Set      │  │    Edit     │  │   Ignore    │      │
│   │  One Skill  │  │  Priority   │  │   Skills    │  │  Conflict   │      │
│   └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘      │
│                                                                             │
│                            │                                               │
│                            ▼                                               │
│                    ┌────────────────┐                                      │
│                    │   Regenerate   │                                      │
│                    │   CLAUDE.md    │                                      │
│                    └────────────────┘                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Textual Specification

| Field | Description |
|-------|-------------|
| **Use Case ID** | UC-12 |
| **Use Case Name** | Resolve Conflicts |
| **Actor(s)** | Developer |
| **Description** | Interactively resolve detected conflicts between skills |
| **Trigger** | User runs `csm conflicts --resolve` or uses TUI conflict resolver |
| **Preconditions** | Conflicts have been detected (UC-11) |
| **Postconditions** | Conflicts are marked resolved or ignored; Skills are modified as needed |

**Main Flow:**
1. System loads unresolved conflicts
2. For each conflict:
   - System displays conflict details (both skills, lines, content)
   - System presents resolution options
   - User selects resolution strategy
   - System applies resolution
   - System marks conflict as resolved
3. System regenerates merged CLAUDE.md
4. System displays resolution summary

**Resolution Strategies:**

| Strategy | Action |
|----------|--------|
| **Disable Skill A** | Disables first skill; Keeps second active |
| **Disable Skill B** | Disables second skill; Keeps first active |
| **Set Priority** | Keeps both enabled; Sets priority so one overrides |
| **Edit Skill A** | Opens editor for first skill; Re-validates after |
| **Edit Skill B** | Opens editor for second skill; Re-validates after |
| **Ignore** | Marks conflict as ignored; Both skills remain |

**Alternative Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| 1a | No unresolved conflicts | System displays "no conflicts to resolve" |
| 2a | User presses 's' (skip) | System moves to next conflict |
| 2b | User presses 'q' (quit) | System saves progress; Exits resolver |
| 3a | Edit introduces new conflict | System detects and adds to queue |

---

### UC-14: Merge Skills

#### UML Diagram
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          UC-14: Merge Skills                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────┐                  ┌─────────────────┐                         │
│   │ System  │                  │                 │                         │
│   │ (Auto)  │─────────────────►│  Merge Skills   │                         │
│   └─────────┘                  │                 │                         │
│                                └────────┬────────┘                         │
│   ┌───┐                                 │                                  │
│   │ D │─────────(manual)───────────────►│                                  │
│   │ e │                                 │                                  │
│   │ v │                                 │                                  │
│   └───┘                                 │                                  │
│                            ┌────────────┼────────────┐                     │
│                            │            │            │                     │
│                   «include»│   «include»│   «include»│                     │
│                            ▼            ▼            ▼                     │
│                    ┌────────────┐┌────────────┐┌────────────┐              │
│                    │   Sort by  ││  Parse     ││  Combine   │              │
│                    │  Priority  ││  Sections  ││  Sections  │              │
│                    └────────────┘└────────────┘└────────────┘              │
│                                                                             │
│                                         │                                  │
│                                         ▼                                  │
│                                 ┌───────────────┐                          │
│                                 │    Write      │                          │
│                                 │  CLAUDE.md    │                          │
│                                 └───────┬───────┘                          │
│                                         │                                  │
│                                         ▼                                  │
│                                 ┌───────────────┐                          │
│                                 │ Claude Code   │                          │
│                                 │  (consumer)   │                          │
│                                 └───────────────┘                          │
│                                                                             │
│   Triggered By:                                                            │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │ UC-02: Add │ UC-03: Remove │ UC-07: Enable │ UC-08: Update │ etc.  │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Textual Specification

| Field | Description |
|-------|-------------|
| **Use Case ID** | UC-14 |
| **Use Case Name** | Merge Skills |
| **Actor(s)** | System (automatically triggered) |
| **Description** | Combine all enabled skills into a single CLAUDE.md file |
| **Trigger** | Any operation that modifies skill state (add, remove, enable, update) |
| **Preconditions** | CSM is initialized |
| **Postconditions** | CLAUDE.md reflects current enabled skills in priority order |

**Main Flow:**
1. System loads all enabled skills for the target scope
2. System sorts skills by priority (descending)
3. For each skill:
   - System reads skill content
   - System parses content into sections
4. System merges sections with same headings
5. System builds final merged content
6. System adds metadata header (generator notice)
7. System writes CLAUDE.md file
8. System updates symlinks if needed

**Merge Rules:**
- Higher priority skills appear first
- Sections with same heading are merged (higher priority content first)
- Global skills are merged before local skills (unless overridden by priority)
- Comments and metadata from original skills are preserved

---

### UC-17: Sync to Project

#### UML Diagram
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        UC-17: Sync to Project                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌───┐                        ┌─────────────────┐                         │
│   │   │                        │                 │                         │
│   │ D │───────────────────────►│    Sync to      │                         │
│   │ e │                        │    Project      │                         │
│   │ v │                        │                 │                         │
│   │   │                        └────────┬────────┘                         │
│   └───┘                                 │                                  │
│                                         │                                  │
│                            ┌────────────┼────────────┐                     │
│                            │            │            │                     │
│                   «include»│   «include»│   «include»│                     │
│                            ▼            ▼            ▼                     │
│                    ┌────────────┐┌────────────┐┌────────────┐              │
│                    │  Resolve   ││  Create    ││  Generate  │              │
│                    │  Project   ││  Symlinks  ││ CLAUDE.md  │              │
│                    └────────────┘└────────────┘└────────────┘              │
│                                                                             │
│                    Symlink Structure:                                       │
│                    ┌────────────────────────────────────────────────────┐  │
│                    │                                                    │  │
│                    │  ~/project/                                        │  │
│                    │  ├── CLAUDE.md ──────► .csm/merged/CLAUDE.md      │  │
│                    │  └── .csm/                                         │  │
│                    │      └── skills/                                   │  │
│                    │          ├── skill-a/ ──► ~/.csm/skills/skill-a/  │  │
│                    │          └── skill-b/ ──► ~/.csm/skills/skill-b/  │  │
│                    │                                                    │  │
│                    └────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Textual Specification

| Field | Description |
|-------|-------------|
| **Use Case ID** | UC-17 |
| **Use Case Name** | Sync to Project |
| **Actor(s)** | Developer |
| **Description** | Synchronize skill configuration to a specific project directory |
| **Trigger** | User runs `csm sync` in a project directory, or `csm local init` |
| **Preconditions** | CSM is initialized; Current directory is a valid project |
| **Postconditions** | Project has `.csm/` directory; Symlinks point to skill sources; CLAUDE.md is generated |

**Main Flow:**
1. System detects current project directory
2. System creates `.csm/` directory in project if not exists
3. System identifies skills assigned to this project (local scope)
4. System identifies global skills that apply
5. For each applicable skill:
   - System creates symlink from `.csm/skills/<name>` to `~/.csm/skills/<name>`
6. System merges skills (UC-14) to generate CLAUDE.md
7. System creates symlink or file for CLAUDE.md
8. System optionally updates `.gitignore` for generated files

**Alternative Flows:**

| ID | Condition | Flow |
|----|-----------|------|
| 2a | `.csm/` already exists | System verifies structure; Updates as needed |
| 5a | Skill already symlinked | System verifies symlink target; Updates if changed |
| 8a | `--no-gitignore` flag | System skips .gitignore modification |
| 8b | .gitignore already has entries | System does not duplicate entries |

---

### UC-19: Share with Team

#### UML Diagram
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        UC-19: Share with Team                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌───────┐                    ┌─────────────────┐                         │
│   │ Team  │                    │                 │                         │
│   │ Lead  │───────────────────►│   Share with    │                         │
│   │       │                    │     Team        │                         │
│   └───────┘                    └────────┬────────┘                         │
│                                         │                                  │
│                            ┌────────────┴────────────┐                     │
│                            │                         │                     │
│                   «extend» │                «extend» │                     │
│                            ▼                         ▼                     │
│                    ┌────────────────┐        ┌────────────────┐            │
│                    │  Commit to     │        │   Share via    │            │
│                    │    Repo        │        │   Export URL   │            │
│                    └───────┬────────┘        └────────────────┘            │
│                            │                                               │
│                            ▼                                               │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                                                                     │  │
│   │   Team Member Workflow:                                             │  │
│   │                                                                     │  │
│   │   ┌─────────┐      ┌─────────┐      ┌─────────┐      ┌─────────┐   │  │
│   │   │  Clone  │─────►│  csm    │─────►│  Skills │─────►│  Use    │   │  │
│   │   │  Repo   │      │  sync   │      │ Imported│      │ Skills  │   │  │
│   │   └─────────┘      └─────────┘      └─────────┘      └─────────┘   │  │
│   │                                                                     │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│   Repository Structure:                                                     │
│   ┌─────────────────────────────────────────────────────────────────────┐  │
│   │                                                                     │  │
│   │   my-project/                                                       │  │
│   │   ├── .claude/                   # Committed to repo               │  │
│   │   │   └── skills/                                                   │  │
│   │   │       └── team-standards/                                       │  │
│   │   │           └── CLAUDE.md      # Team skill content              │  │
│   │   ├── .csm/                      # Local only (gitignored)         │  │
│   │   │   └── config.toml                                               │  │
│   │   ├── CLAUDE.md                  # Generated (gitignored)          │  │
│   │   └── src/                                                          │  │
│   │                                                                     │  │
│   └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Textual Specification

| Field | Description |
|-------|-------------|
| **Use Case ID** | UC-19 |
| **Use Case Name** | Share with Team |
| **Actor(s)** | Team Lead |
| **Description** | Share skills with team members via repository or export |
| **Trigger** | Team lead wants to distribute skills to team members |
| **Preconditions** | Skills exist in CSM; Team has shared repository |
| **Postconditions** | Skills are accessible to team members |

**Main Flow (Repository Method):**
1. Team lead creates/selects skill to share
2. Team lead exports skill to `.claude/skills/` directory
3. Team lead commits skill files to repository
4. Team lead pushes to shared repository
5. Team members pull repository updates
6. Team members run `csm sync`
7. CSM detects skills in `.claude/skills/`
8. CSM imports skills to local registry
9. CSM generates CLAUDE.md with team skills

**Alternative Flow (Export URL Method):**
1. Team lead exports skill as URL/gist
2. Team lead shares URL with team
3. Team members run `csm add <url>`
4. Skills are added to each member's CSM

---

## 4. Use Case Relationships

### 4.1 Include Relationships

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        «include» Relationships                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   UC-02: Add Skill                                                          │
│       │                                                                     │
│       ├──«include»──► Parse Source                                         │
│       ├──«include»──► Fetch Content (GitHub/Local/URL)                     │
│       ├──«include»──► Validate Content                                     │
│       ├──«include»──► UC-11: Detect Conflicts                              │
│       └──«include»──► UC-14: Merge Skills                                  │
│                                                                             │
│   UC-08: Update Skills                                                      │
│       │                                                                     │
│       ├──«include»──► Check for Updates                                    │
│       ├──«include»──► Fetch New Content                                    │
│       ├──«include»──► Validate Content                                     │
│       ├──«include»──► UC-11: Detect Conflicts                              │
│       └──«include»──► UC-14: Merge Skills                                  │
│                                                                             │
│   UC-11: Detect Conflicts                                                   │
│       │                                                                     │
│       ├──«include»──► Find Duplicates                                      │
│       ├──«include»──► Find Contradictions                                  │
│       └──«include»──► Find Overlaps                                        │
│                                                                             │
│   UC-14: Merge Skills                                                       │
│       │                                                                     │
│       ├──«include»──► Sort by Priority                                     │
│       ├──«include»──► Parse Sections                                       │
│       └──«include»──► Write CLAUDE.md                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Extend Relationships

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        «extend» Relationships                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   UC-02: Add Skill                                                          │
│       │                                                                     │
│       ◄──«extend»── Fetch from GitHub (when source is github:)             │
│       ◄──«extend»── Read Local File (when source is path)                  │
│       ◄──«extend»── Fetch from URL (when source is http/https)             │
│                                                                             │
│   UC-08: Update Skills                                                      │
│       │                                                                     │
│       ◄──«extend»── UC-10: Auto Update (when triggered by scheduler)       │
│       ◄──«extend»── Notify User (when update-mode is "notify")             │
│                                                                             │
│   UC-12: Resolve Conflicts                                                  │
│       │                                                                     │
│       ◄──«extend»── Interactive Resolution (default)                       │
│       ◄──«extend»── Batch Resolution (when --batch flag)                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.3 Trigger Relationships

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Trigger Relationships                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   These use cases automatically trigger other use cases:                    │
│                                                                             │
│   ┌─────────────────┐                                                       │
│   │ UC-02: Add      │──────triggers────►┌─────────────────────────────┐    │
│   │                 │                    │ UC-11: Detect Conflicts     │    │
│   └─────────────────┘                    │ UC-14: Merge Skills         │    │
│                                          └─────────────────────────────┘    │
│                                                                             │
│   ┌─────────────────┐                                                       │
│   │ UC-03: Remove   │──────triggers────►┌─────────────────────────────┐    │
│   │                 │                    │ UC-14: Merge Skills         │    │
│   └─────────────────┘                    └─────────────────────────────┘    │
│                                                                             │
│   ┌─────────────────┐                                                       │
│   │ UC-07: Enable/  │──────triggers────►┌─────────────────────────────┐    │
│   │       Disable   │                    │ UC-11: Detect Conflicts     │    │
│   └─────────────────┘                    │ UC-14: Merge Skills         │    │
│                                          └─────────────────────────────┘    │
│                                                                             │
│   ┌─────────────────┐                                                       │
│   │ UC-08: Update   │──────triggers────►┌─────────────────────────────┐    │
│   │                 │                    │ UC-11: Detect Conflicts     │    │
│   └─────────────────┘                    │ UC-14: Merge Skills         │    │
│                                          └─────────────────────────────┘    │
│                                                                             │
│   ┌─────────────────┐                                                       │
│   │ UC-12: Resolve  │──────triggers────►┌─────────────────────────────┐    │
│   │                 │                    │ UC-14: Merge Skills         │    │
│   └─────────────────┘                    └─────────────────────────────┘    │
│                                                                             │
│   ┌─────────────────┐                                                       │
│   │ System Scheduler│──────triggers────►┌─────────────────────────────┐    │
│   │                 │                    │ UC-10: Auto Update          │    │
│   └─────────────────┘                    └─────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Use Case Priority Matrix

| Use Case | Priority | Complexity | Dependencies |
|----------|----------|------------|--------------|
| UC-01: Initialize System | P0 | Low | None |
| UC-02: Add Skill | P0 | High | UC-01, UC-04, UC-05, UC-11, UC-14 |
| UC-03: Remove Skill | P0 | Low | UC-01, UC-14 |
| UC-04: Fetch from GitHub | P0 | Medium | None |
| UC-05: Validate Content | P0 | Medium | None |
| UC-06: List Skills | P0 | Low | UC-01 |
| UC-07: Enable/Disable | P0 | Low | UC-01, UC-11, UC-14 |
| UC-08: Update Skills | P1 | Medium | UC-01, UC-04, UC-11, UC-14 |
| UC-09: Search Skills | P1 | Medium | UC-01 |
| UC-10: Auto Update | P1 | Medium | UC-08 |
| UC-11: Detect Conflicts | P0 | High | UC-01 |
| UC-12: Resolve Conflicts | P0 | Medium | UC-11, UC-14 |
| UC-13: Set Priority | P1 | Low | UC-01 |
| UC-14: Merge Skills | P0 | High | UC-01 |
| UC-15: Export Skills | P1 | Medium | UC-01 |
| UC-16: Import Skills | P1 | Medium | UC-01, UC-02 |
| UC-17: Sync to Project | P0 | Medium | UC-01, UC-14 |
| UC-18: Create Skill | P1 | Low | UC-01 |
| UC-19: Share with Team | P2 | Medium | UC-15 |
| UC-20: Publish Skill | P2 | High | UC-18 |

---

## 6. Traceability Matrix

| Use Case | CLI Command | TUI Screen | API Endpoint |
|----------|-------------|------------|--------------|
| UC-01 | `csm init` | First-run wizard | `init()` |
| UC-02 | `csm add` | Add Skill form | `add_skill()` |
| UC-03 | `csm remove` | Skill detail (delete) | `remove_skill()` |
| UC-06 | `csm list` | Skills list | `list_skills()` |
| UC-07 | `csm enable/disable` | Skill list (toggle) | `toggle_skill()` |
| UC-08 | `csm update` | Updates screen | `update_skills()` |
| UC-09 | `csm search` | Search bar | `search_skills()` |
| UC-11 | `csm conflicts` | Conflicts screen | `detect_conflicts()` |
| UC-12 | `csm conflicts --resolve` | Conflict resolver | `resolve_conflict()` |
| UC-13 | `csm priority` | Skill detail | `set_priority()` |
| UC-14 | (automatic) | (automatic) | `merge_skills()` |
| UC-15 | `csm export` | Export dialog | `export_skills()` |
| UC-16 | `csm import` | Import dialog | `import_skills()` |
| UC-17 | `csm sync` | Dashboard action | `sync_project()` |
| UC-18 | `csm create` | Create form | `create_skill()` |
