# AGENTS.md — phenoData

Pheno data-layer workspace: SurrealDB embedded bridge, Postgres + pgvector bridge, and unified `pheno-query` planner.

## Repository identity

- Language: Rust 1.84+ (workspace uses `resolver = "3"`, see `rust-toolchain.toml`).
- Entry point: `Cargo.toml` (root workspace).
- Status: maintenance.
- Crates (verified from README and `crates/` layout):
  - `surreal-bridge` — SurrealDB embedded integration.
  - `pg-bridge` — Postgres + pgvector for vector search.
  - `pheno-query` — Unified query planner across data stores.

## External requirements (verified from README)

- Rust 1.84+.
- PostgreSQL 14+ with `pgvector` extension (`CREATE EXTENSION IF NOT EXISTS vector;`).
- SurrealDB runs embedded — no external server.

## Build & test (verified from README)

```bash
cargo build  --workspace
cargo test   --workspace
cargo clippy --workspace -- -D warnings
```

First build compiles SurrealDB and pgvector bindings — expect a long initial `cargo build`.

## Governance

- Triple license: MIT / Apache-2.0 (`LICENSE`, `LICENSE-APACHE`, `LICENSE-MIT`).
- Security: `SECURITY.md`. Contributing: `CONTRIBUTING.md`. Owners: `CODEOWNERS`.
- Changelog: `CHANGELOG.md`.

## Commit & branch convention

- Conventional Commits.
- Branch: `<type>/<topic>`.
- Status is "maintenance" — favor bug fixes and dependency hygiene over new features.

## Agent guardrails

- `pg-bridge` requires a live Postgres with `pgvector`; integration tests must skip when the env is unavailable rather than fail.
- Cross-crate query changes must update `pheno-query` planner tests to keep traceability.
