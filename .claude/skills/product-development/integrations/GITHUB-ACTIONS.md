# GitHub Actions CI Integration

## Overview

GitHub Actions CI/CD templates integrated with the product development workflow. These templates support Nix/devenv builds, Podman containers, and the planning structure.

## Workflow Structure

```
.github/
├── workflows/
│   ├── ci.yml                 # Main CI pipeline
│   ├── release.yml            # Release automation
│   ├── deploy-staging.yml     # Staging deployment
│   ├── deploy-production.yml  # Production deployment
│   └── docs.yml               # Documentation build
├── actions/
│   └── setup-env/             # Custom action for environment setup
│       └── action.yml
└── CODEOWNERS
```

## Main CI Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  # ==========================================================================
  # Lint and Static Analysis
  # ==========================================================================
  lint:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: cachix/install-nix-action@v27
        with:
          github_access_token: ${{ secrets.GITHUB_TOKEN }}

      - uses: cachix/cachix-action@v15
        with:
          name: devenv

      - name: Check Nix flake
        run: nix flake check

      - name: Lint
        run: nix develop --command just lint

  # ==========================================================================
  # Unit Tests
  # ==========================================================================
  test:
    name: Test
    runs-on: ubuntu-latest
    needs: lint
    steps:
      - uses: actions/checkout@v4

      - uses: cachix/install-nix-action@v27
        with:
          github_access_token: ${{ secrets.GITHUB_TOKEN }}

      - uses: cachix/cachix-action@v15
        with:
          name: devenv

      - name: Run tests
        run: nix develop --command just test

      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          token: ${{ secrets.CODECOV_TOKEN }}
          files: coverage.out
          fail_ci_if_error: true

  # ==========================================================================
  # Build
  # ==========================================================================
  build:
    name: Build
    runs-on: ubuntu-latest
    needs: test
    outputs:
      version: ${{ steps.version.outputs.version }}
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # For version calculation

      - name: Calculate version
        id: version
        run: |
          if [[ "${{ github.ref }}" == "refs/heads/main" ]]; then
            VERSION=$(git describe --tags --always)
          else
            VERSION="${{ github.sha }}"
          fi
          echo "version=${VERSION}" >> $GITHUB_OUTPUT

      - uses: cachix/install-nix-action@v27
        with:
          github_access_token: ${{ secrets.GITHUB_TOKEN }}

      - uses: cachix/cachix-action@v15
        with:
          name: devenv

      - name: Build
        run: nix develop --command just build

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: binary-${{ steps.version.outputs.version }}
          path: bin/
          retention-days: 7

  # ==========================================================================
  # Container Build
  # ==========================================================================
  container:
    name: Container
    runs-on: ubuntu-latest
    needs: build
    permissions:
      contents: read
      packages: write
    steps:
      - uses: actions/checkout@v4

      - name: Set up QEMU
        uses: docker/setup-qemu-action@v3

      - name: Set up Buildx
        uses: docker/setup-buildx-action@v3

      - name: Login to Container Registry
        if: github.event_name != 'pull_request'
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Extract metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
          tags: |
            type=ref,event=branch
            type=ref,event=pr
            type=semver,pattern={{version}}
            type=sha

      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          file: ./Containerfile
          platforms: linux/amd64,linux/arm64
          push: ${{ github.event_name != 'pull_request' }}
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          build-args: |
            VERSION=${{ needs.build.outputs.version }}
            COMMIT=${{ github.sha }}
          cache-from: type=gha
          cache-to: type=gha,mode=max

      - name: Scan image
        if: github.event_name != 'pull_request'
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${{ github.sha }}
          format: 'sarif'
          output: 'trivy-results.sarif'

      - name: Upload scan results
        if: github.event_name != 'pull_request'
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: 'trivy-results.sarif'

  # ==========================================================================
  # Integration Tests
  # ==========================================================================
  integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    needs: container
    if: github.event_name != 'pull_request'
    services:
      postgres:
        image: postgres:16-alpine
        env:
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
          POSTGRES_DB: test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7-alpine
        ports:
          - 6379:6379
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v4

      - uses: cachix/install-nix-action@v27
        with:
          github_access_token: ${{ secrets.GITHUB_TOKEN }}

      - name: Run integration tests
        env:
          DATABASE_URL: postgres://test:test@localhost:5432/test
          REDIS_URL: redis://localhost:6379
        run: nix develop --command just integration-test
