# PhenoData - Data Layer Workspace

Data-related crates for the Pheno ecosystem.

## Crates

| Crate | Description |
|-------|-------------|
| `surreal-bridge` | SurrealDB embedded with MCP extensions |
| `pg-bridge` | PostgreSQL/pgvector compatibility |
| `pheno-query` | Unified query builder |

## Usage

```toml
# Cargo.toml
[dependencies]
surreal-bridge = { path = "../phenoData/crates/surreal-bridge" }
pg-bridge = { path = "../phenoData/crates/pg-bridge" }
```
