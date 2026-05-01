# macwin-debug-mvp

Cross-platform Rust IPC project. Same `serve_one` / `send_recv` API runs on
every macwin target — Windows named pipes, Unix sockets on macOS/Linux.

## Try it

```bash
# Native run on this Mac:
macwin run --target mac-arm -- info
macwin run --target mac-arm -- ipc hello

# Cross-compile + run on every supported platform via macwin's runner:
macwin smoke --matrix mac-arm,linux-arm,linux-x86,win-arm -- ipc hello

# Inner loop: save → see test result < 2s
macwin watch --target mac-arm --tests
```

## Adding remote-Windows execution

```bash
gh auth login                                        # one time per machine
macwin remote init                                   # writes .github/workflows/macwin-remote.yml
git add .github && git commit -m "macwin remote runner" && git push
macwin run --target win-arm --runner github-actions -- ipc hello
```

## Project shape

- `src/lib.rs` — IPC primitives (`serve_one`, `send_recv`, `info`)
- `src/main.rs` — CLI: `info`, `ipc <message>`
- `tests/integration.rs` — cross-platform IPC roundtrip tests via [`interprocess`](https://crates.io/crates/interprocess)
- `rust-toolchain.toml` — pins channel + targets macwin uses
