# Kubefwd Daemon Requirements Document

## 1. Overview

### 1.1 Problem Statement

[kubefwd](https://github.com/txn2/kubefwd) is a powerful tool for bulk port-forwarding Kubernetes services to local development workstations. However, users experience reliability issues:

- Port forwarding connections drop when underlying pods restart
- Built-in auto-reconnect is unreliable in practice
- No centralized management or monitoring of forwarding state
- Manual intervention required to restore forwarding after failures
- No persistent configuration across system restarts
- No isolation between multiple projects using the same namespaces

### 1.2 Solution

Implement a **kubefwd daemon** that provides:

- Robust process supervision of a single kubefwd instance (via REST API)
- Real-time event monitoring via SSE for instant reconnection awareness
- Declarative configuration for port-forwarding profiles
- Project isolation via unique domain suffixes (`--domain` flag)
- System service integration (systemd/launchd) and devenv integration via Nix
- CLI interface for daemon control and status inspection

### 1.3 Goals

1. **Reliability**: Ensure port forwarding remains stable despite pod restarts, network issues, or kubefwd crashes
2. **Observability**: Provide clear visibility into forwarding state and connection health via kubefwd's REST API
3. **Isolation**: Enable multiple projects to use kubefwd simultaneously without conflicts
4. **Ease of Use**: Simple declarative configuration with sensible defaults and intuitive CLI
5. **Portability**: Work across Linux and macOS via Nix packaging

---

## 2. Functional Requirements

### 2.1 Daemon Core (FR-CORE)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-CORE-01 | Daemon shall run as a background service (system, user, or devenv) | Must |
| FR-CORE-02 | Daemon shall manage a single kubefwd process in idle + API mode | Must |
| FR-CORE-03 | Daemon shall automatically restart kubefwd on crash/exit | Must |
| FR-CORE-04 | Daemon shall support graceful shutdown with cleanup | Must |
| FR-CORE-05 | Daemon shall persist state across restarts | Should |
| FR-CORE-06 | Daemon shall support hot-reload of configuration | Should |
| FR-CORE-07 | Daemon shall control kubefwd via REST API (add/remove namespaces) | Must |
| FR-CORE-08 | Daemon shall monitor kubefwd events via SSE stream | Must |
| FR-CORE-09 | Daemon shall validate sudoers configuration at startup (use `sudo -n`) | Must |

### 2.2 Connection Management (FR-CONN)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-CONN-01 | Daemon shall detect connection drops via SSE events from kubefwd | Must |
| FR-CONN-02 | Daemon shall track reconnection attempts reported by kubefwd | Must |
| FR-CONN-03 | Daemon shall restart kubefwd with exponential backoff if process crashes | Must |
| FR-CONN-04 | Exponential backoff shall be configurable (initial delay, max delay, multiplier) | Should |
| FR-CONN-05 | Daemon shall support maximum retry attempts (with optional infinite retry) | Should |
| FR-CONN-06 | Daemon shall track connection uptime and failure counts per service | Must |
| FR-CONN-07 | Daemon shall health-check kubefwd API on startup before adding namespaces | Must |

### 2.3 Project Isolation (FR-ISO)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-ISO-01 | Each daemon instance shall use a unique domain suffix via `--domain` flag | Must |
| FR-ISO-02 | Default domain suffix shall be deterministically derived from project path hash | Must |
| FR-ISO-03 | Users shall be able to override auto-generated domain suffix | Must |
| FR-ISO-04 | Multiple daemon instances shall be able to run concurrently without conflict | Must |
| FR-ISO-05 | Each project's services shall resolve to unique /etc/hosts entries | Must |
| FR-ISO-06 | Daemon shall validate no domain conflicts before starting kubefwd | Should |

### 2.4 Configuration (FR-CFG)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-CFG-01 | Support declarative configuration via YAML file | Must |
| FR-CFG-02 | Support multiple named forwarding profiles | Must |
| FR-CFG-03 | Each profile shall specify: namespace(s), label selectors, kubeconfig, context | Must |
| FR-CFG-04 | Support profile-specific retry/backoff configuration | Should |
| FR-CFG-05 | Support enabling/disabling profiles without removal | Should |
| FR-CFG-06 | Configuration shall be validated on load with clear error messages | Must |
| FR-CFG-07 | Support Nix-based configuration for NixOS/home-manager/devenv | Must |

### 2.5 CLI Interface (FR-CLI)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-CLI-01 | CLI shall support `start` command to start the daemon | Must |
| FR-CLI-02 | CLI shall support `stop` command to stop the daemon | Must |
| FR-CLI-03 | CLI shall support `status` command showing daemon, kubefwd, and service state | Must |
| FR-CLI-04 | CLI shall support `restart` command for daemon restart | Must |
| FR-CLI-05 | CLI shall support `reload` command for config hot-reload | Should |
| FR-CLI-06 | CLI shall support `logs` command to view daemon logs | Should |
| FR-CLI-07 | CLI shall support `services` command to list forwarded services | Must |
| FR-CLI-08 | CLI shall provide colored, human-readable output | Should |
| FR-CLI-09 | CLI shall support `--json` flag for machine-readable output | Should |

### 2.6 Monitoring & Observability (FR-MON)

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-MON-01 | Daemon shall log all significant events (start, stop, reconnect, errors) | Must |
| FR-MON-02 | Daemon shall expose metrics (uptime, reconnect count, current state) | Must |
| FR-MON-03 | Daemon shall track per-service connection status from SSE events | Must |
| FR-MON-04 | Support optional Prometheus metrics endpoint | Could |
| FR-MON-05 | Support desktop notifications on connection state changes | Could |
| FR-MON-06 | Daemon shall log kubefwd stdout/stderr with proper attribution | Must |

---

## 3. Non-Functional Requirements

### 3.1 Performance (NFR-PERF)

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-PERF-01 | Daemon memory footprint | < 50 MB |
| NFR-PERF-02 | Event detection latency (via SSE) | < 1 second |
| NFR-PERF-03 | Daemon startup time | < 2 seconds |
| NFR-PERF-04 | kubefwd API ready detection | < 3 seconds |

### 3.2 Reliability (NFR-REL)

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-REL-01 | Daemon shall not crash on kubefwd failure | 100% |
| NFR-REL-02 | Daemon shall recover from transient network issues | Automatic |
| NFR-REL-03 | Daemon shall handle kubeconfig/context changes | Graceful restart |
| NFR-REL-04 | SSE connection shall auto-reconnect on disconnect | Automatic |

### 3.3 Security (NFR-SEC)

| ID | Requirement | Notes |
|----|-------------|-------|
| NFR-SEC-01 | Daemon shall run with minimal required privileges | sudo only for kubefwd child |
| NFR-SEC-02 | Users must configure sudoers for passwordless kubefwd execution | Documented prerequisite |
| NFR-SEC-03 | Configuration files shall support restricted permissions | 0600 recommended |
| NFR-SEC-04 | Daemon shall not expose sensitive data in logs | Mask kubeconfig paths |
| NFR-SEC-05 | Unix socket for CLI communication shall have restricted permissions | User-only access |
| NFR-SEC-06 | kubefwd REST API accessible via `kubefwd.internal` hostname | /etc/hosts entry |

### 3.4 Compatibility (NFR-COMPAT)

| ID | Requirement | Notes |
|----|-------------|-------|
| NFR-COMPAT-01 | Support Linux (x86_64, aarch64) | Primary target |
| NFR-COMPAT-02 | Support macOS (x86_64, aarch64) | Secondary target |
| NFR-COMPAT-03 | **Require kubefwd >= v1.25.0** | REST API + idle mode required |
| NFR-COMPAT-04 | Support Kubernetes 1.25+ | Current supported versions |

---

## 4. Configuration Schema

### 4.1 Example Configuration (YAML)

```yaml
# ~/.config/kubefwd-daemon/config.yaml

daemon:
  log_level: info                      # trace, debug, info, warn, error
  log_file: ~/.local/log/kubefwd-daemon.log
  socket_path: /run/user/1000/kubefwd-daemon.sock
  state_file: ~/.local/state/kubefwd-daemon/state.json

  # kubefwd process settings
  kubefwd:
    domain: "proj1.local"              # Override auto-generated domain suffix (optional)

defaults:
  retry:
    initial_delay: 1s
    max_delay: 60s
    multiplier: 2.0
    max_attempts: 0                    # 0 = infinite

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
    services: []                       # Empty = all services
    retry:
      max_delay: 30s                   # Override default

  staging-api:
    enabled: false
    kubeconfig: ~/.kube/config
    context: staging-cluster
    namespaces:
      - staging
    labels:
      - "component=api"
```

### 4.2 Devenv Configuration

```nix
# devenv.nix (in project root)
{ pkgs, lib, ... }:

{
  services.kubefwd = {
    enable = true;
    namespaces = [ "development" "shared-infra" ];
    context = "my-dev-cluster";
    labels = [ "app.kubernetes.io/part-of=ecommerce" ];

    # Optional: specific services only
    services = [ "api-gateway" "postgres" "redis" ];

    # Optional: override auto-generated domain suffix
    # domain = "myproject.local";  # Default: hash-based from project path
  };
}
```

### 4.3 home-manager Configuration

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

## 5. Prerequisites

### 5.1 kubefwd Version

**kubefwd >= v1.25.0 is required.** This version introduced:
- REST API with 40+ endpoints for programmatic control (accessed via `kubefwd.internal`)
- Idle mode (start without namespaces, add via API)
- SSE event streaming for real-time monitoring at `/api/v1/events`
- `--domain` flag for project isolation (custom hostname suffix)

### 5.2 Sudoers Configuration

kubefwd requires root privileges to modify `/etc/hosts`. Users must configure passwordless sudo for kubefwd:

```bash
# /etc/sudoers.d/kubefwd
%wheel ALL=(ALL) NOPASSWD: /usr/bin/kubefwd
# Or for Nix:
%wheel ALL=(ALL) NOPASSWD: /run/current-system/sw/bin/kubefwd
# Or for home-manager:
your-username ALL=(ALL) NOPASSWD: /home/your-username/.nix-profile/bin/kubefwd
```

---

## 6. User Stories

### 6.1 Developer Daily Workflow

> As a developer, I want kubefwd to automatically maintain connections to my dev cluster services so that I can code without manually restarting port forwarding when pods cycle.

**Acceptance Criteria:**
- Daemon starts automatically on login (or with devenv shell)
- Forwarding recovers within 30 seconds of pod restart
- No manual intervention required for typical pod lifecycle events

### 6.2 Multi-Cluster Development

> As a developer working with multiple clusters, I want to define separate forwarding profiles so that I can easily switch between projects.

**Acceptance Criteria:**
- Can define multiple profiles in configuration
- Can enable/disable profiles via CLI
- Each profile uses its own kubeconfig/context

### 6.3 Multi-Project Development (Isolation)

> As a developer working on multiple projects simultaneously, I want each project to have isolated port forwarding so that services from different projects don't conflict even if they use the same namespace names.

**Acceptance Criteria:**
- Each devenv project gets its own kubefwd instance with unique domain suffix
- Each project uses a different domain suffix (e.g., `svc.proj1.local`) to avoid /etc/hosts conflicts
- Domain assignment is automatic by default (hash-based from project path)
- Can manually override domain suffix if needed
- Running `devenv up` in Project A doesn't affect forwarding in Project B
- Services with same name in different projects resolve to different hostnames

### 6.4 Debugging Connection Issues

> As a developer, I want to easily see the status of my port forwarding and recent events so that I can diagnose connectivity issues.

**Acceptance Criteria:**
- `kubefwd-ctl status` shows daemon state, kubefwd state, and all services
- Per-service connection status (up, reconnecting, failed)
- Recent reconnection events are visible
- Logs are easily accessible

### 6.5 CI/CD Integration

> As a DevOps engineer, I want to programmatically control the daemon so that I can integrate it into automation scripts.

**Acceptance Criteria:**
- CLI supports `--json` output
- Exit codes are meaningful (0=success, non-zero=error)
- Can start/stop specific profiles programmatically

### 6.6 Devenv Integration

> As a developer using Nix devenv, I want kubefwd to start automatically when I enter my project shell and stop when I exit.

**Acceptance Criteria:**
- kubefwd starts when running `devenv up` or entering the shell
- kubefwd stops gracefully when devenv processes are stopped
- Configuration is declarative in `devenv.nix`
- No global system configuration required

---

## 7. Out of Scope (v1.0)

The following are explicitly out of scope for the initial release:

1. **Web UI** - CLI-only for v1.0
2. **Windows support** - Linux/macOS only
3. **Custom port mapping** - Use kubefwd's native capabilities
4. **Service mesh integration** - Standard Kubernetes services only
5. **Multi-user support** - Single-user daemon per installation
6. **Remote daemon control** - Local Unix socket only
7. **kubefwd < v1.25.0** - REST API required

---

## 8. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Mean time to recover (MTTR) after pod restart | < 30 seconds | Automated testing |
| Event detection latency (SSE) | < 1 second | Log analysis |
| Configuration errors caught at load time | 100% | Validation coverage |
| User-reported connection drops requiring manual intervention | 0 per week | User feedback |
| Project isolation conflicts | 0 | Integration testing |

---

## 9. Technical Constraints

1. **Language**: Rust (for reliability, performance, and Nix ecosystem compatibility)
2. **Packaging**: Nix flake with binary cache support
3. **Service Manager**: systemd (Linux) / launchd (macOS) / process-compose (devenv)
4. **IPC**: Unix domain socket for CLI-daemon communication
5. **kubefwd Integration**: REST API for control, SSE for event monitoring
6. **Dependencies**:
   - kubefwd >= v1.25.0 must be available in PATH
   - Passwordless sudo configured for kubefwd

---

## 10. Deployment Modes

| Mode | Use Case | Lifecycle | Isolation |
|------|----------|-----------|-----------|
| **System Service** | Always-on forwarding for all users | systemd service | Shared |
| **User Service** | Per-user forwarding | systemd user service / launchd | Per-user |
| **Devenv Service** | Per-project forwarding | process-compose | Per-project (unique domain) |

---

## 11. Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-01-15 | - | Initial requirements |
| 2.0 | 2025-01-16 | - | Updated for REST API architecture, added isolation requirements, devenv integration, multi-project user story, kubefwd v1.25.0 requirement |
| 2.1 | 2025-01-16 | - | Corrected kubefwd API details: use `--domain` flag for isolation (not `--api-port`/`--ip-prefix`), API accessed via `kubefwd.internal` hostname, added FR-CORE-09 for sudoers validation |
