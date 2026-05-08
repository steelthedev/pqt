Length-prefixed framing is used (rather than newline-delimited) because shell commands and log lines may legitimately contain newlines. The receiver reads exactly 4 bytes, parses the length, then reads exactly that many more bytes — no escaping, no ambiguity.

Messages are JSON with a `serde`-tagged enum representation. Currently implemented:

**Request**
```json
{"op": "ping"}
```

**Response**
```json
{"kind": "pong"}
```

```json
{"kind": "error", "message": "..."}
```

New operations are added by extending the `Request` and `Response` enums in `src/protocol.rs`. Both binaries pick up the new variants automatically.

## Building

Requires Rust 1.75 or newer.

```bash
git clone <repo-url> pq
cd pq
cargo build --release
```

Binaries land in `target/release/pq` and `target/release/pqd`.

## Running

In one terminal, start the daemon:

```bash
cargo run --bin pqd
```

You should see a log line indicating the daemon is listening. It will block on accepting connections.

In another terminal, run the client:

```bash
cargo run --bin pq
```

The client sends a `Ping`, the daemon replies with `Pong`, and the client prints the response and exits. The daemon stays up to handle further connections.

## Testing

Manual smoke test:

```bash
# Terminal 1
cargo run --bin pqd

# Terminal 2
cargo run --bin pq    # should print: daemon said: Pong
```

Inspect the socket:

```bash
ls -la ~/.local/share/pq/pq.sock   # leading 's' indicates a socket
file ~/.local/share/pq/pq.sock     # should report: socket
```

Automated tests:

```bash
cargo test
```

Lint and formatting checks (run before every commit):

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## Roadmap

- [x] **PQ-001a** — UDS transport with length-prefixed JSON framing; Ping/Pong handshake
- [ ] **PQ-001b** — `add` and `list` subcommands; in-memory queue
- [ ] **PQ-001c** — Job execution with parallelism cap; per-job log capture
- [ ] **PQ-001d** — `logs`, `kill`, `rm` subcommands
- [ ] **PQ-001e** — State persistence and recovery across daemon restarts
- [ ] **PQ-001f** — `pause` / `resume`; finished-job history cap

Out of scope for v1: distributed scheduling, job dependencies, web UI, resource limits (cgroups), authentication.

## Design decisions

A few choices worth recording while they're fresh:

**Two binaries instead of one with a `pq daemon` subcommand.** Either approach works (pueue uses the single-binary model). The split makes the client/daemon boundary compiler-enforced, keeps each binary small, and makes process management (`pgrep pqd`, `systemctl status pqd`) unambiguous.

**JSON over a faster format like `bincode`.** Message volume is low and human-readability during debugging is worth more than encoding speed.

**Length-prefixed framing over newline-delimited.** Commands and log output contain newlines.

**`anyhow` everywhere instead of `thiserror`.** No public API to design errors for; this is a single-binary internal tool.

## License

TBD.
