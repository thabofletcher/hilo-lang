# HILO Interop & AST

HILO is designed to be transformed into an AST suitable for execution in any host language.

## Canonical JSON AST (excerpt)

```json
{
  "module": "org.example.mod",
  "imports": ["core.io", "core.text"],
  "decls": [
    {
      "kind": "record",
      "name": "User",
      "fields": [
        {"name": "id", "type": "String"},
        {"name": "email", "type": "String?", "optional": true}
      ]
    },
    {
      "kind": "func",
      "name": "greet",
      "params": [{"name": "name", "type": "String", "default": ""world""}],
      "ret": "String",
      "body": ["return concat("Hello, ", name, "!")"]
    }
  ]
}
```

## Mapping Notes

- `Option[T]` → `"T?"` in abbreviated string form.
- `Result[T,E]` → `{ "ok": T } | { "err": E }` union in JSON.
- Enums become tagged unions: `{ "tag": "Case", "args": [...] }`.
- Lambdas are serialized as nested `func` nodes with `"isLambda": true`.

## Embedding in Markdown

You can include fenced blocks labeled `hilo`:

````markdown
```hilo
task main() {
  io.print("Hello")
}
```
````

Parsers should detect these blocks and extract the code for execution or analysis.
