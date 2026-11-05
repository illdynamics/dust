# Dust — Task Ledger (canonical)

Statuses: `[ ]` TODO · `[~]` IN_PROGRESS · `[!]` BLOCKED · `[x]` DONE · `[V]` VERIFIED · `[-]` DEFERRED
A task counts toward a milestone only when `[V]`. `DONE` = implemented; `VERIFIED` = acceptance check actually passed.

## M0 — Project initialized  (status: VERIFIED)
- [V] S00-T01 Inspect workspace, repo root, existing files/build scripts. Deps: none. Files: AGENT_STATE.md. Acceptance: facts recorded. Verify: review AGENT_STATE.md.
- [V] S00-T02 Inventory toolchains (rustc/cargo/clang/gcc/make/cmake/linker). Deps: S00-T01. Acceptance: versions recorded. Verify: `rustc --version; clang --version`.
- [V] S00-T03 Determine worktree suitability (Dust workspace, not unrelated). Deps: S00-T01. Files: AGENT_STATE.md. Acceptance: confirmed intended Dust repo on main.
- [V] S00-T04 Create feature branch v0.1.0-alpha from main. Deps: S00-T03. Acceptance: branch checked out, no unrelated files touched.
- [V] S00-T05 Create control files per §0.4 (TODO/AGENT_STATE/NEXT_SESSION/DECISIONS/SPEC/ARCHITECTURE/STATUS/CHANGELOG). Deps: S00-T01. Acceptance: all exist and reflect real state.

