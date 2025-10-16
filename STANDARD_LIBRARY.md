# HILO Conceptual Standard Library

> The stdlib is conceptual—runtimes may map these to host-language APIs.

## core.io

- `print(x: Any) -> Unit`
- `read_file(path: String) -> String`
- `write_file(path: String, data: String | Bytes) -> Unit`
- `json.stringify(x: Any) -> String`
- `json.parse(s: String) -> Any`
- `log.debug(...), log.info(...), log.warn(...), log.error(...)`

## core.fs

- `exists(path: String) -> Bool`
- `list_dir(path: String) -> List[String]`
- `stat(path: String) -> FileInfo`
- `mkdir(path: String, recursive: Bool = false) -> Unit`
- `remove(path: String, recursive: Bool = false) -> Unit`
- `open(path: String, mode: FileMode) -> FileHandle`
- `FileHandle.read(max_bytes?: Int) -> Bytes`
- `FileHandle.write(data: Bytes | String) -> Unit`
- `FileHandle.flush() -> Unit`
- `FileHandle.close() -> Unit`

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

- `http.get(url: String, headers?: Map[String,String], timeout?: Duration) -> Response`
- `http.post(url: String, body: Bytes | String, headers?: Map[String,String], timeout?: Duration) -> Response`
- `tcp.connect(addr: Address, timeout?: Duration) -> TcpStream`
- `tcp.listen(addr: Address, backlog?: Int) -> TcpListener`
- `TcpStream.read(max_bytes?: Int) -> Bytes`
- `TcpStream.write(data: Bytes) -> Unit`
- `udp.bind(addr: Address) -> UdpSocket`
- `udp.sendto(socket: UdpSocket, addr: Address, data: Bytes) -> Unit`
- `udp.recvfrom(socket: UdpSocket, max_bytes: Int) -> (Address, Bytes)`
- `websocket.connect(url: String, headers?: Map[String,String]) -> WebSocket`
- `tls.wrap(stream: TcpStream, params: TlsConfig) -> SecureStream`

## core.concurrency

- `channel[T](capacity: Int = 0) -> Channel[T]`
- `send(ch: Channel[T], msg: T) -> Unit`
- `recv(ch: Channel[T]) -> T`
- `select(cases: List[Case], timeout?: Duration) -> CaseResult`
- `spawn(task: () -> T, options?: TaskOptions) -> Task[T]`
- `Task.cancel() -> Unit`
- `Task.result(timeout?: Duration) -> Result[T, TimeoutError]`

## ai.prompt (adapters for LLM runtimes)

- `render(system: String, user: String, guidelines?: List[String], policy?: Policy, vars?: Map[String,Any]) -> Prompt`
- `call(model: String, prompt: Prompt, temperature?: Float, max_tokens?: Int) -> LLMResponse`

## core.db

- `connect(driver: String, dsn: String, options?: Map[String,Any]) -> DbConnection`
- `DbConnection.query(sql: String, params?: List[Any]) -> ResultSet`
- `DbConnection.execute(sql: String, params?: List[Any]) -> Int`
- `DbConnection.transaction(fn: (Tx) -> T) -> T`
- `Tx.query(sql: String, params?: List[Any]) -> ResultSet`
- `Tx.commit() -> Unit`
- `Tx.rollback() -> Unit`

## core.crypto

- `hash.sha256(data: Bytes | String) -> Bytes`
- `hash.sha512(data: Bytes | String) -> Bytes`
- `hmac.sha256(key: Bytes, data: Bytes | String) -> Bytes`
- `cipher.aes.gcm_encrypt(key: Bytes, iv: Bytes, plaintext: Bytes, aad?: Bytes) -> (Bytes, Bytes)`
- `cipher.aes.gcm_decrypt(key: Bytes, iv: Bytes, ciphertext: Bytes, tag: Bytes, aad?: Bytes) -> Bytes`
- `random.bytes(len: Int) -> Bytes`
- `signature.ed25519.sign(private_key: Bytes, message: Bytes) -> Bytes`
- `signature.ed25519.verify(public_key: Bytes, message: Bytes, sig: Bytes) -> Bool`

> **Safety note**: stdlib **must not** return or expose raw chain-of-thought. Responses should be *answers* with optional short rationales and sources.
