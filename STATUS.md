# Status

Last updated: 2026-06-08

## Build
GitHub Actions billing-blocked org-wide. Workflows are configured but not running.

## Quality gates (enrolled, awaiting billing for live runs)
- cargo-deny.yml: Monday 09:00 UTC cron + push/PR + workflow_dispatch
- codeql-rust.yml: Tuesday 04:17 UTC cron + push/PR + workflow_dispatch
- cargo-audit.yml: Wednesday 05:37 UTC cron + push/PR + workflow_dispatch
- pre-commit: client-side (cargo fmt + check + gitleaks)
- branch protection: 1 reviewer required, no force-push, dismiss stale

## Current state
- Branch: `main` (default)
- Working tree: clean
- Stashes: 0
- Open PRs: 0
- Build tool: `justfile` (standardized 2026-06-08)

## Recent changes
- Merged PR #59 (surrealdb bump to 3.1.2)
- Skipped PR #60 (workflow hygiene — superseded by #61)
- Standardized on `justfile`; removed `Taskfile.yml`
