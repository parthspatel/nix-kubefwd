# Kubefwd Daemon Architecture Document

## 1. System Overview

### 1.1 High-Level Architecture (REST API Approach)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              User Space                                      │
│                                                                              │
│  ┌──────────────┐     ┌──────────────────────────────────────────────────┐  │
│  │              │     │              kubefwd-daemon (Rust)                │  │
│  │  kubefwd-ctl │────▶│  ┌────────────────────────────────────────────┐  │  │
│  │    (CLI)     │ IPC │  │              Daemon Core                    │  │  │
│  │              │◀────│  │  ┌──────────┐  ┌───────────┐  ┌──────────┐ │  │  │
│  └──────────────┘     │  │  │  Config  │  │   State   │  │   IPC    │ │  │  │
│                       │  │  │  Loader  │  │  Manager  │  │  Server  │ │  │  │
│                       │  │  └────┬─────┘  └─────┬─────┘  └──────────┘ │  │  │
│                       │  │       │              │                      │  │  │
│                       │  │       ▼              ▼                      │  │  │
│                       │  │  ┌─────────────────────────────────────┐   │  │  │
│                       │  │  │       Kubefwd Controller            │   │  │  │
│                       │  │  │  ┌─────────────┐  ┌──────────────┐  │   │  │  │
│                       │  │  │  │ REST Client │  │ SSE Listener │  │   │  │  │
│                       │  │  │  │ (reqwest)   │  │ (async)      │  │   │  │  │
│                       │  │  │  └──────┬──────┘  └───────┬──────┘  │   │  │  │
│                       │  │  └─────────┼─────────────────┼─────────┘   │  │  │
│                       │  └────────────┼─────────────────┼─────────────┘  │  │
│                       │               │                 │                │  │
│                       └───────────────┼─────────────────┼────────────────┘  │
│                                       │                 │                   │
│                                       ▼                 ▼                   │
│                          ┌─────────────────────────────────────────────┐   │
│                          │         kubefwd (single process)            │   │
│                          │         --api --idle mode                   │   │
│                          │                                             │   │
│                          │  REST API: http://kubefwd.internal/api/v1   │   │
│                          │  SSE Events: /api/v1/events                 │   │
│                          └─────────────────────────────────────────────┘   │
│                                              │                              │
└──────────────────────────────────────────────┼──────────────────────────────┘
                                               │
                                               ▼
                                     ┌───────────────────┐
                                     │   Kubernetes API  │
                                     │   (port-forward)  │
                                     └───────────────────┘
```

### 1.2 Deployment Modes

The daemon supports **three deployment modes** to handle different use cases:

| Mode | Use Case | Isolation | Lifecycle |
|------|----------|-----------|-----------|
| **System Service** | Always-on forwarding | Shared across all projects | systemd/launchd managed |
| **User Service** | Per-user forwarding | Shared within user session | systemd user service |
| **Devenv Service** | Per-project forwarding | Isolated per project | devenv process-compose |

### 1.3 Component Summary

| Component | Responsibility |
|-----------|----------------|
| **kubefwd-ctl** | CLI tool for user interaction |
| **Daemon Core** | Process lifecycle and coordination |
| **Config Loader** | Parse, validate, watch configuration |
| **State Manager** | Track connection state and metrics |
| **Kubefwd Controller** | REST API client + SSE event listener |
| **IPC Server** | Unix socket for CLI communication |

---

## 2. Project Isolation Strategy

### 2.1 The Isolation Problem

When multiple projects use kubefwd simultaneously, conflicts can occur:

```
Project A (e-commerce)          Project B (analytics)
─────────────────────          ─────────────────────
namespace: prod                namespace: prod
service: api → 127.0.0.1:8080  service: api → 127.0.0.1:8080  ← CONFLICT!
service: db  → 127.0.0.1:5432  service: db  → 127.0.0.1:5432  ← CONFLICT!
```

### 2.2 Isolation Mechanisms

#### Option A: Per-Project Kubefwd Instance (Recommended for Devenv)

Each project runs its own kubefwd instance with isolated networking:

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Host Machine                                  │
│                                                                      │
│  ┌─────────────────────────┐    ┌─────────────────────────┐        │
│  │   Project A (devenv)    │    │   Project B (devenv)    │        │
│  │                         │    │                         │        │
│  │  kubefwd instance       │    │  kubefwd instance       │        │
│  │  API: localhost:9876    │    │  API: localhost:9877    │        │
│  │  IP range: 127.1.0.x    │    │  IP range: 127.2.0.x    │        │
│  │                         │    │                         │        │
│  │  api.prod → 127.1.0.1   │    │  api.prod → 127.2.0.1   │        │
│  │  db.prod  → 127.1.0.2   │    │  db.prod  → 127.2.0.2   │        │
│  └─────────────────────────┘    └─────────────────────────┘        │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

**Implementation:**
```yaml
# Project A: .envrc or devenv.nix
KUBEFWD_API_PORT: 9876
KUBEFWD_IP_PREFIX: "127.1"

