# Kubefwd Daemon Architecture Document

## 1. System Overview

### 1.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              User Space                                  │
│  ┌──────────────┐     ┌─────────────────────────────────────────────┐  │
│  │              │     │           kubefwd-daemon                     │  │
│  │  kubefwd-ctl │────▶│  ┌─────────────────────────────────────┐   │  │
│  │    (CLI)     │ IPC │  │         Daemon Core                  │   │  │
│  │              │◀────│  │  ┌─────────┐  ┌──────────────────┐  │   │  │
│  └──────────────┘     │  │  │  Config │  │  State Manager   │  │   │  │
│                       │  │  │  Loader │  │                  │  │   │  │
│                       │  │  └────┬────┘  └────────┬─────────┘  │   │  │
│                       │  │       │                │            │   │  │
│                       │  │       ▼                ▼            │   │  │
│                       │  │  ┌─────────────────────────────┐   │   │  │
│                       │  │  │    Profile Supervisor       │   │   │  │
│                       │  │  └─────────────────────────────┘   │   │  │
│                       │  │       │         │         │        │   │  │
│                       │  └───────┼─────────┼─────────┼────────┘   │  │
│                       │          │         │         │            │  │
│                       │          ▼         ▼         ▼            │  │
│                       │  ┌───────────┐ ┌───────────┐ ┌───────────┐│  │
│                       │  │  Profile  │ │  Profile  │ │  Profile  ││  │
│                       │  │  Worker 1 │ │  Worker 2 │ │  Worker N ││  │
│                       │  └─────┬─────┘ └─────┬─────┘ └─────┬─────┘│  │
│                       │        │             │             │      │  │
│                       └────────┼─────────────┼─────────────┼──────┘  │
│                                │             │             │         │
│                                ▼             ▼             ▼         │
│                        ┌───────────┐ ┌───────────┐ ┌───────────┐    │
│                        │  kubefwd  │ │  kubefwd  │ │  kubefwd  │    │
│                        │  process  │ │  process  │ │  process  │    │
│                        └───────────┘ └───────────┘ └───────────┘    │
└─────────────────────────────────────────────────────────────────────────┘
                                        │
                                        ▼
                              ┌───────────────────┐
                              │   Kubernetes API  │
                              │   (port-forward)  │
                              └───────────────────┘
```

### 1.2 Component Summary

| Component | Responsibility |
|-----------|----------------|
| **kubefwd-ctl** | CLI tool for user interaction with daemon |
| **Daemon Core** | Main daemon process lifecycle and coordination |
| **Config Loader** | Parse, validate, and watch configuration files |
| **State Manager** | Track and persist connection/profile states |
| **Profile Supervisor** | Manage lifecycle of profile workers |
| **Profile Worker** | Manage single kubefwd process with reconnection logic |

---

## 2. Component Design

### 2.1 Daemon Core

The daemon core is the main entry point and orchestrator.

```rust
// Conceptual structure
pub struct Daemon {
    config: Arc<RwLock<DaemonConfig>>,
    state: Arc<StateManager>,
    supervisor: ProfileSupervisor,
    ipc_server: IpcServer,
    shutdown_tx: broadcast::Sender<()>,
}

impl Daemon {
    pub async fn run(&mut self) -> Result<()>;
    pub async fn shutdown(&mut self) -> Result<()>;
    pub async fn reload_config(&mut self) -> Result<()>;
}
```

**Responsibilities:**
- Initialize all subsystems
- Handle Unix signals (SIGTERM, SIGHUP, SIGINT)
- Manage IPC server for CLI communication
- Coordinate graceful shutdown

**Signal Handling:**
| Signal | Action |
|--------|--------|
| SIGTERM | Graceful shutdown |
| SIGINT | Graceful shutdown |
| SIGHUP | Reload configuration |
| SIGUSR1 | Dump state to logs |

### 2.2 Configuration System

#### 2.2.1 Configuration File Structure

```
~/.config/kubefwd-daemon/
├── config.yaml          # Main configuration
└── profiles.d/          # Optional: split profiles
    ├── dev.yaml
    └── staging.yaml
