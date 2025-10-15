# AGENTS — Using HILO for AI–Human Collaboration

This guide shows how to model safe, effective multi-agent work with HILO.

---

## 1) Roles

We’ll build three cooperating agents:

- **Researcher** — finds sources and extracts key facts.
- **Writer** — drafts content from structured notes.
- **Reviewer** — checks facts, clarity, and policy compliance.

---

## 2) Policies (Safety Defaults)

All agents share these defaults:

```hilo
policy {
  refuse.chain_of_thought = true       // do not reveal private reasoning
  avoid.hallucination = true           // do not invent citations
  cite.sources.min = 2                 // at least 2 independent sources when summarizing
  do.not.exfiltrate.secrets = true     // never print API keys or private data
  require.human_approval = ["publish"] // human gate before releasing externally
}
```

---

## 3) Tools

Explicit, typed tools with narrow scope:

```hilo
tools {
  web.search(query: String, max_results: Int = 5) -> List[Url]
  web.open(url: String) -> Page
  io.print(x: Any) -> Unit
}
```

---

## 4) Agent Definitions

```hilo
agent Researcher {
  profile {
    name: "Researcher"
    purpose: "Locate credible sources and extract key points."
    style: { tone: "neutral", citations: "required" }
  }

  capabilities {
    inputs:  { topic: String, scope?: String }
    outputs: { notes: List[String], sources: List[String] }
  }

  tools {
    web.search(query: String) -> List[Url]
    web.open(url: String) -> Page
  }

  policy { refuse.chain_of_thought = true }

  func run(topic: String, scope?: String) -> Result[Map[String,Any], Error] {
    let urls = web.search(topic)
    let pages = urls |> map(fn(u) -> web.open(u))
    let notes = extract.key_points(pages, max=8)
    return Ok({ "notes": notes, "sources": urls })
  }
}

agent Writer {
  profile {
    name: "Writer"
    purpose: "Draft clear, concise content for engineers."
    audience: ["Engineer", "PM"]
    style: { tone: "succinct", reading_level: "11" }
  }
  capabilities {
    inputs:  { notes: List[String], sources: List[String], audience?: String }
    outputs: { draft: String }
  }
  policy { refuse.chain_of_thought = true }

  func run(notes: List[String], sources: List[String], audience?: String) -> String {
    return compose.draft(notes=notes, sources=sources, target=audience ?? "Engineer")
  }
}

agent Reviewer {
  profile {
    name: "Reviewer"
    purpose: "Check accuracy, clarity, and policy compliance."
  }
  capabilities {
    inputs:  { draft: String, sources: List[String] }
    outputs: { report: String, revised: String }
  }
  policy {
    refuse.chain_of_thought = true
    cite.sources.min = 2
  }
  func run(draft: String, sources: List[String]) -> Map[String,String] {
    let report = quality.check(draft, sources)
    let revised = quality.revise(draft, report)
    return { "report": report, "revised": revised }
  }
}
```

---

## 5) Orchestrating the Workflow

```hilo
task ProduceBrief(topic: String) -> String {
  let r = Researcher.run(topic)
  if r is Err { throw r.err }
  let notes = r.ok["notes"]
  let sources = r.ok["sources"]

  let draft = Writer.run(notes, sources)

  let review = Reviewer.run(draft, sources)
  io.print(review["report"])
  policy { require.human_approval = ["publish"] }
  return review["revised"]
}

workflow ArticleFlow {
  start ProduceBrief("HILO Language Overview")
  then { io.print("Flow complete.") }
}
```

---

## 6) Collaboration Tips

- Use `capabilities.inputs/outputs` as *contracts* between agents and humans.
- Keep `policy` blocks explicit—what to refuse, what to cite, what to ask approval for.
- Prefer `Result[T,E]` for actions that can fail and surface errors early.
- Include `test` blocks for critical behaviors.

See `project/src/` for a complete example.