# Project B: .envrc or devenv.nix
KUBEFWD_API_PORT: 9877
KUBEFWD_IP_PREFIX: "127.2"
```

#### Option B: Shared Daemon with Profile Namespacing

Single daemon manages all projects with logical isolation:

```yaml
# ~/.config/kubefwd-daemon/config.yaml
profiles:
  ecommerce-dev:
    project_id: "ecommerce"      # Used for /etc/hosts prefixing
    namespaces: [development]
    context: ecommerce-cluster

  analytics-dev:
    project_id: "analytics"
    namespaces: [development]
    context: analytics-cluster
```

**Hosts file isolation:**
```
# /etc/hosts (managed by kubefwd)
127.1.0.1  api.development.svc.cluster.local      # ecommerce
127.1.0.2  db.development.svc.cluster.local       # ecommerce
127.2.0.1  api.development.svc.cluster.local      # analytics (different IP!)
127.2.0.2  db.development.svc.cluster.local       # analytics
```

#### Option C: Context-Based Isolation (Simplest)

Different Kubernetes contexts naturally isolate:

```yaml
profiles:
  project-a:
    context: project-a-cluster   # Points to cluster A
    namespaces: [default]

  project-b:
    context: project-b-cluster   # Points to cluster B
    namespaces: [default]
```

### 2.3 Recommended Isolation by Use Case

| Scenario | Recommended Approach |
|----------|---------------------|
| Different K8s clusters | Context-based (Option C) |
| Same cluster, different namespaces | Profile namespacing (Option B) |
| Same cluster, same namespace names | Per-project instance (Option A) |
| CI/CD pipelines | Per-project instance (Option A) |

---

## 3. Nix Integration

### 3.1 Integration Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Nix Integration Layers                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐      │
│  │   NixOS Module   │  │  home-manager    │  │  devenv Module   │      │
│  │                  │  │     Module       │  │                  │      │
│  │  System service  │  │  User service    │  │  Project service │      │
│  │  Always running  │  │  Per-user        │  │  Per-shell       │      │
│  └────────┬─────────┘  └────────┬─────────┘  └────────┬─────────┘      │
│           │                     │                     │                 │
│           ▼                     ▼                     ▼                 │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                    kubefwd-daemon package                         │  │
│  │                    (Rust binary + kubefwd)                        │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Flake Structure

```
nix-kubefwd-daemon/
├── flake.nix
├── flake.lock
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── daemon.rs
│   ├── kubefwd/
│   │   ├── mod.rs
│   │   ├── client.rs        # REST API client
│   │   ├── events.rs        # SSE event handling
│   │   └── process.rs       # Process supervision
│   ├── state.rs
│   ├── ipc/
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   └── protocol.rs
│   └── cli/
│       ├── mod.rs
│       └── commands.rs
├── nix/
│   ├── package.nix          # Package derivation
│   ├── module.nix           # NixOS module
│   ├── home-module.nix      # home-manager module
│   └── devenv-module.nix    # devenv module
└── docs/
    ├── REQUIREMENTS.md
    └── ARCHITECTURE.md
```

### 3.3 Flake Definition

```nix
# flake.nix
{
  description = "kubefwd daemon for reliable Kubernetes port forwarding";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common build inputs
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = with pkgs; [
            openssl
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        # Build artifacts (dependencies only)
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # The actual package
        kubefwd-daemon = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        packages = {
          default = kubefwd-daemon;
          inherit kubefwd-daemon;
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            rust-analyzer
            cargo-watch
            kubefwd
          ];
        };

        # Expose the module for devenv
        lib.mkDevenvModule = import ./nix/devenv-module.nix;
      }
    ) // {
      # Flake-level outputs (not per-system)
      nixosModules.default = import ./nix/module.nix;
      homeManagerModules.default = import ./nix/home-module.nix;

      # Overlay for easy integration
      overlays.default = final: prev: {
        kubefwd-daemon = self.packages.${final.system}.default;
      };
    };
}
```

### 3.4 Devenv Module (Per-Project Isolation)

```nix
# nix/devenv-module.nix
{ pkgs, lib, config, ... }:

let
  cfg = config.services.kubefwd;

  # Generate a deterministic port based on project path
  projectHash = builtins.hashString "sha256" (toString config.devenv.root);
  defaultPort = 10000 + (lib.mod (lib.fromHexString (builtins.substring 0 4 projectHash)) 10000);

  # Generate IP prefix (127.X.0.0/16) based on project
  ipOctet = 1 + (lib.mod (lib.fromHexString (builtins.substring 4 2 projectHash)) 254);
  defaultIpPrefix = "127.${toString ipOctet}";

  configFile = pkgs.writeText "kubefwd-devenv.yaml" (builtins.toJSON {
    daemon = {
      log_level = cfg.logLevel;
      api_port = cfg.apiPort;
      ip_prefix = cfg.ipPrefix;
      socket_path = "${config.devenv.runtime}/kubefwd.sock";
      state_file = "${config.devenv.state}/kubefwd-state.json";
    };
    profiles.devenv = {
      enabled = true;
      namespaces = cfg.namespaces;
      context = cfg.context;
      kubeconfig = cfg.kubeconfig;
      labels = cfg.labels;
      services = cfg.services;
    };
  });
