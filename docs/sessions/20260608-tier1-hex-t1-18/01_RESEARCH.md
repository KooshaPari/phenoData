# Research

- Verified workspace members from the root Cargo.toml: surreal-bridge, pg-bridge, pheno-query, smoke-tests.
- Verified branch tier1-hex-T1.18-phenoData was effectively at origin/main and did not contain Dataset, Record, SurrealDataset, PgDataset, or load() APIs.
- Chosen approach: add a small pheno-data-core crate for the dataset port and keep adapter testability via injected closures rather than real DB clients.
