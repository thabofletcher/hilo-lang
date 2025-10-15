# HILO Security & Safety Guidance

This file encodes recommended defaults for safe AI–human collaboration.

## 1) Privacy & Secrets

- Treat environment variables, API keys, and customer data as **secrets**.
- Never print secrets in logs or outputs.
- Use `policy { do.not.exfiltrate.secrets = true }` in agents that can access IO/tools.

## 2) Refusals & Boundaries

- Agents must **not** reveal hidden chain-of-thought or private scratchpads.
- If asked for chain-of-thought, respond with a helpful **brief answer** or **summary of steps** instead.

## 3) Source Quality

- Prefer **primary sources** and **high-quality domains**.
- Cite where applicable; ensure at least 2 independent sources when decisions are high-stakes.

## 4) Human-in-the-Loop

- For decisions with legal, medical, or financial impact, add an explicit approval step:

```hilo
policy {
  require.human_approval = ["publish", "purchase", "delete", "deploy"]
}
```

## 5) Tool Safety

- Declare tools with strict signatures.
- Validate inputs against declared types/schemas.
- Log tool use with minimal necessary detail and without secrets.

## 6) Redaction

- Mark internal notes with `@internal` attributes or place within `policy { private.notes = true }`.
- Output should be **safe-to-share** unless explicitly stated otherwise.

These norms are encoded into the language shape (see `LANGUAGE_SPEC.md` and `AGENTS.md`).
