# Operations

## Build

```bash
cargo build --workspace
```

First builds can take longer because SurrealDB and database-related dependencies
compile a large transitive graph.

## Test

```bash
cargo test --workspace
```

Postgres-backed tests must detect missing database configuration and skip rather
than fail. This keeps basic workspace validation useful on machines without a
local `pgvector` database.

## Lint

```bash
cargo clippy --workspace -- -D warnings
```

## Docs Site

```bash
bun install
bun run docs:build
```

GitHub Pages builds the docs with `GITHUB_PAGES=true`, which sets the VitePress
base path to `/phenoData/`.