in
{
  options.services.kubefwd = {
    enable = lib.mkEnableOption "kubefwd port forwarding";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.kubefwd-daemon;
      description = "The kubefwd-daemon package to use";
    };

    namespaces = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ "default" ];
      description = "Kubernetes namespaces to forward";
      example = [ "development" "shared-services" ];
    };

    context = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "Kubernetes context to use (defaults to current)";
    };

    kubeconfig = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      description = "Path to kubeconfig file";
    };

    labels = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      description = "Label selectors to filter services";
      example = [ "app.kubernetes.io/part-of=myapp" ];
    };

    services = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      description = "Specific services to forward (empty = all)";
      example = [ "api-gateway" "postgres" "redis" ];
    };

    apiPort = lib.mkOption {
      type = lib.types.port;
      default = defaultPort;
      description = "Port for kubefwd REST API (auto-generated for isolation)";
    };

    ipPrefix = lib.mkOption {
      type = lib.types.str;
      default = defaultIpPrefix;
      description = "IP prefix for service addresses (e.g., 127.1 → 127.1.0.x)";
    };

    logLevel = lib.mkOption {
      type = lib.types.enum [ "trace" "debug" "info" "warn" "error" ];
      default = "info";
    };

    autoStart = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Automatically start kubefwd when entering devenv";
    };
  };

  config = lib.mkIf cfg.enable {
    # Add kubefwd-daemon and kubefwd to packages
    packages = [
      cfg.package
      pkgs.kubefwd
    ];

    # Process-compose service definition
    processes.kubefwd = {
      exec = "${cfg.package}/bin/kubefwd-daemon --config ${configFile}";
      process-compose = {
        readiness_probe = {
          http_get = {
            host = "127.0.0.1";
            port = cfg.apiPort;
            path = "/api/v1/status";
          };
          initial_delay_seconds = 2;
          period_seconds = 5;
        };
        shutdown = {
          signal = "SIGTERM";
          timeout_seconds = 10;
        };
      };
    };

    # Environment variables for the project
    env = {
      KUBEFWD_API_URL = "http://127.0.0.1:${toString cfg.apiPort}";
      KUBEFWD_IP_PREFIX = cfg.ipPrefix;
    };

    # Shell hook to show status
    enterShell = lib.mkIf cfg.autoStart ''
      echo "🔀 kubefwd: Forwarding services from namespaces: ${lib.concatStringsSep ", " cfg.namespaces}"
      echo "   API: http://127.0.0.1:${toString cfg.apiPort}"
      echo "   IP Range: ${cfg.ipPrefix}.0.x"
    '';
  };
}
```

### 3.5 Example Devenv Usage

```nix
# devenv.nix (in project root)
{ pkgs, lib, ... }:

{
  # Import the kubefwd module from the flake
  imports = [
    (builtins.getFlake "github:user/nix-kubefwd-daemon").lib.mkDevenvModule
  ];

  # Or if using flake inputs:
  # imports = [ inputs.kubefwd-daemon.lib.mkDevenvModule ];

  services.kubefwd = {
    enable = true;
    namespaces = [ "development" "shared-infra" ];
    context = "my-dev-cluster";
    labels = [ "app.kubernetes.io/part-of=ecommerce" ];

    # Optional: specific services only
    services = [
      "api-gateway"
      "user-service"
      "postgres"
      "redis"
    ];
  };

  # Other devenv config...
  languages.rust.enable = true;
}
```

### 3.6 NixOS Module (System Service)

```nix
# nix/module.nix
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.kubefwd-daemon;
  settingsFormat = pkgs.formats.yaml { };

  configFile = settingsFormat.generate "kubefwd-daemon.yaml" {
    daemon = {
      log_level = cfg.logLevel;
      socket_path = "/run/kubefwd-daemon/kubefwd.sock";
      state_file = "/var/lib/kubefwd-daemon/state.json";
      pid_file = "/run/kubefwd-daemon/kubefwd.pid";
    };
    defaults = cfg.defaults;
    profiles = cfg.profiles;
  };
