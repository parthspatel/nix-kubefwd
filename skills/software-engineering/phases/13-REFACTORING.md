# Phase 13: Refactoring

## Objective

Improve code structure without changing external behavior. Apply systematic refactoring techniques to reduce technical debt, improve maintainability, and enhance code clarity while maintaining a green test suite.

## Step 1: Refactoring Assessment

Identify what needs refactoring and why:

```markdown
### Refactoring Assessment

**Target Area**: [Module / Class / Function]

**Current Issues**:
| Code Smell | Location | Severity | Impact |
|------------|----------|----------|--------|
| [Smell 1] | [File:Line] | High | [What it affects] |
| [Smell 2] | [File:Line] | Medium | [What it affects] |

**Refactoring Goals**:
- [ ] [Goal 1: e.g., Reduce function complexity]
- [ ] [Goal 2: e.g., Improve naming clarity]
- [ ] [Goal 3: e.g., Extract reusable components]

**Constraints**:
- [ ] Must not change external API/interface
- [ ] Must maintain backward compatibility
- [ ] Must keep all tests passing
- [ ] Time budget: [X hours]

**Success Criteria**:
| Metric | Before | Target |
|--------|--------|--------|
| Cyclomatic complexity | [X] | [Y] |
| Lines of code | [X] | [Y] |
| Test coverage | [X]% | [Y]% |
| Duplication | [X]% | [Y]% |
```

## Step 2: Code Smell Identification

Reference guide for common smells:

```markdown
### Code Smell Reference

**Bloaters** (Code that grows too large):
| Smell | Signs | Refactoring |
|-------|-------|-------------|
| Long Method | >20 lines, multiple levels | Extract Method |
| Large Class | >300 lines, many responsibilities | Extract Class |
| Long Parameter List | >3 parameters | Introduce Parameter Object |
| Data Clumps | Same fields appear together | Extract Class |
| Primitive Obsession | Primitives instead of small objects | Replace with Value Object |

**OO Abusers**:
| Smell | Signs | Refactoring |
|-------|-------|-------------|
| Switch Statements | Repeated switch on type | Replace with Polymorphism |
| Refused Bequest | Subclass ignores parent methods | Replace Inheritance with Delegation |
| Alternative Classes | Similar classes, different interfaces | Unify Interface |
| Temporary Field | Fields only used sometimes | Extract Class |

**Change Preventers**:
| Smell | Signs | Refactoring |
|-------|-------|-------------|
| Divergent Change | One class changed for many reasons | Extract Class |
| Shotgun Surgery | One change requires many class edits | Move Method, Inline Class |
| Parallel Inheritance | Create subclass pair for every addition | Collapse Hierarchy |

**Dispensables**:
| Smell | Signs | Refactoring |
|-------|-------|-------------|
| Dead Code | Unreachable, unused code | Remove |
| Speculative Generality | Unused abstraction "for future" | Collapse Hierarchy, Inline |
| Duplicate Code | Same structure in multiple places | Extract Method/Class |
| Comments | Comments explaining bad code | Refactor, then remove comments |

**Couplers**:
| Smell | Signs | Refactoring |
|-------|-------|-------------|
| Feature Envy | Method uses another class more | Move Method |
| Inappropriate Intimacy | Classes know too much about each other | Move Method, Extract Class |
| Message Chains | a.b().c().d() | Hide Delegate |
| Middle Man | Class only delegates | Remove Middle Man |
```

## Step 3: Safe Refactoring Process

Refactor with confidence:

```markdown
### Refactoring Process: [Component Name]

**Pre-Refactoring Checks**:
```bash
# Ensure clean starting point
git status                    # No uncommitted changes
npm test                      # All tests pass
npm run lint                  # No lint errors
git checkout -b refactor/[description]  # New branch
```

**Refactoring Cycle**:
```
┌─────────────────────────────────────────┐
│        SAFE REFACTORING CYCLE           │
├─────────────────────────────────────────┤
│                                         │
│  1. Verify Tests Pass (Green)           │
│          ↓                              │
│  2. Make ONE Small Change               │
│          ↓                              │
│  3. Run Tests                           │
│          ↓                              │
│  4. Tests Pass?                         │
│      ╱      ╲                           │
│    Yes       No                         │
│     ↓         ↓                         │
│  Commit    Revert & Retry               │
│     ↓                                   │
│  Repeat                                 │
│                                         │
└─────────────────────────────────────────┘
```

**Change Log**:
| Step | Change | Tests | Commit |
|------|--------|-------|--------|
| 1 | [Change 1] | ✅ | [hash] |
| 2 | [Change 2] | ✅ | [hash] |
| 3 | [Change 3] | ✅ | [hash] |
```

