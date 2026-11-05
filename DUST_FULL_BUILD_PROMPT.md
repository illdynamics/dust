# DUST — Full Language Build Specification and Execution Mandate

**Document type:** Master engineering prompt  
**Project:** Dust programming language  
**Scope:** Design, implement, test, document, package, and prepare a new systems programming language from an empty repository to a credible, usable release  
**Authority:** This document is the engineering brief. The repository's `SPEC.md`, `DECISIONS.md`, `ARCHITECTURE.md`, `TODO.md`, and tests become the durable, version-controlled implementation authority once created.

---

# 0. Mission, identity, and non-negotiable operating rules

You are the principal language designer, compiler engineer, runtime engineer, test engineer, and release engineer for a new programming language named **Dust**. Build Dust from scratch in the supplied working repository. Do not merely write a plan, scaffold empty files, or describe code that ought to exist. Inspect the environment, create real source files, implement the compiler and runtime in dependency order, run the actual build and tests, repair failures, and continue until the achievable work is exhausted or a genuine blocker is reached.

The goal is to get as far as possible toward a coherent, working language and toolchain. A small compiler that actually compiles, links, and runs real Dust programs is more valuable than a huge collection of unimplemented abstractions. Build the vertical slice first, then extend it without breaking it.

## 0.1 What Dust is

Dust is a new native systems language aimed primarily at constrained and edge devices: microcontrollers, IoT equipment, sensors, wearables, phones, embedded Linux systems, and other resource-sensitive machines. It should combine:

- Native ahead-of-time compilation and predictable execution.
- Small code and runtime footprints where measured and achievable.
- A memory model based on ownership, inferred lexical regions, escape analysis, and explicit safe fallbacks rather than a tracing garbage collector as the default.
- Affine resource handling by default, with opt-in linear guarantees for resources that must be consumed exactly once.
- Shape-aware generic compilation to reduce unnecessary code duplication without pretending that all equal-sized types are interchangeable.
- A staged compile-time facility (`comptime`) for deterministic specialization and compile-time computation.
- Explicit capabilities and effect-checked modular interfaces for hardware, I/O, and other side effects.
- A path to native modular components, content-addressed definitions, and typed hot-patching on platforms that can support them safely.
- C interoperability and practical embedded toolchain integration.

Dust is **not** an attempt to copy an existing language or an existing project named Dust. Do not clone, adopt, or silently base this implementation on `DustLanguage/Dust`, `kadir014/Dust`, Dust DPL, or another same-named project. They may be used as external reference material only when relevant and explicitly recorded. This is a new language with its own specification, compiler, runtime, CLI, tests, and roadmap.

## 0.2 Primary engineering priorities

Use this priority order when trade-offs arise:

1. **Correctness and safety:** defined language semantics, sound compiler checks, no silent miscompilation, no unverified claims of memory safety.
2. **A working end-to-end vertical slice:** source file → diagnostics → typed representation → generated native program → execution with correct output.
3. **Predictable resource behavior:** explicit allocation and effects, bounded core runtime, no hidden tracing GC or surprise heap use in the core profile.
4. **Portability:** host development first, then cross-compilation and embedded targets through a small backend/runtime boundary.
5. **Code size and performance:** measure against C and Rust equivalents; never claim superiority without reproducible benchmarks.
6. **Modularity and dynamic deployment:** add effects, capabilities, components, and hot-patching only after the stable core and ABI exist.
7. **Ergonomics and ecosystem:** documentation, formatter, package workflow, editor support, examples, CI, and releases.

The phrase “better than Rust” is not an acceptance criterion. The measurable thesis is that Dust should offer a useful combination of predictable memory, compact native output, embedded portability, and modularity. It may outperform competitors on selected benchmarks; it must not claim universal superiority.

## 0.3 Absolute rules for execution

- Work in the actual current workspace. Inspect `pwd`, directory contents, repository status, installed toolchains, and existing project files before changing anything.
- Preserve existing user work. Do not overwrite an unrelated project, delete untracked files, reset branches, clean directories destructively, or force-push.
- If the current directory is an empty or clearly intended Dust workspace, use it. If it is non-empty and unrelated, create a clearly named `dust-lang/` subdirectory rather than modifying unrelated files.
- Do not run `sudo`, modify system configuration, publish packages, create public releases, push to remotes, or install system-wide dependencies without explicit authorization.
- Do not invent command results, benchmark numbers, CI status, hardware tests, or completed features. Report exactly what ran and what did not.
- Do not create placeholder modules that are presented as implemented. A stub must be explicitly marked `TODO`, return a clear unsupported diagnostic, or be excluded from the supported language surface.
- Do not silently weaken safety checks to make tests pass. Add a regression test for every bug fix.
- Keep dependencies minimal. Prefer standard libraries and installed toolchains. Add a dependency only when it has a clear benefit, a maintained version, a compatible license, and a recorded reason.
- Do not block on non-essential uncertainty. Choose a conservative, reversible default, record it in `DECISIONS.md`, and continue. Ask the operator only when permission, data loss, external publication, or an irreversible choice is genuinely required.
- Do not attempt every advanced feature in one coding burst. Follow the phases and dependency graph in this document.

## 0.4 Required persistent project-control files

Create and maintain these files early. They are not optional project-management decoration; they are how the work survives context limits and model restarts.

- `TODO.md` — canonical hierarchical task list with task IDs, dependencies, acceptance tests, and statuses.
- `AGENT_STATE.md` — concise current-state dashboard: project phase, last verified commit/hash if available, passing commands, active task, blockers, and next exact action.
- `NEXT_SESSION.md` — compact handoff that another model instance can use without rereading the entire prompt.
- `DECISIONS.md` — dated architecture and language decisions, alternatives considered, rationale, consequences, and whether a decision is provisional.
- `SPEC.md` — authoritative user-visible language specification, including syntax and semantics actually implemented or deliberately planned.
- `ARCHITECTURE.md` — compiler, IR, runtime, backend, and module architecture.
- `STATUS.md` — implemented / partial / planned / experimental / unsupported feature matrix.
- `CHANGELOG.md` — user-visible changes.
- `docs/` — detailed design, user guide, internals, examples, and release notes.

