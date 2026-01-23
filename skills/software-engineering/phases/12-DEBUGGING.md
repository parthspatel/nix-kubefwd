# Phase 12: Debugging & Troubleshooting

## Objective

Systematically diagnose and resolve software defects using structured analysis, hypothesis testing, and root cause identification. Transform chaotic debugging into a repeatable process.

## Step 1: Issue Characterization

Gather information before diving into code:

```markdown
### Issue Characterization: [Issue ID/Title]

**Symptom Description**:
- What: [Observable behavior]
- When: [Frequency - always, sometimes, specific conditions]
- Where: [Environment - prod, staging, local, specific user]
- Since: [When first observed, any recent changes]

**Expected Behavior**:
[What should happen instead]

**Reproduction Steps**:
1. [Step 1]
2. [Step 2]
3. [Observe: symptom]

**Reproduction Rate**: [Always / Intermittent / Rare]

**Initial Data Collection**:
```bash
# Gather logs around incident time
grep -A 10 -B 10 "ERROR" /var/log/app.log

# Check recent deployments
git log --oneline -10

# System state
free -m && df -h && uptime
```

**Severity Assessment**:
| Factor | Value |
|--------|-------|
| User impact | [Critical / High / Medium / Low] |
| Frequency | [Constant / Frequent / Rare] |
| Data loss risk | [Yes / No] |
| Workaround exists | [Yes / No] |
| **Priority** | [P0 / P1 / P2 / P3] |
```

## Step 2: Hypothesis Formation

Generate and prioritize theories:

```markdown
### Hypothesis List

**Based on Symptoms**:

| # | Hypothesis | Likelihood | Evidence For | Evidence Against |
|---|------------|------------|--------------|------------------|
| 1 | [Theory 1] | High | [Supporting data] | [Contradicting data] |
| 2 | [Theory 2] | Medium | [Supporting data] | [Contradicting data] |
| 3 | [Theory 3] | Low | [Supporting data] | [Contradicting data] |

**Investigation Order** (by likelihood × ease of verification):
1. [Hypothesis to test first]
2. [Second hypothesis]
3. [Third hypothesis]

**Quick Elimination Checks**:
```bash
# Rule out obvious causes
git log --oneline -5  # Recent changes?
docker ps             # Services running?
curl localhost:8080/health  # App responding?
```
```

## Step 3: Systematic Investigation

Test hypotheses methodically:

```markdown
### Investigation: Hypothesis [N]

**Theory**: [What we think is wrong]

**Test Approach**:
[How to confirm or refute]

**Commands/Actions**:
```bash
# Investigation commands
[Commands to run]
```

**Results**:
```
[Actual output]
```

**Conclusion**: [Confirmed / Refuted / Inconclusive]

**Next Step**: [Continue with this theory / Move to next hypothesis / Gather more data]
```

## Step 4: Debugging Techniques Reference

Apply appropriate techniques:

```markdown
### Debugging Techniques

**Binary Search (Bisection)**:
Use when: Large changeset, unclear which change caused issue
```bash
# Git bisect for finding bad commit
git bisect start
git bisect bad HEAD
git bisect good v1.0.0
# Test each commit, mark good/bad until found
git bisect reset
```

**Print/Log Debugging**:
Use when: Understanding flow, variable state
```python
# Strategic logging
import logging
logger = logging.getLogger(__name__)

def problematic_function(data):
    logger.debug(f"Input: {data}")
    result = process(data)
    logger.debug(f"After process: {result}")
    # ... continue tracing
```

**Divide and Conquer**:
Use when: Large system, unknown component
```markdown
System Components:
[✓] Database - confirmed working (direct query succeeds)
[✓] API Layer - confirmed working (health check passes)
[?] Service Layer - INVESTIGATING
[ ] External API - not yet tested
```

**Rubber Duck Debugging**:
Use when: Logic errors, unclear flow
```markdown
Explaining the code:
1. This function takes [input]
2. It should [expected behavior]
3. First it [step 1]
4. Then it [step 2]
5. Wait... at step [N], I assumed [X] but actually [Y]
```

**Minimal Reproduction**:
Use when: Complex system, need isolation
```markdown
Reduction Steps:
1. Full system reproduction - ✓ reproduces
2. Without external services - ✓ still reproduces
3. Without database - ✓ still reproduces
4. Single function call - ✓ REPRODUCES
5. Minimal input: `function("edge_case")` - ROOT CAUSE FOUND
```
```

## Step 5: Common Bug Patterns

Quick reference for frequent issues:

