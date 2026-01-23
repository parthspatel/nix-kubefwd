# Phase 15: Performance Optimization

## Objective

Identify and resolve performance bottlenecks using data-driven analysis, particularly flame graphs and profiling tools. Optimize based on evidence rather than assumptions, measuring impact of each change.

## Step 1: Performance Assessment

Establish baseline before optimizing:

```markdown
### Performance Assessment: [Component/System]

**Current Performance Baseline**:
| Metric | Current | Target | Gap |
|--------|---------|--------|-----|
| Response time (p50) | [X]ms | [Y]ms | [Z]ms |
| Response time (p95) | [X]ms | [Y]ms | [Z]ms |
| Response time (p99) | [X]ms | [Y]ms | [Z]ms |
| Throughput | [X] req/s | [Y] req/s | [Z] req/s |
| Memory usage | [X]MB | [Y]MB | [Z]MB |
| CPU usage | [X]% | [Y]% | [Z]% |

**Measurement Method**:
```bash
# Load testing baseline
k6 run --vus 10 --duration 60s baseline.js

# Application profiling
node --prof app.js
# Or
py-spy record -o profile.svg -- python app.py
```

**Performance Budget**:
| Operation | Budget | Rationale |
|-----------|--------|-----------|
| API response | <200ms | User experience |
| Page load | <3s | Industry standard |
| Background job | <30s | SLA requirement |

**Symptoms Observed**:
- [ ] Slow response times
- [ ] High CPU usage
- [ ] Memory growth over time
- [ ] Timeout errors
- [ ] Database connection exhaustion
```

## Step 2: Flame Graph Analysis

Use flame graphs to identify bottlenecks:

```markdown
### Flame Graph Generation

**CPU Flame Graph**:
```bash
# Node.js (using 0x)
npx 0x app.js
# Output: flamegraph.html

# Python (using py-spy)
py-spy record -o flamegraph.svg --pid [PID]
# Or during execution
py-spy record -o flamegraph.svg -- python app.py

# Go (using pprof)
go tool pprof -http=:8080 http://localhost:6060/debug/pprof/profile

# Java (using async-profiler)
./profiler.sh -d 30 -f flamegraph.html [PID]

# Rust (using flamegraph)
cargo flamegraph --bin myapp
```

**Reading Flame Graphs**:
```
┌─────────────────────────────────────────────────────────────┐
│                          main                                │
├──────────────────────────────┬──────────────────────────────┤
│      handleRequest (40%)     │     processData (60%)        │
├───────────────┬──────────────┼──────────────┬───────────────┤
│ parseJSON(25%)│ validate(15%)│ query(45%)   │ transform(15%)│
├───────────────┴──────────────┴──────────────┴───────────────┤

Width = Time spent (wider = more time)
Height = Call stack depth
Look for: Wide blocks (time sinks), unexpected functions

Key Patterns:
- Wide plateau: Single function taking lots of time → optimize it
- Wide base: Many calls to same function → reduce call count
- Tall narrow tower: Deep call stack → possible recursion issue
```

**Flame Graph Insights Template**:
```markdown
### Flame Graph Analysis: [Profile Name]

**Top Time Consumers**:
| Function | Time % | Category | Action |
|----------|--------|----------|--------|
| [func1] | 45% | Database | Optimize query |
| [func2] | 25% | Serialization | Consider caching |
| [func3] | 15% | Computation | Algorithm review |

**Unexpected Findings**:
- [Function X is called Y times when once expected]
- [Logging takes Z% of request time]

**Optimization Priorities**:
1. [Highest impact target]
2. [Second target]
3. [Third target]
```
```

## Step 3: Memory Profiling

Identify memory issues:

```markdown
### Memory Analysis

**Memory Flame Graph (Allocation Profiling)**:
```bash
# Node.js heap snapshot
node --heapsnapshot-signal=SIGUSR2 app.js
kill -SIGUSR2 [PID]
# Open in Chrome DevTools

# Python memory profiling
mprof run python app.py
mprof plot  # Generate graph

# Go memory profiling
go tool pprof http://localhost:6060/debug/pprof/heap
```

**Memory Leak Detection**:
```markdown
### Memory Leak Investigation

**Symptoms**:
- [ ] Memory grows continuously over time
- [ ] OOM errors after extended operation
- [ ] GC pauses increasing

**Heap Snapshot Comparison**:
| Snapshot | Time | Heap Size | Delta |
|----------|------|-----------|-------|
| Initial | T+0 | [X]MB | - |
| After 1h | T+1h | [Y]MB | +[Z]MB |
| After 2h | T+2h | [A]MB | +[B]MB |

**Retained Objects Analysis**:
| Object Type | Count | Size | Likely Cause |
|-------------|-------|------|--------------|
| [Type 1] | [N] | [X]MB | [Cause] |
| [Type 2] | [N] | [X]MB | [Cause] |

