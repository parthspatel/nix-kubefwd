# Kubefwd Daemon Requirements Document

## 1. Overview

### 1.1 Problem Statement

[kubefwd](https://github.com/txn2/kubefwd) is a powerful tool for bulk port-forwarding Kubernetes services to local development workstations. However, users experience reliability issues:

- Port forwarding connections drop when underlying pods restart
- Built-in auto-reconnect is unreliable in practice
- No centralized management or monitoring of forwarding state
- Manual intervention required to restore forwarding after failures
- No persistent configuration across system restarts

### 1.2 Solution

Implement a **kubefwd daemon** that provides:

- Robust process management and supervision of kubefwd instances
- Reliable automatic reconnection with configurable retry strategies
- Health monitoring and connection state tracking
- Declarative configuration for port-forwarding profiles
- System service integration (systemd/launchd) via Nix
- CLI interface for daemon control and status inspection

### 1.3 Goals

1. **Reliability**: Ensure port forwarding remains stable despite pod restarts, network issues, or kubefwd crashes
2. **Observability**: Provide clear visibility into forwarding state and connection health
3. **Ease of Use**: Simple declarative configuration and intuitive CLI
4. **Portability**: Work across Linux and macOS via Nix packaging

---

## 2. Functional Requirements

### 2.1 Daemon Core (FR-CORE)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-CORE-01 | Daemon shall run as a background system service | Must |
| FR-CORE-02 | Daemon shall manage one or more kubefwd processes | Must |
| FR-CORE-03 | Daemon shall automatically restart kubefwd on crash/exit | Must |
| FR-CORE-04 | Daemon shall support graceful shutdown with cleanup | Must |
| FR-CORE-05 | Daemon shall persist state across restarts | Should |
| FR-CORE-06 | Daemon shall support hot-reload of configuration | Should |

### 2.2 Connection Management (FR-CONN)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-CONN-01 | Daemon shall detect when port forwarding connections drop | Must |
| FR-CONN-02 | Daemon shall automatically reconnect with exponential backoff | Must |
| FR-CONN-03 | Exponential backoff shall be configurable (initial delay, max delay, multiplier) | Should |
| FR-CONN-04 | Daemon shall support maximum retry attempts (with optional infinite retry) | Should |
| FR-CONN-05 | Daemon shall track connection uptime and failure counts | Must |
| FR-CONN-06 | Daemon shall support health checks to verify forwarding is working | Should |

### 2.3 Configuration (FR-CFG)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-CFG-01 | Support declarative configuration via YAML/TOML file | Must |
| FR-CFG-02 | Support multiple named forwarding profiles | Must |
| FR-CFG-03 | Each profile shall specify: namespace(s), label selectors, kubeconfig | Must |
| FR-CFG-04 | Support profile-specific retry/backoff configuration | Should |
| FR-CFG-05 | Support enabling/disabling profiles without removal | Should |
| FR-CFG-06 | Configuration shall be validated on load with clear error messages | Must |
| FR-CFG-07 | Support Nix-based configuration for NixOS/home-manager | Should |

### 2.4 CLI Interface (FR-CLI)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-CLI-01 | CLI shall support `start` command to start the daemon | Must |
| FR-CLI-02 | CLI shall support `stop` command to stop the daemon | Must |
| FR-CLI-03 | CLI shall support `status` command showing all profiles and their state | Must |
| FR-CLI-04 | CLI shall support `restart` command for daemon restart | Must |
| FR-CLI-05 | CLI shall support `reload` command for config hot-reload | Should |
| FR-CLI-06 | CLI shall support `logs` command to view daemon logs | Should |
| FR-CLI-07 | CLI shall support `enable/disable <profile>` for profile management | Should |
| FR-CLI-08 | CLI shall provide colored, human-readable output | Should |
| FR-CLI-09 | CLI shall support `--json` flag for machine-readable output | Should |

### 2.5 Monitoring & Observability (FR-MON)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-MON-01 | Daemon shall log all significant events (start, stop, reconnect, errors) | Must |
| FR-MON-02 | Daemon shall expose metrics (uptime, reconnect count, current state) | Should |
| FR-MON-03 | Support optional Prometheus metrics endpoint | Could |
| FR-MON-04 | Support desktop notifications on connection state changes | Could |
| FR-MON-05 | Daemon shall log kubefwd stdout/stderr with proper attribution | Must |

---

## 3. Non-Functional Requirements

### 3.1 Performance (NFR-PERF)

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-PERF-01 | Daemon memory footprint | < 50 MB |
| NFR-PERF-02 | Reconnection detection latency | < 5 seconds |
| NFR-PERF-03 | Daemon startup time | < 2 seconds |

### 3.2 Reliability (NFR-REL)

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-REL-01 | Daemon shall not crash on kubefwd failure | 100% |
| NFR-REL-02 | Daemon shall recover from transient network issues | Automatic |
| NFR-REL-03 | Daemon shall handle kubeconfig/context changes | Graceful restart |

### 3.3 Security (NFR-SEC)

| ID | Requirement | Notes |
|----|-------------|-------|
| NFR-SEC-01 | Daemon shall run with minimal required privileges | sudo only for kubefwd child |
| NFR-SEC-02 | Configuration files shall support restricted permissions | 0600 recommended |
| NFR-SEC-03 | Daemon shall not expose sensitive data in logs | Mask kubeconfig paths |
| NFR-SEC-04 | Unix socket for CLI communication shall have restricted permissions | User-only access |

### 3.4 Compatibility (NFR-COMPAT)

| ID | Requirement | Notes |
|----|-------------|-------|
| NFR-COMPAT-01 | Support Linux (x86_64, aarch64) | Primary target |
| NFR-COMPAT-02 | Support macOS (x86_64, aarch64) | Secondary target |
| NFR-COMPAT-03 | Support kubefwd v1.x | Current stable |
| NFR-COMPAT-04 | Support Kubernetes 1.25+ | Current supported versions |

---

## 4. Configuration Schema

### 4.1 Example Configuration (YAML)

```yaml
# ~/.config/kubefwd-daemon/config.yaml

daemon:
  log_level: info                    # debug, info, warn, error
  log_file: /var/log/kubefwd-daemon.log
  pid_file: /run/kubefwd-daemon.pid
  socket_path: /run/kubefwd-daemon.sock

defaults:
  retry:
    initial_delay: 1s
    max_delay: 60s
    multiplier: 2.0
    max_attempts: 0                   # 0 = infinite

profiles:
  dev-services:
    enabled: true
    kubeconfig: ~/.kube/config
    context: dev-cluster
    namespaces:
      - development
      - shared-services
    labels:
      - "app.kubernetes.io/part-of=myapp"
    exclude_services:
      - "legacy-*"
    retry:
      max_delay: 30s                  # Override default

  staging-api:
    enabled: false
    kubeconfig: ~/.kube/config
    context: staging-cluster
    namespaces:
      - staging
    labels:
      - "component=api"
```

### 4.2 Nix Configuration (home-manager)

```nix
# Example home-manager module usage
services.kubefwd-daemon = {
  enable = true;
  profiles = {
    dev-services = {
      namespaces = [ "development" "shared-services" ];
      context = "dev-cluster";
      labels = [ "app.kubernetes.io/part-of=myapp" ];
    };
  };
};
```

---

## 5. User Stories

### 5.1 Developer Daily Workflow

> As a developer, I want kubefwd to automatically maintain connections to my dev cluster services so that I can code without manually restarting port forwarding when pods cycle.

**Acceptance Criteria:**
- Daemon starts automatically on login
- Forwarding recovers within 30 seconds of pod restart
- No manual intervention required for typical pod lifecycle events

### 5.2 Multi-Cluster Development

> As a developer working with multiple clusters, I want to define separate forwarding profiles so that I can easily switch between projects.

**Acceptance Criteria:**
- Can define multiple profiles in configuration
- Can enable/disable profiles via CLI
- Each profile uses its own kubeconfig/context

### 5.3 Debugging Connection Issues

> As a developer, I want to easily see the status of my port forwarding and recent events so that I can diagnose connectivity issues.

**Acceptance Criteria:**
- `kubefwd-daemon status` shows all profiles with state and uptime
- Recent reconnection events are visible
- Logs are easily accessible

### 5.4 CI/CD Integration

> As a DevOps engineer, I want to programmatically control the daemon so that I can integrate it into automation scripts.

**Acceptance Criteria:**
- CLI supports `--json` output
- Exit codes are meaningful (0=success, non-zero=error)
- Can start/stop specific profiles programmatically

---

## 6. Out of Scope (v1.0)

The following are explicitly out of scope for the initial release:

1. **Web UI** - CLI-only for v1.0
2. **Windows support** - Linux/macOS only
3. **Custom port mapping** - Use kubefwd's native capabilities
4. **Service mesh integration** - Standard Kubernetes services only
5. **Multi-user support** - Single-user daemon per installation
6. **Remote daemon control** - Local Unix socket only

---

## 7. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Mean time to recover (MTTR) after pod restart | < 30 seconds | Automated testing |
| False positive reconnection rate | < 1% | Log analysis |
| Configuration errors caught at load time | 100% | Validation coverage |
| User-reported connection drops requiring manual intervention | 0 per week | User feedback |

---

## 8. Technical Constraints

1. **Language**: Rust (for reliability, performance, and Nix ecosystem compatibility)
2. **Packaging**: Nix flake with binary cache support
3. **Service Manager**: systemd (Linux) / launchd (macOS) integration
4. **IPC**: Unix domain socket for CLI-daemon communication
5. **Dependencies**: Minimal - kubefwd binary must be available in PATH

---

## 9. Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-01-15 | Initial | Initial requirements |
