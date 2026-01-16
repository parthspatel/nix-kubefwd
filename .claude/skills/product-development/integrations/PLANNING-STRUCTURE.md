# Planning Structure

## Directory Structure

```
./planning/
└── {YYYYMMDD}_{project-name}/
    ├── PROJECT.md                    # Project overview & status
    ├── requirements/
    │   ├── PRODUCT-REQUIREMENTS.md   # Phase 1 output
    │   ├── TECHNICAL-REQUIREMENTS.md # Phase 2 output
    │   └── TRACEABILITY.md           # Phase 3 output
    ├── diagrams/                     # Phase 4 & 5 outputs
    │   ├── architecture/
    │   ├── behavior/
    │   ├── data/
    │   └── rendered/
    ├── design/
    │   ├── ARCHITECTURE.md           # Phase 5 output
    │   ├── INTERFACES.md             # Phase 6 output
    │   └── contracts/
    ├── epics/
    │   └── {epic-id}_{epic-name}/
    │       ├── EPIC.md               # Epic definition
    │       └── stories/
    │           └── {story-id}_{story-name}/
    │               ├── STORY.md      # Story definition
    │               └── tasks/
    │                   └── {task-id}_{task-name}.md
    └── docs/                         # Sphinx documentation source
        ├── conf.py
        ├── index.rst
        └── ...
```

## Naming Conventions

| Element | Format | Example |
|---------|--------|---------|
| Project | `{YYYYMMDD}_{kebab-case}` | `20260116_user-auth-service` |
| Epic | `{E###}_{kebab-case}` | `E001_user-management` |
| Story | `{S###}_{kebab-case}` | `S001_user-registration` |
| Task | `{T###}_{kebab-case}.md` | `T001_create-user-model.md` |

## Project Template

```markdown
# Project: {Project Name}

**Created**: {YYYY-MM-DD}
**Status**: 🟡 In Progress | 🟢 Complete | 🔴 Blocked | ⚪ Not Started
**Owner**: {Name}

## Overview

{Brief description of what this project delivers}

## Goals

- [ ] {Goal 1}
- [ ] {Goal 2}
- [ ] {Goal 3}

## Success Criteria

| Metric | Target | Current |
|--------|--------|---------|
| {Metric 1} | {Target} | {Current} |

## Timeline

| Phase | Status | Approved |
|-------|--------|----------|
| 1. Product Requirements | ⚪ | - |
| 2. Technical Requirements | ⚪ | - |
| 3. Requirements Integration | ⚪ | - |
| 4. Diagrams & Artifacts | ⚪ | - |
| 5. Architecture & Design | ⚪ | - |
| 6. Interface Contracts | ⚪ | - |
| 7. Code Scaffolding | ⚪ | - |
| 8. Test Development | ⚪ | - |
| 9. Implementation | ⚪ | - |
| 10. Simulation Testing | ⚪ | - |

## Epic Summary

| ID | Epic | Stories | Progress |
|----|------|---------|----------|
| E001 | {Epic Name} | {X} | ░░░░░░░░░░ 0% |

## Links

- Requirements: [PRODUCT-REQUIREMENTS.md](requirements/PRODUCT-REQUIREMENTS.md)
- Architecture: [ARCHITECTURE.md](design/ARCHITECTURE.md)
- Documentation: [docs/](docs/)
```

## Epic Template

```markdown
# Epic: {Epic Name}

**ID**: E{###}
**Status**: 🟡 In Progress | 🟢 Complete | 🔴 Blocked | ⚪ Not Started
**Priority**: 🔴 Critical | 🟠 High | 🟡 Medium | 🟢 Low

## Description

{What this epic delivers and why it matters}

## User Value

**As a** {user type}
**I want** {capability}
**So that** {benefit}

## Acceptance Criteria

- [ ] {Criterion 1}
- [ ] {Criterion 2}
- [ ] {Criterion 3}

## Stories

| ID | Story | Points | Status | Assignee |
|----|-------|--------|--------|----------|
| S001 | {Story Name} | {X} | ⚪ | - |
| S002 | {Story Name} | {X} | ⚪ | - |

## Dependencies

| Dependency | Type | Status |
|------------|------|--------|
| {Epic/Story/External} | Blocks/Blocked By | {Status} |

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| {Risk} | {H/M/L} | {Mitigation} |

## Progress

```
[░░░░░░░░░░] 0% (0/X stories complete)
```

## Notes

{Any additional context or decisions}
```