**Common Leak Patterns**:
- Event listeners not removed
- Closures holding references
- Cache without eviction
- Global/static collections growing
- Timer callbacks holding references
```
```

## Step 4: Database Performance

Optimize database interactions:

```markdown
### Database Performance Analysis

**Query Profiling**:
```sql
-- PostgreSQL: Enable query logging
SET log_min_duration_statement = 100;  -- Log queries > 100ms

-- Analyze slow query
EXPLAIN (ANALYZE, BUFFERS, FORMAT TEXT)
SELECT * FROM orders WHERE customer_id = 123;
```

**Query Analysis Template**:
```markdown
### Slow Query: [Query Description]

**Query**:
```sql
[The slow query]
```

**Execution Plan**:
```
[EXPLAIN ANALYZE output]
```

**Issues Identified**:
| Issue | Impact | Solution |
|-------|--------|----------|
| Sequential scan on large table | High | Add index |
| N+1 query pattern | High | Use JOIN or batch |
| Missing index | Medium | Create index |
| Suboptimal join order | Medium | Rewrite query |

**Index Recommendations**:
```sql
-- Proposed indexes
CREATE INDEX idx_orders_customer ON orders(customer_id);
CREATE INDEX idx_orders_date ON orders(created_at) WHERE status = 'pending';
```
```

**N+1 Query Detection**:
```python
# Before: N+1 pattern
for user in users:
    orders = db.query("SELECT * FROM orders WHERE user_id = ?", user.id)

# After: Batch query
user_ids = [u.id for u in users]
orders = db.query("SELECT * FROM orders WHERE user_id IN (?)", user_ids)
orders_by_user = group_by(orders, 'user_id')
```

**Connection Pool Tuning**:
| Parameter | Current | Recommended | Rationale |
|-----------|---------|-------------|-----------|
| Pool size | [X] | [Y] | [Why] |
| Idle timeout | [X]s | [Y]s | [Why] |
| Max lifetime | [X]s | [Y]s | [Why] |
```

## Step 5: Optimization Techniques

Apply targeted optimizations:

```markdown
### Optimization Techniques

**Caching Strategies**:
```python
# Application-level caching
@cache(ttl=300)  # 5 minute cache
def get_user_profile(user_id):
    return db.query("SELECT * FROM users WHERE id = ?", user_id)

# Query result caching
cache_key = f"user:{user_id}:orders"
orders = cache.get(cache_key)
if orders is None:
    orders = fetch_orders(user_id)
    cache.set(cache_key, orders, ttl=60)
```

**Algorithm Optimization**:
| Pattern | Before | After | Improvement |
|---------|--------|-------|-------------|
| Linear search | O(n) | O(log n) hash/tree | 100x for n=10000 |
| Nested loops | O(n²) | O(n) with hash | 1000x for n=1000 |
| String concat | O(n²) | O(n) builder | 10x for n=100 |

**Async/Parallel Processing**:
```python
# Before: Sequential
result1 = fetch_data_a()  # 100ms
result2 = fetch_data_b()  # 100ms
result3 = fetch_data_c()  # 100ms
# Total: 300ms

# After: Parallel
import asyncio
result1, result2, result3 = await asyncio.gather(
    fetch_data_a(),
    fetch_data_b(),
    fetch_data_c()
)
# Total: 100ms (limited by slowest)
```

**Memory Optimization**:
```python
# Before: Load all into memory
data = list(fetch_all_records())  # 1GB in memory
process(data)

# After: Stream processing
for batch in fetch_in_batches(size=1000):
    process(batch)  # 1MB at a time
```

**Lazy Loading**:
```python
# Before: Eager load everything
class User:
    def __init__(self, id):
        self.profile = fetch_profile(id)      # Always loaded
        self.orders = fetch_orders(id)        # Always loaded
        self.preferences = fetch_prefs(id)    # Always loaded

# After: Lazy load on access
class User:
    def __init__(self, id):
        self._id = id
        self._profile = None

    @property
    def profile(self):
        if self._profile is None:
            self._profile = fetch_profile(self._id)
        return self._profile
```
```

## Step 6: Optimization Iteration Loop

Measure, optimize, verify:

