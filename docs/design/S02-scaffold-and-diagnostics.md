# Section Summary — S02: Project scaffold, build system, diagnostics foundation

**Status:** VERIFIED. Gate met: clean `cargo build -p dustc`, `--version`/`--help` work, tests pass from a fresh checkout (documented below).

## What was built
- Cargo workspace (`Cargo.toml`) + `crates/dustc` bin crate. Minimal deps (none runtime; `tempfile` dev-dep for harness).
- `source.rs`: `SourceFile` (UTF-8 bytes, precomputed line starts, stable file id), byte spans `[start,end)`, `offset_to_location` (1-based line/byte-col), `render_snippet` (trims CRLF, returns underline columns).
- `diagnostics.rs`: `Severity{Error,Warning}` (+Default), `Label`, `Diagnostic{severity,code,message,labels,notes}`, and a self-contained `render()` producing `sev[code] msg` / `--> file:line:col` / source line + `~` underline / `= label:` / `= note:`. No external crate (D-S02-2).
- `cli.rs`: hand-rolled argv parse — `--version/-V`, `--help/-h`, `--no-colors/--nocolor`, `-o/--output`, `--emit c|<backend>`, single positional `.dust`. Plus usage text.
- `driver.rs` + thin `main.rs`: `SourceMap`, `compile()` (accepts, 0 diagnostics pre-front-end), `emit_diagnostics`, `run(cli)`.

## Verification
- `cargo build -p dustc` → Finished.
- `cargo test -p dustc` → 11 passed (8 source + 3 diagnostics), 0 failed.
- Binary smoke: `--version` prints `… 0.1.0-alpha`; empty and non-empty `.dust` files report "compiled … (0 diagnostics)"; bad flag exits 1 with a clear message.

## Tests added
- `crates/dustc/tests/source_tests.rs`: line/col math, UTF-8 multibyte columns, span_text, underline markers, CRLF trim, span constructors.
- `crates/dustc/tests/diagnostics_tests.rs`: severity+code+message header, location+snippet+underline+label+note rendering, warning path.

## Known gaps (carried forward)
- No lexer/parser/type-checker/backend yet — front end accepts any source for now.
- S02-T07 (rustfmt + CI) pending next session.
