# Nix, devenv, direnv & SOPS Integration

## Overview

This integration provides reproducible development, staging, and production environments using:

- **Nix**: Reproducible package management and builds
- **devenv**: Developer environment configuration
- **direnv**: Automatic environment activation
- **SOPS**: Secrets management with encryption

## Project Structure

```
project/
├── flake.nix              # Nix flake configuration
├── flake.lock             # Locked dependencies
├── devenv.nix             # Developer environment
├── devenv.yaml            # devenv configuration
├── .envrc                 # direnv configuration
├── .sops.yaml             # SOPS configuration
├── secrets/
│   ├── dev.yaml           # Development secrets (encrypted)
│   ├── staging.yaml       # Staging secrets (encrypted)
│   └── prod.yaml          # Production secrets (encrypted)
└── nix/
    ├── packages/          # Custom packages
    ├── modules/           # NixOS modules
    └── overlays/          # Nix overlays
```

## Initial Setup

### 1. flake.nix

```nix
{
  description = "Project development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    flake-utils.url = "github:numtide/flake-utils";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in
      {
        packages.default = pkgs.callPackage ./nix/packages/default.nix { };

        devShells.default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [ ./devenv.nix ];
        };
      }
    );
}
```

### 2. devenv.nix

```nix
{ pkgs, lib, config, inputs, ... }:

{
  # Environment name
  name = "my-project";

  # Environment variables
  env = {
    PROJECT_NAME = "my-project";
    # Non-secret environment variables
  };

  # Packages available in the environment
  packages = with pkgs; [
    # Build tools
    gnumake
    just

    # Languages
    go_1_22
    nodejs_20
    python312

    # Development tools
    git
    gh
    jq
    yq-go

    # Infrastructure
    podman
    podman-compose
    kubectl
    k9s

    # Secrets management
    sops
    age

    # Documentation
    python312Packages.sphinx
    python312Packages.sphinx-rtd-theme
  ];

  # Language-specific configurations
  languages = {
    go = {
      enable = true;
      package = pkgs.go_1_22;
    };

    javascript = {
      enable = true;
      package = pkgs.nodejs_20;
    };

    python = {
      enable = true;
      package = pkgs.python312;
      poetry.enable = true;
    };
  };

  # Services (databases, etc.)
  services = {
    postgres = {
      enable = true;
      package = pkgs.postgresql_16;
      initialDatabases = [{ name = "myproject_dev"; }];
      port = 5432;
    };

    redis = {
      enable = true;
      port = 6379;
    };
  };

  # Pre-commit hooks
  pre-commit.hooks = {
    nixpkgs-fmt.enable = true;
    gofmt.enable = true;
    govet.enable = true;
    eslint.enable = true;
    prettier.enable = true;
  };

  # Scripts available in the environment
  scripts = {
    dev.exec = ''
      echo "Starting development server..."
      # Add your dev server command
    '';

    test.exec = ''
      echo "Running tests..."
      go test ./...
    '';

    build.exec = ''
      echo "Building project..."
      go build -o bin/app ./cmd/app
    '';

    db-migrate.exec = ''
      echo "Running database migrations..."
      # Add migration command
    '';

    secrets-edit.exec = ''
      sops secrets/$1.yaml
    '';
  };

  # Shell hooks
  enterShell = ''
    echo "🚀 Entering $PROJECT_NAME development environment"
    echo ""
    echo "Available commands:"
    echo "  dev          - Start development server"
    echo "  test         - Run tests"
    echo "  build        - Build the project"
    echo "  db-migrate   - Run database migrations"
    echo "  secrets-edit - Edit secrets (dev/staging/prod)"
    echo ""

    # Load secrets if available
    if [ -f secrets/dev.yaml ]; then
      export $(sops -d secrets/dev.yaml | yq -o=shell '.' | xargs)
    fi
  '';

  # Process manager (for running multiple services)
  processes = {
    api.exec = "go run ./cmd/api";
    worker.exec = "go run ./cmd/worker";
  };
}
```

### 3. .envrc

```bash
# .envrc - direnv configuration

# Use devenv
if ! has nix_direnv_version || ! nix_direnv_version 3.0.4; then
  source_url "https://raw.githubusercontent.com/nix-community/nix-direnv/3.0.4/direnvrc" \
    "sha256-DzlYZ33mWF/Gs8DDeyjr8mnVmQGx7ASYqA5WlxwrBG4="
fi

use flake . --impure

# Load environment-specific overrides
if [ -f .envrc.local ]; then
  source .envrc.local
fi

# Set environment
export APP_ENV="${APP_ENV:-development}"

# Layout for local binaries
layout go
PATH_add bin
```

### 4. .sops.yaml

```yaml
# .sops.yaml - SOPS configuration

creation_rules:
  # Development secrets - encrypted with age key
  - path_regex: secrets/dev\.yaml$
    age:
      - age1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  # Dev key

  # Staging secrets - encrypted with age key
  - path_regex: secrets/staging\.yaml$
    age:
      - age1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy  # Staging key

  # Production secrets - encrypted with age + AWS KMS
  - path_regex: secrets/prod\.yaml$
    kms:
      - arn:aws:kms:us-east-1:123456789:key/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    age:
      - age1zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz  # Prod key
```

### 5. secrets/dev.yaml (example - encrypted)

