# Crates

## `surreal-bridge`

Embedded SurrealDB integration for local graph/document-style storage. Use it
for skill, embedding, and metadata workloads that benefit from embedded storage.

## `pg-bridge`

PostgreSQL integration with `pgvector` support. Use it for durable relational
storage and vector-search workloads where the environment can provide a live
Postgres database.

## `pheno-query`

Unified query planner across data stores. Query planning changes should preserve
traceability across the bridge crates and keep planner tests current.

## Dependency Policy

Workspace dependencies live in the root `Cargo.toml`. Prefer shared dependency
versions there unless a crate has a clear compatibility reason to diverge.
