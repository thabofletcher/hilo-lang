# HILO Parser Roadmap (Rust)

## Milestone 1 — Grammar Foundation
- [ ] Lock the formal grammar: export `GRAMMAR.md` into a machine-friendly form (e.g., `.pest`).
- [ ] Decide on parser toolkit (`chumsky` for combinators vs `pest` for grammar-driven parsing).
- [ ] Scaffold workspace: `cargo new hilo-parser` with `src/lib.rs` exposing `parse_module`.
- [ ] Write token definitions & lexer (if toolkit requires manual lexing).

## Milestone 2 — AST & Parsing
- [ ] Design Rust AST structs/enums mirroring `LANGUAGE_SPEC.md`.
- [ ] Implement parser for core declarations (modules, imports, records, enums, funcs).
- [ ] Add agents/tasks/workflows parsing, including policy and tools blocks.
- [ ] Cover expressions/statements: precedence, pipelines, async/await, struct literals.
- [ ] Build property-based tests for tricky constructs (named args, optional types, lambdas).

## Milestone 3 — Semantic Checks
- [ ] Module-level symbol table & name resolution.
- [ ] Type skeleton: ensure optional markers, struct types, and generics are recognized.
- [ ] Validate capability/tool signatures align with grammar constraints.
- [ ] Emit actionable diagnostics with spans.

## Milestone 4 — Backends & Interop
- [ ] Define JSON AST serialization compatible with `INTEROP.md`.
- [ ] Implement IR lowering targeting JVM bytecode (via `kaffeine`/`noderive` or custom).
- [ ] Prototype a CLI: `hilo-compiler parse file.hilo --out ast.json`.
- [ ] Add hooks for future bytecode/native compilation stages.

## Milestone 5 — Tooling & Distribution
- [ ] Continuous integration (fmt, clippy, tests).
- [ ] Benchmark suite for parsing large HILO projects.
- [ ] Package release artifacts (crates.io, GitHub releases).
- [ ] Author developer documentation & contribution guide.

> Keep commits small (`fix:`, `feat:`) and track outstanding questions in `agent-findings.md`.