```markdown
### Common Bug Patterns

**Race Conditions**:
| Symptom | Investigation | Typical Fix |
|---------|---------------|-------------|
| Intermittent failures | Add timing logs | Proper synchronization |
| Works on retry | Check concurrent access | Locks, atomic operations |
| Data inconsistency | Review shared state | Transactions, queues |

**Memory Issues**:
| Symptom | Investigation | Typical Fix |
|---------|---------------|-------------|
| Slow over time | Memory profiling | Fix leaks, add cleanup |
| OOM crashes | Heap dumps | Limit collections, streaming |
| GC pauses | GC logs | Tune GC, reduce allocations |

**State Management**:
| Symptom | Investigation | Typical Fix |
|---------|---------------|-------------|
| Stale data | Check cache TTL | Invalidation, refresh |
| Incorrect state | Trace state changes | State machine validation |
| Persistence issues | Check transaction boundaries | Proper commits |

**Integration Issues**:
| Symptom | Investigation | Typical Fix |
|---------|---------------|-------------|
| Timeout errors | Network tracing | Retry logic, circuit breaker |
| Data format errors | Log payloads | Schema validation |
| Auth failures | Check token expiry | Token refresh, error handling |

**Environment Issues**:
| Symptom | Investigation | Typical Fix |
|---------|---------------|-------------|
| Works locally only | Compare environments | Config parity, env vars |
| Missing dependencies | Check installed packages | Lock files, container images |
| Permission denied | Check file/user permissions | Proper ownership, modes |
```

## Step 6: Root Cause Analysis

Document findings thoroughly:

```markdown
### Root Cause Analysis: [Issue ID]

**Summary**:
[One sentence description of the root cause]

**Causal Chain**:
```
[Trigger Event]
    ↓
[Intermediate Effect 1]
    ↓
[Intermediate Effect 2]
    ↓
[Observable Symptom]
```

**Root Cause Details**:
- **What**: [Technical description]
- **Where**: [File:line, component]
- **Why it occurred**: [Contributing factors]
- **Why it wasn't caught**: [Testing gap, monitoring gap]

**Evidence**:
```
[Logs, traces, or other proof]
```

**Classification**:
| Aspect | Value |
|--------|-------|
| Category | [Logic / Data / Config / Integration / Environment] |
| Introduction | [Commit hash, date] |
| Detection method | [User report / Monitoring / Testing] |
| Time to resolution | [Duration] |
```

## Step 7: Fix Verification

Ensure the fix is complete:

```markdown
### Fix Verification

**Fix Applied**:
```diff
- [Old code]
+ [New code]
```

**Verification Steps**:

1. **Reproduce original issue**:
   - [ ] Issue no longer reproduces

2. **Test the specific fix**:
   - [ ] Unit test added for this case
   - [ ] Test passes

3. **Regression testing**:
   - [ ] Related tests still pass
   - [ ] Full test suite passes

4. **Edge cases**:
   | Case | Result |
   |------|--------|
   | [Edge case 1] | ✅ |
   | [Edge case 2] | ✅ |

5. **Environment validation**:
   - [ ] Works in development
   - [ ] Works in staging
   - [ ] Ready for production

**Rollback Plan**:
[How to revert if issues arise in production]
```

## Step 8: Knowledge Capture

Prevent future occurrences:

```markdown
### Post-Incident Learning

**What went well**:
- [Effective debugging technique]
- [Good collaboration]

**What could improve**:
- [Gap that allowed bug]
- [Detection improvement needed]

**Action Items**:
| Action | Owner | Priority |
|--------|-------|----------|
| Add monitoring for [X] | [Name] | High |
| Improve test coverage for [Y] | [Name] | Medium |
| Document [Z] pattern | [Name] | Low |

**Runbook Update**:
[If applicable, update operations runbook]

**Similar Issues to Watch**:
- [Pattern 1 that might exist elsewhere]
- [Pattern 2 to audit]
```

## Debugging Loop Limits

```markdown
### Debugging Escalation

**Time Limits**:
| Phase | Max Time | Escalation |
|-------|----------|------------|
| Initial investigation | 30 min | Form specific hypotheses |
| Single hypothesis test | 15 min | Move to next hypothesis |
| Total debugging session | 2 hours | Escalate for help |

**Escalation Triggers**:
- [ ] No progress after 3 hypotheses tested
- [ ] Issue requires access/knowledge you don't have
- [ ] Production impact requires immediate workaround

**Escalation Template**:
```markdown
**Issue**: [Summary]
**Time Invested**: [Duration]
**Hypotheses Tested**:
1. [Hypothesis 1] - [Result]
2. [Hypothesis 2] - [Result]
**Remaining Hypotheses**: [List]
**Help Needed**: [Specific ask]
```
```

## Phase 12 Approval Gate

```markdown
## Phase 12 Summary: Debugging Complete

### Resolution Status

**Issue**: [Issue ID/Title]
**Status**: [Resolved / Mitigated / Escalated]

**Root Cause**:
[One sentence summary]

**Fix Summary**:
| Change | File | Purpose |
|--------|------|---------|
| [Change 1] | [File] | [Why] |

### Verification

- [ ] Issue no longer reproduces
- [ ] Tests added and passing
- [ ] No regressions introduced
- [ ] Fix deployed to [environment]
- [ ] Monitoring confirms resolution

### Prevention

- [ ] Root cause documented
- [ ] Action items created
- [ ] Similar patterns audited

---

**Issue resolved and verified?**
```
