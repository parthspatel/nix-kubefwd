# Podman Deployment Integration

## Overview

Podman is a daemonless container engine for developing, managing, and running OCI containers. This integration covers container builds, local development, and production deployments.

## Why Podman over Docker

| Feature | Podman | Docker |
|---------|--------|--------|
| Daemonless | ✅ No daemon required | ❌ Requires dockerd |
| Rootless | ✅ Native support | ⚠️ Experimental |
| Systemd | ✅ Native integration | ❌ Separate service |
| Docker Compatible | ✅ CLI compatible | - |
| Pods | ✅ Native pod support | ❌ Requires Compose |
| Security | ✅ Better isolation | ⚠️ Daemon is root |

## Project Structure

```
project/
├── Containerfile              # Main container definition
├── Containerfile.dev          # Development container
├── .containerignore           # Files to exclude
├── compose.yaml               # Podman Compose configuration
├── containers/
│   ├── app/
│   │   └── Containerfile
│   ├── worker/
│   │   └── Containerfile
│   └── migrations/
│       └── Containerfile
└── deploy/
    ├── quadlet/               # Systemd quadlet files
    │   ├── app.container
    │   ├── app.network
    │   └── app.volume
    └── kubernetes/            # K8s manifests (if needed)
        └── ...
```

## Containerfile Templates

### Multi-Stage Build (Go)

```dockerfile
# Containerfile - Multi-stage build for Go application

# =============================================================================
# Stage 1: Build
# =============================================================================
FROM docker.io/library/golang:1.22-alpine AS builder

# Install build dependencies
RUN apk add --no-cache git ca-certificates tzdata

# Set working directory
WORKDIR /build

# Copy go mod files first (better caching)
COPY go.mod go.sum ./
RUN go mod download

# Copy source code
COPY . .

# Build the application
ARG VERSION=dev
ARG COMMIT=unknown
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build \
    -ldflags="-w -s -X main.Version=${VERSION} -X main.Commit=${COMMIT}" \
    -o /build/app ./cmd/app

# =============================================================================
# Stage 2: Runtime
# =============================================================================
FROM docker.io/library/alpine:3.19 AS runtime

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -g 1000 app && \
    adduser -u 1000 -G app -s /bin/sh -D app

# Copy binary from builder
COPY --from=builder /build/app /usr/local/bin/app

# Copy migrations if needed
COPY --from=builder /build/migrations /migrations

# Set user
USER app

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Run the application
ENTRYPOINT ["/usr/local/bin/app"]
CMD ["serve"]
```

### Multi-Stage Build (Node.js)

```dockerfile
# Containerfile - Multi-stage build for Node.js application

# =============================================================================
# Stage 1: Dependencies
# =============================================================================
FROM docker.io/library/node:20-alpine AS deps

WORKDIR /app

# Copy package files
COPY package.json package-lock.json ./

# Install dependencies
RUN npm ci --only=production

# =============================================================================
# Stage 2: Build
# =============================================================================
FROM docker.io/library/node:20-alpine AS builder

WORKDIR /app

# Copy package files and install all dependencies
COPY package.json package-lock.json ./
RUN npm ci

# Copy source and build
COPY . .
RUN npm run build

# =============================================================================
# Stage 3: Runtime
# =============================================================================
FROM docker.io/library/node:20-alpine AS runtime

WORKDIR /app

# Create non-root user
RUN addgroup -g 1000 app && \
    adduser -u 1000 -G app -s /bin/sh -D app

# Copy production dependencies
COPY --from=deps /app/node_modules ./node_modules

# Copy built application
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package.json ./

# Set user
USER app

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

CMD ["node", "dist/index.js"]
```

### Development Container

```dockerfile
# Containerfile.dev - Development container with hot reload

FROM docker.io/library/golang:1.22-alpine

# Install development tools
RUN apk add --no-cache git make curl

# Install air for hot reload
RUN go install github.com/air-verse/air@latest

WORKDIR /app

# Copy go mod files
COPY go.mod go.sum ./
RUN go mod download

# Don't copy source - mount as volume instead

EXPOSE 8080

CMD ["air", "-c", ".air.toml"]
```

## .containerignore