```

## Release Workflow

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write
  packages: write

jobs:
  release:
    name: Release
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Generate changelog
        id: changelog
        uses: orhun/git-cliff-action@v3
        with:
          config: cliff.toml
          args: --latest --strip header

      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          body: ${{ steps.changelog.outputs.content }}
          draft: false
          prerelease: ${{ contains(github.ref, '-rc') || contains(github.ref, '-beta') }}

  build-binaries:
    name: Build Binaries
    runs-on: ${{ matrix.os }}
    needs: release
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            goos: linux
            goarch: amd64
          - os: ubuntu-latest
            goos: linux
            goarch: arm64
          - os: macos-latest
            goos: darwin
            goarch: amd64
          - os: macos-latest
            goos: darwin
            goarch: arm64
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-go@v5
        with:
          go-version: '1.22'

      - name: Build
        env:
          GOOS: ${{ matrix.goos }}
          GOARCH: ${{ matrix.goarch }}
        run: |
          VERSION=${GITHUB_REF#refs/tags/}
          go build -ldflags="-s -w -X main.Version=${VERSION}" \
            -o myapp-${{ matrix.goos }}-${{ matrix.goarch }} \
            ./cmd/app

      - name: Upload to release
        uses: softprops/action-gh-release@v2
        with:
          files: myapp-${{ matrix.goos }}-${{ matrix.goarch }}
```

## Deployment Workflows

### Staging Deployment

```yaml
# .github/workflows/deploy-staging.yml
name: Deploy Staging

on:
  push:
    branches: [main]
  workflow_dispatch:

concurrency:
  group: deploy-staging
  cancel-in-progress: false

jobs:
  deploy:
    name: Deploy to Staging
    runs-on: ubuntu-latest
    environment:
      name: staging
      url: https://staging.example.com
    steps:
      - uses: actions/checkout@v4

      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v4
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: us-east-1

      - name: Deploy
        run: |
          # Your deployment command here
          echo "Deploying to staging..."

      - name: Run smoke tests
        run: |
          curl -f https://staging.example.com/health || exit 1

      - name: Notify on failure
        if: failure()
        uses: slackapi/slack-github-action@v1
        with:
          payload: |
            {
              "text": "❌ Staging deployment failed: ${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}"
            }
        env:
          SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK_URL }}
```

### Production Deployment

```yaml
# .github/workflows/deploy-production.yml
name: Deploy Production

on:
  release:
    types: [published]
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to deploy'
        required: true
        type: string

concurrency:
  group: deploy-production
  cancel-in-progress: false

jobs:
  deploy:
    name: Deploy to Production
    runs-on: ubuntu-latest
    environment:
      name: production
      url: https://example.com
    steps:
      - uses: actions/checkout@v4

      - name: Get version
        id: version
        run: |
          if [ "${{ github.event_name }}" == "release" ]; then
            echo "version=${{ github.event.release.tag_name }}" >> $GITHUB_OUTPUT
          else
            echo "version=${{ inputs.version }}" >> $GITHUB_OUTPUT
          fi

      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v4
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: us-east-1

      - name: Deploy
        run: |
          echo "Deploying version ${{ steps.version.outputs.version }} to production..."
          # Your deployment command here

      - name: Run smoke tests
        run: |
          curl -f https://example.com/health || exit 1

      - name: Notify success
        uses: slackapi/slack-github-action@v1
        with:
          payload: |
            {
              "text": "✅ Production deployment successful: ${{ steps.version.outputs.version }}"
            }
        env:
          SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK_URL }}

      - name: Notify on failure
        if: failure()
        uses: slackapi/slack-github-action@v1
        with:
          payload: |
            {
              "text": "❌ Production deployment failed: ${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}"
            }
        env:
          SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK_URL }}
```

