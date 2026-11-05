# Dust — Feature Status Matrix

Legend: `[x]` implemented & verified · `[~]` implemented, not yet fully verified · `[D]` designed/recorded · `[E]` experimental · `[-]` deferred · `[ ]` unsupported/planned.

| Capability | Status | Notes / Evidence |
|---|---|---|
| Cargo workspace + dustc crate | [x] | builds clean; `cargo build/test -p dustc` green |
| `dustc --version` / `--help` | [x] | verified via binary smoke tests |
| Source loader (IDs, line starts, UTF-8, spans) | [x] | 8 unit tests pass |
| Structured diagnostics + terminal rendering w/ snippets | [x] | 3 integration tests pass |
| Lexer (identifiers/keywords/literals/comments/operators) | [ ] | TODO — S03 pending |
| Parser + AST + recovery | [ ] | TODO — S04 pending |
| Name resolution + type checker | [ ] | TODO — S05 pending |
| Ownership / affine / regions / escape analysis | [D] | designed in SPEC; impl S06 |
| HIR / DIR IR | [ ] | TODO — S07 |
| C11 backend + native exe via clang | [ ] | TODO — S08 (path to M3) |
| Runtime profiles core/alloc/std, C FFI | [D] | designed; impl S09 |
| Generics / shape polymorphism / comptime | [ ] | TODO — S10 (post types+backend) |
| Effects / capabilities / typestate | [ ] | TODO — S11 |
| Concurrency / cross-thread ownership | [ ] | TODO — S12 |
| Content-addressed modules / Micro-AOT / hot-patch | [ ] | TODO — S13 (advanced; gated) |
| CLI project mgmt, manifests, packages | [D] | designed; impl S14 |
| Formatter / LSP / dev tools | [ ] | TODO — S15 |
| Cross-target / embedded | [ ] | TODO — S16 |
| Fuzzing / safety validation / benchmarks | [ ] | TODO — S17 (benchmarks = hypotheses) |
| Docs / CI / packaging / release / bootstrap | [D] | designed; impl S18 |

## Language feature table (SPEC cross-ref)
Implemented subset so far: source loading, diagnostics. Full language surface defined in `SPEC.md`; only features with `[x]`/`[~]` above are real. No designed feature is labeled implemented.