```
# .containerignore

# Version control
.git
.gitignore

# Build artifacts
bin/
dist/
build/
*.exe
*.dll
*.so
*.dylib

# Dependencies (will be installed in container)
node_modules/
vendor/

# IDE and editor
.idea/
.vscode/
*.swp
*.swo
*~

# Testing
coverage/
*.cover
*.out

# Environment and secrets
.env
.env.*
secrets/
*.pem
*.key

# Documentation
docs/
*.md
!README.md

# CI/CD
.github/
.gitlab-ci.yml
Jenkinsfile

# Container files for other stages
Containerfile.dev
compose.dev.yaml
```

## Compose Configuration

### compose.yaml (Production-like)

```yaml
# compose.yaml - Production-like configuration

version: "3"

services:
  app:
    build:
      context: .
      dockerfile: Containerfile
      args:
        VERSION: ${VERSION:-dev}
        COMMIT: ${COMMIT:-unknown}
    image: myapp:${VERSION:-latest}
    ports:
      - "8080:8080"
    environment:
      - APP_ENV=production
      - DATABASE_URL=postgres://postgres:5432/myapp
      - REDIS_URL=redis://redis:6379
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "wget", "--spider", "-q", "http://localhost:8080/health"]
      interval: 30s
      timeout: 5s
      retries: 3
      start_period: 10s
    restart: unless-stopped
    networks:
      - backend

  worker:
    build:
      context: .
      dockerfile: containers/worker/Containerfile
    image: myapp-worker:${VERSION:-latest}
    environment:
      - APP_ENV=production
      - DATABASE_URL=postgres://postgres:5432/myapp
      - REDIS_URL=redis://redis:6379
    depends_on:
      - postgres
      - redis
    restart: unless-stopped
    networks:
      - backend

  postgres:
    image: docker.io/library/postgres:16-alpine
    environment:
      POSTGRES_USER: myapp
      POSTGRES_PASSWORD_FILE: /run/secrets/db_password
      POSTGRES_DB: myapp
    volumes:
      - postgres_data:/var/lib/postgresql/data
    secrets:
      - db_password
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U myapp"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - backend

  redis:
    image: docker.io/library/redis:7-alpine
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - backend

volumes:
  postgres_data:
  redis_data:

networks:
  backend:
    driver: bridge

secrets:
  db_password:
    file: ./secrets/db_password.txt
```

### compose.dev.yaml (Development)

```yaml
# compose.dev.yaml - Development with hot reload

version: "3"

services:
  app:
    build:
      context: .
      dockerfile: Containerfile.dev
    volumes:
      - .:/app
      - go_cache:/go/pkg/mod
    ports:
      - "8080:8080"
    environment:
      - APP_ENV=development
      - DATABASE_URL=postgres://myapp:password@postgres:5432/myapp_dev
      - REDIS_URL=redis://redis:6379
    depends_on:
      - postgres
      - redis
    networks:
      - backend

  postgres:
    image: docker.io/library/postgres:16-alpine
    environment:
      POSTGRES_USER: myapp
      POSTGRES_PASSWORD: password
      POSTGRES_DB: myapp_dev
    ports:
      - "5432:5432"
    volumes:
      - postgres_dev_data:/var/lib/postgresql/data
    networks:
      - backend

  redis:
    image: docker.io/library/redis:7-alpine
    ports:
      - "6379:6379"
    networks:
      - backend

volumes:
  postgres_dev_data:
  go_cache:

networks:
  backend:
```

## Podman Commands

### Build

```bash
# Build image
podman build -t myapp:latest .

# Build with build args
podman build \
  --build-arg VERSION=$(git describe --tags) \
  --build-arg COMMIT=$(git rev-parse HEAD) \
  -t myapp:$(git describe --tags) .

# Build multi-platform
podman build --platform linux/amd64,linux/arm64 -t myapp:latest .
```

### Run

```bash
# Run container
podman run -d --name myapp -p 8080:8080 myapp:latest

# Run with environment file
podman run -d --name myapp --env-file .env -p 8080:8080 myapp:latest

# Run with secrets
podman run -d --name myapp \
  --secret db_password,type=env,target=DATABASE_PASSWORD \
  -p 8080:8080 myapp:latest

# Run with volume
podman run -d --name myapp \
  -v myapp_data:/data \
  -p 8080:8080 myapp:latest
```

### Compose

```bash
# Start services
podman-compose up -d

# Start with dev config
podman-compose -f compose.yaml -f compose.dev.yaml up -d

# View logs
podman-compose logs -f app

# Stop services
podman-compose down

# Stop and remove volumes
podman-compose down -v
```

## Systemd Quadlet Integration

Quadlet allows running Podman containers as systemd services.

### app.container

