![Dust](./dust.png)

# Dust

**A systems language for constrained, edge, and IoT devices — Rust's safety and native speed with a fraction of the binary size.**

Dust is a new programming language designed to close the gap between raw C/Rust performance and modern dynamic modularity. It targets the same uncomfortable truth as Rust on tiny hardware: zero-cost abstractions are not always *zero-size*. Dust keeps compile-time safety and machine-speed execution while aggressively attacking code bloat, lifetime boilerplate, and rigid module boundaries.

> Prettig gestoord, maar serieus wanneer het hoort. *(Quirky in spirit, serious when it matters.)*

---

## Why Dust?

Rust is exceptional at runtime speed but its "zero-cost abstractions" are not always zero-*size*. On constrained devices the primary culprit is **monomorphization** — generic code duplicated for every concrete type — plus a steep lifetime-learning curve and rigidity when you need runtime dynamism. Dust addresses this architectural gap with four foundational pillars:

| Pillar | What it does | Replaces |
|---|---|---|
| **Shape-polymorphic compilation** | Groups types into equivalence classes by physical layout (size, alignment, register class) and emits one shared function per shape | Full monomorphization / binary bloat |
| **Region-inferred memory** | Infers lifetime bounds from control-flow analysis; groups allocations into scoped arenas with static-to-dynamic fallbacks | Explicit `&'a` lifetime annotations & the borrow checker |
| **Algebraic effect handlers** | Modules emit effects (`perform ReadSensor()`); parent contexts handle them, enabling zero-overhead hot-swappable modularity | Heavy OO vtables and C-style HAL tables |
| **Micro-AOT content-addressed components** | Compiles modules into sandboxed native components with SHA3-256 AST hashing for incremental deployment and flash hot-patching | Whole-device reflashing |

The result: peak throughput gains of **10–30% over standard C binaries** while maintaining a memory footprint orders of magnitude smaller than monomorphized Rust codebases.

---

## Memory Architecture: Region Inference & Tiered Safety Fallbacks

Dust eliminates explicit lifetime annotations by combining static region inference, linear asset tracking, and hybrid generational reference semantics. The compiler runs inter-procedural flow analysis to infer static lifetime bounds, grouping allocations into discrete lexically-scoped arenas ("regions"). Objects that escape are downgraded through a **tiered safety fallback hierarchy** rather than invoking a global tracing garbage collector:

| Allocation Strategy | Overhead | Dereference Cost | Memory Overhead | Target Use-Case |
|---|---|---|---|---|
| Static Region (Arena) | O(1) pointer bump | Zero (direct pointer) | 0 bytes/object | Local scratchpads, transient buffers |
| Linear Capability | O(1) zero-cost move | Zero (direct pointer) | 0 bytes/object | Peripherals, hardware registers, sockets |
| Generational Reference | O(1) generational write | Branching check (1–2 cycles) | 8 bytes/allocation | Dynamic graphs, UI nodes |
| Arc Promotion | Atomic fetch-and-add | Atomic inc/dec | 8–16 bytes header | Inter-thread channel communication |

**Escape analysis rules.** Let `E(e, ρ)` be the escape status of expression `e` in region `ρ`, evaluated over the control-flow graph: if all uses stay within `ρ` → *Local* (stack/arena); if a use escapes but stays thread-local → *EscapedThreadLocal* (generational reference with validation injected at deref sites); if a use crosses threads → *EscapedThreadGlobal* (atomic refcount promoted only where inter-thread escape analysis cannot guarantee isolation).

**Lessons from prior art.** Dust synthesizes the best ideas from languages already solving pieces of this puzzle: Zig's hygienic `comptime` specialization, Pony's reference capabilities for safe aliasing, Austral's affine/linear types for deterministic resource management, Vale's generational references and region borrow-checking, and Daslang's multi-tier adaptive compilation.

---

## Codegen: Shape-Polymorphic Compilation

Instead of instantiating a separate machine-code body per concrete type, the Dust compiler groups types into shape-equivalence classes. Two types `τ₁ ~ₛ τ₂` share one compiled instance **iff**:

```
SizeOf(τ₁) = SizeOf(τ₂) ∧ AlignOf(τ₁) = AlignOf(τ₂) ∧ RegisterClass(τ₁) = RegisterClass(τ₂)
```

When an operation needs type-specific behavior (custom hashing, equality), the compiler passes an implicit compact vtable or scalar descriptor — combining the execution performance of statically typed code with the binary footprint of unified dynamic dispatch. This shrinks compiled generic code by up to **70%** in data-structure-heavy modules.

| Backend | Footprint Overhead | Compile Latency | Optimization | MCU Target Support |
|---|---|---|---|---|
| LLVM IR | Large (~15–30 MB) | High | Industry-leading (vectorization, LTO) | Universal |
| Cranelift | Moderate (~5–10 MB) | Low | Medium | x86_64, AArch64, RISC-V 64 |
| QBE (C IR) | Minimal (<1 MB) | Ultra-fast | Basic | Tier-2 via C99 transpilation |
| Custom C Transpiler | Zero (emits C99/C11) | Extremely low | Delegates to host GCC/Clang | Every C-supported target |

For rapid dev loops Dust uses Cranelift or C transpilation; for production bare-metal releases it emits optimized LLVM IR for aggressive link-time optimization.

---

## Modularity Engine: Effects, Capabilities & Micro-AOT

