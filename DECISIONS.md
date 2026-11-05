# Dust — Decision Log

Dated architecture/language/build decisions. Status: `provisional` | `final`.

## D-S02-1 — Compiler implementation language = stable Rust (host tool)
- **Date:** 2026-11-05
- **Decision:** Implement `dustc` in stable Rust as a host tool. First native backend emits C11; an installed clang/gcc produces the executable.
- **Basis:** Master §2.1 bootstrap architecture + §0.4 priorities (vertical slice first). Toolchain inventory confirms rustc/cargo 1.95.0 and Apple clang 17 available; no LLVM binding dependency in first slice.
- **Alternatives:** Zig host, C host, or direct LLVM. Rejected: Rust is installed, well-suited to a hand-built compiler, and keeps the portable C backend reachable without an LLVM dep.
- **Consequences:** `rust-toolchain.toml` optional (pin only if justified). Backend stays behind a clean interface for a future LLVM/Cranelift backend (§2.1).
- **Status:** final

## D-S02-2 — Diagnostics rendered hand-rolled, not codespan-reporting
- **Date:** 2026-11-05
- **Decision:** Build the diagnostic model (`Severity`, `Label`, `Diagnostic`) and a self-contained terminal reporter in `src/diagnostics.rs`. No external crate.
- **Basis:** Master §0.3 "keep dependencies minimal"; S02-T04/T05 require severity/code/span/labels/notes + source snippets & line/column.
- **Alternatives:** Use `codespan-reporting` (widely used, nice output). Rejected to honor the explicit minimize-dependencies rule and to keep rendering deterministic and unit-testable with zero deps. Rationale recorded; can adopt codespan later behind a stable `Diagnostic` type if desired without rewriting call sites.
- **Consequences:** Slightly more code than a one-liner, but transparent and testable today.
- **Status:** final

## D-S02-3 — CLI parsed by hand (no clap/structopt)
- **Date:** 2026-11-05
- **Decision:** `src/cli.rs` parses argv with the std iterator directly; supports `--version/-V`, `--help/-h`, `--no-colors/--nocolor`, `-o/--output <path>`, `--emit c|<backend>`, and a single positional `.dust` input.
- **Basis:** Minimal-dependency policy + S02-T02.
- **Consequences:** Fewer deps; surface fully documented in `--help`. Add clap later only if ergonomics demand it (recorded reason).
- **Status:** final

## D-S02-4 — Source model: bytes + precomputed line-starts, byte columns
- **Date:** 2026-11-05
- **Decision:** `SourceFile` stores UTF-8 bytes and a `Vec<offset>` of line starts; spans are half-open byte ranges `[start,end)`; locations use 1-based line and 1-based *byte* column. Snippets trim trailing `\r`.
- **Basis:** S02-T03 (file IDs, line starts, UTF-8, span offsets) + need for stable spans through all front-end stages (§2.2).
- **Consequences:** Byte columns can differ from grapheme/column display for wide emoji; adequate for diagnostics today. Unicode-identifier policy documented in SPEC.
- **Status:** final

## D-S03..S18 — Incremental, dependency-gated feature rollout (default posture)
- **Date:** 2026-11-05
- **Decision:** Follow §3 section order S00→S18 with dependency gates; advance language features only after their semantic prerequisites exist (e.g., shape-sharing after generics+layouts+C backend; hot-patching after component ABI + ownership + quiescence). Research performance claims stay hypotheses until measured (§1.3); unrunnable benchmarks recorded `NOT RUN` with a reproducible recipe.
- **Status:** provisional (revisit when each milestone gate is reached)
