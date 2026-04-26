# PhenoData — Data Layer Workspace

**Status:** maintenance

Data-related crates for the Pheno ecosystem.

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

## Examples

Standalone examples are not yet published. See each crate's `src/` and inline tests
(`#[cfg(test)]` modules) for current usage patterns. Tracking issue for examples
will be opened once the public API stabilizes.

## License

Dual-licensed under MIT or Apache-2.0 — see [`LICENSE-MIT`](./LICENSE-MIT) and
[`LICENSE-APACHE`](./LICENSE-APACHE).

## License

MIT — see [LICENSE](./LICENSE).