Never rely on conversational memory as the sole record of project state. Keep `AGENT_STATE.md` short enough to read first (target under 150 lines). `TODO.md` is the canonical task ledger; do not create competing task lists with inconsistent status.

---

# 1. Design inputs and interpretation of the research

The design inputs describe a distinctive combination of four main pillars and supporting features. Treat them as design goals and research hypotheses, not as proof that the proposed implementation is already solved.

## 1.1 Core pillars

### A. Shape-polymorphic code generation

The goal is to avoid generating a complete copy of a generic function for every concrete type when multiple instantiations can safely share code. Types may be grouped by a representation/ABI shape, and a shared body may receive compact type-specific operation witnesses or layout descriptors where required.

A candidate shape signature may need to include more than size and alignment. Consider at least:

- Size and alignment.
- Target ABI classification and register passing class.
- Field offsets and aggregate layout when fields are accessed.
- Pointer/reference representation and provenance rules.
- Drop/destructor behavior and ownership class.
- Whether operations require type-specific behavior such as equality, hashing, formatting, or destruction.
- Enum/tag representation and niche/layout rules.

Two types with the same byte size are **not automatically interchangeable**. A correct shared body must preserve each type's semantics. Use explicit witnesses/descriptors or keep separate specialized bodies when required. Specialization may be retained as an escape hatch when it materially improves hot code. Compare binary size and runtime performance empirically.

Do not implement shape polymorphism before the language has working types, generic semantics, layout calculation, and a reliable baseline backend. First support ordinary generic instantiation in a simple, correct way; then add sharing as an optimization.

### B. Region-inferred memory and escape handling

The intended ergonomic direction is to infer safe storage regions and lifetimes from lexical scope and control flow rather than requiring users to annotate every lifetime. The intended allocation hierarchy is:

1. Static storage for constants and static data.
2. Stack/local storage where lifetime and escape rules prove it safe.
3. Lexical regions/arenas with bulk reclamation for temporary allocations.
4. Generational references for selected objects that escape a local region but remain within a controlled single-threaded runtime.
5. Explicit atomic reference-counted ownership for cross-thread sharing when required and supported.
6. No mandatory tracing garbage collector in the embedded core profile.

Affine values may be consumed at most once and may be dropped; linear values must be consumed exactly once. Hardware handles, DMA buffers, sockets, locks, and other resources are candidates for affine/linear capabilities.

This memory model is a major language research problem. Implement a conservative, explainable subset first. If the compiler cannot prove an escape safe, it must reject the program with an actionable diagnostic or require an explicit safe ownership/promotion construct. It must never quietly leave a dangling reference. Do not claim all lifetimes can be inferred in all programs.

Treat reference counting as a real runtime cost: atomic increments/decrements consume instructions and may require platform support. Treat generational checks as real runtime cost and ensure generation wraparound cannot revive stale references. A region allocator must define alignment, capacity exhaustion, reset, nested region behavior, and destruction ordering.

### C. Effects, capabilities, and modularity

Effects describe what a function may do (for example I/O, allocation, clocks, sensor reads, packet transmission, or task spawning). A module declares required operations and receives implementations through an enclosing handler or explicit capability. This supports static effect checking, tests with mock handlers, and target-specific drivers.

Do not conflate three separate concepts:

- An **effect system** statically records and checks permitted side effects.
- An **effect handler** supplies an implementation for an effect.
- **Dynamic dispatch or hot-swapping** selects a different implementation at runtime.

For tiny targets, initially restrict effects to first-order/tail-resumptive, lexically scoped handlers that can lower to direct calls or simple function references. Do not implement arbitrary continuation capture, stack reification, or a heavyweight handler runtime unless there is a measured use case and an explicit target profile.

Capabilities represent explicit authority. A module receives only the capabilities passed to it. A capability representing exclusive hardware access should be non-copyable/affine or linear. Safe Dust code must not acquire arbitrary hardware or memory access from ambient global state. Keep unsafe FFI and raw hardware access narrow and explicit.

### D. Micro-AOT components and typed hot-patching

The long-term goal is independently built native components with:

- A stable, versioned ABI and explicit target/architecture metadata.
- Declared imports, exports, effects, and required capabilities.
- Content hashes for identity and dependency resolution.
- Cryptographic authenticity/signature verification for updates.
- Relocation and compatibility validation.
- A safe quiescence/activation protocol so code is not replaced while old stack frames or references remain active.
- Transactional/double-buffered update storage and rollback behavior.
- Platform-specific memory protection or software fault isolation where available.

A content hash identifies bytes/content; it does **not** authenticate the publisher. Signed updates need signatures and a trusted key policy. Native code loading is not automatically sandboxed. A microcontroller without a suitable MPU or SFI mechanism cannot safely isolate hostile native modules merely because they declare capabilities. In such cases disable untrusted dynamic native loading or use a restricted portable bytecode/runtime format.

Dynamic loading and patching are late-stage, opt-in features. Do not promise near-zero overhead or universal MCU support. The loader, ABI, memory model, and security protocol must be independently tested before the feature is called usable.

## 1.2 Supporting features to plan for

- Hygienic and deterministic `comptime` / staged programming.
- Explicit integer semantics: checked/trapping arithmetic by default, with named wrapping and saturating operations; no undefined behavior in safe Dust arithmetic.
- Explicit layout controls, alignment, packed/extern layouts, and SoA/DMA-oriented layout where supportable.
- `noheap` / bounded-allocation effects and static proof where possible.
- FFI with C ABI from the early usable compiler.
- Typestate for hardware setup and peripheral lifecycle.
- Optional actor-style concurrency with reference/capability rules and static allocation where suitable.
- Content-addressed packages and incremental compilation.
- Optional interpreter/bytecode for tooling or portable modules; optional JIT only on sufficiently capable host/phone-class targets. Do not make a JIT a dependency of the MCU profile.
- Timing/energy annotations as measured estimates initially. Do not claim a worst-case execution-time or power proof unless the compiler performs the necessary analysis with documented assumptions.
- Tooling: diagnostics, formatter, test runner, package tool, documentation generator, language server, debugger integration, binary-size reporting, capability audit, and cross-compilation support.

## 1.3 Claims that must remain hypotheses until measured

