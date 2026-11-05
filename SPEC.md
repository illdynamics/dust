# Dust Language Specification (working)

Authoritative user-visible spec. Only features marked implemented are real; designed-but-unimplemented ones are clearly labeled **designed** and live in TODO/M-markers. Syntax is a small coherent subset chosen now and used consistently thereafter — no ad hoc grammar.

## 1. Identity & scope
- Language name: **Dust**. Extension: `.dust`. New language, not a fork of any existing `Dust` project.
- Goals: native AOT compilation, small footprint, region-inferred ownership memory model (no default tracing GC), affine resources by default with opt-in linear guarantees, shape-aware generics, staged `comptime`, explicit capabilities/effects, path to content-addressed components + hot-patching. C interoperability and embedded portability.
- Profiles: `core` (no OS/heap), `alloc` (optional allocator/containers), `std` (host/OS facilities), `embedded`/platform.

## 2. Implemented surface (this release)
The following actually works now:
- Source loading with stable file IDs, UTF-8, byte spans `[start,end)`.
- Diagnostics: severity (`error`/`warning`), optional stable code, labeled primary/secondary spans, notes; terminal rendering with source line snippet, `(line,col)` location, and `~` underline.

## 3. Lexical (designed; S03)
- Identifiers: `[A-Z a-z_][A-Za-z0-9_]*`; Unicode identifiers **planned** — policy TBD in S03/TBD.
- Keywords (planned initial set): `fn let var mut copy move ref struct enum impl trait mod import as pub private return if else while loop break continue comptime capability effect perform`. Non-reserved lowercase words are identifiers unless listed here; add to the list only when adopted.
- Literals: integer (dec/hex `0x`/`0b`/`0o`, `_` digit separators), float (planned initial), string `"..."` with escapes `\n \t \\ \" \' \r \0 \u{...}`, char `'c'`. Unterminated/malformed → span-correct lexical error.
- Comments: line `//…`, block `/* … */` **with nesting** (planned). Invalid UTF-8 policy documented in S03.

## 4. Types (designed; S05–S10)
Primitives planned: `bool, char, u8/u16/u32/u64/u128/usize, i8/…/isize, f32/f64`. Ownership classes: `Copy`, move-affine, `ref` borrow, generational/shared (S06). Structs/enums/arrays/slices/options/results with layout guarantees. Explicit casts with narrowing/signedness rules. Generics + shape polymorphism post types/backend (S10).

## 5. Semantics
- Statements vs expressions: block-statement language; `let`/`var` bindings, assignment, expression statements, `return`. Control flow `if`/`while`/`loop`.
- Ownership/move default; explicit borrow via `ref`; escape analysis drives local-region allocation with generational/Arc fallbacks (S06). Destructors for non-Copy at end of scope. `unsafe` boundary documented.
- Calling convention, C ABI, symbol mangling, target data layout, integer/pointer widths fixed per §2.2 and emitted deterministically by the C11 backend using `<stdint.h>`.

## 6. Error behavior
Prefer explicit `Result`/`Option`; no hidden exceptions. Errors carry stable codes + spans; compilation stops at first error batch's diagnostics before codegen (S05 gate).

## 7. Unresolved research questions (tracked)
- Unicode identifier policy; nested block comment depth limit; comptime termination bound; shape-class equivalence criteria for register class; generational reference validation cost model. Record answers in DECISIONS.md when settled.
