---
name: ci-monitor
description: Monitor GitHub Actions CI workflows, diagnose failures, and fix issues. Use after pushing code to check if CI passes, or when CI fails to identify and resolve problems.
---

# CI Monitor - GitHub Actions Workflow Monitoring and Debugging

This skill helps you monitor CI workflow runs, diagnose failures, and fix issues.

## After Pushing Code

### Step 1: Check Latest CI Run Status

List recent workflow runs:
  gh run list --limit 5

Get the latest run ID:
  RUN_ID=$(gh run list --limit 1 --json databaseId --jq .[0].databaseId)

Watch the run in real-time (if still running):
  gh run watch $RUN_ID

Or check status:
  gh run view $RUN_ID

### Step 2: If CI Fails - Get Failed Job Details

Get failed job names:
  gh run view $RUN_ID --json jobs --jq .jobs[]

View failed logs:
  gh run view $RUN_ID --log-failed

### Step 3: Parse Error Messages

Look for common error patterns:
- error: - Nix build/check errors
- FAILED - Test failures  
- SC#### - ShellCheck warnings/errors
- would reformat - Formatting issues

## Common CI Failures and Fixes

### 1. Formatting Issues

Symptoms: Format Check job fails, logs mention changed files

Diagnosis - Run formatter locally in check mode:
  nix fmt -- --ci .

Fix - Apply formatting:
  nix fmt
  git add -A && git commit -m "style: apply formatting" && git push

### 2. ShellCheck Errors

Symptoms: Lint job fails, logs show SC#### codes

Common ShellCheck Issues:
- SC2034 - Unused variable: Remove or use the variable
- SC2086 - Unquoted variable: Add quotes
- SC2295 - Unquoted expansion in pattern
- SC2154 - Variable referenced but not assigned
- SC2155 - Declare and assign separately

Diagnosis - Run shellcheck locally:
  nix build .#checks.aarch64-darwin.shellcheck --print-build-logs

### 3. Nix Flake Check Failures

Diagnosis:
  nix flake check
  nix flake check --print-build-logs

### 4. Test Failures

Diagnosis:
  nix build .#checks.aarch64-darwin.unit-tests --print-build-logs

## Quick Commands Reference

List recent runs: gh run list --limit 10
Get latest failed: gh run list --status failure --limit 1
View run: gh run view RUN_ID
View failed logs: gh run view RUN_ID --log-failed
Re-run failed: gh run rerun RUN_ID --failed
Cancel: gh run cancel RUN_ID

## CI Verification Checklist

Before pushing run these locally:
1. Format check: nix fmt -- --ci .
2. Flake check: nix flake check
3. Build: nix build

## Debugging Tips

1. Always reproduce locally first
2. Check the exact error line in CI logs
3. Look at recent changes with git diff HEAD~1
4. Check if flaky by re-running the job
5. Use git bisect if needed
