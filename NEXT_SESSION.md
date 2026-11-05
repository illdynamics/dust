# NEXT SESSION — Dust (v0.1.0-alpha)

## Resume state
- Branch `v0.1.0-alpha` off `main`. Milestone **M1 in progress**; M0 done.
- S02 scaffold + diagnostics implemented and VERIFIED (11 tests pass).

## First command to run
```bash
cargo test -p dustc            # must stay green before touching anything else
cargo fmt --all -- --check     # keep formatting clean
```

## Milestone M1 done; next milestone M2 starts with the lexer (`S03`).
1. Add `rustfmt.toml` at repo root (stable style). Run `cargo fmt --check`.
2. Add `.github/workflows/ci.yml`: build+test `-p dustc`, `cargo fmt --check` on host toolchain.
3. Update TODO.md/AGENT_STATE.md; mark S02-T07 `[V]`; finish M1 (gate: tests pass).

## Then: start `S03-T01` — Lexer
- Read master prompt § **Section S03** only (`grep -n '^#' DUST_FULL_BUILD_PROMPT.md` → "## Section S03").
- Implement identifiers+keywords first in `crates/dustc/src/lexer/mod.rs` + `tokens.rs`; add `tests/lexer/*`.
- Keep spans byte-offset half-open; lexical errors must be span-correct and never silently skip.

## Context discipline (do not skip)
- Read the master prompt ONE major section at a time via line-range grep; do NOT cat it whole.
- After each task: update TODO.md + AGENT_STATE.md, add regression test, re-run `cargo test -p dustc`.
- Keep AGENT_STATE.md < 150 lines; this file is the handoff — keep it short.

## Facts to preserve
- Toolchain: rustc/cargo 1.95.0, Apple clang 17.0.0, Make 3.81, CMake 4.4.5. No sudo unless approved.
- Deps so far: none in dustc runtime; `tempfile` dev-dep only (S02-T06 harness).
