# Phase 11: Code Review

## Objective

Review code changes systematically for correctness, security, performance, and maintainability. Provide actionable feedback and ensure changes meet quality standards before merging.

## Step 1: Review Preparation

Gather context before reviewing:

```markdown
### Code Review: [PR/MR Title]

**Change Summary**:
- Files changed: [N]
- Lines added: [+X]
- Lines removed: [-Y]
- Type: [Feature / Bug Fix / Refactor / Docs / Config]

**Context Gathering**:
```bash
# View the diff
git diff main...feature-branch

# Understand commit history
git log main..feature-branch --oneline

# Check related issues/tickets
gh pr view [PR_NUMBER]
```

**Review Scope**:
| Area | Priority | Notes |
|------|----------|-------|
| Core logic changes | High | [Files] |
| Test changes | High | [Files] |
| Config changes | Medium | [Files] |
| Documentation | Low | [Files] |
```

## Step 2: Review Checklist

Systematic review across multiple dimensions:

```markdown
### Review Checklist

**Correctness**:
- [ ] Code does what it claims to do
- [ ] Edge cases handled appropriately
- [ ] Error conditions handled gracefully
- [ ] No logic errors or off-by-one mistakes
- [ ] Types are correct and consistent

**Security**:
- [ ] No hardcoded secrets or credentials
- [ ] Input validation present for user data
- [ ] SQL injection prevention (parameterized queries)
- [ ] XSS prevention (output encoding)
- [ ] Authentication/authorization checks in place
- [ ] No sensitive data in logs
- [ ] Dependencies are from trusted sources

**Performance**:
- [ ] No obvious N+1 query patterns
- [ ] Appropriate use of caching
- [ ] No unnecessary loops or iterations
- [ ] Database queries use indexes
- [ ] No blocking operations in async code
- [ ] Memory allocations are reasonable

**Maintainability**:
- [ ] Code is readable and self-documenting
- [ ] Functions/methods have single responsibility
- [ ] No excessive complexity (cyclomatic)
- [ ] Magic numbers/strings are constants
- [ ] DRY - no unnecessary duplication
- [ ] Consistent naming conventions

**Testing**:
- [ ] New code has test coverage
- [ ] Tests are meaningful (not just coverage)
- [ ] Edge cases are tested
- [ ] No flaky test patterns
- [ ] Test names describe behavior

**Documentation**:
- [ ] Public APIs are documented
- [ ] Complex logic has explanatory comments
- [ ] README updated if needed
- [ ] Breaking changes documented
```

## Step 3: Review Feedback Format

Structure feedback clearly:

```markdown
### Review Feedback

**Overall Assessment**: [Approve / Request Changes / Comment]

---

#### Critical (Must Fix)

**[File:Line] - [Category]**
> [Code snippet]

**Issue**: [Description of the problem]
**Risk**: [What could go wrong]
**Suggestion**:
```[language]
[Suggested fix]
```

---

#### Suggestions (Should Consider)

**[File:Line] - [Category]**
> [Code snippet]

**Observation**: [What could be improved]
**Suggestion**: [How to improve it]

---

#### Nitpicks (Optional)

**[File:Line]**
- [Minor style/formatting issue]

---

#### Positive Feedback

- [What was done well]
- [Good patterns observed]
```

## Step 4: Common Issues Reference

Quick reference for common problems:

```markdown
### Common Review Issues

**Security Issues**:
| Pattern | Problem | Fix |
|---------|---------|-----|
| `query = f"SELECT * FROM users WHERE id = {user_id}"` | SQL injection | Use parameterized queries |
| `innerHTML = userInput` | XSS vulnerability | Use textContent or sanitize |
| `password` in git history | Credential exposure | Use environment variables |
| `cors: { origin: '*' }` | Overly permissive CORS | Restrict to known origins |

**Performance Issues**:
| Pattern | Problem | Fix |
|---------|---------|-----|
| Loop with DB query inside | N+1 queries | Batch fetch, use JOINs |
| `await` in loop | Sequential execution | Use Promise.all() |
| Missing index on query column | Slow queries | Add database index |
| Large objects in memory | Memory bloat | Stream or paginate |

**Code Smell Indicators**:
| Pattern | Problem | Fix |
|---------|---------|-----|
| Function >50 lines | Too complex | Extract smaller functions |
| >3 nested levels | Hard to follow | Early returns, extract |
| Boolean parameters | Unclear intent | Use options object or enum |
| Comments explaining "what" | Code not readable | Rename variables/functions |
```

## Step 5: Review Conversation Flow

Handle back-and-forth productively:

```markdown
### Review Iteration: Round [N]

**Previous Feedback Status**:
| Issue | Status | Notes |
|-------|--------|-------|
| SQL injection in auth.py:45 | ✅ Fixed | Now uses parameterized |
| Missing error handling | ✅ Fixed | Added try/catch |
| N+1 query in users list | 🔄 Partially | Still one case remaining |
| Add unit tests | ❌ Pending | Not yet addressed |

**New Issues Found**:
[Any new issues from the latest changes]

**Outstanding Items**:
1. [Remaining issue 1]
2. [Remaining issue 2]

**Recommendation**: [Approve / Request another round]
```

## Step 6: Review Types

Different review depths for different situations:

```markdown
### Review Type Selection

| Type | When | Focus | Time |
|------|------|-------|------|
| **Quick Review** | Small changes, config updates | Obvious errors, security | 5-10 min |
| **Standard Review** | Features, bug fixes | Full checklist | 30-60 min |
| **Deep Review** | Security-critical, core changes | Line-by-line, threat model | 1-2 hours |
| **Architecture Review** | Major changes, new systems | Design patterns, scalability | Half day |

**Selected Type**: [Type]

**Justification**: [Why this depth is appropriate]
```

## Step 7: Automated Checks Integration

Complement manual review with automation:

```markdown
### Automated Check Results

**CI Pipeline**:
- [ ] Build: [Pass/Fail]
- [ ] Unit Tests: [Pass/Fail] - [X/Y passing]
- [ ] Integration Tests: [Pass/Fail]
- [ ] Lint: [Pass/Fail] - [N issues]
- [ ] Type Check: [Pass/Fail]

**Security Scanning**:
- [ ] Dependency audit: [Pass/Fail] - [N vulnerabilities]
- [ ] SAST: [Pass/Fail] - [N findings]
- [ ] Secret detection: [Pass/Fail]

**Coverage**:
- Current: [X]%
- Change: [+/-Y]%
- Target: [Z]%

**Issues to Address from Automation**:
| Tool | Finding | Severity | Action |
|------|---------|----------|--------|
| [Tool] | [Finding] | [H/M/L] | [Required/Optional] |
```

## Phase 11 Approval Gate

```markdown
## Phase 11 Summary: Code Review

### Review Outcome

**Decision**: [Approved / Changes Requested / Blocked]

**Review Statistics**:
| Metric | Count |
|--------|-------|
| Files reviewed | [N] |
| Critical issues | [N] |
| Suggestions | [N] |
| Review rounds | [N] |

### Issues Summary

**Resolved**:
| Issue | Resolution |
|-------|------------|
| [Issue 1] | [How fixed] |

**Accepted (Won't Fix)**:
| Issue | Reason |
|-------|--------|
| [Issue 1] | [Why accepted] |

### Final Checklist

- [ ] All critical issues resolved
- [ ] Security concerns addressed
- [ ] Tests adequate and passing
- [ ] CI pipeline green
- [ ] Documentation updated

---

**Merge approved?**
```
