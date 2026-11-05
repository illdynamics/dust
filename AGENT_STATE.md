# AGENT_STATE — Dust

**Branch:** `v0.1.0-alpha` (from `main`; main is at `b9ce578`). Repo: https://github.com/illdynamics/dust.git
**Milestones:** M0 VERIFIED · **M1 VERIFIED** (compiler skeleton: version/help, source loader, diagnostics, tests). Next: M2 (typed core).
**Last verified task set:** S02-T01…S02-T06 (all `[V]`); 11 unit/integration tests passing.

M1 complete. Next milestone M2 begins with S03 lexer → S04 parser → S05 type checker. See NEXT_SESSION.md.

## Toolchain inventory (verified)
- rustc/cargo 1.95.0 · Apple clang 17.0.0 (cc/gcc/clang) · GNU Make 3.81 · CMake 4.4.5 · ld (Apple).
- Bootstrap decision: Rust host compiler `dustc`, C11 backend, output via installed clang. Matches reality.

## Files changed this session (uncommitted until commit below)
- Cargo.toml (workspace), crates/dustc/Cargo.toml
- crates/dustc/src/{lib.rs,main.rs,cli.rs,diagnostics.rs,source.rs,driver.rs}
- crates/dustc/tests/{source_tests.rs,diagnostics_tests.rs}

## Build / test commands (all green now)
- `cargo build -p dustc` → Finished dev profile.
- `cargo test -p dustc` → 11 passed (8 source + 3 diagnostics), 0 failed.
- `$BIN --version` → `target/debug/dustc dustc 0.1.0-alpha`; `--help` prints usage; empty/smoke `.dust` compiles to "compiled … (0 diagnostics)".

## Known limitations / not-yet-implemented
- No lexer/parser/type-checker/backend yet (S03–S08 pending). Front end currently accepts any loaded source with 0 diagnostics.
- `codespan-reporting` intentionally NOT used — diagnostics rendered hand-rolled to keep deps minimal & output deterministic/testable. Rationale in DECISIONS.md.
- CI/S15/S16/S17/S18 not built.

## Blockers
None. (No sudo/system installs performed; none needed.)

## Next exact task
`S03-T01` — Lexer: identifiers + keywords. — add `rustfmt.toml`, run `cargo fmt --check`, and a minimal `.github/workflows/ci.yml` for the host toolchain. Then start `S03-T01` (lexer: identifiers + keywords).