The research includes aspirational figures such as 10–30% throughput gains, 50–70% binary reductions, under-5-ms hot patches, and close-to-C assembly parity. These are not guaranteed targets. Record them in a benchmark hypothesis table with:

- The workload and exact source revision.
- Target CPU, board, compiler/backend version, flags, and linker.
- Equivalent C and Rust implementations and optimization settings.
- Flash/text, rodata, data, BSS, stack, heap, cycle count, latency, and energy if measurable.
- Repeated-run method and error/variance.
- Whether the comparison is fair and equivalent in functionality/safety.

Never fabricate or hard-code a claimed result. A benchmark that cannot be run on the current machine must be marked `NOT RUN` and supplied with a reproducible recipe.

---

# 2. Recommended implementation strategy and toolchain

## 2.1 Bootstrap architecture

Unless repository inspection reveals a compelling, documented reason to change it, use this pragmatic bootstrap strategy:

- Implement the first Dust compiler (`dustc`) in **stable Rust**, as a host tool. Rust is the compiler implementation language, not Dust's language definition.
- Use Cargo workspaces and Rust's standard library initially. Add crates only when they materially improve correctness or reduce code.
- Make the first complete backend emit readable, deterministic **C11** plus a small Dust runtime. Then invoke an installed `clang` or `gcc` toolchain to produce native executables and object files.
- Keep the backend behind a clean interface so a direct LLVM IR backend or another native backend can be added later without rewriting the front end.
- Avoid an LLVM binding dependency in the first vertical slice. LLVM/Cranelift can be evaluated after the language and typed IR exist. If an LLVM backend is added, pin and document compatible versions and keep the portable C backend working.
- The first supported execution target is the host architecture/OS actually available in the workspace (typically Linux x86_64). Add ARM64, Windows, macOS, Cortex-M, RISC-V, ESP32, or Xtensa only when the relevant C compiler, linker, SDK, ABI, and runtime are available and tested.

This is a default, not a religion. If Rust is absent and cannot be installed without privileged/system changes, do not fake its availability. Record the blocker and either use an already-installed suitable host language/toolchain or provide the smallest justified setup requirement. Do not make an unreviewed pivot midway through implementation.

## 2.2 Backend and runtime boundary

The first compiler pipeline should be conceptually:

`Dust source → lexer → AST/parser → name resolution → type/effect/ownership checks → typed HIR → Dust IR (DIR) → lowering/optimization → C11 backend → C compiler/linker → executable`

Keep source spans and stable symbol/type IDs through all front-end stages so diagnostics and debugging remain useful.

Generated C must be deterministic, readable enough to debug, and conservative about C undefined behavior. Use fixed-width integer types from `<stdint.h>`, explicit overflow handling, explicit layout assertions where needed, safe symbol mangling, and target ABI-aware declarations. Do not assume C `long` has the same width on every target.

The runtime should be split by profile:

- `core`: no OS dependency, no mandatory heap, minimal panic/trap and memory primitives.
- `alloc`: optional allocator and containers.
- `std`: host/OS facilities such as output, files, clocks, and process interfaces.
- `embedded` / platform modules: target-specific HAL support supplied through explicit capabilities/effects.

A freestanding program must not accidentally link the host standard library. Dead code and unused runtime handlers should be removable by the backend/linker.

## 2.3 Workspace layout target

Adapt to the actual repository, but aim for a structure comparable to:

```text
.
├── Cargo.toml
├── rust-toolchain.toml                 # only if pinning is justified
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CHANGELOG.md
├── SPEC.md
├── ARCHITECTURE.md
├── DECISIONS.md
├── TODO.md
├── AGENT_STATE.md
├── NEXT_SESSION.md
├── STATUS.md
├── crates/
│   ├── dustc/                           # compiler and driver
│   │   └── src/
│   │       ├── main.rs
│   │       ├── cli.rs
│   │       ├── source.rs
│   │       ├── diagnostics.rs
│   │       ├── lexer/
│   │       ├── parser/
│   │       ├── ast/
│   │       ├── resolve/
│   │       ├── types/
│   │       ├── ownership/
│   │       ├── hir/
│   │       ├── ir/
│   │       ├── passes/
│   │       ├── codegen/
│   │       │   └── c11/
│   │       └── driver/
│   ├── dust-runtime/                    # headers and C runtime sources
│   └── dust-test/                       # optional integration harness
├── library/
│   ├── core/
│   ├── alloc/
│   ├── std/
│   └── platform/
├── tests/
│   ├── lexer/
│   ├── parser/
│   ├── typecheck/
│   ├── ownership/
│   ├── compile-pass/
│   ├── compile-fail/
│   ├── runtime/
│   ├── ffi/
│   ├── fuzz/
│   └── benchmarks/
├── examples/
├── scripts/
├── docs/
│   ├── design/
│   ├── user-guide/
│   ├── internals/
│   ├── reference/
│   └── release/
└── .github/workflows/
```

Do not create every directory empty on day one. Add directories when their subsystem begins.

---

# 3. Dependency-ordered master build phases

The project must be divided into small, verifiable work packages. The section IDs below are stable references for `TODO.md` and `DECISIONS.md`. Each section is a subsystem specification, not an instruction to implement it all at once.

## Section S00 — Agent operating contract and repository safety

**Deliverables:** repository inspection report; project-control files; initial task DAG; toolchain inventory; safe worktree selection.

Tasks:

1. Inspect current working directory, parent, repository root, `git status`, existing files, and any existing build scripts.
2. Identify available `rustc`, `cargo`, `clang`, `gcc`, `cmake`, `make`, Python, linker, and target toolchains. Record versions and actual availability.
3. Identify whether the worktree is empty, already a Dust project, or unrelated.
4. Create the control files from Section 0.4 without destroying existing work.
5. Build a complete hierarchical TODO with IDs, prerequisites, acceptance criteria, verification commands, and initial status.
6. Mark unknown hardware/toolchains as unverified rather than assuming them.

**Gate:** the project root is safe, the plan is durable, and the next task is unambiguous.

## Section S01 — Requirements, language identity, and specification freeze

**Deliverables:** `SPEC.md`, `DECISIONS.md`, a feature-status matrix, a syntax sketch, a compatibility policy, and a list of unresolved research questions.

Define before substantial code:

