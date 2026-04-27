# PhenoData

PhenoData is the Phenotype data-layer workspace for storage adapters and query
planning. It currently ships three Rust crates:

- `surreal-bridge` for embedded SurrealDB-backed skill and embedding storage.
- `pg-bridge` for PostgreSQL with `pgvector`.
- `pheno-query` for a unified query planner across data stores.

The repository is in maintenance mode. Changes should focus on correctness,
dependency hygiene, compatibility, and clear documentation for existing APIs.

## Quick Start

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

`pg-bridge` requires PostgreSQL 14+ with the `pgvector` extension for integration
work. `surreal-bridge` uses embedded SurrealDB and does not require a standalone
server.

## Public Surfaces

| Surface | Purpose |
| --- | --- |
| Workspace root | Rust workspace and shared dependency policy |
| `crates/surreal-bridge` | Embedded SurrealDB integration |
| `crates/pg-bridge` | PostgreSQL and `pgvector` integration |
| `crates/pheno-query` | Cross-store query planner |
| GitHub Pages docs | This VitePress docs shell |

## Status

PhenoData is a maintenance project. Prefer narrowly scoped fixes, reproducible
tests, and dependency/security updates over new product surface area.