```ini
# deploy/quadlet/app.container

[Unit]
Description=My Application
After=network-online.target
Wants=network-online.target

[Container]
Image=myapp:latest
PublishPort=8080:8080
Environment=APP_ENV=production
Secret=db_password,type=env,target=DATABASE_PASSWORD
Volume=app_data:/data:Z
Network=app.network
HealthCmd=wget --spider -q http://localhost:8080/health
HealthInterval=30s
HealthRetries=3
HealthStartPeriod=10s

[Service]
Restart=always
TimeoutStartSec=300

[Install]
WantedBy=multi-user.target default.target
```

### app.network

```ini
# deploy/quadlet/app.network

[Network]
Subnet=10.89.0.0/24
Gateway=10.89.0.1
```

### app.volume

```ini
# deploy/quadlet/app.volume

[Volume]
# Options for the volume
```

### Deploy with Quadlet

```bash
# Copy quadlet files to systemd directory
cp deploy/quadlet/*.container ~/.config/containers/systemd/
cp deploy/quadlet/*.network ~/.config/containers/systemd/
cp deploy/quadlet/*.volume ~/.config/containers/systemd/

# Reload systemd
systemctl --user daemon-reload

# Start the service
systemctl --user start app

# Enable on boot
systemctl --user enable app

# Check status
systemctl --user status app

# View logs
journalctl --user -u app -f
```

## CI/CD Integration

### Build and Push

```bash
#!/bin/bash
# scripts/build-and-push.sh

set -euo pipefail

REGISTRY="${REGISTRY:-ghcr.io}"
IMAGE_NAME="${IMAGE_NAME:-myorg/myapp}"
VERSION="${VERSION:-$(git describe --tags --always)}"

# Build
podman build \
  --build-arg VERSION="${VERSION}" \
  --build-arg COMMIT="$(git rev-parse HEAD)" \
  -t "${REGISTRY}/${IMAGE_NAME}:${VERSION}" \
  -t "${REGISTRY}/${IMAGE_NAME}:latest" \
  .

# Login to registry
echo "${REGISTRY_PASSWORD}" | podman login -u "${REGISTRY_USERNAME}" --password-stdin "${REGISTRY}"

# Push
podman push "${REGISTRY}/${IMAGE_NAME}:${VERSION}"
podman push "${REGISTRY}/${IMAGE_NAME}:latest"
```

### Deployment Script

```bash
#!/bin/bash
# scripts/deploy.sh

set -euo pipefail

IMAGE="${1:?Image required}"
ENV="${2:-production}"

# Pull latest image
podman pull "${IMAGE}"

# Stop existing container
podman stop myapp || true
podman rm myapp || true

# Run new container
podman run -d \
  --name myapp \
  --env-file "/etc/myapp/${ENV}.env" \
  --secret db_password \
  -p 8080:8080 \
  --restart unless-stopped \
  "${IMAGE}"

# Wait for health check
for i in {1..30}; do
  if podman healthcheck run myapp; then
    echo "Container is healthy"
    exit 0
  fi
  echo "Waiting for container to be healthy... ($i/30)"
  sleep 2
done

echo "Container failed health check"
exit 1
```

## Security Best Practices

1. **Run as non-root user** - Always specify `USER` in Containerfile
2. **Use multi-stage builds** - Minimize final image size and attack surface
3. **Scan images** - Use `podman image scan` or integrate with Trivy
4. **Sign images** - Use `podman image sign` for supply chain security
5. **Use secrets** - Never embed secrets in images
6. **Read-only filesystem** - Use `--read-only` when possible
7. **Drop capabilities** - Use `--cap-drop=ALL` and add only needed caps
8. **Network isolation** - Use custom networks, not host networking

### Security Scan

```bash
# Install Trivy
# Then scan image
trivy image myapp:latest

# Scan and fail on HIGH/CRITICAL
trivy image --exit-code 1 --severity HIGH,CRITICAL myapp:latest
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Permission denied | Check SELinux (`:Z` volume suffix), user mapping |
| Port already in use | `podman ps -a`, stop conflicting container |
| Image not found | Check registry login, image name/tag |
| Out of disk space | `podman system prune -a` |
| Slow builds | Use BuildKit cache, optimize layer order |

### Debug Commands

```bash
# List containers
podman ps -a

# View container logs
podman logs -f myapp

# Execute in container
podman exec -it myapp /bin/sh

# Inspect container
podman inspect myapp

# View resource usage
podman stats

# Clean up
podman system prune -a --volumes
```
