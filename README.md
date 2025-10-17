# HILO — Agentic DSL for GPTs, AIs, and Operational Tooling (v1.0)

**HILO** is a compact, agent-focused DSL crafted so GPT-class models, automation AIs, and operational tooling can parse, reason about, and coordinate complex workflows. It stays human-readable while foregrounding agent roles, policies, tools, and orchestration primitives—turning `.hilo` files into executable playbooks for autonomous systems.

> Status: Draft v1.0 • 2025-10-17

## Why HILO?

- **Simple, explicit syntax**: braces for blocks; newlines end statements (semicolons optional).
- **Gradual types**: types help structure; omit when obvious.
- **Functional core, OO shell**: pure functions + lightweight classes/records/enums.
- **First-class workflows**: `agent`, `task`, and `workflow` primitives.
- **Safety & governance** built in: `policy` blocks, explicit tool declarations, and redaction hints.
- **Parse-friendly**: reserved keywords and EBNF are provided; tokens avoid ambiguity.

HILO is **not** a runtime. It is the coordination DSL that glues GPTs, AIs, and software tooling together—codifying how they collaborate safely and correctly.

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

## Getting started

1. Add `bin` to your PATH or invoke it via `./bin/hilo`.
2. Run `hilo init <target-directory>` to scaffold a fresh project; the script copies the language spec, sample agents, and bootstrap guidance into the target.
3. Open the entry `AGENTS.hilo` and follow its instructions, then explore imported `.hilo` files as needed.

> HILO is designed to *coordinate* systems. When you implement it, treat HILO as a **source of truth** for intent, contracts, and workflows—even if your runtime is Python, JS/TS, Go, Rust, or another stack.
