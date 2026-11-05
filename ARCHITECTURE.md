# Dust — Architecture

## Pipeline (target, §2.2)
```
Dust source
  -> lexer            (src/lexer/*: tokens + spans + lexical errors)
  -> parser/AST        (src/parser/*, src/ast/*: grammar, recovery, precedence table)
  -> resolve            (src/resolve/*: scopes, symbols, visibility, duplicates)
  -> typecheck          (src/types/*: primitives, fn sigs, expr typing, casts, control flow)
  -> ownership          (src/ownership/*: moves, refs, regions, escape analysis)
  -> HIR                (src/hir/*: typed high-level IR + stable IDs)
  -> DIR (Dust IR)      (src/ir/*: invariants + verification passes)
  -> codegen/c11        (src/codegen/c11/*: deterministic C11 emission)
  -> host clang/gcc     (link -> native executable / object)
```
Source spans and stable symbol/type IDs are threaded unchanged through all front-end stages (§2.2) so diagnostics stay useful.

## Module layout (crates/dustc/src)
- `lib.rs` — re-exports modules for tests; bin is a thin wrapper (`main.rs`).
- `source.rs` — `SourceFile`, `Span`, offsets/line-starts, `offset_to_location`, `render_snippet`.
- `diagnostics.rs` — `Severity`, `Label`, `Diagnostic`, `render()` (self-contained terminal reporter).
- `cli.rs` — hand-rolled arg parsing (`--version/-V/--help/-o/--emit/--no-colors`), usage text.
- `driver.rs` — `SourceMap`, `compile()`, `emit_diagnostics()`, `run(cli)`.
- (future) `lexer/ parser/ ast/ resolve/ types/ ownership/ hir/ ir/ passes/ codegen/c11/` per §2.3.

## Diagnostics contract (S02-T04/T05)
Every diagnostic has: severity, optional stable code string, message, ≥0 labeled spans (one may be primary), and 0+ notes. Rendering emits `sev[code] msg`, `  --> file:line:col`, the source line, a `~` underline at the right column, an optional `= label: ...`, and `= note: ...`. Deterministic & unit-tested; no external crate.

## Backend boundary
C11 must be deterministic, readable, conservative about UB: fixed-width `<stdint.h>` types, explicit overflow handling, layout assertions where needed, safe symbol mangling, ABI-aware declarations. Never assume C `long` width across targets. Runtime split into core/alloc/std/embedded profiles; freestanding programs must not link host stdlib.

## Dependency policy
Minimal, standard-library-first. Add a crate only with a recorded rationale in DECISIONS.md (none added so far).
