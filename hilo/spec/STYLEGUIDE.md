# HILO Style Guide

- **Modules**: reverse-DNS style (`org.example.project`).
- **Indentation**: 2 spaces; no tabs.
- **Line length**: aim for ≤ 100 chars.
- **Naming**:
  - `CamelCase` for `record`, `enum`, and `class` names.
  - `snake_case` for variables and parameters.
  - `lowerCamelCase` for functions and methods.
  - `UPPER_SNAKE_CASE` for `const`.
- **Docs**: prefer `///` doc-comments above declarations.
- **Purity**: mark pure computations with `@pure`; IO should be explicit.
- **Errors**: prefer `Result[T,E]` or `Option[T]` over exceptions.
- **Agents**: separate **profile**, **capabilities**, **tools**, and **policy**.
- **Prompts**: avoid requesting chain-of-thought; ask for **concise** reasoning summaries only.
- **Workflows**: small, composable tasks. Keep each `task` under ~50 lines when possible.
- **Tests**: one behavior per `test` block; name in natural language (`test "parses empty list"`).
