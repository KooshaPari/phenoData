> **Pinned references (Phenotype-org)**
> - MSRV: see rust-toolchain.toml
> - cargo-deny config: see deny.toml
> - cargo-audit: rustsec/audit-check@v2 weekly
> - Branch protection: 1 reviewer required, no force-push
> - Authority: phenotype-org-governance/SUPERSEDED.md

# PhenoData — Data Layer Workspace

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Quality Gate](https://github.com/KooshaPari/phenoData/actions/workflows/quality-gate.yml/badge.svg)](https://github.com/KooshaPari/phenoData/actions/workflows/quality-gate.yml)
[![Docs](https://github.com/KooshaPari/phenoData/actions/workflows/pages.yml/badge.svg)](https://github.com/KooshaPari/phenoData/actions/workflows/pages.yml)
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