```

#### 2.2.2 Configuration Schema (Rust Types)

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DaemonConfig {
    pub daemon: DaemonSettings,
    pub defaults: DefaultSettings,
    pub profiles: HashMap<String, ProfileConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DaemonSettings {
    pub log_level: LogLevel,
    pub log_file: Option<PathBuf>,
    pub pid_file: PathBuf,
    pub socket_path: PathBuf,
    pub state_file: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DefaultSettings {
    pub retry: RetryConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetryConfig {
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
    pub max_attempts: u32,  // 0 = infinite
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileConfig {
    pub enabled: bool,
    pub kubeconfig: Option<PathBuf>,
    pub context: Option<String>,
    pub namespaces: Vec<String>,
    pub labels: Vec<String>,
    pub exclude_services: Vec<String>,
    pub retry: Option<RetryConfig>,  // Override defaults
}
```

#### 2.2.3 Config Loader

```rust
pub struct ConfigLoader {
    config_path: PathBuf,
    watcher: Option<notify::RecommendedWatcher>,
}

impl ConfigLoader {
    pub fn load(&self) -> Result<DaemonConfig>;
    pub fn validate(&self, config: &DaemonConfig) -> Result<()>;
    pub fn watch(&mut self, tx: mpsc::Sender<ConfigEvent>) -> Result<()>;
}
```

**Validation Rules:**
- All paths must be absolute or expandable (~)
- At least one profile must be defined
- Namespace list cannot be empty for enabled profiles
- Retry multiplier must be > 1.0
- Kubeconfig file must exist (if specified)

### 2.3 State Manager

Tracks runtime state and persists across restarts.