- Language goals, non-goals, supported profiles, and target classes.
- Source file extension `.dust`, module/import rules, identifiers, comments, literals, keywords, punctuation, and operator precedence.
- Expressions versus statements, block and return semantics, function syntax, type syntax, and declaration visibility.
- Primitive types and integer overflow semantics.
- Structs, enums, arrays, slices, strings, options/results, and layout guarantees.
- Mutability, ownership/move semantics, Copy semantics, references, region scope, escape rules, destructors, and unsafe boundary.
- Function calling convention, C ABI, symbol mangling, target data layout, and integer/pointer widths.
- Effects and capability syntax as a planned extension with a documented minimal semantics.
- Generic semantics and future shape-sharing strategy.
- Error behavior, panic/trap behavior, and whether exceptions exist (prefer explicit `Result`/`Option`; do not add hidden exceptions without a reason).
- Module/package semantics, versioning, and future component ABI.

Use the conceptual examples from the research only as sketches. For example, `capability I2CBus`, `module TemperatureSensor`, `perform ReadSensor()`, and `comptime { ... }` express the intended direction but are not a frozen grammar. Choose a small coherent syntax and use it consistently in all examples, tests, and docs.

Create a language feature table with these statuses: `implemented`, `partially implemented`, `designed`, `experimental`, `deferred`, `unsupported`. Never label a merely designed feature as implemented.

**Gate:** the first language subset and its semantics are written down. No parser/codegen may invent syntax ad hoc; changes update the spec and decision log.

## Section S02 — Project scaffold, build system, diagnostics foundation

**Deliverables:** buildable compiler executable skeleton, workspace, source-file abstraction, diagnostic model, CI starter, version output, and smoke test.

Tasks:

1. Create the Cargo workspace and compiler crate.
2. Add a compiler binary that supports `--version` and `--help`.
3. Add source loading with file IDs, line starts, UTF-8 handling, and span offsets.
4. Add structured diagnostics with severity, code, message, primary span, labels, and notes.
5. Add stable, human-readable diagnostics with source snippets and line/column numbers.
6. Add unit-test conventions and a smoke test.
7. Add formatting/lint commands and minimal CI for the host toolchain.

**Gate:** clean build, help/version commands, and tests pass from a fresh checkout using documented commands.

## Section S03 — Lexer and token model

**Deliverables:** tested lexer, token definitions, source spans, lexical error reporting, and token-dump/debug command.

Implement incrementally:

- Identifiers and keywords.
- Decimal, hexadecimal, binary, and (if specified) octal integer literals with separators.
- Floating-point literals if in the initial grammar.
- String and character literals with escapes and clear malformed-literal diagnostics.
- Line and block comments; define nested block comment behavior.
- Operators and punctuation, including multi-character tokens with longest-match behavior.
- Whitespace, newlines, EOF, and invalid UTF-8 handling policy.

Tests must cover every token family, boundary cases, malformed numbers, unterminated strings/comments, Unicode identifiers policy, and source-span correctness. Lexer errors must not silently skip arbitrary input.

**Gate:** tokenization is deterministic and all lexical positive/negative tests pass.

## Section S04 — Parser, AST, grammar, and recovery

**Deliverables:** grammar document, AST types, parser, parse diagnostics, parser tests, and AST/tree dump.

Start with the smallest vertical-slice grammar:

- Module/file declarations and imports as specified.
- Top-level constants, functions, structs, and enums.
- Function parameters and return types.
- Blocks, `let`/`var`, assignment, expression statements, and return.
- Literals, variable references, calls, unary/binary operators, and grouping.
- `if` and `while`/`loop` only when the grammar is clear.
- Struct field access and construction when types are ready.

Then extend with `match`, arrays/slices, generic syntax, attributes, capabilities, effects, and comptime in their dependent sections.

Use a parser architecture suitable for useful diagnostics. Add error recovery at declaration/statement boundaries so one syntax error does not create an avalanche of nonsense diagnostics. Define operator precedence and associativity in one place and test it.

**Gate:** representative valid programs parse into stable ASTs; malformed input produces precise diagnostics without panics.

## Section S05 — Name resolution and core type checker

**Deliverables:** scopes/symbol tables, module resolver, typed AST/HIR, type rules, and compile-fail tests.

Implement:

- Lexical scopes and shadowing rules.
- Function and type declarations, forward references if supported, and visibility.
- Duplicate declaration and unresolved name diagnostics.
- Primitive types, function signatures, parameter/return checking, and call arity.
- Expression typing, assignment compatibility, mutability, and operator constraints.
- Struct/enum declaration and construction checks.
- Explicit casts/conversions with well-defined narrowing and signedness rules.
- Return-path and control-flow checks.
- Type inference only where deterministic and tractable (e.g. local bindings); keep public APIs and function signatures explicit initially.

Prefer a small, explainable type system over a half-implemented trait solver. Do not implement traits/interfaces/type classes until there is a concrete, specified need. Add a negative test for every type error category.

**Gate:** well-typed programs pass; ill-typed programs fail before code generation with stable error codes and useful spans.

## Section S06 — Ownership, affine/linear resources, regions, and escape analysis

**Deliverables:** written memory-model rules, ownership dataflow pass, borrow/alias checks, local region model, escape diagnostics, and memory-model tests.

The first implementation must be conservative and sound rather than ambitious and unsound.

### S06.1 Initial ownership subset

- Define `Copy` types narrowly (at least primitive scalar types; extend only when semantically valid).
- Non-Copy values move on assignment/argument passing/return unless an explicit borrow is used.
- Affine values may be used zero or one times; a moved value cannot be used again.
- Linear values must be consumed exactly once at all exits, with an explicit safe disposal operation where appropriate.
- Track initialization state across branches and loops (initialized, moved/uninitialized, maybe initialized).
- Define drop order, destructor behavior, and panic/trap cleanup semantics before relying on destructors for safety.
- Ensure branching cannot duplicate an affine/linear resource.

### S06.2 References and aliasing

- Immutable borrows may coexist when compatible.
- A mutable borrow is exclusive for its valid region.
- Mutation and moves that conflict with active borrows must be rejected.
- References cannot outlive the value or region they refer to.
- No safe reference may be fabricated from an integer or arbitrary raw pointer.
- Raw pointers and pointer dereference remain inside an explicit `unsafe` boundary until their full semantics are specified.

