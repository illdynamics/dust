# Changelog

Format follows [Keep a Changelog](https://keepachangelog.com) and SemVer. Versioning aligns with the alpha milestone line (`0.1.0-alpha`).

## [0.1.0-alpha] — Unreleased (in progress)

### Added
- Cargo workspace + `dustc` compiler crate (Rust host tool, C11 backend target).
- Source-file model: byte spans, precomputed line starts, UTF-8, stable file IDs.
- Structured diagnostics with severity, error codes, labeled spans, notes, and a self-contained terminal reporter rendering source snippets + (line,col) locations.
- `dustc --version` / `--help`, plus `-o/--output`, `--emit=BACKEND`, `--no-colors` flags.
- Unit + integration tests: 8 source-model tests, 3 diagnostics tests (all passing).

### Changed
- Minimal-dependency policy enforced: no external crates in the first slice; diagnostics rendered hand-rolled for deterministic, testable output.

### Notes / Under construction
- Lexer/parser/type-checker/C11 backend are planned (S03–S08); the compiler currently accepts loaded sources with 0 diagnostics until the front end lands.
- Research performance claims (10–30% throughput, 50–70% size) remain unmeasured hypotheses (§1.3).

## [Unreleased] — History
_Entries added below this line as milestones complete._
