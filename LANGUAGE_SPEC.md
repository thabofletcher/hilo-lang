# HILO Language Spec (v1.0)

HILO (Human-Integrated Language for Orchestration) is a pragmatic, parseable pseudocode
language centered on clarity and collaboration. This spec covers syntax, typing, semantics,
and built-in constructs for agents, tasks, and workflows.

---

## 1. Design Principles

- **Readable first**: stakeholders should understand a HILO file without a compiler.  
- **Parse-friendly**: minimal ambiguity; braces `{}` for blocks; newlines terminate statements; semicolons optional.  
- **Gradual typing**: annotate where helpful; inference elsewhere.  
- **Functional + OO + Modular**: pure functions, records/enums/classes, clear module boundaries.  
- **Workflow-native**: `agent`, `task`, `workflow`, `policy`, `tool` are first-class.  
- **Safety by default**: norms for privacy, consent, and refusal are explicit and declarative.  
- **Deterministic evaluation**: expression order is left-to-right; no implicit magic.

---

## 2. Lexical Structure

- **Identifiers**: `^[A-Za-z_][A-Za-z0-9_]*$`
- **Keywords** (reserved):

  ```
  module import as from export
  const let var
  func return async await yield
  record enum type trait class new prop
  if else for while match in break continue
  try catch throw defer using
  agent policy tools capabilities profile
  task workflow test
  true false null
  and or not
  spawn channel send recv select timeout
  with where impl
  ```

- **Literals**: integers (`42`), floats (`3.14`), strings (`"text"` with `"` and `\`), booleans, `null`.
- **Comments**: `// line`, `/* block */`, and `/// doc-comment`.
- **Attributes**: `@name(args...)` attach metadata to next declaration.
- **Whitespace**: insignificant except to terminate statements by newline. A trailing `;` is allowed but not required.

---

## 3. Types

- **Primitives**: `Int`, `Float`, `Bool`, `String`, `Bytes`, `Time`, `Duration`, `Any`.
- **Generics**: `List[T]`, `Set[T]`, `Map[K,V]`, `Tuple[T1,T2,...]`, `Option[T]`, `Result[T,E]`.
- **Type aliases**: `type UserId = String`.
- **Records** (product types): shallow, value-oriented.

  ```hilo
  record User {
    id: String
    name: String
    email?: String     // optional field
  }
  ```

- **Enums** (sum types):

  ```hilo
  enum FileOp {
    Read(path: String)
    Write(path: String, data: Bytes)
    Delete(path: String)
  }
  ```

- **Traits** (interfaces) and **Classes**:

  ```hilo
  trait Formatter {
    func format(x: Any) -> String
  }

  class JsonFormatter implements Formatter {
    new() {}
    func format(x: Any) -> String {
      return io.json.stringify(x)
    }
  }
  ```

- **Function types**: `(A, B) -> C`. Lambdas: `fn (x: A, y: B) -> C { ... }`.

- **Nullability**: use `Option[T]` or `?` suffix for fields (records/classes). `expr ?? fallback` for coalesce; `expr?.prop` for optional chaining.

---

## 4. Variables, Constants, and Scope

- `const` is compile-time constant; `let` is immutable; `var` is mutable.

```hilo
const VERSION: String = "1.0"
let threshold = 0.75
var counter: Int = 0
```

- Block scope; shadowing allowed; `defer { ... }` runs at scope exit.

---

## 5. Control Flow

```hilo
if cond { ... } else { ... }
for x in list { ... }
while cond { ... }
break; continue
```

**Pattern matching**:

```hilo
match op {
  FileOp.Read(p)        => io.print("read " + p)
  FileOp.Write(p, _)    => io.print("write " + p)
  _                     => io.warn("other")
}
```

---

## 6. Errors

- `Result[T,E]` is preferred.  
- Exceptions are explicit:

```hilo
try {
  risky()
} catch (e) {
  log.error("failed", e)
  throw e
}
```

---

## 7. Functions

```hilo
func add(a: Int, b: Int) -> Int {
  return a + b
}

// default values & named args
func connect(host: String, port: Int = 443, secure: Bool = true) -> Conn { ... }
let c = connect(port=8443, host="example.com")
```

- `@pure` indicates no IO/tool calls; `@memo` enables caching.

---

## 8. Modules and Imports

Top of file:

```hilo
module my.app.http

import core.io
import core.text { trim, split } as t
```

- Everything is private by default. Use `export` to expose:

```hilo
export func public_api() { ... }
export record PublicThing { ... }
```

---

## 9. Concurrency (Tasks & Channels)

- `async func` declares an async function.  
- `spawn` launches a task; returns `Task[T]`.  
- `await` awaits a task.  
- `channel[T](capacity=0)` creates a typed channel (0 = unbuffered).  
- `send ch <- value`, `recv ch -> value`, and `select { ... }`.

```hilo
async func fetch(url: String) -> String { ... }

task parallel() {
  let a = spawn fetch("https://a")
  let b = spawn fetch("https://b")
  let text = (await a) + (await b)
  io.print(text)
}
```

---

## 10. Agents, Policies, and Tools

Agents are explicit. They declare **profile**, **capabilities**, **tools**, and **policy**.

```hilo
agent Researcher {
  profile {
    name: "Researcher"
    purpose: "Find and summarize credible sources."
    audience: ["Engineer", "PM"]
    style: { tone: "succinct", citations: "required" }
  }

  capabilities {
    inputs:  { query: String, scope?: String }
    outputs: { brief: String, sources: List[String] }
  }

  tools {
    web.search(query: String) -> List[Url]
    web.open(url: String) -> Page
  }

  policy {
    // Privacy & safety defaults:
    refuse.chain_of_thought = true
    cite.sources.min = 2
    avoid.hallucination = true
    do.not.exfiltrate.secrets = true
  }

  func run(query: String, scope?: String) -> Result[String, Error] {
    let urls = web.search(query)
    let pages = urls |> map(fn (u) -> web.open(u))
    let brief = summarize(pages, max=200)
    return Ok(brief)
  }
}
```

**Best practice:** agents must *never* disclose hidden deliberations; provide **concise rationales** only. Policies are enforceable constraints for runtimes.

---

## 11. Tasks and Workflows

- `task` is a named body of work; may orchestrate agents/tools.
- `workflow` composes tasks and edges:

```hilo
task DraftSpec(topic: String) -> Doc { ... }
task ReviewSpec(doc: Doc) -> Doc { ... }

workflow SpecFlow {
  start DraftSpec("HILO")
  then ReviewSpec
  on_error { notify.team("SpecFlow failed") }
}
```

---

## 12. Testing

```hilo
test "sum works" {
  assert add(2,3) == 5
}
```

---

## 13. Standard Library (conceptual)

- `core.io` — `print`, `read_file`, `write_file`, `json.stringify/parse`
- `core.text` — `split`, `join`, `trim`, `match`, `replace`
- `core.time` — `now`, `sleep`, `format`
- `core.math` — `min`, `max`, `mean`, `median`
- `core.collections` — `map`, `filter`, `reduce`, `group_by`

(See `STANDARD_LIBRARY.md` for details.)

---

## 14. Serialization & Interop

HILO maps cleanly to JSON/YAML; an AST schema is provided in `INTEROP.md`.

---

## 15. Formatting Conventions

- 2 spaces indent; trailing commas allowed; single quotes prohibited in strings for uniformity.
- One top-level declaration per section; keep files < 400 lines where possible. See `STYLEGUIDE.md`.

---
