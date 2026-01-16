---
name: product-development
description: Guides full product development lifecycle from requirements through implementation. Use when building new features, services, CLI tools, APIs, or infrastructure. Covers product requirements, technical design, architecture, testing, and implementation with user approval gates at each phase.
---

# Product Development Workflow

A structured methodology for building software products with user collaboration at every phase.

## When to Use

- Building new backend services, APIs, or microservices
- Creating CLI tools or developer utilities
- Developing frontend applications or components
- Designing infrastructure or platform capabilities
- Any feature requiring systematic requirements gathering and design

## Workflow Overview

```
Phase 1: Product Requirements    → User Approval Required
Phase 2: Technical Requirements  → User Approval Required
Phase 3: Requirements Integration → User Approval Required
Phase 4: Diagrams & Artifacts    → User Approval Required
Phase 5: Architecture & Design   → User Approval Required
Phase 6: Interface Contracts     → User Approval Required
Phase 7: Code Scaffolding        → User Approval Required
Phase 8: Test Development        → User Approval Required
Phase 9: Implementation          → User Approval Required
Phase 10: Simulation Testing     → User Approval Required (if applicable)
```

## Product Type Guides

Select the appropriate guide based on what you're building:

- **Backend Services/APIs**: See [product-types/BACKEND.md](product-types/BACKEND.md)
- **CLI Tools**: See [product-types/CLI.md](product-types/CLI.md)
- **Frontend Applications**: See [product-types/FRONTEND.md](product-types/FRONTEND.md)
- **Infrastructure**: See [product-types/INFRASTRUCTURE.md](product-types/INFRASTRUCTURE.md)

## Phase Execution Pattern

For each phase, follow this pattern:

### 1. Present Options to User

Always present decisions with this format:

```markdown
### Decision: [Decision Name]

| Option | Description | Pros | Cons | User Impact |
|--------|-------------|------|------|-------------|
| **Option A (Recommended)** | [Description] | [Pros] | [Cons] | [Impact] |
| Option B | [Description] | [Pros] | [Cons] | [Impact] |
| Option C | [Description] | [Pros] | [Cons] | [Impact] |

**Recommendation**: Option A because [reasoning tied to user expectations]
```

### 2. Wait for User Approval

Before proceeding to the next phase:
- Summarize all decisions made
- Present artifacts created
- Ask explicitly: "Do you approve this phase? Any changes needed?"
- Do NOT proceed until user confirms

### 3. Document Decisions

Track all decisions in a running summary accessible to the user.

---

## Phase Details

### Phase 1: Product Requirements

See [phases/01-PRODUCT-REQUIREMENTS.md](phases/01-PRODUCT-REQUIREMENTS.md)

**Goal**: Define WHO uses the product and WHAT they expect.

**Key Activities**:
- Identify user personas and their goals
- Write user stories: "As a [X], I expect to [Y] so that [Z]"
- Define product boundaries and constraints
- Establish success criteria

**Critical Reflection Questions**:
- Who is NOT a user? What are we explicitly excluding?
- What happens when the user's expectation isn't met?
- Are there conflicting user needs?

---

### Phase 2: Technical Requirements

See [phases/02-TECHNICAL-REQUIREMENTS.md](phases/02-TECHNICAL-REQUIREMENTS.md)

**Goal**: Define WHAT the technology must do to meet user needs.

**Key Activities**:
- Translate user stories into technical capabilities
- Define non-functional requirements (performance, security, scalability)
- Identify technical constraints and dependencies
- Specify data requirements and integration points

**Critical Reflection Questions**:
- Can we actually build this with available resources?
- What are the failure modes?
- What technical debt are we accepting?

---

### Phase 3: Requirements Integration

See [phases/03-REQUIREMENTS-INTEGRATION.md](phases/03-REQUIREMENTS-INTEGRATION.md)

**Goal**: Link product and technical requirements, identify gaps.

