# HILO Conceptual Standard Library

> The stdlib is conceptual—runtimes may map these to host-language APIs.

## core.io

- `print(x: Any) -> Unit`
- `read_file(path: String) -> String`
- `write_file(path: String, data: String | Bytes) -> Unit`
- `json.stringify(x: Any) -> String`
- `json.parse(s: String) -> Any`
- `log.debug(...), log.info(...), log.warn(...), log.error(...)`

## core.text

- `split(s: String, by: String) -> List[String]`
- `join(xs: List[String], by: String) -> String`
- `trim(s: String) -> String`
- `lower(s: String) -> String`
- `upper(s: String) -> String`
- `match(s: String, rx: String) -> List[Match]`
- `replace(s: String, rx: String, with: String) -> String`

## core.time

- `now() -> Time`
- `sleep(d: Duration) -> Unit`
- `format(t: Time, pattern: String) -> String`

## core.collections

- `map(xs: List[A], f: (A) -> B) -> List[B]`
- `filter(xs: List[A], p: (A) -> Bool) -> List[A]`
- `reduce(xs: List[A], f: (A, A) -> A, init: A) -> A`
- `group_by(xs: List[A], f: (A) -> K) -> Map[K,List[A]]`

## core.math

- `min(a: Number, b: Number) -> Number`
- `max(a: Number, b: Number) -> Number`
- `mean(xs: List[Float]) -> Float`
- `median(xs: List[Float]) -> Float`

## core.net (optional)

- `http.get(url: String, headers?: Map[String,String]) -> Response`
- `http.post(url: String, body: Bytes | String, headers?: Map[String,String]) -> Response`

## core.concurrency

- `channel[T](capacity: Int = 0) -> Channel[T]`
- `send(ch: Channel[T], msg: T) -> Unit`
- `recv(ch: Channel[T]) -> T`
- `select(cases: List[Case], timeout?: Duration) -> CaseResult`

## ai.prompt (adapters for LLM runtimes)

- `render(system: String, user: String, guidelines?: List[String], policy?: Policy, vars?: Map[String,Any]) -> Prompt`
- `call(model: String, prompt: Prompt, temperature?: Float, max_tokens?: Int) -> LLMResponse`

> **Safety note**: stdlib **must not** return or expose raw chain-of-thought. Responses should be *answers* with optional short rationales and sources.
