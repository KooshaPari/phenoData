# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **CI coverage** — new `coverage.yml` workflow runs `cargo-llvm-cov` over the workspace,
  enforces a 60% line-coverage floor (`--fail-under-lines 60`), and uploads
  `lcov.info` to Codecov plus a 14-day GitHub Actions artifact.
- **`tarpaulin.toml`** — workspace coverage-threshold baseline (HTML, XML, Lcov
  outputs; `--features all`; mirrors the CI `--fail-under-lines` value).
- **Gitleaks workflow** (`gitleaks.yml`) — weekly + push/PR secret scan
  complementing the existing TruffleHog jobs; pinned `gitleaks/gitleaks-action`.
- **`just coverage`** and **`just coverage-lcov`** recipes (mirrored in both
  `justfile` and `Justfile`) — local equivalents of the new CI coverage job.
- **Dependabot coverage** — `.github/dependabot.yml` now tracks `cargo`, `npm`
  (VitePress sidecar), and `github-actions` ecosystems, all weekly on Monday
  with grouped PRs and labels (`dependencies` + ecosystem tag).
- **README sections** — `Workspace layout`, `Test`, and `Coverage` sections
  added with directory tree and `cargo-llvm-cov` usage.

### Changed

- `.github/dependabot.yml` extended from `cargo`-only to three ecosystems
  (cargo + npm + github-actions) with grouping and labels.
- `.gitignore` — removed the two duplicate `Cargo.lock` lines. They were being
  silently concatenated (missing final newline on the first line) into a
  non-matching `Cargo.lockCargo.lock` pattern by git's parser. `Cargo.lock`
  is committed in this library workspace for reproducible downstream builds;
  no ignore is the correct policy here. Added an inline comment explaining the
  decision and linking to the Cargo reference.

[Unreleased]: https://github.com/KooshaPari/phenoData/compare/main...HEAD