### S06.3 Region inference

Begin with lexical, intraprocedural control-flow analysis. Compute a conservative region/lifetime bound for each local allocation/reference. Reject values that escape a region unless the chosen representation explicitly supports that escape.

Initially allow only provable stack/static/lexical-region cases. Later add interprocedural inference with bounded summaries; do not claim unrestricted whole-program inference. Add explicit source-level annotations only where inference cannot decide safely, and document their cost.

### S06.4 Generational and shared fallbacks

Only after the initial model is stable:

- Define a generational handle as a slot/index plus generation (or another formally justified representation), not merely an unchecked raw pointer plus counter.
- Validate generation on every access path and define slot reuse and generation wrap policy.
- Ensure allocation metadata cannot be forged or corrupted by safe code.
- Add optional single-threaded generational storage for escape cases.
- Add atomic reference counting only for explicitly shared objects and only when the target supports the required atomic operations. Define memory ordering, cycle/leak behavior, and counter overflow.
- Never silently promote every escaping allocation to atomic RC. Promotions must be visible in diagnostics/IR/profiling and cost documentation.

### S06.5 Validation

Test use-after-move, double consumption, missing linear consumption, branch merge, loop behavior, mutable aliasing, escaping locals, returning references, nested regions, early returns, error paths, drop order, and generational stale-handle access. Include adversarial negative tests and property/fuzz tests for the checker where feasible.

**Gate:** the implemented subset is described accurately; no known safe-language path can create a dangling reference or duplicate an affine capability. Do not claim a formal proof unless one exists.

## Section S07 — HIR, Dust IR (DIR), and compiler pass architecture

**Deliverables:** typed high-level IR, lower-level typed/SSA-like DIR, verifier, pass manager, dumps, and IR tests.

Keep AST separate from semantically typed representation. Preserve type, ownership, effect, region, debug-span, and target-layout information until the relevant lowering pass has consumed it.

Define:

- Basic blocks and terminators.
- Explicit control flow and return paths.
- Typed operands and values.
- Stack slots, region allocation operations, and memory effects.
- Calls, function symbols, aggregate construction/access, and control-flow operations.
- Explicit checked arithmetic/traps.
- Debug/source locations.
- Pass invariants and verifier checks.

Add a DIR verifier and run it after every transformation in debug/test configurations. Start with minimal, obviously correct passes: constant folding with defined integer semantics, unreachable block removal, simple dead-code elimination, and copy propagation. Do not add unsafe alias-based optimizations until provenance/alias rules are formalized.

**Gate:** all valid core programs lower to verified IR; verifier rejects malformed IR in its own tests.

## Section S08 — C11 backend and native executable generation

**Deliverables:** deterministic C11 emitter, target description, generated-C tests, build/link driver, and end-to-end native programs.

Implement in small stages:

1. Map every supported Dust type to a defined C representation.
2. Emit functions, globals, locals, basic blocks, branches, calls, structs, enums, and returns.
3. Generate stable, collision-resistant C identifiers and a documented Dust symbol-mangling scheme.
4. Emit exact-width integer types and explicit arithmetic checks/traps according to Dust semantics.
5. Emit layouts with `_Alignas`, `offsetof`, and `_Static_assert` where appropriate and supported.
6. Emit calls to the minimal Dust runtime for traps, regions, and other supported operations.
7. Invoke the installed target C compiler/linker using safe process argument APIs; never build shell commands by concatenating untrusted source text.
8. Support debug and optimized output, intermediate C retention, object output if feasible, and useful error forwarding from the C compiler.

Never emit C constructs whose behavior depends on undefined behavior. Test signed overflow, shifts, division by zero, narrow/wide conversions, alignment, and target-dependent widths. C code generation must not claim a Dust safety guarantee that the lowering has not preserved.

**Gate:** compile and run real Dust `hello`, arithmetic, function, branch, struct, and enum examples on the available host. Compare actual output and exit codes.

## Section S09 — Runtime, standard library, C FFI, and embedded profile boundaries

**Deliverables:** small runtime, `core`/`alloc`/`std` separation, C FFI examples, allocator interface, panic/trap path, and embedded profile documentation.

Implement a minimal usable core:

- Defined integer and boolean operations.
- Basic aggregate support.
- Explicit panic/trap behavior and exit status.
- Minimal string-literal representation and output interface for host examples.
- Optional allocator interface, not a hidden always-on allocator.
- Region arena primitive with alignment, bounds, exhaustion behavior, and reset semantics when the language feature is implemented.
- C ABI declarations and calls with explicit safety contracts.

C FFI requirements:

- Document calling convention, data layout, symbol naming, and ownership transfer.
- Make FFI declarations explicit and distinguish safe wrappers from unsafe foreign calls.
- Never assume C strings, pointers, or lifetimes are safe without an explicit wrapper contract.
- Provide at least one example calling a small C function and one C-callable Dust export when feasible.

Profiles:

- `host`: OS process, console, files, standard allocator as explicitly selected.
- `freestanding`: no OS startup assumptions; user-provided entry point/runtime hooks.
- `embedded`: no mandatory heap, target-specific startup/linker script/HAL supplied explicitly.

**Gate:** documented host program works; the `core` profile does not pull in host-only facilities; FFI tests pass on the host.

## Section S10 — Generics, shape polymorphism, layout synthesis, and comptime

**Deliverables:** generic specification, baseline generic implementation, shape signature/witness model, measured code-sharing pass, and restricted comptime evaluator.

### Generics baseline

Implement the simplest correct generic model first. It may initially monomorphize or use explicit dictionary/witness passing. Generic constraints and operations must be checked statically. Do not let code sharing change program semantics.

### Shape polymorphism

- Define shape signatures from the target ABI and semantic requirements, not just `sizeof`.
- Separate representation compatibility from operation semantics.
- Use witnesses/descriptors for type-specific drop, equality, hashing, formatting, field access, or other operations when required.
- Decide whether fields are directly accessed by compile-time offset, descriptor offset, or specialized code; document the trade-off.
- Preserve specialization for cases where constant propagation/inlining/layout knowledge materially matters.
- Cache compiled instances by stable shape key and generic body identity.
- Make debug info and diagnostics identify both the shared implementation and concrete instantiation.
- Test multiple semantically distinct types with identical layout, types with different alignment, nested aggregates, zero-sized types if supported, enums, destructor-bearing values, and different target ABIs.

