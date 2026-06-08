# Phenotype-org standard justfile

default:
    @just --list

# Build workspace
build:
    cargo build --workspace

# Run tests
test:
    cargo test --workspace

# Lint (clippy + fmt --check)
lint:
    cargo clippy --workspace -- -D warnings
    cargo fmt --check

# Format code
fmt:
    cargo fmt

# Security audits (cargo-deny + cargo-audit)
audit:
    cargo deny check
    cargo audit

# Find unused dependencies
unused:
    cargo machete

# Full local CI sweep
ci: lint test audit unused

# Generate docs
docs:
    cargo doc --no-deps --workspace

# Generate HTML coverage report (cargo-llvm-cov; install with `cargo install cargo-llvm-cov --locked`)
coverage:
    cargo llvm-cov --workspace --html

# Generate lcov.info (CI uses this artifact path)
coverage-lcov:
    cargo llvm-cov --workspace --lcov --output-path lcov.info