## Step 4: Common Refactoring Techniques

Detailed techniques with examples:

```markdown
### Extract Method

**When**: Long method, code with comments explaining blocks

**Before**:
```python
def process_order(order):
    # Validate order
    if not order.items:
        raise ValueError("Empty order")
    if not order.customer:
        raise ValueError("No customer")

    # Calculate total
    subtotal = sum(item.price * item.quantity for item in order.items)
    tax = subtotal * 0.1
    total = subtotal + tax

    # Apply discount
    if order.customer.is_premium:
        total = total * 0.9

    return total
```

**After**:
```python
def process_order(order):
    validate_order(order)
    subtotal = calculate_subtotal(order.items)
    total = apply_tax(subtotal)
    return apply_discount(total, order.customer)

def validate_order(order):
    if not order.items:
        raise ValueError("Empty order")
    if not order.customer:
        raise ValueError("No customer")

def calculate_subtotal(items):
    return sum(item.price * item.quantity for item in items)

def apply_tax(amount, rate=0.1):
    return amount * (1 + rate)

def apply_discount(amount, customer):
    if customer.is_premium:
        return amount * 0.9
    return amount
```

---

### Extract Class

**When**: Class has multiple responsibilities

**Before**:
```python
class User:
    def __init__(self, name, email, street, city, zip_code):
        self.name = name
        self.email = email
        self.street = street
        self.city = city
        self.zip_code = zip_code

    def get_full_address(self):
        return f"{self.street}, {self.city} {self.zip_code}"
```

**After**:
```python
class Address:
    def __init__(self, street, city, zip_code):
        self.street = street
        self.city = city
        self.zip_code = zip_code

    def format(self):
        return f"{self.street}, {self.city} {self.zip_code}"

class User:
    def __init__(self, name, email, address):
        self.name = name
        self.email = email
        self.address = address

    def get_full_address(self):
        return self.address.format()
```

---

### Replace Conditional with Polymorphism

**When**: Type-based switch/if statements

**Before**:
```python
def calculate_pay(employee):
    if employee.type == "hourly":
        return employee.hours * employee.rate
    elif employee.type == "salaried":
        return employee.salary / 12
    elif employee.type == "contractor":
        return employee.hours * employee.rate * 1.5
```

**After**:
```python
class Employee(ABC):
    @abstractmethod
    def calculate_pay(self):
        pass

class HourlyEmployee(Employee):
    def calculate_pay(self):
        return self.hours * self.rate

class SalariedEmployee(Employee):
    def calculate_pay(self):
        return self.salary / 12

class Contractor(Employee):
    def calculate_pay(self):
        return self.hours * self.rate * 1.5
```

---

### Introduce Parameter Object

**When**: Multiple parameters travel together

**Before**:
```python
def create_report(start_date, end_date, include_charts, format, customer_id):
    pass

def export_data(start_date, end_date, include_charts, format, customer_id):
    pass
```

**After**:
```python
@dataclass
class ReportConfig:
    start_date: date
    end_date: date
    include_charts: bool
    format: str
    customer_id: int

def create_report(config: ReportConfig):
    pass

def export_data(config: ReportConfig):
    pass
```
```

## Step 5: Refactoring Patterns by Goal

Goal-oriented refactoring strategies:

