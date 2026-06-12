<!-- AI-DD-META:START -->
<!-- This repository is planned, maintained, and managed by AI Agents only. -->
<!-- Slop issues are expected and intentionally present as part of an HITL-less -->
<!-- /minimized AI-DD metaproject of learning, refining, and building brute-force -->
<!-- training for both agents and the human operator. -->
![Downloads](https://img.shields.io/github/downloads/KooshaPari/phenoData/total?style=flat-square&label=downloads&color=blue)
![GitHub release](https://img.shields.io/github/v/release/KooshaPari/phenoData?style=flat-square&label=release)
![License](https://img.shields.io/github/license/KooshaPari/phenoData?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)
![HITL-less](https://img.shields.io/badge/HITL--less%20AI--DD-metaproject-yellow?style=flat-square)

> ⚠️ **AI-Agent-Only Repository**
>
> This repo is **planned, maintained, and managed exclusively by AI Agents**.
> Slop issues, rough edges, and AI artifacts are **expected and intentionally
> present** as part of an **HITL-less / minimized AI-DD** metaproject focused
> on learning, refining, and brute-force training both the agents and the
> human operator. Bug reports and contributions are still welcome, but please
> expect AI-generated code, comments, and documentation throughout.
<!-- AI-DD-META:END -->
## Work State

| Field | Value |
|---|---|
| Last commit | 2026-06-08 18:49:21 -0700 |
| Open issues | unknown (GitHub API rate-limited) |
| Open PRs | unknown (GitHub API rate-limited) |
| Focus | Data-layer crates and SurrealDB integration |

Progress: ████████░░ 80%

> **Work state:** ACTIVE · **Progress:** `██████░░░░ 55%`
> Rust data-layer crates (storage/surreal bridge); on main, audits landing · updated 2026-06-02

> **Pinned references (Phenotype-org)**
> - MSRV: see rust-toolchain.toml
> - cargo-deny config: see deny.toml
> - cargo-audit: rustsec/audit-check@v2 weekly
> - Branch protection: 1 reviewer required, no force-push
> - Authority: phenotype-org-governance/SUPERSEDED.md

# PhenoData — Data Layer Workspace

## State

Progress: `[██████░░░░] 55%` — Rust data-layer crates on main, audits landing.

_Updated 2026-06-08 — audit pass._

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/KooshaPari/phenoData/actions/workflows/ci.yml/badge.svg)](https://github.com/KooshaPari/phenoData/actions/workflows/ci.yml)
[![Coverage](https://github.com/KooshaPari/phenoData/actions/workflows/coverage.yml/badge.svg)](https://github.com/KooshaPari/phenoData/actions/workflows/coverage.yml)
[![Quality Gate](https://github.com/KooshaPari/phenoData/actions/workflows/quality-gate.yml/badge.svg)](https://github.com/KooshaPari/phenoData/actions/workflows/quality-gate.yml)
[![Gitleaks](https://github.com/KooshaPari/phenoData/actions/workflows/gitleaks.yml/badge.svg)](https://github.com/KooshaPari/phenoData/actions/workflows/gitleaks.yml)
[![CodeQL (Rust)](https://github.com/KooshaPari/phenoData/actions/workflows/codeql-rust.yml/badge.svg)](https://github.com/KooshaPari/phenoData/actions/workflows/codeql-rust.yml)
[![Docs](https://github.com/KooshaPari/phenoData/actions/workflows/pages.yml/badge.svg)](https://github.com/KooshaPari/phenoData/actions/workflows/pages.yml)
[![codecov](https://codecov.io/gh/KooshaPari/phenoData/branch/main/graph/badge.svg)](https://codecov.io/gh/KooshaPari/phenoData)
[![Rust](https://img.shields.io/badge/rust-1.84%2B-orange.svg)](https://www.rust-lang.org)

**Status:** maintenance

Data-related crates for the Pheno ecosystem.

Docs shell: `docs/` is a VitePress site published through GitHub Pages.

## Crates

This workspace currently ships **3 crates** (verified against [`Cargo.toml`](./Cargo.toml)):

| Crate | Description |
|-------|-------------|
| [`surreal-bridge`](./crates/surreal-bridge) | SurrealDB embedded integration (skill/embedding storage) |
| [`pg-bridge`](./crates/pg-bridge) | PostgreSQL with `pgvector` for vector search |
| [`pheno-query`](./crates/pheno-query) | Unified query planner across data stores |

## Workspace layout

```
phenoData/
├── Cargo.toml              # Workspace manifest (3 members, resolver = "3")
├── Cargo.lock              # Locked dep graph (committed)
├── justfile / Justfile     # Phenotype-org standard recipes
├── tarpaulin.toml          # Coverage threshold baseline (see Coverage)
├── deny.toml               # cargo-deny config (licenses, advisories, sources)
├── rust-toolchain.toml     # Stable channel + rustfmt + clippy
├── package.json            # VitePress docs sidecar
├── bun.lock                # Locked docs deps
├── docs/                   # VitePress site (published via GitHub Pages)
├── crates/
│   ├── surreal-bridge/     # SurrealDB embedded integration
│   ├── pg-bridge/          # PostgreSQL + pgvector
│   └── pheno-query/        # Unified query planner
├── .github/
│   ├── workflows/          # CI: ci, coverage, gitleaks, trufflehog, codeql-rust, ...
│   └── dependabot.yml      # cargo + npm + github-actions (weekly, grouped)
└── worklogs/               # Per-session agent work logs
```

## Requirements

- **Rust 1.84+** — workspace uses `resolver = "3"` (stabilized in 1.84).
- **PostgreSQL 14+ with `pgvector`** — required for `pg-bridge`. Install:
  ```bash
  # macOS
  brew install postgresql@16 pgvector
  # Debian/Ubuntu
  sudo apt install postgresql-16 postgresql-16-pgvector
  ```
  Then enable the extension in your database:
  ```sql
  CREATE EXTENSION IF NOT EXISTS vector;
  ```
- **SurrealDB** — `surreal-bridge` runs SurrealDB embedded; no external server required.

## Usage

Add the crate(s) you need to your `Cargo.toml`:

```toml
[dependencies]
surreal-bridge = { path = "../phenoData/crates/surreal-bridge" }
pg-bridge      = { path = "../phenoData/crates/pg-bridge" }
pheno-query    = { path = "../phenoData/crates/pheno-query" }
```

> First build compiles SurrealDB and pgvector bindings — expect a longer initial `cargo build`.

## Build & Test

```bash
cargo build --workspace
cargo test  --workspace
cargo clippy --workspace -- -D warnings
```

### Test

The full test sweep runs the `ci.yml` job on every push and pull request:

```bash
# Local equivalent
just test
cargo test --workspace
```

`pg-bridge` integration tests skip cleanly when PostgreSQL with `pgvector` is
not reachable (see `AGENTS.md` agent guardrails). No external services are
required for the default test path.

### Coverage

Coverage is produced by [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov)
in the [`coverage.yml`](.github/workflows/coverage.yml) CI job and uploaded to
Codecov. The baseline threshold (60% line coverage) is mirrored between
[`tarpaulin.toml`](./tarpaulin.toml) and the `--fail-under-lines` flag in CI.

```bash
# Install once
cargo install cargo-llvm-cov --locked

# Local equivalent of CI
just coverage           # HTML report at target/llvm-cov/html/index.html
just coverage-lcov      # lcov.info at repo root (CI artifact)
```

To bypass the threshold locally, drop the `--fail-under-lines` flag from the
recipe — but keep CI strict.

Integration smoke tests live in `crates/smoke-tests/tests/smoke.rs` and are part of the workspace test run.

## Docs

```bash
bun install
bun run docs:build
```

The Pages workflow builds with `GITHUB_PAGES=true`, which serves the site under
the `/phenoData/` GitHub Pages base path.

## Examples

Standalone examples are not yet published. See each crate's `src/` and inline tests
(`#[cfg(test)]` modules) for current usage patterns. Tracking issue for examples
will be opened once the public API stabilizes.

## License

Dual-licensed under MIT or Apache-2.0 — see [`LICENSE-MIT`](./LICENSE-MIT) and
[`LICENSE-APACHE`](./LICENSE-APACHE). The repository also carries the org-standard
[`LICENSE`](./LICENSE) file.

## Description

Phenotype data-layer workspace — Rust crates for `surreal-bridge` (SurrealDB embedded integration), `pg-bridge` (PostgreSQL + `pgvector`), and `pheno-query` (unified query planner across data stores).

## Install

Requirements: Rust 1.84+, PostgreSQL 14+ with `pgvector` for `pg-bridge` (see `## Requirements` above). Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
surreal-bridge = { path = "../phenoData/crates/surreal-bridge" }
pg-bridge      = { path = "../phenoData/crates/pg-bridge" }
pheno-query    = { path = "../phenoData/crates/pheno-query" }
```

First build compiles SurrealDB + pgvector bindings — expect a longer initial `cargo build`.

## Contributing

PRs welcome. See `CONTRIBUTING.md`. New crates go in `crates/` and join the workspace via `Cargo.toml`. Schema changes for `pg-bridge` need a matching pgvector migration in the crate's `migrations/` dir.