in
{
  options.services.kubefwd-daemon = {
    enable = mkEnableOption "kubefwd daemon system service";

    package = mkOption {
      type = types.package;
      default = pkgs.kubefwd-daemon;
      description = "The kubefwd-daemon package";
    };

    logLevel = mkOption {
      type = types.enum [ "trace" "debug" "info" "warn" "error" ];
      default = "info";
    };

    defaults = mkOption {
      type = settingsFormat.type;
      default = {
        retry = {
          initial_delay = "1s";
          max_delay = "60s";
          multiplier = 2.0;
          max_attempts = 0;
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
          user = mkOption {
            type = types.nullOr types.str;
            default = null;
            description = "Run this profile as specific user (for kubeconfig access)";
          };
        };
      });
      default = { };
      example = {
        dev-services = {
          namespaces = [ "development" ];
          context = "dev-cluster";
        };
      };
    };
  };

  config = mkIf cfg.enable {
    # Ensure kubefwd is available
    environment.systemPackages = [ pkgs.kubefwd cfg.package ];

    systemd.services.kubefwd-daemon = {
      description = "kubefwd daemon for Kubernetes port forwarding";
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];

      serviceConfig = {
        Type = "notify";
        ExecStart = "${cfg.package}/bin/kubefwd-daemon --config ${configFile}";
        ExecReload = "${pkgs.coreutils}/bin/kill -HUP $MAINPID";
        Restart = "on-failure";
        RestartSec = 5;

        # Hardening
        NoNewPrivileges = false;  # Needs sudo for kubefwd
        ProtectSystem = "strict";
        ProtectHome = "read-only";
        ReadWritePaths = [
          "/var/lib/kubefwd-daemon"
          "/run/kubefwd-daemon"
          "/etc/hosts"  # kubefwd needs this
        ];
        RuntimeDirectory = "kubefwd-daemon";
        StateDirectory = "kubefwd-daemon";

        # Capabilities for /etc/hosts modification
        AmbientCapabilities = [ "CAP_NET_BIND_SERVICE" ];
      };
    };

    # Socket activation for CLI
    systemd.sockets.kubefwd-daemon = {
      wantedBy = [ "sockets.target" ];
      socketConfig = {
        ListenStream = "/run/kubefwd-daemon/kubefwd.sock";
        SocketMode = "0660";
        SocketUser = "root";
        SocketGroup = "wheel";
      };
    };
  };
}
```

### 3.7 home-manager Module (User Service)

```nix
# nix/home-module.nix
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.kubefwd-daemon;
  settingsFormat = pkgs.formats.yaml { };

  runtimeDir = config.xdg.runtimeDir or "/run/user/${toString config.home.uid}";
  stateDir = "${config.xdg.stateHome}/kubefwd-daemon";

  configFile = settingsFormat.generate "kubefwd-daemon.yaml" {
    daemon = {
      log_level = cfg.logLevel;
      socket_path = "${runtimeDir}/kubefwd-daemon.sock";
      state_file = "${stateDir}/state.json";
    };
    defaults = cfg.defaults;
    profiles = cfg.profiles;
  };