```markdown
### Refactoring by Goal

**Goal: Improve Readability**
| Technique | When to Use |
|-----------|-------------|
| Rename Variable/Method | Unclear names |
| Extract Method | Long methods |
| Introduce Explaining Variable | Complex expressions |
| Replace Magic Number with Constant | Hardcoded values |

**Goal: Reduce Complexity**
| Technique | When to Use |
|-----------|-------------|
| Decompose Conditional | Complex if/else |
| Replace Nested Conditional with Guard Clauses | Deep nesting |
| Replace Conditional with Polymorphism | Type switching |
| Extract Method | Long methods |

**Goal: Improve Flexibility**
| Technique | When to Use |
|-----------|-------------|
| Extract Interface | Multiple implementations needed |
| Replace Inheritance with Delegation | Rigid hierarchy |
| Introduce Parameter Object | Growing parameter lists |
| Replace Constructor with Factory | Complex creation |

**Goal: Remove Duplication**
| Technique | When to Use |
|-----------|-------------|
| Extract Method | Same code in one class |
| Extract Class | Same code across classes |
| Pull Up Method | Same code in siblings |
| Form Template Method | Same algorithm, different steps |

**Goal: Simplify Dependencies**
| Technique | When to Use |
|-----------|-------------|
| Move Method | Method uses other class more |
| Dependency Injection | Hard-coded dependencies |
| Extract Interface | Reduce coupling |
| Hide Delegate | Law of Demeter violations |
```

## Step 6: Refactoring Safety Net

Ensure refactoring doesn't break anything:

```markdown
### Safety Net Checklist

**Before Starting**:
- [ ] All tests pass
- [ ] Test coverage is adequate (>70%)
- [ ] Code is in version control
- [ ] Working on feature branch

**During Refactoring**:
- [ ] Making small, incremental changes
- [ ] Running tests after each change
- [ ] Committing after each successful change
- [ ] No behavior changes (only structure)

**Test Verification**:
```bash
# Run full test suite
npm test

# Check coverage didn't drop
npm test -- --coverage

# Run specific tests for changed area
npm test -- --grep "[ComponentName]"

# Verify no type errors (if applicable)
npm run typecheck
```

**If Tests Fail**:
1. STOP immediately
2. Revert last change: `git checkout -- .`
3. Re-run tests to confirm green
4. Try smaller step or different approach

**Code Review Points**:
- [ ] No functional changes (diff should show only structure)
- [ ] No new features snuck in
- [ ] Naming is clearer
- [ ] Complexity reduced or unchanged
```

## Step 7: Refactoring Progress Tracking

Monitor refactoring impact:

```markdown
### Refactoring Progress

**Metrics Comparison**:
| Metric | Before | Current | Target | Status |
|--------|--------|---------|--------|--------|
| Cyclomatic Complexity | [X] | [Y] | [Z] | ✅/❌ |
| Lines of Code | [X] | [Y] | [Z] | ✅/❌ |
| Test Coverage | [X]% | [Y]% | [Z]% | ✅/❌ |
| Duplication | [X]% | [Y]% | [Z]% | ✅/❌ |
| Lint Warnings | [X] | [Y] | [Z] | ✅/❌ |

**Changes Made**:
| Component | Refactoring | Impact |
|-----------|-------------|--------|
| [Component 1] | Extract Method x3 | -15 lines, clearer flow |
| [Component 2] | Extract Class | Single responsibility |

**Remaining Work**:
| Target | Smell | Planned Refactoring |
|--------|-------|---------------------|
| [File] | [Smell] | [Technique] |
```

## Phase 13 Approval Gate

```markdown
## Phase 13 Summary: Refactoring Complete

### Refactoring Outcome

**Scope**: [What was refactored]
**Approach**: [Key techniques used]

### Metrics Improvement

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Cyclomatic Complexity | [X] | [Y] | [±Z] |
| Lines of Code | [X] | [Y] | [±Z] |
| Test Coverage | [X]% | [Y]% | [±Z]% |
| Code Duplication | [X]% | [Y]% | [±Z]% |

### Changes Summary

**Structural Changes**:
| Change | Files Affected | Commits |
|--------|----------------|---------|
| [Change 1] | [N] | [hash] |

**No Behavioral Changes**:
- [ ] All original tests pass
- [ ] No new tests required (behavior unchanged)
- [ ] API/interfaces unchanged

### Code Quality

- [ ] All tests passing
- [ ] Coverage maintained or improved
- [ ] No new lint warnings
- [ ] Code review approved

---

**Refactoring complete and verified?**
```
