# Phase 14: Migrations & Upgrades

## Objective

Safely migrate systems, upgrade dependencies, and evolve APIs while maintaining backward compatibility and minimizing downtime. Execute changes incrementally with rollback capability at every step.

## Step 1: Migration Planning

Assess scope and risk before starting:

```markdown
### Migration Assessment: [Migration Name]

**Migration Type**:
- [ ] Dependency upgrade (library, framework, runtime)
- [ ] Database schema migration
- [ ] API version migration
- [ ] Infrastructure migration (cloud, platform)
- [ ] Data migration (format, storage)

**Current State**:
| Component | Version/State | Notes |
|-----------|---------------|-------|
| [Component 1] | [Current] | [Health/Issues] |

**Target State**:
| Component | Version/State | Benefits |
|-----------|---------------|----------|
| [Component 1] | [Target] | [Why upgrading] |

**Risk Assessment**:
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Breaking changes | [H/M/L] | [H/M/L] | [Strategy] |
| Data loss | [H/M/L] | [H/M/L] | [Strategy] |
| Downtime | [H/M/L] | [H/M/L] | [Strategy] |
| Performance regression | [H/M/L] | [H/M/L] | [Strategy] |

**Rollback Strategy**:
[How to revert if migration fails]

**Timeline**:
| Phase | Description | Duration |
|-------|-------------|----------|
| Preparation | [Tasks] | [Est.] |
| Execution | [Tasks] | [Est.] |
| Validation | [Tasks] | [Est.] |
| Cleanup | [Tasks] | [Est.] |
```

## Step 2: Dependency Upgrades

Systematic approach to updating dependencies:

```markdown
### Dependency Upgrade Process

**Step 1: Audit Current Dependencies**
```bash
# Node.js
npm outdated
npm audit

# Python
pip list --outdated
pip-audit

# Go
go list -m -u all

# Rust
cargo outdated
```

**Step 2: Categorize Updates**
| Package | Current | Latest | Type | Risk | Priority |
|---------|---------|--------|------|------|----------|
| [pkg1] | 1.0.0 | 2.0.0 | Major | High | Review breaking changes |
| [pkg2] | 1.0.0 | 1.1.0 | Minor | Low | Update directly |
| [pkg3] | 1.0.0 | 1.0.1 | Patch | Low | Security fix - urgent |

**Step 3: Review Breaking Changes**
```markdown
### Breaking Changes: [Package] v[X] → v[Y]

**Changelog Review**:
[Summary of breaking changes from changelog]

**Code Impact Analysis**:
| Breaking Change | Files Affected | Migration Path |
|-----------------|----------------|----------------|
| [Change 1] | [Files] | [How to update] |

**Migration Code**:
```[language]
// Before
[old code]

// After
[new code]
```
```

**Step 4: Upgrade Sequence**
```bash
# 1. Create branch
git checkout -b upgrade/[package-name]-[version]

# 2. Update single package
npm install [package]@[version]

# 3. Run tests
npm test

# 4. Fix any issues

# 5. Commit and repeat for next package
```

**Step 5: Verification Matrix**
| Check | Status | Notes |
|-------|--------|-------|
| All tests pass | ✅/❌ | |
| No new deprecation warnings | ✅/❌ | |
| Build succeeds | ✅/❌ | |
| App starts correctly | ✅/❌ | |
| Critical paths work | ✅/❌ | |
```

## Step 3: Database Migrations

Safe database schema evolution:

```markdown
### Database Migration Process

**Migration File**:
```sql
-- Migration: [name]
-- Created: [date]
-- Description: [what this migration does]

-- Up Migration
BEGIN;

-- Add new column (with default to avoid locking)
ALTER TABLE users
ADD COLUMN status VARCHAR(20) DEFAULT 'active';

-- Create index concurrently (no lock)
CREATE INDEX CONCURRENTLY idx_users_status ON users(status);

COMMIT;

-- Down Migration (Rollback)
BEGIN;

DROP INDEX IF EXISTS idx_users_status;
ALTER TABLE users DROP COLUMN IF EXISTS status;

COMMIT;
```

**Migration Safety Checklist**:
- [ ] Migration is backward compatible (old code still works)
- [ ] Migration is reversible (down migration tested)
- [ ] Large tables use non-blocking operations
- [ ] Indexes created CONCURRENTLY
- [ ] No data loss in rollback
- [ ] Tested on production-like data volume

**Multi-Phase Migration Pattern**:
For breaking changes, use expand-contract pattern:

```markdown
### Expand-Contract Migration