Measure code size and runtime impact against the baseline generic implementation. A size win that adds unacceptable runtime overhead must be documented rather than hidden.

### Comptime

Begin with deterministic compile-time constants, conditional target selection, and bounded evaluation. Later add limited function evaluation and type/layout reflection. Define a fuel/step limit, recursion limit, allowed operations, deterministic integer/string semantics, and memory limit. No network, filesystem, arbitrary process execution, or nondeterministic host access from comptime. Separate compile-time capabilities from runtime capabilities.

### Layout controls

Add explicit representation attributes only with clear target rules: `repr(C)`, alignment, packed layout, explicit field order, and DMA/SoA hints where they can be validated. Reject impossible or ABI-incompatible requests.

**Gate:** generic tests pass; shape sharing is demonstrably semantics-preserving and measured; comptime is deterministic and bounded.

## Section S11 — Effects, explicit capabilities, and hardware typestate

**Deliverables:** effect declarations/types, lexical handlers, effect checking, capability declarations, affine/linear capability checking, test mocks, and platform abstraction examples.

Implement in dependency order:

1. Effect names and operation signatures.
2. Function effect declarations/inferred effect sets.
3. Check that every performed effect is permitted by the enclosing function/module.
4. Lexically scoped handlers with a narrow, first-order/tail-resumptive semantics.
5. Lower handlers to direct calls or explicit function references; record generated overhead.
6. Capability declarations as nominal, non-forgeable types.
7. Capabilities are passed explicitly and cannot be copied when exclusive.
8. Trusted constructors live in platform/runtime/unsafe code; safe general code cannot invent a hardware token.
9. Mock handlers for tests and host simulation.
10. Typestate transitions for peripheral setup/use/shutdown where the type system can prove valid sequences.

Keep effect checking distinct from runtime module replacement. Start with static handler selection. Dynamic driver swapping is only added after ABI and runtime support exist.

**Gate:** an example sensor/serial/GPIO interface can be tested with a mock and compiled with a static handler. A module lacking the required capability or effect handler is rejected.

## Section S12 — Concurrency, actors, and cross-thread ownership

**Deliverables:** concurrency design, send/share rules, optional task/actor runtime, and race/ownership tests.

Do not introduce concurrency before ownership and capability semantics are stable. Define:

- Which targets support threads/tasks/interrupt contexts.
- Whether shared mutation is prohibited, synchronized, or actor-mediated.
- Transfer semantics for affine values and capabilities.
- Requirements for `Send`/`Share`-like traits or equivalent compiler markers.
- Memory ordering and atomic requirements.
- Static task/actor allocation for constrained systems.
- Optional richer runtime scheduler on host/phone-class targets.

Avoid claims of lock-free or data-race-free behavior unless the compiler enforces all relevant rules. Cross-thread promotion to atomic RC must be explicit in the IR and cost report. If the model cannot verify a pattern, reject it or require an explicitly unsafe boundary.

**Gate:** supported concurrency examples pass race-oriented tests and unsupported target/thread models fail clearly.

## Section S13 — Content-addressed modules, Micro-AOT, and typed hot-patching

**Deliverables:** component format specification, hash pipeline, manifest verifier, static component linker, host-side loader prototype if feasible, and a security threat model.

Implement this late, in stages:

1. Define normalized source/typed-IR hashing and versioning. Do not hash unstable debug paths or nondeterministic metadata into semantic identity.
2. Use a standard, well-reviewed cryptographic hash for content identity; pin the algorithm and format version. Hash identity alone is not publisher authentication.
3. Define component metadata: target triple, architecture, ABI version, compiler version range, imports, exports, effect manifest, capability requirements, memory requirements, relocation model, and dependencies.
4. Implement static component packaging/linking first.
5. Add signature verification and trusted-key policy for updates. Use a maintained cryptographic implementation; do not invent cryptography.
6. Define a relocation model and validate all relocation records before activation.
7. Define safe quiescence: no active frames or stale handles may refer to the old implementation when its storage is reclaimed.
8. Define transactional update slots, atomic activation, rollback, anti-rollback policy where needed, and flash wear considerations.
9. Enforce MPU isolation only on verified target hardware with configured regions; otherwise use documented SFI or prohibit loading untrusted native code.
10. Add host simulation tests before any physical embedded loader work.

Native hot-patching must be disabled by default until all verification conditions pass. Never execute an unverified component during tests. Treat malformed manifests and relocation tables as hostile input.

**Gate:** only call static packaging complete when reproducible tests pass; only call hot-patching experimental/usable with explicit platform scope and a documented security audit checklist.

## Section S14 — CLI, project manifests, packages, and dependency management

**Deliverables:** stable command-line interface, `dust.toml`, lockfile policy, workspace/module resolution, local package workflow, and user-facing errors.

Recommended commands (adapt names consistently):

- `dust --version`, `dust --help`
- `dust new <name>`
- `dust check <file-or-project>`
- `dust build <file-or-project> [--target ...] [--release]`
- `dust run <file-or-project>`
- `dust emit-c <file-or-project>`
- `dust test [filter]`
- `dust fmt [--check]`
- `dust doc`
- `dust package` / `dust add` only when package semantics are specified
- `dust doctor` to diagnose installed build tools and target support

Create a simple, versioned manifest format. Resolve local paths safely, reject path traversal, define deterministic dependency resolution, and provide a lockfile before remote dependency fetching. Begin with local/path dependencies. Remote registry publishing is a later, explicitly authorized activity.

**Gate:** a new project can be created, checked, built, run, tested, and documented with clear help output.

## Section S15 — Developer tooling and experience

**Deliverables:** formatter, syntax-aware diagnostics, LSP, debug metadata, test runner, and package/docs UX as feasible.

Order:

1. Formatter that parses valid syntax and does not corrupt comments/strings.
2. Golden diagnostic tests and clear error explanations.
3. Compiler `--emit`/dump modes for AST, HIR, DIR, generated C, and timings.
4. Debug symbols/source maps where supported by the C backend/toolchain.
5. Language Server Protocol implementation only after parser and diagnostics APIs are stable. Begin with document sync, diagnostics, completion for local identifiers, and go-to-definition.
6. Debugger integration guidance based on the generated native binary and supported host debugger.
7. Binary size and optimization report command.
8. Capability/effect audit output for embedded projects.

