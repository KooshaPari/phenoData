# Guide

## Workspace Model

PhenoData keeps each data concern in a dedicated crate:

- Use `surreal-bridge` when the caller needs embedded document or graph-style
  storage without running a separate database service.
- Use `pg-bridge` when the caller needs PostgreSQL durability, SQL semantics, or
  `pgvector` similarity search.
- Use `pheno-query` when a caller needs one planner surface across supported
  stores.

## Local Requirements

- Rust 1.84+.
- PostgreSQL 14+ and `pgvector` for `pg-bridge` integration work.
- SurrealDB embedded dependencies for `surreal-bridge`.

Enable `pgvector` in a development database before running Postgres-backed tests:

```sql
CREATE EXTENSION IF NOT EXISTS vector;
```

## Validation Loop

Run the workspace checks before publishing changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

If Postgres is not available, integration tests should skip cleanly rather than
fail the entire workspace.
