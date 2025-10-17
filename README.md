# HILO — Human-Integrated Language for Orchestration

**HILO** is a compact, human-first *pseudocode* language that is easy for people to read and for AI/agents to parse.
It blends **modular**, **functional**, and **object-oriented** styles while adding **agent** and **workflow** primitives,
and a clear **policy** model for safe AI–human collaboration.

> Status: Draft v1.0 • 2025-10-15

## Why HILO?

- **Simple, explicit syntax**: braces for blocks; newlines end statements (semicolons optional).
- **Gradual types**: types help structure; omit when obvious.
- **Functional core, OO shell**: pure functions + lightweight classes/records/enums.
- **First-class workflows**: `agent`, `task`, and `workflow` primitives.
- **Safety & governance** built in: `policy` blocks, explicit tool declarations, and redaction hints.
- **Parse-friendly**: reserved keywords and EBNF are provided; tokens avoid ambiguity.

HILO is **not** a runtime. It is a *design/coordination language* for projects that involve people and AI systems.

## Quick look

```hilo
module demo.hello

func greet(name: String = "world") -> String {
  return "Hello, " + name + "!"
}

task main() {
  let msg = greet()
  io.print(msg)
}
```

## Repo layout (this zip)

- `hilo/spec/LANGUAGE_SPEC.md` — complete language semantics and constructs  
- `hilo/spec/GRAMMAR.md` — EBNF grammar suitable for writing a parser  
- `hilo/spec/STANDARD_LIBRARY.md` — the conceptual stdlib surface for examples  
- `hilo/spec/STYLEGUIDE.md` — conventions for readable, consistent code  
- `hilo/spec/SECURITY.md` — best practices for safe AI–human work  
- `hilo/spec/INTEROP.md` — AST/JSON mapping + embedding guidance  
- `project/docs/AGENTS.md` — how to model multi-agent projects with HILO  
- `examples/` — small runnable samples (pseudocode)  
- `project/` — a sample multi-agent project in HILO

## Getting started

1. Read `hilo/spec/LANGUAGE_SPEC.md` and `hilo/spec/GRAMMAR.md`.  
2. Skim `project/docs/AGENTS.md` for collaboration and safety practices.  
3. Explore `examples/` and `project/src/` for patterns you can reuse.

> HILO is designed to *coordinate* systems. When you implement it, treat HILO as a **source of truth** for intent, contracts, and workflows—even if your runtime is Python, JS/TS, Go, Rust, or another stack.