## Documentation Workflow

```yaml
# .github/workflows/docs.yml
name: Documentation

on:
  push:
    branches: [main]
    paths:
      - 'docs/**'
      - '.github/workflows/docs.yml'
  pull_request:
    paths:
      - 'docs/**'

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    name: Build Documentation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: cachix/install-nix-action@v27
        with:
          github_access_token: ${{ secrets.GITHUB_TOKEN }}

      - name: Build docs
        run: nix develop --command just docs-build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: docs/_build/html

  deploy:
    name: Deploy Documentation
    runs-on: ubuntu-latest
    needs: build
    if: github.ref == 'refs/heads/main'
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

## Custom Action for Environment Setup

```yaml
# .github/actions/setup-env/action.yml
name: 'Setup Environment'
description: 'Set up Nix and devenv environment'

inputs:
  cachix-name:
    description: 'Cachix cache name'
    required: false
    default: 'devenv'
  cachix-auth-token:
    description: 'Cachix auth token'
    required: false

runs:
  using: 'composite'
  steps:
    - uses: cachix/install-nix-action@v27
      with:
        github_access_token: ${{ github.token }}

    - uses: cachix/cachix-action@v15
      with:
        name: ${{ inputs.cachix-name }}
        authToken: ${{ inputs.cachix-auth-token }}

    - name: Verify environment
      shell: bash
      run: |
        nix develop --command echo "Environment ready"
```

## Reusable Workflows

### Reusable Test Workflow

```yaml
# .github/workflows/reusable-test.yml
name: Reusable Test

on:
  workflow_call:
    inputs:
      go-version:
        type: string
        default: '1.22'
    secrets:
      CODECOV_TOKEN:
        required: false

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-go@v5
        with:
          go-version: ${{ inputs.go-version }}

      - name: Test
        run: go test -race -coverprofile=coverage.out ./...

      - uses: codecov/codecov-action@v4
        if: ${{ secrets.CODECOV_TOKEN != '' }}
        with:
          token: ${{ secrets.CODECOV_TOKEN }}
```

## Branch Protection Rules

Configure in repository settings:

```yaml
# Recommended branch protection for main
main:
  required_status_checks:
    strict: true
    contexts:
      - "Lint"
      - "Test"
      - "Build"
      - "Container"
  required_pull_request_reviews:
    required_approving_review_count: 1
    dismiss_stale_reviews: true
  require_signed_commits: true
  enforce_admins: false
```

## Secrets Management

Required secrets:

| Secret | Purpose | Required |
|--------|---------|----------|
| `GITHUB_TOKEN` | Auto-provided | ✅ |
| `CACHIX_AUTH_TOKEN` | Nix binary cache | Optional |
| `CODECOV_TOKEN` | Coverage reports | Optional |
| `AWS_ACCESS_KEY_ID` | AWS deployments | For deploy |
| `AWS_SECRET_ACCESS_KEY` | AWS deployments | For deploy |
| `SLACK_WEBHOOK_URL` | Notifications | Optional |

## Integration with Planning Structure

When a story is ready for implementation:

1. Create feature branch from `develop`
2. Link branch to story in planning:
   ```markdown
   ## Implementation
   - Branch: `feature/S001-user-registration`
   - PR: [#123](https://github.com/org/repo/pull/123)
   - CI Status: [![CI](badge-url)](action-url)
   ```
3. CI runs automatically on push
4. Merge to `develop` triggers staging deploy
5. Merge to `main` (via release) triggers production deploy