## Story Template

```markdown
# Story: {Story Name}

**ID**: S{###}
**Epic**: [{Epic Name}](../EPIC.md)
**Status**: 🟡 In Progress | 🟢 Complete | 🔴 Blocked | ⚪ Not Started
**Points**: {Estimate}
**Assignee**: {Name or Unassigned}

## User Story

**As a** {specific user persona}
**I want to** {action/capability}
**So that** {benefit/value}

## Acceptance Criteria

```gherkin
Given {precondition}
When {action}
Then {expected result}

Given {precondition}
When {action}
Then {expected result}
```

## Technical Notes

{Implementation guidance, constraints, or considerations}

## Tasks

| ID | Task | Status | Hours |
|----|------|--------|-------|
| T001 | {Task Name} | ⚪ | {X} |
| T002 | {Task Name} | ⚪ | {X} |

## Test Cases

| ID | Test | Type | Status |
|----|------|------|--------|
| TC001 | {Test description} | Unit | ⚪ |
| TC002 | {Test description} | Integration | ⚪ |

## Related

- Use Case: [UC-{###}](../../diagrams/behavior/use-case-{name}.puml)
- Sequence: [sequence-{name}](../../diagrams/behavior/sequence-{name}.puml)
- API Endpoint: `{METHOD} /api/v1/{resource}`

## Definition of Done

- [ ] Code complete and reviewed
- [ ] Unit tests passing (>80% coverage)
- [ ] Integration tests passing
- [ ] Documentation updated
- [ ] Deployed to staging
- [ ] QA approved

## Progress

```
[░░░░░░░░░░] 0% (0/X tasks complete)
```
```

## Task Template

```markdown
# Task: {Task Name}

**ID**: T{###}
**Story**: [{Story Name}](../STORY.md)
**Status**: 🟡 In Progress | 🟢 Complete | 🔴 Blocked | ⚪ Not Started
**Estimate**: {X} hours
**Actual**: {X} hours
**Assignee**: {Name}

## Description

{Clear description of what needs to be done}

## Subtasks

- [ ] {Subtask 1}
- [ ] {Subtask 2}
- [ ] {Subtask 3}

## Technical Details

### Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `path/to/file.ts` | Create | {Description} |
| `path/to/file.ts` | Modify | {Description} |

### Dependencies

- [ ] {Dependency 1 - link to task/story}
- [ ] {Dependency 2}

## Implementation Notes

{Any specific implementation guidance}

## Verification

- [ ] Code compiles without errors
- [ ] Linter passes
- [ ] Tests pass
- [ ] Manual verification complete

## Blockers

| Blocker | Status | Resolution |
|---------|--------|------------|
| {Blocker} | {Status} | {Resolution} |

## Time Log

| Date | Hours | Notes |
|------|-------|-------|
| {YYYY-MM-DD} | {X} | {Work done} |
```

## Progress Tracking

### Status Icons

| Icon | Meaning |
|------|---------|
| ⚪ | Not Started |
| 🟡 | In Progress |
| 🟢 | Complete |
| 🔴 | Blocked |
| 🔵 | In Review |

### Progress Bars

```
Empty:     [░░░░░░░░░░] 0%
Quarter:   [██░░░░░░░░] 25%
Half:      [█████░░░░░] 50%
Three-qtr: [███████░░░] 75%
Complete:  [██████████] 100%
```

## Automation Scripts

### Create New Project