Do not spend early effort on an elaborate IDE or GUI. Prioritize reliable command-line workflows and editor interoperability.

**Gate:** formatter tests pass and basic editor diagnostics work if LSP is implemented.

## Section S16 — Targets, cross-compilation, linker integration, and embedded support

**Deliverables:** target descriptor model, target capability matrix, cross-compilation recipes, linker integration, and at least one verified embedded sample when hardware/toolchains permit.

Target progression:

1. Host target detected in the environment.
2. Linux x86_64 host build (if available).
3. Linux AArch64 / macOS / Windows host builds in CI as runners and toolchains permit.
4. ARM64 Linux cross-compilation.
5. ARM Cortex-M bare-metal C toolchain and startup/linker integration.
6. RISC-V embedded target when toolchain and runtime details are known.
7. ESP32/Xtensa only with the actual supported SDK/toolchain and ABI.

For each target record: triple, CPU, ABI, data layout, pointer width, endianness, C compiler, linker, runtime/startup, linker script, standard-library profile, supported atomics, MPU/SFI capabilities, and test status.

Do not label a generic C-emitted program as “MCU supported” until the generated object/binary has been linked for a concrete target and run or otherwise validated. Cross compile is not the same as a successful hardware test.

**Gate:** every claimed target has reproducible build commands and an accurate support status.

## Section S17 — Testing, fuzzing, safety validation, performance, and benchmarking

**Deliverables:** layered automated tests, fuzz harnesses, benchmark suite, security notes, and truthful validation report.

### Required test layers

- Unit tests for lexer/parser/resolver/type checker/ownership/IR passes/codegen/runtime/CLI.
- Compile-pass fixtures for valid programs.
- Compile-fail fixtures for invalid programs, with expected error code/span/category.
- End-to-end compile/link/run tests with exact stdout/stderr/exit status.
- C FFI integration tests.
- Deterministic generated-output tests.
- IR verifier tests.
- Regression suite for every fixed bug.
- Property-based and fuzz testing for lexer/parser, manifest reader, component format, and selected compiler passes when tools are available.
- Sanitizer/Valgrind/ASan/UBSan runs where compatible with the host compiler.
- Cross-target compile tests and physical-board tests when hardware is available.

### Safety validation

Create a threat model and soundness review checklist for safe references, moves, aliasing, region closure, generational handles, atomic sharing, FFI, effects, capabilities, components, and hot-patching. Clearly distinguish:

- Tested behavior.
- Static guarantees implemented by the compiler.
- Assumptions made about external C and platform code.
- Features not yet formally verified.

Do not market the language as memory-safe until the implemented safe subset and its boundaries have been seriously reviewed. Keep an `UNSAFE.md` or equivalent inventory of unsafe blocks and assumptions in the compiler/runtime.

### Performance benchmarks

Start with microbenchmarks, then three representative workloads:

1. Bare-metal-style sensor fusion / numeric loop.
2. Network packet parse/validate/serialize.
3. Modular driver replacement or component dispatch when that phase exists.

Compare equivalent Dust, C99/C11, and Rust `no_std` or appropriate host versions. Keep optimization flags and functionality equivalent. Record compiler, linker, architecture, board, source revision, and all measured sections/sizes. Never optimize based solely on an isolated benchmark that changes functionality.

**Gate:** all required host tests pass; benchmark claims are reproducible; outstanding safety limitations are explicit.

## Section S18 — Documentation, CI, packaging, release, maintenance, and bootstrap path

**Deliverables:** complete developer/user documentation, CI matrix, reproducible artifacts, release checklist, and long-term roadmap.

### Documentation

Maintain:

- README with status, features, install/build/test commands, quick start, and limitations.
- Language reference derived from `SPEC.md`.
- Tutorial: Hello Dust, functions/types/control flow, errors, memory/ownership, FFI, and modules.
- Compiler architecture and contribution guide.
- Runtime and target-porting guide.
- Safety model, unsafe boundary, effect/capability model, and component security guide.
- Example programs and benchmark methodology.
- Changelog and migration/version policy.

Every code example must be compiled/tested by CI where feasible. Do not document unsupported syntax as current functionality.

### CI

Start with one reliable host job, then expand to Linux/macOS/Windows and x86_64/AArch64 only as actual runner availability and toolchains allow. CI should run formatting, static checks, build, unit tests, compile-pass/compile-fail, end-to-end tests, and docs checks. Add fuzz smoke tests and benchmark reporting when practical. Pin action versions and toolchain versions responsibly.

### Packaging

Produce a host release binary/archive first. Then add platform-specific packages (DEB/RPM/Windows archive/macOS archive) only after they can be installed and verified. Docker images are optional and not a substitute for native packages. Never publish to a registry without explicit authorization. Include version, source revision, target triple, checksums, and signatures where appropriate.

### Release discipline

Use semantic versioning after public compatibility policy exists. Until then use clearly labeled `0.x` milestones. Every release must include:

- Reproducible build instructions.
- Changelog and known limitations.
- Passing test report.
- Supported-target table.
- Artifact checksums.
- Security/safety status.

Do not declare `1.0.0` merely because a generic 12-week plan says so. Release readiness is earned by feature completeness, compatibility, tests, security review, and documentation.

### Bootstrap and self-hosting

Do not attempt self-hosting before the language is stable enough to express meaningful compiler code, the standard library supports required data structures/I/O, and the compiler can generate reliable native binaries. Keep the first compiler in Rust. Later, port a small independent tool or compiler subsystem to Dust, then consider a self-hosting compiler. Use differential tests and compare generated artifacts. Do not rewrite the compiler just to claim self-hosting.

**Gate:** a reproducible host release is available and honest; embedded/dynamic features are accurately labeled by target and maturity.

---

# 4. Milestones and acceptance gates

Use these milestones. Completion requires the gates, not a calendar estimate.

## M0 — Project initialized

- Safe repository chosen.
- Toolchain inventory recorded.
- Control files created.
- Specification outline and task DAG saved.