**Phase 1: Expand** (Backward compatible)
```sql
-- Add new column, keep old
ALTER TABLE users ADD COLUMN email_new VARCHAR(255);
```

**Phase 2: Migrate** (Dual-write)
```python
# Application writes to both columns
user.email = value
user.email_new = normalize(value)
```

**Phase 3: Backfill** (Background job)
```sql
UPDATE users SET email_new = normalize(email) WHERE email_new IS NULL;
```

**Phase 4: Switch** (Use new column)
```python
# Application reads from new column
email = user.email_new
```

**Phase 5: Contract** (Remove old)
```sql
ALTER TABLE users DROP COLUMN email;
ALTER TABLE users RENAME COLUMN email_new TO email;
```
```

**Migration Execution**:
```bash
# 1. Backup database
pg_dump -Fc mydb > backup_$(date +%Y%m%d).dump

# 2. Run migration in transaction (if possible)
psql -f migration.sql

# 3. Verify migration
psql -c "SELECT COUNT(*) FROM users WHERE status IS NOT NULL;"

# 4. If issues, rollback
psql -f migration_rollback.sql
```
```

## Step 4: API Migrations

Evolve APIs without breaking clients:

```markdown
### API Migration Strategy

**Versioning Approaches**:
| Approach | Pros | Cons | Use When |
|----------|------|------|----------|
| URL versioning (/v1/, /v2/) | Clear, cacheable | URL pollution | Public APIs |
| Header versioning | Clean URLs | Hidden version | Internal APIs |
| Query param (?version=2) | Easy to test | Ugly | Transitional |

**Deprecation Timeline**:
```
v1 (current) ──────────────────────────────────────┐
                                                    │ Sunset
v2 (new)     ├── Beta ──├── GA ──├── Recommended ──┴────────
             │          │        │
           Announce   Release  Deprecate v1
```

**Backward Compatible Changes** (Safe):
- Adding new endpoints
- Adding optional fields to requests
- Adding fields to responses
- Adding new enum values (if client handles unknown)

**Breaking Changes** (Require new version):
- Removing/renaming endpoints
- Removing/renaming fields
- Changing field types
- Changing authentication
- Changing error formats

**Migration Implementation**:
```python
# Versioned endpoint handler
@app.route('/api/v2/users', methods=['GET'])
def get_users_v2():
    users = fetch_users()
    return jsonify({
        'data': [user.to_v2_dict() for user in users],
        'meta': {'version': 'v2'}
    })

# Keep v1 working during transition
@app.route('/api/v1/users', methods=['GET'])
def get_users_v1():
    log_deprecation_warning('v1/users')
    users = fetch_users()
    return jsonify([user.to_v1_dict() for user in users])
```

**Client Migration Guide**:
```markdown
## Migrating from API v1 to v2

### Endpoint Changes
| v1 | v2 | Notes |
|----|----|----- |
| GET /v1/users | GET /v2/users | Response format changed |

### Response Changes
```json
// v1 Response
[{"id": 1, "name": "John"}]

// v2 Response
{"data": [{"id": 1, "name": "John"}], "meta": {...}}
```

### Migration Steps
1. Update base URL to /v2
2. Update response parsing for wrapper object
3. Handle new error format
```
```

## Step 5: Data Migrations

Move and transform data safely:

```markdown
### Data Migration Process

**Migration Specification**:
| Aspect | Details |
|--------|---------|
| Source | [Source system/format] |
| Target | [Target system/format] |
| Volume | [Number of records] |
| Transformation | [What changes] |

**Data Mapping**:
| Source Field | Target Field | Transformation |
|--------------|--------------|----------------|
| user_id | id | Direct copy |
| created_at | created_at | Convert timezone |
| status | status | Map: 0→'inactive', 1→'active' |

**Migration Script**:
```python
def migrate_users(batch_size=1000):
    """Migrate users from old to new format."""
    offset = 0
    migrated = 0
    errors = []

    while True:
        # Fetch batch
        old_users = source_db.query(
            "SELECT * FROM users LIMIT %s OFFSET %s",
            [batch_size, offset]
        )

        if not old_users:
            break

        # Transform and insert
        for old in old_users:
            try:
                new = transform_user(old)
                target_db.insert('users', new)
                migrated += 1
            except Exception as e:
                errors.append({'id': old['id'], 'error': str(e)})

        offset += batch_size
        log.info(f"Migrated {migrated} users, {len(errors)} errors")

    return {'migrated': migrated, 'errors': errors}
```

**Validation Queries**:
```sql
-- Count comparison
SELECT 'source' as db, COUNT(*) FROM source.users
UNION ALL
SELECT 'target' as db, COUNT(*) FROM target.users;