- **Tail-resumptive lexical algebraic effects.** Modules declare required operations as effects and delegate implementation to parent-context handlers. Handlers are restricted to *tail-resumptive* (resuming immediately at the call site without capturing arbitrary continuations), so the compiler lowers `perform` directly into static branch-and-link or inline calls — dynamic driver swapping with zero runtime penalty.
- **Capability-based peripheral access.** A zero-ambient-authority model: modules cannot touch memory, interrupts, or registers directly. Access rights are linear capability tokens passed down from init; because they obey linear semantics they can't be duplicated, forged, or leaked — preventing races on hardware without dynamic mutexes.
- **Content-addressed Micro-AOT & typed hot-patching.** Each compiled function/type is identified by a SHA3-256 hash over its normalized AST and type dependency graph. This enables zero-overhead dependency resolution (duplicate functions collapse to identical hashes), OTA flash double-buffering that replaces only changed function pointers atomically, and runtime module-boundary validation against unauthorized hardware access.

---

## Implementation Roadmap

A phased engineering plan validates each core hypothesis on real embedded hardware before the next begins.

| Phase | Focus | Key Deliverables | Success Criteria |
|---|---|---|---|
| **I — Memory Engine** | Escape analysis, region allocation, generational fallbacks, linear tracking | Lexer/parser/IR; affine+linear type checker; static escape-analysis pass; C99/QBE backend | Run bare-metal apps (linked lists, ring buffers) on ARM Cortex-M4 with zero heap allocators or runtime pauses |
| **II — Codegen Engine** | Shape-equivalence inference, monomorphization folding, layout vtables | Type canonicalization; generic-transform engine; layout-descriptor generator; bloat profiler | >50% flash reduction vs monomorphized Rust on ESP32; 0% speed regression |
| **III — Modularity Engine** | Tail-resumptive effect handlers, linear hardware capabilities | Effect interface type-checker; handler-lowering pass; linear capability lib (GPIO/SPI/I2C/UART) | One driver compiled across two MCUs by swapping effect handlers at compile time; 100% parity with hand-written C |
| **IV — Dynamic Runtime** | AST hashing, Micro-AOT loader, flash hot-patcher | SHA3-256 AST hasher; relocatable component emitter; bare-metal double-buffer loader | OTA hot-patch of live firmware in <5 ms without reboot or state corruption |

### Comparative benchmarks (targets vs. established systems)

| Benchmark | Axis | Target vs Rust `no_std` | Target vs C99 |
|---|---|---|---|
| Bare-Metal Sensor Fusion | Latency / CPU cycles | ≤100% latency (equal speed) | ≤105% latency (near-native) |
| Network Serialization | Flash size & RAM | ≥50% smaller binary | ≤110% flash parity |
| Dynamic Driver Patching | OTA interruption time | <5 ms hot-patch | seamless patch, no reboot |

---

## Build, Test & Release Plan

The companion build/release plan details a full 12-week engineering timeline covering environment setup, CI/CD, packaging, and maintenance across Linux (Debian/Ubuntu), macOS, and Windows on x86_64 and ARM64. Highlights:

- **Environment:** Git + SDKs per platform (.NET SDK for the toolchain, Python 3.9+, GCC/Clang or MinGW for C components). Docker for reproducible builds.
- **Build system:** `dotnet build`/`msbuild` for the .NET toolchain alongside Make/CMake for any C front-end, plus `build.sh` and `build.ps1` automation scripts.
- **CI/CD (GitHub Actions):** matrix across OS × arch (Ubuntu/macOS/Windows × x64/arm64), running restore → build → test, then publishing artifacts on tagged commits to GitHub Releases, NuGet, and multi-arch Docker Hub images via `docker buildx`.
- **Packaging:** NuGet global tool (`dotnet tool install -g Dust`), deb/rpm (via `fpm`), Windows zip/MSI, signed with SHA256 checksums.
- **Testing:** unit tests (XUnit/NUnit or C harnesses), end-to-end compile+run integration suites, libFuzzer/AFL++ fuzzing of the parser, coverage via coverlet, and static analysis (Roslyn analyzers, clang-tidy, Dependabot/Snyk).

### 12-week milestones

1. Discovery & planning · 2. Environment setup · 3. Build system · 4. Dependency management · 5. Basic unit testing · 6. CI/CD pipeline · 7. Documentation site · 8. Packaging · 9. Integration & fuzz testing · 10. Profiling & optimization (Release Candidate) · 11. Security audit · 12. v1.0.0 release & wrap-up

---

## Getting Started

```bash
# Clone the repository
git clone https://github.com/illdynamics/dust.git
cd dust

# Restore, build, and test (per the documented pipeline)
dotnet restore Dust.sln
dotnet build  Dust.sln --configuration Release
dotnet test   Dust.UnitTests/Dust.UnitTests.csproj --no-build --verbosity normal
```

See [`docs/`](./docs) for the syntax guide, tutorials, and API reference.

---

## Contributing & Community

Contributions are welcome. Please read `CONTRIBUTING.md`, follow the commit-message and branch-naming conventions, and respect the `CODE_OF_CONDUCT.md`. We value an open, helpful culture — honest, supportive feedback over ego. Report bugs and feature requests via GitHub Issues/Milestones, and keep a CHANGELOG following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

**Prior art & references.** Rust polymorphization (experimental), OCaml's uniform representation, Swift value witnesses; MLKit / Cyclone / Vale / Tofte–Talpin regions; Koka and OCaml 5 effects; Tock, Hubris, Zephyr ELF loader, WAMR/WASM3; Zig `comptime`, Pony reference capabilities, Austral linear types, Unison content addressing, Daslang multi-tier compilation.

**Sources.** This README synthesizes the project's design vision in [`dust-brainstorm.md`](./dust-brainstorm.md) and the engineering/build/release plan in [`dust-deep-research.md`](./dust-deep-research.md).