**Key Activities**:
- Create traceability matrix: user story → technical requirement
- Identify orphaned requirements (technical without product justification)
- Find gaps (user needs without technical solutions)
- Prioritize based on user impact

**Critical Reflection Questions**:
- Does every technical requirement serve a user need?
- Are all user stories technically addressed?
- What's the minimum viable scope?

---

### Phase 4: Diagrams & Artifacts

See [phases/04-DIAGRAMS-ARTIFACTS.md](phases/04-DIAGRAMS-ARTIFACTS.md)

**Goal**: Visualize user journeys and system behavior.

**Key Artifacts**:
- Use case diagrams (actor → system interactions)
- State diagrams (system states and transitions)
- User flow diagrams (step-by-step user journeys)
- Error/edge case mappings

**Templates**: See [templates/](templates/) directory

---

### Phase 5: Architecture & Design

See [phases/05-ARCHITECTURE-DESIGN.md](phases/05-ARCHITECTURE-DESIGN.md)

**Goal**: Define system structure and component relationships.

**Key Artifacts**:
- Component/class diagrams
- Sequence diagrams (key interactions)
- Data models and schemas
- Dependency graphs

---

### Phase 6: Interface Contracts

See [phases/06-INTERFACE-CONTRACTS.md](phases/06-INTERFACE-CONTRACTS.md)

**Goal**: Define all interaction boundaries.

**Interface Types by Product**:
- **APIs**: OpenAPI/Swagger specs, request/response schemas
- **CLI**: Command syntax, flags, input/output formats
- **Frontend**: Component props, API contracts, state shapes
- **Infrastructure**: Configuration schemas, resource definitions

---

### Phase 7: Code Scaffolding

See [phases/07-CODE-SCAFFOLDING.md](phases/07-CODE-SCAFFOLDING.md)

**Goal**: Create project structure without implementation.

**Key Activities**:
- Directory structure matching architecture
- Empty modules/classes with signatures
- Configuration files
- Build/dependency setup

---

### Phase 8: Test Development

See [phases/08-TEST-DEVELOPMENT.md](phases/08-TEST-DEVELOPMENT.md)

**Goal**: Write tests before implementation (TDD approach).

**Test Categories**:
- Unit tests for each component
- Integration tests for interfaces
- Contract tests for APIs
- End-to-end tests for user flows

---

### Phase 9: Implementation

See [phases/09-IMPLEMENTATION.md](phases/09-IMPLEMENTATION.md)

**Goal**: Build the application to pass all tests.

**Approach**:
- Implement one component at a time
- Run tests after each component
- Document decisions and deviations
- Keep implementation aligned with design

---

### Phase 10: Simulation Testing

See [phases/10-SIMULATION-TESTING.md](phases/10-SIMULATION-TESTING.md)

**Goal**: Validate system behavior under realistic conditions.

**When Required**:
- Distributed systems
- High-throughput services
- Complex state machines
- External dependency interactions

---

## Quick Start Checklist

Copy this checklist to track progress:

```
Product Development Progress:
- [ ] Phase 1: Product Requirements (Approved: ___)
- [ ] Phase 2: Technical Requirements (Approved: ___)
- [ ] Phase 3: Requirements Integration (Approved: ___)
- [ ] Phase 4: Diagrams & Artifacts (Approved: ___)
- [ ] Phase 5: Architecture & Design (Approved: ___)
- [ ] Phase 6: Interface Contracts (Approved: ___)
- [ ] Phase 7: Code Scaffolding (Approved: ___)
- [ ] Phase 8: Test Development (Approved: ___)
- [ ] Phase 9: Implementation (Approved: ___)
- [ ] Phase 10: Simulation Testing (Approved: ___ / Skipped: ___)
```

## Interaction Style

Throughout all phases:

1. **Be Interactive**: Present options, don't assume
2. **Show Trade-offs**: Every decision has pros/cons
3. **Recommend Clearly**: Mark recommended option with **
4. **Wait for Approval**: Never proceed without user confirmation
5. **Link to User Impact**: Connect technical decisions to user expectations