-- Data integrity check
SELECT s.id, s.email, t.email
FROM source.users s
LEFT JOIN target.users t ON s.id = t.id
WHERE s.email != t.email OR t.id IS NULL;

-- Spot check samples
SELECT * FROM target.users ORDER BY RANDOM() LIMIT 10;
```

**Rollback Data**:
```bash
# Backup before migration
pg_dump -t users target_db > users_backup.sql

# If migration fails, restore
psql target_db < users_backup.sql
```
```

## Step 6: Migration Execution Checklist

Structured execution with checkpoints:

```markdown
### Migration Execution: [Migration Name]

**Pre-Migration** (T-24h):
- [ ] Migration script tested on staging
- [ ] Rollback script tested
- [ ] Backup completed and verified
- [ ] Team notified of maintenance window
- [ ] Monitoring alerts reviewed

**Pre-Migration** (T-1h):
- [ ] Final backup taken
- [ ] Current metrics baseline recorded
- [ ] Team on standby
- [ ] Communication sent to stakeholders

**Execution**:
```
Time | Step | Status | Notes
-----|------|--------|------
HH:MM | Start migration | |
HH:MM | Step 1: [action] | ✅/❌ |
HH:MM | Step 2: [action] | ✅/❌ |
HH:MM | Validation | ✅/❌ |
HH:MM | Complete | ✅/❌ |
```

**Validation Checkpoints**:
- [ ] Application starts without errors
- [ ] Health checks pass
- [ ] Sample transactions succeed
- [ ] Monitoring shows normal metrics
- [ ] No error spike in logs

**Post-Migration**:
- [ ] Monitor for 1 hour
- [ ] Run full test suite
- [ ] Update documentation
- [ ] Archive migration artifacts
- [ ] Post-migration review scheduled

**Go/No-Go Decision Points**:
| Checkpoint | Criteria | Decision |
|------------|----------|----------|
| Pre-migration | Tests pass, backup valid | GO / NO-GO |
| Mid-migration | No critical errors | CONTINUE / ROLLBACK |
| Post-migration | Validation passes | COMPLETE / ROLLBACK |
```

## Step 7: Rollback Procedures

Be prepared to undo changes:

```markdown
### Rollback Procedures

**Automatic Rollback Triggers**:
- [ ] Error rate exceeds [X]%
- [ ] Response time exceeds [Y]ms p95
- [ ] Critical functionality fails
- [ ] Data corruption detected

**Rollback Steps**:

**For Dependency Upgrade**:
```bash
# Revert to previous lock file
git checkout HEAD~1 -- package-lock.json
npm ci
# Or restore from backup
npm ci --package-lock-only
```

**For Database Migration**:
```bash
# Run down migration
psql -f migration_rollback.sql

# Or restore from backup
pg_restore -d mydb backup.dump
```

**For API Migration**:
```bash
# Revert deployment
kubectl rollout undo deployment/api

# Or switch traffic back
kubectl patch service api -p '{"spec":{"selector":{"version":"v1"}}}'
```

**For Data Migration**:
```bash
# Restore from backup
psql target_db < pre_migration_backup.sql

# Re-sync from source if available
./sync_from_source.sh
```

**Rollback Verification**:
- [ ] Application functioning normally
- [ ] Data integrity verified
- [ ] Metrics returned to baseline
- [ ] Users can complete key workflows
```

## Phase 14 Approval Gate

```markdown
## Phase 14 Summary: Migration Complete

### Migration Status

**Migration**: [Migration Name]
**Type**: [Dependency / Database / API / Data / Infrastructure]
**Status**: [Complete / Rolled Back / Partial]

### Execution Summary

| Metric | Value |
|--------|-------|
| Duration | [Time] |
| Downtime | [Time or "None"] |
| Records migrated | [N] |
| Errors encountered | [N] |
| Rollbacks performed | [N] |

### Validation Results

| Check | Before | After | Status |
|-------|--------|-------|--------|
| All tests pass | ✅ | ✅ | ✅ |
| Performance baseline | [X]ms | [Y]ms | ✅/❌ |
| Error rate | [X]% | [Y]% | ✅/❌ |
| Data integrity | N/A | Verified | ✅ |

### Outstanding Items

| Item | Status | Owner |
|------|--------|-------|
| Remove deprecated code | Pending | [Name] |
| Update documentation | Done | [Name] |
| Deprecation notices | Sent | [Name] |

### Lessons Learned

**What went well**:
- [Item 1]

**What could improve**:
- [Item 1]

---

**Migration complete and verified?**
```