```markdown
### Optimization Iteration

**Iteration Loop**:
```
┌─────────────────────────────────────────┐
│      OPTIMIZATION LOOP                   │
├─────────────────────────────────────────┤
│                                         │
│  1. Profile (Flame Graph)               │
│          ↓                              │
│  2. Identify Top Bottleneck             │
│          ↓                              │
│  3. Hypothesize Solution                │
│          ↓                              │
│  4. Implement Fix                       │
│          ↓                              │
│  5. Measure Impact                      │
│          ↓                              │
│  6. Target Met?                         │
│      ╱      ╲                           │
│    Yes       No                         │
│     ↓         ↓                         │
│   Done      Return to Step 1            │
│                                         │
└─────────────────────────────────────────┘
```

**Iteration Log**:
| # | Bottleneck | Change | Before | After | Improvement |
|---|------------|--------|--------|-------|-------------|
| 1 | DB query in loop | Batch query | 500ms | 50ms | 90% |
| 2 | JSON serialization | Use orjson | 100ms | 20ms | 80% |
| 3 | Missing index | Add index | 200ms | 10ms | 95% |

**Current vs Target**:
| Metric | Baseline | Current | Target | Status |
|--------|----------|---------|--------|--------|
| p95 latency | 800ms | 150ms | 200ms | ✅ |
| Throughput | 50 req/s | 200 req/s | 100 req/s | ✅ |
| Memory | 2GB | 500MB | 1GB | ✅ |
```

## Step 7: Performance Testing

Validate optimizations under load:

```markdown
### Performance Test Suite

**Load Test Script (k6)**:
```javascript
import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: '1m', target: 10 },   // Ramp up
    { duration: '3m', target: 10 },   // Steady state
    { duration: '1m', target: 50 },   // Spike
    { duration: '2m', target: 50 },   // Sustained high
    { duration: '1m', target: 0 },    // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<200'],
    http_req_failed: ['rate<0.01'],
  },
};

export default function () {
  const res = http.get('http://localhost:8080/api/endpoint');
  check(res, {
    'status is 200': (r) => r.status === 200,
    'response time < 200ms': (r) => r.timings.duration < 200,
  });
  sleep(1);
}
```

**Test Scenarios**:
| Scenario | VUs | Duration | Success Criteria |
|----------|-----|----------|------------------|
| Baseline | 10 | 5m | p95 < 200ms |
| Normal load | 50 | 10m | p95 < 200ms, errors < 1% |
| Peak load | 100 | 5m | p95 < 500ms, errors < 5% |
| Spike | 10→100→10 | 3m | Recovery < 30s |
| Endurance | 50 | 1h | No memory leak, stable latency |

**Results Summary**:
```
Scenario: Normal Load
Duration: 10m
VUs: 50

Requests:
  Total: 30,000
  Rate: 50/s
  Failed: 12 (0.04%)

Latency:
  p50: 45ms
  p95: 120ms
  p99: 180ms

Throughput: 50 req/s sustained
```
```

## Step 8: Performance Regression Prevention

Keep performance gains:

```markdown
### Performance Monitoring

**Continuous Performance Testing**:
```yaml
# CI pipeline performance gate
performance-test:
  script:
    - k6 run --out json=results.json load-test.js
    - python check_thresholds.py results.json
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

**Performance Budgets in CI**:
```javascript
// performance-budget.json
{
  "metrics": {
    "p95_latency_ms": { "max": 200 },
    "p99_latency_ms": { "max": 500 },
    "error_rate": { "max": 0.01 },
    "memory_mb": { "max": 512 }
  }
}
```

**Alerting Thresholds**:
| Metric | Warning | Critical | Action |
|--------|---------|----------|--------|
| p95 latency | >150ms | >300ms | Page on-call |
| Error rate | >0.5% | >2% | Page on-call |
| Memory | >80% | >95% | Auto-scale / page |
| CPU | >70% | >90% | Auto-scale |

**Performance Dashboard**:
```
Key Metrics to Track:
- Request rate (req/s)
- Error rate (%)
- Latency percentiles (p50, p95, p99)
- Saturation (CPU, memory, connections)
- Apdex score
```
```

## Phase 15 Approval Gate

```markdown
## Phase 15 Summary: Performance Optimization

### Optimization Results

**Before vs After**:
| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| p50 latency | [X]ms | [Y]ms | [Z]% |
| p95 latency | [X]ms | [Y]ms | [Z]% |
| p99 latency | [X]ms | [Y]ms | [Z]% |
| Throughput | [X] req/s | [Y] req/s | [Z]% |
| Memory usage | [X]MB | [Y]MB | [Z]% |
| CPU usage | [X]% | [Y]% | [Z]% |

### Optimizations Applied

| Optimization | Component | Impact |
|--------------|-----------|--------|
| [Opt 1] | [Component] | [X]% improvement |
| [Opt 2] | [Component] | [X]% improvement |

### Flame Graph Summary

**Before Optimization**:
- Top consumer: [Function] at [X]%
- Key bottleneck: [Description]

**After Optimization**:
- Top consumer: [Function] at [Y]%
- Bottleneck resolved: [How]

### Verification

- [ ] Performance targets met
- [ ] Load tests pass
- [ ] No regressions in functionality
- [ ] Monitoring in place
- [ ] Performance budgets enforced in CI

### Remaining Opportunities

| Opportunity | Potential Gain | Effort | Priority |
|-------------|----------------|--------|----------|
| [Item 1] | [X]% | [H/M/L] | [H/M/L] |

---

**Performance targets achieved?**
```
