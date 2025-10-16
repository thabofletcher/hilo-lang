# Agent Findings — Grammar vs Sample Code

This log tracks QA findings for the HILO surface area. Items move to “Resolved” once the spec and samples align.

## Resolved Inconsistencies (P0 blockers)

- Import alias order now supports both `import path as Alias { … }` and `import path { … } as Alias` (`GRAMMAR.md:8-14`).
- Tools blocks accept signature-style declarations (`GRAMMAR.md:52-60`), matching `project/src/agents/Researcher.hilo:20-23`.
- Inline record/struct types are legal in signatures via `StructType` (`GRAMMAR.md:146-149`).
- Capability entries parse as labeled struct literals (`GRAMMAR.md:54-57`).
- Named arguments accept `name=value` syntax (`GRAMMAR.md:109-111`).
- Record construction via `Type { field: value }` is handled by postfix initializers (`GRAMMAR.md:102-114`).
- Lambdas allow optional types and expression bodies (`GRAMMAR.md:118-123`).
- Pipelines can invoke call/member chains on the RHS (`GRAMMAR.md:105-108`), covering `urls |> map(...)`.
- `throw` statements are first-class (`GRAMMAR.md:82-90`), reflecting `project/src/main.hilo:28`.
- `async func` is supported in function signatures and decls (`GRAMMAR.md:40-45`).
- Optional field/type suffix `?` is part of the grammar (`GRAMMAR.md:20-23`, `GRAMMAR.md:141-149`).
- `Literal` is defined and usable in pattern matching (`GRAMMAR.md:153-154`).
- Boolean precedence is documented in the spec (`LANGUAGE_SPEC.md:103`).

## Open Performance / Semantic Questions

- None outstanding after clarifying pipeline borrowing semantics and map typing (`LANGUAGE_SPEC.md:97-122`).

## Standard Library Coverage

- Filesystem, networking (including sockets, TLS), concurrency controls, database access, and cryptography primitives are now spelled out (`STANDARD_LIBRARY.md:10-58`).

_Status: 0 open P0 issues. Continue regression-testing future edits against these resolved cases._