#### 2.3.1 State Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonState {
    pub started_at: DateTime<Utc>,
    pub profiles: HashMap<String, ProfileState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileState {
    pub status: ProfileStatus,
    pub pid: Option<u32>,
    pub started_at: Option<DateTime<Utc>>,
    pub last_healthy_at: Option<DateTime<Utc>>,
    pub reconnect_count: u64,
    pub consecutive_failures: u32,
    pub last_error: Option<String>,
    pub current_backoff: Duration,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ProfileStatus {
    Starting,
    Running,
    Reconnecting,
    Backoff,
    Stopped,
    Failed,
    Disabled,
}
```

#### 2.3.2 State Persistence

```rust
pub struct StateManager {
    state: Arc<RwLock<DaemonState>>,
    state_file: PathBuf,
    persist_interval: Duration,
}

impl StateManager {
    pub async fn load_or_init(&mut self) -> Result<()>;
    pub async fn persist(&self) -> Result<()>;
    pub fn update_profile(&self, name: &str, update: impl FnOnce(&mut ProfileState));
    pub fn get_snapshot(&self) -> DaemonState;
}
```

State is persisted:
- Periodically (every 30 seconds)
- On significant state changes
- On graceful shutdown

### 2.4 Profile Supervisor

Manages the lifecycle of all profile workers.

```rust
pub struct ProfileSupervisor {
    workers: HashMap<String, ProfileWorkerHandle>,
    state: Arc<StateManager>,
    event_tx: mpsc::Sender<DaemonEvent>,
}

impl ProfileSupervisor {
    pub async fn start_profile(&mut self, name: &str, config: ProfileConfig) -> Result<()>;
    pub async fn stop_profile(&mut self, name: &str) -> Result<()>;
    pub async fn restart_profile(&mut self, name: &str) -> Result<()>;
    pub async fn stop_all(&mut self) -> Result<()>;
    pub fn get_profile_status(&self, name: &str) -> Option<ProfileStatus>;
}
```

### 2.5 Profile Worker

Each profile worker manages a single kubefwd process.

#### 2.5.1 Worker Structure

```rust
pub struct ProfileWorker {
    name: String,
    config: ProfileConfig,
    retry_config: RetryConfig,
    state: Arc<StateManager>,
    shutdown_rx: broadcast::Receiver<()>,
}

impl ProfileWorker {
    pub async fn run(&mut self) -> Result<()>;
    async fn spawn_kubefwd(&self) -> Result<Child>;
    async fn monitor_process(&mut self, child: Child) -> ProcessExitReason;
    async fn wait_backoff(&mut self);
    fn calculate_next_backoff(&mut self);
    fn reset_backoff(&mut self);
}
```

#### 2.5.2 Worker State Machine

```
                    ┌─────────────────────────────────────────────────┐
                    │                                                 │
                    ▼                                                 │
              ┌──────────┐                                           │
     ┌───────▶│ Starting │                                           │
     │        └────┬─────┘                                           │
     │             │ spawn kubefwd                                   │
     │             ▼                                                 │
     │        ┌──────────┐  healthy for 30s   ┌──────────┐          │
     │        │ Running  │──────────────────▶│ Running  │          │
     │        │ (probing)│                    │ (stable) │          │
     │        └────┬─────┘                    └────┬─────┘          │
     │             │                               │                 │
     │             │ process exit                  │ process exit    │
     │             ▼                               ▼                 │
     │        ┌────────────────────────────────────────┐            │
     │        │              Reconnecting              │            │
     │        └────────────────┬───────────────────────┘            │
     │                         │                                     │
     │                         ▼                                     │
     │                    ┌─────────┐                                │
     │                    │ Backoff │────────────────────────────────┘
     │                    └────┬────┘   wait complete
     │                         │
     │                         │ max_attempts reached (if set)
     │                         ▼
     │                    ┌─────────┐
     │                    │ Failed  │
     │                    └────┬────┘
     │                         │
     │                         │ manual restart
     └─────────────────────────┘
```

#### 2.5.3 Kubefwd Process Management

```rust
impl ProfileWorker {
    async fn spawn_kubefwd(&self) -> Result<Child> {
        let mut cmd = Command::new("sudo");
        cmd.arg("-E")
           .arg("kubefwd")
           .arg("svc");

        // Add kubeconfig if specified
        if let Some(ref kubeconfig) = self.config.kubeconfig {
            cmd.env("KUBECONFIG", kubeconfig);
        }

        // Add context if specified
        if let Some(ref context) = self.config.context {
            cmd.args(["--context", context]);
        }

        // Add namespaces
        for ns in &self.config.namespaces {
            cmd.args(["-n", ns]);
        }

        // Add label selectors
        for label in &self.config.labels {
            cmd.args(["-l", label]);
        }

        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .spawn()
    }
}
```

#### 2.5.4 Exponential Backoff Algorithm

```rust
impl ProfileWorker {
    fn calculate_next_backoff(&mut self) {
        let state = self.state.get_profile(&self.name);
        let current = state.current_backoff;

        // Calculate next backoff with jitter
        let next = Duration::from_secs_f64(
            current.as_secs_f64() * self.retry_config.multiplier
        );

        // Apply jitter (±10%)
        let jitter = rand::thread_rng().gen_range(0.9..1.1);
        let next_with_jitter = Duration::from_secs_f64(next.as_secs_f64() * jitter);

        // Clamp to max
        let clamped = next_with_jitter.min(self.retry_config.max_delay);

        self.state.update_profile(&self.name, |s| {
            s.current_backoff = clamped;
            s.consecutive_failures += 1;
        });
    }

    fn reset_backoff(&mut self) {
        self.state.update_profile(&self.name, |s| {
            s.current_backoff = self.retry_config.initial_delay;
            s.consecutive_failures = 0;
        });
    }
}
```

### 2.6 IPC System

#### 2.6.1 Protocol

Communication between CLI and daemon uses a simple JSON-based protocol over Unix domain socket.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: u64,
    pub command: IpcCommand,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcCommand {
    Status,
    GetProfile { name: String },
    StartProfile { name: String },
    StopProfile { name: String },
    RestartProfile { name: String },
    EnableProfile { name: String },
    DisableProfile { name: String },
    ReloadConfig,
    Shutdown,
    GetLogs { lines: u32, profile: Option<String> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: u64,
    pub result: IpcResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcResult {
    Ok(serde_json::Value),
    Error { code: u32, message: String },
}
```

#### 2.6.2 IPC Server

```rust
pub struct IpcServer {
    socket_path: PathBuf,
    handler: Arc<dyn IpcHandler>,
}

impl IpcServer {
    pub async fn run(&self, mut shutdown: broadcast::Receiver<()>) -> Result<()> {
        let listener = UnixListener::bind(&self.socket_path)?;

        // Set permissions (user only)
        std::fs::set_permissions(&self.socket_path, Permissions::from_mode(0o600))?;

        loop {
            tokio::select! {
                accept_result = listener.accept() => {
                    let (stream, _) = accept_result?;
                    let handler = self.handler.clone();
                    tokio::spawn(async move {
                        handle_connection(stream, handler).await;
                    });
                }
                _ = shutdown.recv() => break,
            }
        }

        std::fs::remove_file(&self.socket_path)?;
        Ok(())
    }
}
```

### 2.7 CLI (kubefwd-ctl)

#### 2.7.1 Command Structure

```
kubefwd-ctl <COMMAND>

Commands:
  start              Start the daemon
  stop               Stop the daemon
  restart            Restart the daemon
  reload             Reload configuration
  status             Show daemon and profile status
  logs               View daemon logs
  profile <COMMAND>  Manage profiles
    start <name>     Start a specific profile
    stop <name>      Stop a specific profile
    restart <name>   Restart a specific profile
    enable <name>    Enable a profile
    disable <name>   Disable a profile

Options:
  -s, --socket <PATH>  Unix socket path [default: /run/kubefwd-daemon.sock]
  -j, --json           Output in JSON format
  -v, --verbose        Increase verbosity
  -h, --help           Print help
  -V, --version        Print version
```

#### 2.7.2 Status Output

```
$ kubefwd-ctl status

kubefwd-daemon v0.1.0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Daemon Status: Running (uptime: 2d 4h 12m)
Profiles: 3 active, 1 disabled

┌─────────────────┬──────────┬──────────┬────────────┬───────────┐
│ Profile         │ Status   │ PID      │ Uptime     │ Reconnects│
├─────────────────┼──────────┼──────────┼────────────┼───────────┤
│ dev-services    │ ● Running│ 12345    │ 1d 2h 30m  │ 3         │
│ staging-api     │ ● Running│ 12346    │ 4h 15m     │ 0         │
│ prod-readonly   │ ○ Stopped│ -        │ -          │ 2         │
│ legacy          │ ◌ Disabled│ -       │ -          │ -         │
└─────────────────┴──────────┴──────────┴────────────┴───────────┘

Last reconnection: dev-services at 2025-01-15 10:30:42 UTC
```

---

## 3. Data Flow

### 3.1 Startup Sequence

```
┌──────────────────────────────────────────────────────────────────┐
│                        Startup Sequence                          │
└──────────────────────────────────────────────────────────────────┘

1. Parse CLI arguments
2. Load configuration file
   └─▶ Validate configuration
   └─▶ Exit with error if invalid
3. Initialize logging
4. Check for existing daemon (PID file)
   └─▶ Exit if already running
5. Write PID file
6. Load persisted state (if exists)
7. Initialize StateManager
8. Initialize ProfileSupervisor
9. Start IPC server
10. For each enabled profile:
    └─▶ Spawn ProfileWorker task
11. Enter main event loop
```

### 3.2 Reconnection Flow

```
┌──────────────────────────────────────────────────────────────────┐
│                      Reconnection Flow                           │
└──────────────────────────────────────────────────────────────────┘

ProfileWorker detects kubefwd exit
        │
        ▼
┌───────────────────┐
│ Check exit reason │
└────────┬──────────┘
         │
    ┌────┴────────────────────────────────────┐
    │                                         │
    ▼                                         ▼
┌──────────────┐                    ┌──────────────────┐
│ Normal exit  │                    │ Error/Crash      │
│ (code 0)     │                    │ (code != 0)      │
└──────┬───────┘                    └────────┬─────────┘
       │                                     │
       ▼                                     ▼
┌──────────────────┐              ┌──────────────────┐
│ Wait for backoff │◀─────────────│ Increment        │
│                  │              │ failure count    │
└────────┬─────────┘              └────────┬─────────┘
         │                                 │
         │                    ┌────────────┴───────────┐
         │                    │                        │
         │                    ▼                        ▼
         │          ┌──────────────────┐    ┌──────────────────┐
         │          │ max_attempts     │    │ Under limit      │
         │          │ exceeded?        │    │                  │
         │          └────────┬─────────┘    └────────┬─────────┘
         │                   │                       │
         │                   ▼                       │
         │          ┌──────────────────┐             │
         │          │ Set Failed state │             │
         │          │ (manual restart  │             │
         │          │ required)        │             │
         │          └──────────────────┘             │
         │                                           │
         ▼                                           │
┌──────────────────┐                                 │
│ Log reconnection │◀────────────────────────────────┘
│ attempt          │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Spawn new        │
│ kubefwd process  │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Update state to  │
│ Running          │
└──────────────────┘
```

---

## 4. Nix Integration

### 4.1 Flake Structure

```
nix-kubefwd-daemon/
├── flake.nix
├── flake.lock
├── src/
│   └── ...
├── Cargo.toml
├── Cargo.lock
└── nix/
    ├── package.nix          # Package derivation
    ├── module.nix           # NixOS module
    └── home-module.nix      # home-manager module
```

### 4.2 Flake Definition

```nix
# flake.nix
{
  description = "kubefwd daemon for reliable Kubernetes port forwarding";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        packages = {
          default = pkgs.callPackage ./nix/package.nix { };
          kubefwd-daemon = self.packages.${system}.default;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            pkg-config
            openssl
          ];
        };
      }
    ) // {
      nixosModules.default = import ./nix/module.nix;
      homeManagerModules.default = import ./nix/home-module.nix;
    };
}
```

### 4.3 NixOS Module

```nix
# nix/module.nix
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.kubefwd-daemon;
  settingsFormat = pkgs.formats.yaml { };
in
{
  options.services.kubefwd-daemon = {
    enable = mkEnableOption "kubefwd daemon";

    package = mkOption {
      type = types.package;
      default = pkgs.kubefwd-daemon;
      description = "The kubefwd-daemon package to use.";
    };

    settings = mkOption {
      type = settingsFormat.type;
      default = { };
      description = "Configuration for kubefwd-daemon.";
    };

    profiles = mkOption {
      type = types.attrsOf (types.submodule {
        options = {
          enabled = mkOption {
            type = types.bool;
            default = true;
          };
          namespaces = mkOption {
            type = types.listOf types.str;
          };
          context = mkOption {
            type = types.nullOr types.str;
            default = null;
          };
          labels = mkOption {
            type = types.listOf types.str;
            default = [ ];
          };
        };
      });
      default = { };
    };
  };

  config = mkIf cfg.enable {
    systemd.services.kubefwd-daemon = {
      description = "kubefwd daemon";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/kubefwd-daemon";
        Restart = "on-failure";
        RestartSec = 5;
      };

      environment = {
        KUBEFWD_DAEMON_CONFIG = settingsFormat.generate "config.yaml"
          (cfg.settings // { profiles = cfg.profiles; });
      };
    };
  };
}
```

### 4.4 home-manager Module

```nix
# nix/home-module.nix
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.kubefwd-daemon;
  settingsFormat = pkgs.formats.yaml { };
  configFile = settingsFormat.generate "kubefwd-daemon.yaml" (
    cfg.settings // { profiles = cfg.profiles; }
  );
in
{
  options.services.kubefwd-daemon = {
    enable = mkEnableOption "kubefwd daemon";

    package = mkOption {
      type = types.package;
      default = pkgs.kubefwd-daemon;
    };

    settings = mkOption {
      type = settingsFormat.type;
      default = {
        daemon = {
          log_level = "info";
          socket_path = "${config.xdg.runtimeDir}/kubefwd-daemon.sock";
        };
      };
    };

    profiles = mkOption {
      type = types.attrsOf (types.submodule {
        options = {
          enabled = mkOption {
            type = types.bool;
            default = true;
          };
          namespaces = mkOption {
            type = types.listOf types.str;
          };
          context = mkOption {
            type = types.nullOr types.str;
            default = null;
          };
          kubeconfig = mkOption {
            type = types.nullOr types.path;
            default = null;
          };
          labels = mkOption {
            type = types.listOf types.str;
            default = [ ];
          };
        };
      });
      default = { };
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile."kubefwd-daemon/config.yaml".source = configFile;

    systemd.user.services.kubefwd-daemon = {
      Unit = {
        Description = "kubefwd daemon";
        After = [ "network.target" ];
      };

      Service = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/kubefwd-daemon --config %h/.config/kubefwd-daemon/config.yaml";
        Restart = "on-failure";
        RestartSec = 5;
      };

      Install = {
        WantedBy = [ "default.target" ];
      };
    };

    # macOS launchd support
    launchd.agents.kubefwd-daemon = mkIf pkgs.stdenv.isDarwin {
      enable = true;
      config = {
        ProgramArguments = [
          "${cfg.package}/bin/kubefwd-daemon"
          "--config"
          "${config.home.homeDirectory}/.config/kubefwd-daemon/config.yaml"
        ];
        RunAtLoad = true;
        KeepAlive = true;
        StandardOutPath = "${config.home.homeDirectory}/Library/Logs/kubefwd-daemon.log";
        StandardErrorPath = "${config.home.homeDirectory}/Library/Logs/kubefwd-daemon.error.log";
      };
    };
  };
}
```

---

## 5. Error Handling

### 5.1 Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum DaemonError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Profile error: {message}")]
    Profile { name: String, message: String },

    #[error("IPC error: {0}")]
    Ipc(String),

    #[error("kubefwd process error: {0}")]
    Kubefwd(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Read(std::io::Error),

    #[error("Failed to parse config: {0}")]
    Parse(#[from] serde_yaml::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Profile '{name}' error: {message}")]
    Profile { name: String, message: String },
}
```

### 5.2 Error Recovery Strategies

| Error Type | Recovery Strategy |
|------------|-------------------|
| kubefwd crash | Automatic restart with backoff |
| Config file error | Refuse to reload, keep running with old config |
| Network error | Retry with exponential backoff |
| Kubeconfig missing | Mark profile as Failed, log error |
| IPC connection error | Log and continue (don't crash daemon) |
| State file corrupt | Initialize fresh state, log warning |

---

## 6. Logging

### 6.1 Log Levels

| Level | Usage |
|-------|-------|
| ERROR | Unrecoverable errors, profile failures |
| WARN | Recoverable issues, reconnections |
| INFO | Profile start/stop, config reload |
| DEBUG | Detailed operation flow |
| TRACE | IPC messages, process output |

### 6.2 Log Format

```
2025-01-15T10:30:42.123Z INFO  [kubefwd_daemon::supervisor] Profile 'dev-services' started (pid: 12345)
2025-01-15T10:35:15.456Z WARN  [kubefwd_daemon::worker] Profile 'dev-services' kubefwd exited (code: 1), reconnecting in 2s
2025-01-15T10:35:17.789Z INFO  [kubefwd_daemon::worker] Profile 'dev-services' reconnected successfully
```

### 6.3 Structured Logging (JSON mode)

```json
{
  "timestamp": "2025-01-15T10:30:42.123Z",
  "level": "INFO",
  "target": "kubefwd_daemon::supervisor",
  "message": "Profile started",
  "profile": "dev-services",
  "pid": 12345
}
```

---

## 7. Testing Strategy

### 7.1 Unit Tests

| Component | Test Focus |
|-----------|------------|
| ConfigLoader | Parsing, validation, defaults |
| RetryConfig | Backoff calculation, jitter |
| StateManager | Serialization, updates, persistence |
| IPC Protocol | Message encoding/decoding |

### 7.2 Integration Tests

| Test | Description |
|------|-------------|
| Daemon lifecycle | Start, stop, restart |
| Profile management | Enable, disable, status |
| Reconnection | Simulate kubefwd crash, verify recovery |
| Config reload | Hot reload with validation |
| IPC communication | CLI to daemon commands |

### 7.3 Mock kubefwd

For testing, a mock kubefwd binary that can simulate various scenarios:

```rust
// tests/mock_kubefwd.rs
fn main() {
    let args: Vec<String> = env::args().collect();

    match env::var("MOCK_BEHAVIOR").as_deref() {
        Ok("crash_after_5s") => {
            thread::sleep(Duration::from_secs(5));
            std::process::exit(1);
        }
        Ok("run_forever") => {
            loop { thread::sleep(Duration::from_secs(60)); }
        }
        Ok("exit_immediately") => {
            std::process::exit(0);
        }
        _ => {
            // Default: run for 60s then exit cleanly
            thread::sleep(Duration::from_secs(60));
        }
    }
}
```

---

## 8. Security Considerations

### 8.1 Privilege Model

```
┌─────────────────────────────────────────────────────────────────┐
│                     Privilege Separation                         │
└─────────────────────────────────────────────────────────────────┘

   User privileges                Root privileges (via sudo)
   ────────────────               ────────────────────────────

   ┌─────────────────┐            ┌─────────────────┐
   │  kubefwd-ctl    │            │                 │
   │  (CLI)          │            │                 │
   └────────┬────────┘            │                 │
            │                     │                 │
            ▼                     │                 │
   ┌─────────────────┐            │                 │
   │  kubefwd-daemon │            │                 │
   │  (main process) │            │                 │
   │                 │────sudo───▶│  kubefwd       │
   │  - config       │            │  (child proc)  │
   │  - state        │            │                 │
   │  - IPC          │            │  - /etc/hosts  │
   │  - logging      │            │  - port-forward│
   └─────────────────┘            └─────────────────┘
```

### 8.2 Security Measures

1. **Unix Socket Permissions**: IPC socket restricted to user (0600)
2. **PID File**: Prevents multiple instances
3. **No Secrets in Logs**: Kubeconfig paths redacted in debug logs
4. **Minimal sudo Usage**: Only kubefwd child process runs with sudo
5. **Config File Permissions**: Warn if config is world-readable

---

## 9. Future Considerations (v2.0+)

Items explicitly deferred to future versions:

1. **Metrics Endpoint**: Prometheus-compatible /metrics endpoint
2. **Desktop Notifications**: D-Bus/macOS notifications on state changes
3. **Web UI**: Local web interface for status and control
4. **Profile Dependencies**: Start profiles in order based on dependencies
5. **Health Probes**: Active health checking via TCP connect

---

## 10. Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-01-15 | Initial | Initial architecture |
