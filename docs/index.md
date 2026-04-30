# PhenoData

> Data-layer workspace for storage adapters, vector search, and query planning.

PhenoData is the Phenotype data-layer workspace. It keeps the storage concerns
separated into a small set of Rust crates, then documents the workspace through
this VitePress shell.

## Surfaces

| Surface | Purpose |
| --- | --- |
| `crates/surreal-bridge` | Embedded SurrealDB integration for local graph/document workloads |
| `crates/pg-bridge` | PostgreSQL + `pgvector` support for durable storage and similarity search |
| `crates/pheno-query` | Unified planner across supported data stores |
| `docs/` | Maintenance docs, usage notes, and operations guidance |

## Maintenance posture

This repo is in maintenance mode. Favor correctness, dependency hygiene,
compatibility, and documentation clarity over new surface area.

## Quick start

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

`pg-bridge` requires PostgreSQL 14+ with the `pgvector` extension. `surreal-bridge`
runs embedded and does not require a standalone server.

## Public routes

- [`Guide`](/guide) for the workspace model and local requirements
- [`Crates`](/crates) for the three crate summary
- [`Operations`](/operations) for build, test, lint, and docs commands

## Public contract

- Workspace root owns dependency policy and crate coordination
- Bridge crates own store-specific integration
- Query planner owns cross-store orchestration
- Docs site is the public face of the workspace