## M1 — Compiler skeleton

- `dustc --version` and `--help` work.
- Source loader and diagnostics work.
- Lexer and parser tests pass.

## M2 — Typed core

- Name resolution and type checking work.
- Compile-fail tests reject invalid programs.
- Ownership/region subset is documented and tested.

## M3 — First native Dust program

- Dust source emits C11.
- C compiler builds the generated output.
- Executable runs with expected output and exit code.
- End-to-end regression tests exist.

## M4 — Useful language alpha

- Functions, primitive types, structs/enums, control flow, core library, basic errors, and C FFI work as specified.
- CLI can create/check/build/run/test.
- Documentation examples compile.

## M5 — Safer embedded foundation

- Affine/linear values and local region checking have explicit semantics.
- `core`/`alloc`/`std` profiles are distinct.
- A concrete freestanding/embedded target can be built if the toolchain exists.

## M6 — Generic and compile-time features

- Generics work correctly.
- Shape polymorphism is measured against baseline and does not change semantics.
- `comptime` is bounded and deterministic.

## M7 — Effects and capabilities

- Effect checking and static handler lowering work.
- Capabilities are non-forgeable through safe Dust code.
- Mock and target examples compile and test.

## M8 — Modular component alpha

- Component format and manifests are documented and validated.
- Hashing/signature distinction is respected.
- Host-side package/loader prototype passes hostile-input tests.
- Native patching remains disabled for unsupported isolation targets.

## M9 — Toolchain release candidate

- Formatter, tests, CI, docs, installable host package, support matrix, benchmark report, and known limitations are present.
- No unresolved critical correctness issue is hidden.

The project may stop at any milestone if the environment or remaining effort runs out. The agent must leave the repository in a buildable state and record the exact unfinished frontier.

---

# 5. Required engineering workflow for every task

For each TODO item:

1. Read `AGENT_STATE.md`, the relevant `TODO.md` entry, the current `SPEC.md` rules, and only the relevant section of this prompt.
2. Inspect the actual files and callers before editing. Do not assume the repository has the structure you planned.
3. Write a one-paragraph task plan and identify dependencies, files to change, and acceptance tests.
4. Implement one bounded change. Avoid unrelated refactors.
5. Run the smallest focused test first, then the relevant test suite, formatter/linter, and build as appropriate.
6. If tests fail, identify the root cause and repair it. Do not mark the task done because code was written.
7. Add or update regression tests and documentation/specification when semantics change.
8. Update the task status only after verification. Record exact commands and concise results.
9. Update `STATUS.md`, `AGENT_STATE.md`, and `NEXT_SESSION.md` when the phase or next action changes. Keep `DECISIONS.md` current.
10. Continue automatically to the next ready task unless genuinely blocked.

Task statuses: `TODO`, `READY`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `VERIFIED`, `DEFERRED`. `DONE` means implementation exists; `VERIFIED` means the acceptance checks have actually passed. Only `VERIFIED` tasks count toward a completed milestone.

Every TODO task must contain:

- Unique ID (for example `S03-T04`).
- Short title.
- Dependencies (task IDs).
- Files or subsystem expected to change.
- Specific acceptance criteria.
- Exact verification command(s) or a clear manual verification procedure.
- Status and, when relevant, blocker reason.

Keep each work package small enough to implement and verify without rereading unrelated sections. If a task grows too large, split it into child tasks before proceeding.

---

# 6. Context-window management and durable handoff

The supplied local inference configuration has a context setting of 12,288 tokens and an output cap of 6,144 tokens. Treat the context as a scarce shared budget: conversation history, tool calls, source files, terminal output, and generated text all consume it. Do not assume the full master prompt can be loaded alongside the repository and still leave enough room to reason and implement.

- Never reread this entire file in one operation.
- Use the section IDs and heading boundaries. Locate a section by searching its heading, then read only that section and its immediate dependencies.
- Keep tool output narrow. Use `git status --short`, targeted `find`/`grep`, focused test output, and line ranges rather than dumping entire directories or huge logs.
- Process no more than one major section, or at most two tightly related short sections, in a single planning window.
- After each small work package, persist the result to files before moving on.
- Keep a compact `AGENT_STATE.md` and `NEXT_SESSION.md` so a fresh model can continue without loading the whole conversation.
- Before context becomes crowded, stop starting new tasks. Write exact handoff state: task just completed, files changed, commands run and results, known failures, current TODO statuses, and the next single task with its dependencies.
- Never mark incomplete work complete merely to make the handoff look clean.

The model serving configuration is execution context, not a language requirement. The project must remain tool/model agnostic. Do not reconfigure llama-server, expose the model server to the LAN, or change inference settings as part of Dust development.

---

# 7. Final reporting requirements

At the end of each work session, report briefly but concretely:

1. Current milestone and percentage/status estimate, with the basis for the estimate.
2. Tasks verified during this session.
3. Actual files created/changed.
4. Build and test commands run, including pass/fail counts when available.
5. Known bugs, safety limitations, unimplemented features, and blockers.
6. Whether any performance or hardware claims were actually measured.
7. The next exact TODO task ID and action.
8. Where `TODO.md`, `AGENT_STATE.md`, and `NEXT_SESSION.md` were updated.

Do not end with “everything is done” unless the relevant release gate has been verified. Do not ask the operator to restate information already in the repository. If a next action is available and safe, take it now.

---

# 8. Definition of success

A successful execution is not necessarily a complete research-grade language in one session. It is the maximum amount of **real, tested, coherent progress** possible while preserving a reliable path to completion.

The minimum meaningful result is a documented Dust subset with a working compiler that lexes, parses, type-checks, emits native code, builds, and runs examples; a useful test suite; a precise list of what is and is not implemented; and a durable task ledger that another model or engineer can resume.

The aspirational end state is a complete, documented, cross-target systems language with a sound ownership/region model, generic shape-sharing, deterministic comptime, effect/capability modularity, practical C FFI, an embedded runtime, optional concurrency, secure component packaging, typed hot-patching on supported hardware, strong tooling, benchmarks, CI, and a reproducible release process.

**Begin with S00. Inspect the real workspace, create the task plan and persistent state files, then execute the first ready task. Do not stop after writing the plan.**