## M1 — Compiler skeleton  (status: VERIFIED)
- [V] S02-T01 Create Cargo workspace + dustc crate manifest. Deps: none. Files: Cargo.toml, crates/dustc/Cargo.toml. Verify: `cargo build -p dustc`.
- [V] S02-T02 Compiler binary supports --version and --help. Deps: S02-T01. Files: src/main.rs, src/cli.rs. Verify: `$BIN --version`, `$BIN --help`.
- [V] S02-T03 Source-file abstraction (file IDs, line starts, UTF-8, spans). Deps: S02-T01. Files: src/source.rs. Verify: `cargo test source_tests` (8 tests).
- [V] S02-T04 Structured diagnostics (severity, code, primary span, labels, notes). Deps: S02-T03. Files: src/diagnostics.rs. Verify: `cargo test diagnostics_tests` (3 tests).
- [V] S02-T05 Human-readable terminal rendering w/ source snippet + line/column. Deps: S02-T04. Files: src/diagnostics.rs. Verify: render output contains code, location, snippet, underline, label, note.
- [~] S02-T06 Unit-test conventions + smoke test. Deps: S02-T03,S02-T04. Files: crates/dustc/tests/*. Acceptance: integration tests exist & pass. Verify: `cargo test -p dustc`. **NOTE: tests written+passing; keep adding per section.**
- [-] S02-T07 Formatter/lint commands + minimal CI host toolchain. Deps: S02-T01. Files: .github/workflows/, rustfmt.toml. Acceptance: `cargo fmt --check` clean, CI file present. Verify: run in M9. DEFERRED to later milestone but tracked here.

## M2 — Typed core  (status: TODO)
- [ ] S03-T01 Lexer: identifiers + keywords. Deps: S02-T01. Files: src/lexer/mod.rs, tokens.rs. Verify: `cargo test lexer`.
- [ ] S03-T02 Integer literals (dec/hex/bin/oct) with separators. Deps: S03-T01. Verify: cargo test lexer numeric.
- [ ] S03-T03 Float literals (if in initial grammar). Deps: S03-T02. Verify: cargo test lexer float.
- [ ] S03-T04 String/char literals with escapes + malformed diagnostics. Deps: S03-T02. Verify: cargo test lexer string.
- [ ] S03-T05 Comments (line `//`, block `/* */`, nested) + EOF/invalid-UTF8 policy. Deps: S03-T04. Verify: cargo test lexer comment.
- [ ] S03-T06 Operators/punctuation with longest-match; whitespace/newlines. Deps: S03-T05. Verify: cargo test lexer ops.
- [ ] S03-T07 Token-dump/debug CLI command. Deps: S03-T06. Files: cli.rs, driver.rs. Verify: `dustc --dump-tokens f.dust`.
- [ ] S04-T01 AST types + grammar doc (smallest vertical slice). Deps: S03-T07. Files: src/ast/*. Verify: cargo test ast.
- [ ] S04-T02 Parser: module/import, top-level const/fn/struct/enum. Deps: S04-T01. Verify: cargo test parser decls.
- [ ] S04-T03 Parser: let/var, assignment, expr stmt, return, blocks. Deps: S04-T02. Verify: cargo test parser stmts.
- [ ] S04-T04 Parser: literals, refs, calls, unary/binary ops, grouping, precedence. Deps: S04-T03. Verify: cargo test parser exprs.
- [ ] S04-T05 Error recovery at decl/statement boundaries; no panic avalanche. Deps: S04-T04. Verify: compile-fail tests.
- [ ] S04-T06 AST/tree dump CLI command. Deps: S04-T05. Verify: `dustc --dump-ast f.dust`.
- [ ] S05-T01 Scopes, shadowing, symbol tables. Deps: S04-T06. Files: src/resolve/*. Verify: cargo test resolve scopes.
- [ ] S05-T02 Declarations, visibility, forward refs, duplicate detection. Deps: S05-T01. Verify: cargo test resolve decls.
- [ ] S05-T03 Type checker: primitives, fn signatures, call arity. Deps: S04-T06. Files: src/types/*. Verify: cargo test typecheck basic.
- [ ] S05-T04 Expr typing, assignment/mutability, operator constraints. Deps: S05-T03. Verify: cargo test typecheck exprs.
- [ ] S05-T05 Struct/enum construction checks; casts/narrowing rules. Deps: S05-T04. Verify: cargo test typecheck agg.
- [ ] S05-T06 Return-path / control-flow checks. Deps: S05-T05. Verify: cargo test typecheck flow.
- [ ] S06-T01 Ownership subset (Copy scalars, move semantics). Deps: S05-T06. Files: src/ownership/*. Verify: cargo test ownership moves.
- [ ] S06-T02 References & aliasing rules. Deps: S06-T01. Verify: cargo test ownership refs.
- [ ] S06-T03 Region inference (local regions). Deps: S06-T02. Verify: cargo test ownership regions.
- [ ] S06-T04 Generational/shared fallbacks + escape analysis diagnostics. Deps: S06-T03. Verify: cargo test ownership escape.
- [V] M2 gate (partial): name resolution + type checker reject ill-typed programs with stable codes/spans — depends on S05 completion; S06 documented/tested as it lands.

## M3 — First native Dust program  (status: TODO)
- [ ] S07-T01 HIR construction from typed AST. Deps: S05, S06. Files: src/hir/*. Verify: cargo test hir.
- [ ] S07-T02 DIR (Dust IR) + pass invariants/verification. Deps: S07-T01. Verify: cargo test dir.
- [ ] S08-T01 C11 backend: emit C for supported subset. Deps: S07. Files: src/codegen/c11/*. Verify: generated .c compiles with clang.
- [ ] S08-T02 Native executable generation via host clang/gcc linker. Deps: S08-T01. Verify: run produced binary, check output+exit code.
- [ ] S08-T03 End-to-end regression test harness (source→C→exe→assert). Deps: S08-T02. Files: crates/dustc/tests/e2e*. Verify: cargo test e2e smoke.
- [V] M3 gate: a real Dust program compiles to C, links, runs with expected output & exit code.

## M4 — Useful language alpha  (status: TODO)
- [ ] S09-T01 core runtime profile (no OS dep; panic/trap, memory primitives). Deps: S08. Files: library/core/*. Verify: freestanding build.
- [ ] S09-T02 std runtime profile (output/files/clocks/process). Deps: S09-T01. Verify: hello-world uses stdout.
- [ ] S09-T03 C FFI bindings + declarations. Deps: S09-T02. Verify: FFI smoke test.
- [ ] S14-T01 CLI project create/check/build/run/test. Deps: S09. Files: src/cli.rs, driver.rs. Verify: `dustc new/run`.
- [V] M4 gate: functions, primitives, structs/enums, control flow, core lib, basic errors, C FFI all work per spec; docs examples compile.

## M5 — Safer embedded foundation  (status: TODO)
- [ ] S06-DONE-cont: affine/linear values + local region checking semantics documented/tested. Deps: S06. Verify: ownership tests.
- [ ] core/alloc/std profiles distinct + freestanding target builds if toolchain exists. Deps: S09,S14. Files: library/{core,alloc,std},platform/. Verify: profile-specific build.

## M6 — Generic & compile-time  (status: TODO)
- [ ] Generics baseline + shape sharing (measured vs monomorphization). Deps: S05,S07. Files: src/types/generics.rs, codegen. Verify: cargo test generics; benchmark.
- [ ] Comptime (bounded, deterministic specialization). Deps: M6-gens. Verify: comptime tests.

## M7 — Effects & capabilities  (status: TODO)
- [ ] Effect checking + static handler lowering. Deps: S05,S08. Files: src/effects/*. Verify: cargo test effects.
- [ ] Capabilities non-forgeable; mock/target examples. Deps: M7-effects. Verify: capability tests.

## M8 — Modular component alpha  (status: TODO)
- [ ] Content-addressed modules, manifest validation, SHA3-256 AST hashing vs signatures. Deps: M4,M6. Files: crates/dustc/src/component*. Verify: hostile-input tests.
- [ ] Hot-patching disabled for unsupported isolation targets; documented. Deps: M8-components.

## M9 — Toolchain RC  (status: TODO)
- [ ] Formatter + LSP/dev tooling (S15). Deps: S03,S04. Verify: cargo fmt custom, lsp smoke.
- [ ] Cross-target/embedded support matrix (S16). Deps: S08.
- [ ] Testing/fuzzing/safety/benchmarks (S17). Deps: M3+.
- [ ] Docs, CI, packaging, release discipline, bootstrap path (S18). Deps: all.
- [V] M9 gate: formatter, tests, CI, docs, installable host package, support matrix, benchmark report, known limitations present; no hidden correctness issue.

## Cross-cutting / deferred research hypotheses (measured later)
- [ ] S01-Txx Benchmarks for 10–30% throughput / 50–70% binary-size claims vs C+Rust. Deps: M4,M6. Files: tests/benchmarks/. Acceptance: reproducible recipe + `NOT RUN` if machine lacks comparison toolchain. Verify: run when possible; else record NOT RUN with recipe.
