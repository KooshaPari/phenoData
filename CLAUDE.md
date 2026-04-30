# phenoData

Data layer workspace — data-related Rust crates for the Pheno ecosystem.

## Stack
| Layer | Technology |
|-------|------------|
| Core | Rust (cargo workspace, 3 crates) |
| DB | SurrealDB (embedded), PostgreSQL (pg-bridge) |
| Vector | pgvector compatibility |
| Query | Unified query builder |
| Docs | VitePress (published via GitHub Pages) |

## Key Commands
```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --all

# Docs dev
cd docs-site && npm install && npm run docs:dev
```

## Key Files
- `crates/surreal-bridge/` — SurrealDB embedded integration (skill/embedding storage)
- `crates/pg-bridge/` — PostgreSQL/pgvector compatibility
- `crates/pheno-query/` — Unified query builder
- `docs-site/` — VitePress documentation site
- `docs/` — Docs source (crates.md, guide.md, index.md, operations.md)

## Reference
Global Phenotype rules: see `~/.claude/CLAUDE.md` or `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md`