```yaml
# Before encryption (for reference)
database:
  host: localhost
  port: 5432
  user: myapp
  password: dev-password-123
  name: myapp_dev

redis:
  host: localhost
  port: 6379
  password: ""

api:
  jwt_secret: dev-jwt-secret-key
  encryption_key: dev-encryption-key-32chars

external:
  stripe_key: sk_test_xxxxxxxxxxxxx
  sendgrid_key: SG.xxxxxxxxxxxxx
```

## Environment-Specific Configuration

### Development Environment

```nix
# devenv.nix additions for development
{
  env = {
    APP_ENV = "development";
    LOG_LEVEL = "debug";
    DEBUG = "true";
  };

  services.postgres.enable = true;
  services.redis.enable = true;
}
```

### Staging Environment

```nix
# nix/environments/staging.nix
{ pkgs, ... }:

{
  env = {
    APP_ENV = "staging";
    LOG_LEVEL = "info";
    DEBUG = "false";
  };

  # Connect to external services
  env.DATABASE_URL = "postgres://staging-db.example.com:5432/myapp";
  env.REDIS_URL = "redis://staging-redis.example.com:6379";
}
```

### Production Environment

```nix
# nix/environments/production.nix
{ pkgs, ... }:

{
  env = {
    APP_ENV = "production";
    LOG_LEVEL = "warn";
    DEBUG = "false";
  };

  # Production uses secrets from SOPS/Vault
  # Connection strings injected at runtime
}
```

## SOPS Workflow

### Initial Setup

```bash
# Generate age key for development
age-keygen -o ~/.config/sops/age/keys.txt

# Get the public key
age-keygen -y ~/.config/sops/age/keys.txt
# Output: age1xxxxxxxxxx... (add this to .sops.yaml)

# Create initial secrets file
cat > secrets/dev.yaml.decrypted << 'EOF'
database:
  password: dev-password-123
api:
  jwt_secret: dev-jwt-secret
EOF

# Encrypt the file
sops -e secrets/dev.yaml.decrypted > secrets/dev.yaml
rm secrets/dev.yaml.decrypted
```

### Edit Secrets

```bash
# Edit development secrets
sops secrets/dev.yaml

# Edit staging secrets
sops secrets/staging.yaml

# Edit production secrets (requires appropriate key access)
sops secrets/prod.yaml
```

### Use Secrets in Code

```bash
# In shell scripts
export $(sops -d secrets/dev.yaml | yq -o=shell '.' | xargs)

# Access in code
echo $database_password
```

### Rotate Secrets

```bash
# Rotate encryption keys
sops rotate -i secrets/dev.yaml

# Update to new key
sops updatekeys secrets/dev.yaml
```

## Justfile for Common Tasks

```just
# justfile - Task runner

set shell := ["bash", "-c"]
set dotenv-load

default:
  @just --list

# Development
dev:
  devenv up

test:
  go test -v ./...

lint:
  golangci-lint run

build:
  go build -o bin/app ./cmd/app

# Database
db-up:
  devenv processes up postgres

db-migrate:
  goose -dir migrations postgres "$DATABASE_URL" up

db-rollback:
  goose -dir migrations postgres "$DATABASE_URL" down

# Secrets
secrets-dev:
  sops secrets/dev.yaml

secrets-staging:
  sops secrets/staging.yaml

secrets-prod:
  sops secrets/prod.yaml

# Docker/Podman
container-build:
  podman build -t myapp:latest .

container-run:
  podman run -p 8080:8080 myapp:latest

# Documentation
docs-build:
  cd docs && make html

docs-serve:
  cd docs && python -m http.server --directory _build/html

# CI simulation
ci:
  nix flake check
  just lint
  just test
  just build
```

## CI/CD Integration

### GitHub Actions with Nix

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: cachix/install-nix-action@v27
        with:
          github_access_token: ${{ secrets.GITHUB_TOKEN }}

      - uses: cachix/cachix-action@v15
        with:
          name: devenv
          authToken: ${{ secrets.CACHIX_AUTH_TOKEN }}

      - name: Check flake
        run: nix flake check

      - name: Build
        run: nix build

      - name: Test
        run: nix develop --command just test
```

## Multi-Environment Deployment

```
┌─────────────────────────────────────────────────────────────────┐
│                     Development (Local)                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   devenv    │→ │   direnv    │→ │    SOPS     │             │
│  │  services   │  │  auto-load  │  │  dev.yaml   │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Staging                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │  Nix build  │→ │   Podman    │→ │    SOPS     │             │
│  │   artifact  │  │  container  │  │ staging.yaml│             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Production                                │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │  Nix build  │→ │   Podman    │→ │    SOPS     │             │
│  │   artifact  │  │  container  │  │  + AWS KMS  │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
└─────────────────────────────────────────────────────────────────┘
```

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| `direnv: error .envrc is blocked` | Run `direnv allow` |
| `SOPS failed to decrypt` | Check age key in `~/.config/sops/age/keys.txt` |
| `nix build` slow | Set up Cachix binary cache |
| Services won't start | Check `devenv up` logs, ensure ports are free |

### Debug Commands

```bash
# Check Nix flake
nix flake check

# Show devenv info
devenv info

# Test SOPS decryption
sops -d secrets/dev.yaml

# Check direnv status
direnv status
```

## Security Best Practices

1. **Never commit unencrypted secrets**
2. **Use age keys for development, KMS for production**
3. **Rotate secrets regularly**
4. **Audit secret access with Git history**
5. **Use separate keys per environment**
6. **Store age private keys securely (not in repo)**
