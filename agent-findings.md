# Agent Findings — Grammar vs Sample Code

This log captures discrepancies spotted between `GRAMMAR.md` and the sample project. Each entry highlights why a parser built strictly from the published grammar would reject the accompanying code.

- **Import alias order** — Grammar requires `import path as Alias { … }`, but the project uses `import path { … } as Alias` (`project/src/main.hilo:4`).
- **Agent tools signatures** — Grammar treats `tools { … }` as a block of statements; the project lists bare signatures such as `web.search(...) -> …` (`project/src/agents/Researcher.hilo:20-23`), which the grammar cannot parse.
- **Inline record return types** — `{ title: String, text: String, url: String }` in `web.open` (`project/src/agents/Researcher.hilo:22`) is unsupported because `Type` only permits named, generic, tuple, or function types.
- **Capabilities blocks** — Lines like `inputs: { topic: String }` (`project/src/agents/Researcher.hilo:15-18`) have no corresponding production; the grammar doesn’t allow key–value pairs inside agent capability blocks.
- **Named arguments syntax** — Calls such as `Writer.run(..., audience="Engineer")` (`project/src/main.hilo:20`) contradict the grammar, which defines named arguments as `name: value`.
- **Record instantiation literal** — Constructing a record via `Brief { … }` (`project/src/main.hilo:22-26`) lacks grammar support; there’s no rule for pairing an identifier with an inline initializer.
- **Lambda subsets** — Lambdas in the repo omit parameter types and use expression bodies (`project/src/agents/Researcher.hilo:32-36`), but the grammar insists on typed parameters and a block body (`GRAMMAR.md:119-120`).
- **Pipeline invocation** — `urls |> map(fn ...)` (`project/src/agents/Researcher.hilo:32`) expects the pipe target to be callable; the grammar’s `Pipe = "|>" Primary` (`GRAMMAR.md:111`) doesn’t permit the subsequent `Call`, so this syntax can’t be parsed.
- **Throw statements missing** — Exception paths rely on `throw e` (`project/src/main.hilo:28`), yet `Stmt` lacks a `ThrowStmt` production (`GRAMMAR.md:63-92`), leaving no way to express `throw`.
- **Async keyword gap** — The spec uses `async func fetch(...)` (`LANGUAGE_SPEC.md:183-190`), but `FuncDecl` has no allowance for an `async` modifier (`GRAMMAR.md:41`), so the advertised async support can’t be implemented.
- **Optional field/type markers** — Examples show `email?: String` (`LANGUAGE_SPEC.md:59`) and `String?` (`INTEROP.md:17`), but `FieldDecl` (`GRAMMAR.md:28`) and `Type` (`GRAMMAR.md:129-136`) don’t include `?`, blocking nullable shorthand.
- **Undefined literal pattern** — `Pattern` references `Literal` (`GRAMMAR.md:124`), yet `Literal` is never defined, so literal matches cannot be parsed.

Use this list to drive grammar updates or adjust the sample code so the language definition and examples stay in sync.