in
{
  options.services.kubefwd-daemon = {
    enable = mkEnableOption "kubefwd daemon user service";

    package = mkOption {
      type = types.package;
      default = pkgs.kubefwd-daemon;
    };

    logLevel = mkOption {
      type = types.enum [ "trace" "debug" "info" "warn" "error" ];
      default = "info";
    };

    defaults = mkOption {
      type = settingsFormat.type;
      default = {
        retry = {
          initial_delay = "1s";
          max_delay = "60s";
          multiplier = 2.0;
          max_attempts = 0;
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
    home.packages = [ cfg.package pkgs.kubefwd ];

    xdg.configFile."kubefwd-daemon/config.yaml".source = configFile;

    # Ensure state directory exists
    home.activation.kubefwdStateDir = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
      mkdir -p "${stateDir}"
    '';

    # Linux: systemd user service
    systemd.user.services.kubefwd-daemon = mkIf pkgs.stdenv.isLinux {
      Unit = {
        Description = "kubefwd daemon";
        After = [ "network-online.target" ];
        Wants = [ "network-online.target" ];
      };

      Service = {
        Type = "notify";
        ExecStart = "${cfg.package}/bin/kubefwd-daemon --config ${config.xdg.configHome}/kubefwd-daemon/config.yaml";
        ExecReload = "${pkgs.coreutils}/bin/kill -HUP $MAINPID";
        Restart = "on-failure";
        RestartSec = 5;
      };

      Install.WantedBy = [ "default.target" ];
    };

    # macOS: launchd agent
    launchd.agents.kubefwd-daemon = mkIf pkgs.stdenv.isDarwin {
      enable = true;
      config = {
        Label = "com.kubefwd-daemon";
        ProgramArguments = [
          "${cfg.package}/bin/kubefwd-daemon"
          "--config"
          "${config.home.homeDirectory}/.config/kubefwd-daemon/config.yaml"
        ];
        RunAtLoad = true;
        KeepAlive = {
          SuccessfulExit = false;
          Crashed = true;
        };
        StandardOutPath = "${config.home.homeDirectory}/Library/Logs/kubefwd-daemon.log";
        StandardErrorPath = "${config.home.homeDirectory}/Library/Logs/kubefwd-daemon.error.log";
        ProcessType = "Background";
      };
    };
  };
}
```

---

## 4. Kubefwd REST API Integration

### 4.1 API Client Design

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// kubefwd REST API client
pub struct KubefwdClient {
    base_url: String,
    client: Client,
}

impl KubefwdClient {
    pub fn new(api_port: u16) -> Self {
        Self {
            base_url: format!("http://127.0.0.1:{}/api/v1", api_port),
            client: Client::new(),
        }
    }

    /// Add a namespace to forward
    pub async fn add_namespace(&self, req: AddNamespaceRequest) -> Result<NamespaceResponse> {
        self.client
            .post(format!("{}/namespaces", self.base_url))
            .json(&req)
            .send()
            .await?
            .json()
            .await
    }

    /// Remove a namespace
    pub async fn remove_namespace(&self, namespace: &str) -> Result<()> {
        self.client
            .delete(format!("{}/namespaces/{}", self.base_url, namespace))
            .send()
            .await?;
        Ok(())
    }

    /// List all forwarded services
    pub async fn list_services(&self) -> Result<Vec<ServiceInfo>> {
        self.client
            .get(format!("{}/services", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    /// Get daemon status
    pub async fn status(&self) -> Result<StatusResponse> {
        self.client
            .get(format!("{}/status", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    /// Health check
    pub async fn health(&self) -> Result<bool> {
        let resp = self.client
            .get(format!("{}/health", self.base_url))
            .send()
            .await?;
        Ok(resp.status().is_success())
    }
}

#[derive(Debug, Serialize)]
pub struct AddNamespaceRequest {
    pub namespace: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub namespace: String,
    pub local_ip: String,
    pub local_port: u16,
    pub cluster_ip: String,
    pub cluster_port: u16,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub running: bool,
    pub uptime_seconds: u64,
    pub namespaces: Vec<String>,
    pub services_count: usize,
    pub reconnect_count: u64,
}
```

### 4.2 SSE Event Listener

```rust
use futures::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KubefwdEvent {
    ServiceUp {
        service: String,
        namespace: String,
        local_ip: String,
        local_port: u16,
    },
    ServiceDown {
        service: String,
        namespace: String,
        reason: String,
    },
    Reconnecting {
        service: String,
        namespace: String,
        attempt: u32,
    },
    Reconnected {
        service: String,
        namespace: String,
    },
    Error {
        message: String,
        service: Option<String>,
    },
    NamespaceAdded {
        namespace: String,
    },
    NamespaceRemoved {
        namespace: String,
    },
}

pub struct EventListener {
    api_port: u16,
    event_tx: mpsc::Sender<KubefwdEvent>,
}

impl EventListener {
    pub fn new(api_port: u16, event_tx: mpsc::Sender<KubefwdEvent>) -> Self {
        Self { api_port, event_tx }
    }

    pub async fn run(&self, mut shutdown: broadcast::Receiver<()>) -> Result<()> {
        let url = format!("http://127.0.0.1:{}/api/v1/events", self.api_port);

        loop {
            let mut es = EventSource::get(&url);

            loop {
                tokio::select! {
                    event = es.next() => {
                        match event {
                            Some(Ok(Event::Message(msg))) => {
                                if let Ok(event) = serde_json::from_str::<KubefwdEvent>(&msg.data) {
                                    let _ = self.event_tx.send(event).await;
                                }
                            }
                            Some(Err(e)) => {
                                tracing::warn!("SSE error: {}, reconnecting...", e);
                                break; // Reconnect
                            }
                            None => break, // Stream ended
                            _ => {}
                        }
                    }
                    _ = shutdown.recv() => {
                        return Ok(());
                    }
                }
            }

            // Wait before reconnecting
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
```

### 4.3 Process Supervisor

```rust
use std::process::{Child, Command, Stdio};
use tokio::sync::broadcast;

pub struct KubefwdSupervisor {
    api_port: u16,
    ip_prefix: String,
    process: Option<Child>,
    retry_config: RetryConfig,
}

impl KubefwdSupervisor {
    pub fn new(api_port: u16, ip_prefix: String, retry_config: RetryConfig) -> Self {
        Self {
            api_port,
            ip_prefix,
            process: None,
            retry_config,
        }
    }

    /// Start kubefwd in idle + API mode
    pub async fn start(&mut self) -> Result<()> {
        let child = Command::new("sudo")
            .args([
                "-E",
                "kubefwd",
                "--api",
                "--api-port", &self.api_port.to_string(),
                "--ip-prefix", &self.ip_prefix,
                // Idle mode: no namespaces specified, add via API
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        self.process = Some(child);

        // Wait for API to be ready
        self.wait_for_api().await?;

        Ok(())
    }

    /// Wait for kubefwd API to become available
    async fn wait_for_api(&self) -> Result<()> {
        let client = KubefwdClient::new(self.api_port);

        for attempt in 1..=30 {
            match client.health().await {
                Ok(true) => return Ok(()),
                _ => {
                    if attempt == 30 {
                        return Err(anyhow!("kubefwd API failed to start"));
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }

        Ok(())
    }

    /// Stop kubefwd gracefully
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(ref mut child) = self.process {
            // Send SIGTERM
            unsafe {
                libc::kill(child.id() as i32, libc::SIGTERM);
            }

            // Wait for graceful shutdown
            tokio::time::timeout(
                Duration::from_secs(10),
                tokio::task::spawn_blocking({
                    let mut child = self.process.take().unwrap();
                    move || child.wait()
                })
            ).await??;
        }

        Ok(())
    }

    /// Monitor process and restart on crash
    pub async fn supervise(&mut self, mut shutdown: broadcast::Receiver<()>) -> Result<()> {
        let mut backoff = self.retry_config.initial_delay;
        let mut consecutive_failures = 0;

        loop {
            // Start kubefwd if not running
            if self.process.is_none() {
                match self.start().await {
                    Ok(()) => {
                        tracing::info!("kubefwd started (API port: {})", self.api_port);
                        backoff = self.retry_config.initial_delay;
                        consecutive_failures = 0;
                    }
                    Err(e) => {
                        tracing::error!("Failed to start kubefwd: {}", e);
                        consecutive_failures += 1;
                    }
                }
            }

            // Wait for process exit or shutdown signal
            tokio::select! {
                exit_status = self.wait_for_exit() => {
                    tracing::warn!("kubefwd exited: {:?}", exit_status);
                    self.process = None;
                    consecutive_failures += 1;

                    // Check max attempts
                    if self.retry_config.max_attempts > 0
                        && consecutive_failures >= self.retry_config.max_attempts
                    {
                        return Err(anyhow!("Max restart attempts exceeded"));
                    }

                    // Wait with backoff
                    tracing::info!("Restarting kubefwd in {:?}", backoff);
                    tokio::time::sleep(backoff).await;

                    // Increase backoff
                    backoff = (backoff.mul_f64(self.retry_config.multiplier))
                        .min(self.retry_config.max_delay);
                }
                _ = shutdown.recv() => {
                    tracing::info!("Shutdown signal received, stopping kubefwd");
                    self.stop().await?;
                    return Ok(());
                }
            }
        }
    }

    async fn wait_for_exit(&mut self) -> Option<std::process::ExitStatus> {
        if let Some(ref mut child) = self.process {
            // Poll process status
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => return Some(status),
                    Ok(None) => {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                    Err(_) => return None,
                }
            }
        }
        None
    }
}
```

---

## 5. Configuration Schema

### 5.1 Full Configuration (YAML)

```yaml
# ~/.config/kubefwd-daemon/config.yaml

daemon:
  log_level: info                      # trace, debug, info, warn, error
  log_file: ~/.local/log/kubefwd-daemon.log
  socket_path: /run/user/1000/kubefwd-daemon.sock
  state_file: ~/.local/state/kubefwd-daemon/state.json

  # kubefwd process settings
  kubefwd:
    api_port: 9898                     # REST API port
    ip_prefix: "127.1"                 # IP range for services (127.1.0.x)

defaults:
  retry:
    initial_delay: 1s
    max_delay: 60s
    multiplier: 2.0
    max_attempts: 0                    # 0 = infinite

profiles:
  # Development environment
  dev:
    enabled: true
    context: dev-cluster
    kubeconfig: ~/.kube/config
    namespaces:
      - development
      - shared-services
    labels:
      - "environment=development"
    services: []                        # Empty = all services

  # Staging (disabled by default)
  staging:
    enabled: false
    context: staging-cluster
    namespaces:
      - staging
    labels:
      - "app.kubernetes.io/part-of=myapp"
```

### 5.2 Configuration Types (Rust)

```rust
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub daemon: DaemonConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
    #[serde(default)]
    pub profiles: HashMap<String, ProfileConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DaemonConfig {
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
    pub log_file: Option<PathBuf>,
    pub socket_path: PathBuf,
    pub state_file: PathBuf,
    #[serde(default)]
    pub kubefwd: KubefwdConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KubefwdConfig {
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    #[serde(default = "default_ip_prefix")]
    pub ip_prefix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DefaultsConfig {
    #[serde(default)]
    pub retry: RetryConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetryConfig {
    #[serde(with = "humantime_serde", default = "default_initial_delay")]
    pub initial_delay: Duration,
    #[serde(with = "humantime_serde", default = "default_max_delay")]
    pub max_delay: Duration,
    #[serde(default = "default_multiplier")]
    pub multiplier: f64,
    #[serde(default)]
    pub max_attempts: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub context: Option<String>,
    pub kubeconfig: Option<PathBuf>,
    pub namespaces: Vec<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub services: Vec<String>,
    pub retry: Option<RetryConfig>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

// Default value functions
fn default_log_level() -> LogLevel { LogLevel::Info }
fn default_api_port() -> u16 { 9898 }
fn default_ip_prefix() -> String { "127.1".to_string() }
fn default_initial_delay() -> Duration { Duration::from_secs(1) }
fn default_max_delay() -> Duration { Duration::from_secs(60) }
fn default_multiplier() -> f64 { 2.0 }
fn default_true() -> bool { true }
```

---

## 6. State Management

### 6.1 State Structure

```rust
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonState {
    pub version: u32,
    pub started_at: DateTime<Utc>,
    pub kubefwd_pid: Option<u32>,
    pub kubefwd_status: KubefwdStatus,
    pub profiles: HashMap<String, ProfileState>,
    pub services: HashMap<String, ServiceState>,
    pub metrics: Metrics,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum KubefwdStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileState {
    pub enabled: bool,
    pub namespaces_active: Vec<String>,
    pub last_sync: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceState {
    pub name: String,
    pub namespace: String,
    pub local_ip: String,
    pub local_port: u16,
    pub status: ServiceStatus,
    pub connected_at: Option<DateTime<Utc>>,
    pub reconnect_count: u64,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metrics {
    pub total_reconnects: u64,
    pub kubefwd_restarts: u64,
    pub uptime_seconds: u64,
    pub services_forwarded: u64,
}
```

---

## 7. CLI Design

### 7.1 Command Structure

```
kubefwd-ctl [OPTIONS] <COMMAND>

Commands:
  start              Start the daemon
  stop               Stop the daemon
  restart            Restart the daemon
  reload             Reload configuration (hot-reload)
  status             Show daemon and service status
  services           List forwarded services
  logs               View daemon logs

  profile <CMD>      Manage profiles
    list             List all profiles
    enable <name>    Enable a profile
    disable <name>   Disable a profile
    sync <name>      Force sync profile to kubefwd

  namespace <CMD>    Manage namespaces directly
    add <ns>         Add namespace to forwarding
    remove <ns>      Remove namespace from forwarding
    list             List active namespaces

Options:
  -c, --config <PATH>    Config file path
  -s, --socket <PATH>    Unix socket path
  -j, --json             Output as JSON
  -v, --verbose          Increase verbosity
  -h, --help             Print help
  -V, --version          Print version
```

### 7.2 Status Output

```
$ kubefwd-ctl status

kubefwd-daemon v0.1.0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Daemon:     ● Running (uptime: 2d 4h 12m)
kubefwd:    ● Running (pid: 12345, API: http://127.0.0.1:9898)
IP Range:   127.1.0.x

Profiles (2 active, 1 disabled):
┌─────────────┬──────────┬─────────────────────┬────────────┐
│ Profile     │ Status   │ Namespaces          │ Services   │
├─────────────┼──────────┼─────────────────────┼────────────┤
│ dev         │ ● Active │ development, shared │ 12         │
│ staging     │ ○ Disabled│ -                  │ -          │
│ prod-ro     │ ● Active │ production          │ 5          │
└─────────────┴──────────┴─────────────────────┴────────────┘

Services (17 total):
┌─────────────────────┬─────────────┬────────────────┬──────────┐
│ Service             │ Namespace   │ Local Address  │ Status   │
├─────────────────────┼─────────────┼────────────────┼──────────┤
│ api-gateway         │ development │ 127.1.0.1:8080 │ ● Up     │
│ user-service        │ development │ 127.1.0.2:8080 │ ● Up     │
│ postgres            │ shared      │ 127.1.0.3:5432 │ ● Up     │
│ redis               │ shared      │ 127.1.0.4:6379 │ ◐ Reconn │
│ ...                 │ ...         │ ...            │ ...      │
└─────────────────────┴─────────────┴────────────────┴──────────┘

Metrics:
  Total reconnects: 7
  kubefwd restarts: 1
  Last reconnect: redis @ 2025-01-15 10:30:42 UTC
```

---

## 8. Testing Strategy

### 8.1 Test Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Test Layers                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Integration Tests                        │ │
│  │  • Full daemon lifecycle                                    │ │
│  │  • CLI ↔ Daemon ↔ Mock kubefwd                            │ │
│  │  • Config reload scenarios                                  │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Component Tests                          │ │
│  │  • REST client against mock server                          │ │
│  │  • SSE event parsing                                        │ │
│  │  • State persistence                                        │ │
│  │  • IPC protocol                                             │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              │                                   │
│                              ▼                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                      Unit Tests                             │ │
│  │  • Config parsing & validation                              │ │
│  │  • Backoff calculation                                      │ │
│  │  • State transitions                                        │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 8.2 Mock kubefwd Server

```rust
// tests/mock_kubefwd.rs
use axum::{routing::{get, post, delete}, Router, Json};
use tokio::sync::broadcast;

/// Mock kubefwd REST API for testing
pub struct MockKubefwd {
    port: u16,
    namespaces: Arc<RwLock<HashSet<String>>>,
    services: Arc<RwLock<Vec<MockService>>>,
    event_tx: broadcast::Sender<String>,
}

impl MockKubefwd {
    pub fn new(port: u16) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            port,
            namespaces: Arc::new(RwLock::new(HashSet::new())),
            services: Arc::new(RwLock::new(Vec::new())),
            event_tx,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let app = Router::new()
            .route("/api/v1/status", get(Self::status))
            .route("/api/v1/health", get(Self::health))
            .route("/api/v1/namespaces", post(Self::add_namespace))
            .route("/api/v1/namespaces/:ns", delete(Self::remove_namespace))
            .route("/api/v1/services", get(Self::list_services))
            .route("/api/v1/events", get(Self::events))
            .with_state(self.clone());

        axum::Server::bind(&format!("127.0.0.1:{}", self.port).parse()?)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    /// Simulate service reconnection
    pub async fn simulate_reconnect(&self, service: &str) {
        let event = serde_json::json!({
            "type": "reconnecting",
            "service": service,
            "namespace": "default",
            "attempt": 1
        });
        let _ = self.event_tx.send(event.to_string());

        tokio::time::sleep(Duration::from_millis(100)).await;

        let event = serde_json::json!({
            "type": "reconnected",
            "service": service,
            "namespace": "default"
        });
        let _ = self.event_tx.send(event.to_string());
    }

    /// Simulate crash (for supervisor testing)
    pub async fn simulate_crash(&self) {
        // Server will stop, supervisor should restart
        std::process::exit(1);
    }
}
```

### 8.3 Integration Test Example

```rust
#[tokio::test]
async fn test_daemon_lifecycle() {
    // Start mock kubefwd
    let mock = MockKubefwd::new(19898);
    let mock_handle = tokio::spawn(mock.run());

    // Create test config
    let config = Config {
        daemon: DaemonConfig {
            kubefwd: KubefwdConfig {
                api_port: 19898,
                ip_prefix: "127.99".to_string(),
            },
            ..Default::default()
        },
        profiles: hashmap! {
            "test".to_string() => ProfileConfig {
                enabled: true,
                namespaces: vec!["default".to_string()],
                ..Default::default()
            }
        },
        ..Default::default()
    };

    // Start daemon
    let daemon = Daemon::new(config).await.unwrap();
    let daemon_handle = tokio::spawn(daemon.run());

    // Wait for startup
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Verify via CLI
    let status = kubefwd_ctl::status(&socket_path).await.unwrap();
    assert_eq!(status.kubefwd_status, KubefwdStatus::Running);
    assert!(status.profiles["test"].enabled);

    // Test reconnection event handling
    mock.simulate_reconnect("test-service").await;
    tokio::time::sleep(Duration::from_millis(500)).await;

    let status = kubefwd_ctl::status(&socket_path).await.unwrap();
    assert_eq!(status.metrics.total_reconnects, 1);

    // Shutdown
    daemon.shutdown().await.unwrap();
}
```

### 8.4 Nix Integration Test

```nix
# nix/tests/integration.nix
{ pkgs, ... }:

pkgs.nixosTest {
  name = "kubefwd-daemon-integration";

  nodes.machine = { config, pkgs, ... }: {
    imports = [ ../module.nix ];

    services.kubefwd-daemon = {
      enable = true;
      profiles.test = {
        namespaces = [ "default" ];
      };
    };

    # Mock kubernetes for testing
    services.k3s.enable = true;
  };

  testScript = ''
    machine.wait_for_unit("kubefwd-daemon.service")
    machine.wait_for_open_port(9898)

    # Check status
    result = machine.succeed("kubefwd-ctl status --json")
    status = json.loads(result)
    assert status["kubefwd_status"] == "Running"

    # Check service is forwarded
    machine.succeed("curl -f http://127.1.0.1:80 || true")
  '';
}
```

---

## 9. Security Considerations

### 9.1 Privilege Model

```
┌──────────────────────────────────────────────────────────────────────┐
│                         Privilege Separation                          │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│   User Space (unprivileged)           Root Space (privileged)        │
│   ─────────────────────────           ───────────────────────        │
│                                                                       │
│   ┌─────────────────────┐             ┌─────────────────────┐        │
│   │   kubefwd-ctl       │             │                     │        │
│   │   (CLI binary)      │             │                     │        │
│   └──────────┬──────────┘             │                     │        │
│              │ IPC (Unix socket)      │                     │        │
│              ▼                        │                     │        │
│   ┌─────────────────────┐             │                     │        │
│   │   kubefwd-daemon    │             │                     │        │
│   │                     │  sudo -E    │   kubefwd           │        │
│   │   • Config mgmt     │────────────▶│   (child process)   │        │
│   │   • State tracking  │             │                     │        │
│   │   • REST client     │◀───HTTP────▶│   • /etc/hosts mod  │        │
│   │   • SSE listener    │             │   • Port binding    │        │
│   │   • IPC server      │             │   • K8s API access  │        │
│   └─────────────────────┘             └─────────────────────┘        │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

### 9.2 Security Measures

| Measure | Implementation |
|---------|----------------|
| Socket permissions | `chmod 0600` on Unix socket |
| Config file permissions | Warn if world-readable |
| Credential handling | Never log kubeconfig contents |
| Process isolation | kubefwd runs as separate process |
| API binding | localhost only (127.0.0.1) |

---

## 10. Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-01-15 | Initial architecture (multi-process) |
| 2.0 | 2025-01-15 | Revised for REST API approach, added devenv integration, isolation strategies |