```bash
#!/usr/bin/env bash
# create-project.sh

PROJECT_NAME=$1
DATE=$(date +%Y%m%d)
PROJECT_DIR="./planning/${DATE}_${PROJECT_NAME}"

mkdir -p "${PROJECT_DIR}"/{requirements,diagrams/{architecture,behavior,data,rendered/{png,svg,pdf}},design/contracts,epics,docs}

cat > "${PROJECT_DIR}/PROJECT.md" << 'EOF'
# Project: ${PROJECT_NAME}
...
EOF

echo "Created project at ${PROJECT_DIR}"
```

### Create New Epic

```bash
#!/usr/bin/env bash
# create-epic.sh

PROJECT_DIR=$1
EPIC_ID=$2
EPIC_NAME=$3
EPIC_DIR="${PROJECT_DIR}/epics/${EPIC_ID}_${EPIC_NAME}"

mkdir -p "${EPIC_DIR}/stories"

cat > "${EPIC_DIR}/EPIC.md" << 'EOF'
# Epic: ${EPIC_NAME}
...
EOF

echo "Created epic at ${EPIC_DIR}"
```

### Create New Story

```bash
#!/usr/bin/env bash
# create-story.sh

EPIC_DIR=$1
STORY_ID=$2
STORY_NAME=$3
STORY_DIR="${EPIC_DIR}/stories/${STORY_ID}_${STORY_NAME}"

mkdir -p "${STORY_DIR}/tasks"

cat > "${STORY_DIR}/STORY.md" << 'EOF'
# Story: ${STORY_NAME}
...
EOF

echo "Created story at ${STORY_DIR}"
```

## Example: Full Project Structure

```
./planning/20260116_user-auth-service/
├── PROJECT.md
├── requirements/
│   ├── PRODUCT-REQUIREMENTS.md
│   ├── TECHNICAL-REQUIREMENTS.md
│   └── TRACEABILITY.md
├── diagrams/
│   ├── architecture/
│   │   ├── c4-context.puml
│   │   └── c4-container.puml
│   ├── behavior/
│   │   ├── use-case-authentication.puml
│   │   ├── sequence-login.puml
│   │   ├── sequence-registration.puml
│   │   └── state-user-session.puml
│   ├── data/
│   │   └── erd-users.dbml
│   └── rendered/
│       └── png/
│           └── *.png
├── design/
│   ├── ARCHITECTURE.md
│   ├── INTERFACES.md
│   └── contracts/
│       └── openapi.yaml
├── epics/
│   ├── E001_user-registration/
│   │   ├── EPIC.md
│   │   └── stories/
│   │       ├── S001_basic-registration/
│   │       │   ├── STORY.md
│   │       │   └── tasks/
│   │       │       ├── T001_create-user-model.md
│   │       │       ├── T002_registration-endpoint.md
│   │       │       └── T003_email-verification.md
│   │       └── S002_social-login/
│   │           ├── STORY.md
│   │           └── tasks/
│   │               └── ...
│   └── E002_authentication/
│       ├── EPIC.md
│       └── stories/
│           └── ...
└── docs/
    ├── conf.py
    ├── index.rst
    └── api/
        └── ...
```

## Integration with Skill Phases

| Phase | Planning Output |
|-------|-----------------|
| Phase 1 | `requirements/PRODUCT-REQUIREMENTS.md` |
| Phase 2 | `requirements/TECHNICAL-REQUIREMENTS.md` |
| Phase 3 | `requirements/TRACEABILITY.md`, Epic/Story structure |
| Phase 4 | `diagrams/behavior/`, `diagrams/data/` |
| Phase 5 | `design/ARCHITECTURE.md`, `diagrams/architecture/` |
| Phase 6 | `design/INTERFACES.md`, `design/contracts/` |
| Phase 7 | Code scaffolding (separate from planning) |
| Phase 8 | Test files (linked from Story test cases) |
| Phase 9 | Task updates as implementation proceeds |
| Phase 10 | Simulation results in `PROJECT.md` |
