# macwin-example

An example consumer project for **[macwin](https://github.com/benelser/macwin)** —
the CLI that lets an Apple Silicon developer build, run, and debug Rust on every
platform (including Windows) from a Mac.

This crate is a small cross-platform IPC daemon: the same `serve_one` /
`send_recv` API runs on every macwin target — Windows named pipes, Unix sockets
on macOS/Linux — so it's a realistic thing to exercise macwin's run, smoke,
remote, and interactive-debug paths against.

## Get macwin first

```bash
# Prebuilt macOS binary (arm64/x86_64) from GitHub Releases:
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/benelser/macwin/releases/latest/download/macwin-installer.sh | sh

macwin install-deps      # one-time toolchain setup (~3 min)
```

Then clone this repo and run the commands below from its root. Full macwin docs
live in the [macwin repository](https://github.com/benelser/macwin).

## Run it locally

```bash
# Native run on this Mac:
macwin run --target mac-arm -- info
macwin run --target mac-arm -- ipc hello

# Cross-compile + run across the local matrix (linux via OrbStack docker):
macwin smoke --matrix mac-arm,linux-arm,linux-x86 -- ipc hello

# Inner loop: save → see test result in < 2s
macwin watch --target mac-arm --tests
```

## Run on Windows via GitHub Actions

```bash
gh auth login                                        # one time per machine
macwin remote init                                   # writes .github/workflows/macwin-remote.yml
git add .github && git commit -m "macwin remote runner" && git push
macwin run --target win-arm --runner github-actions -- ipc hello
```

The committed `.github/workflows/macwin-remote.yml` is exactly what `macwin
remote init` produces — included here so you can see the runner workflow without
generating it yourself.

## Interactive remote debug

`macwin debug` opens an interactive session on a runner — SSH/exec/push/pull/code
— tunnelled back through a relay you stand up yourself (bring-your-own-relay; see
[macwin's relay self-hosting guide](https://github.com/benelser/macwin/blob/main/docs/src/guides/relay-self-hosting.md)).

```bash
macwin relay init                                    # generate a secret + config for your relay
macwin debug start --target win-arm                  # interactive shell on a Windows runner
```

Config lives in `.macwin/config.toml`. This repo ships a documented template at
[`.macwin/config.toml.example`](.macwin/config.toml.example) — copy it and fill
in your relay:

```bash
cp .macwin/config.toml.example .macwin/config.toml   # then set server + MACWIN_TUNNEL_SECRET
```

Keep the secret out of the checked-in file — provide it via `MACWIN_TUNNEL_SECRET`
or your per-user `~/.config/macwin/config.toml`. The real `.macwin/config.toml` is
gitignored here for that reason.

## Project shape

- `src/lib.rs` — IPC primitives (`serve_one`, `send_recv`, `info`)
- `src/main.rs` — CLI: `info`, `ipc <message>`
- `tests/integration.rs` — cross-platform IPC roundtrip tests via [`interprocess`](https://crates.io/crates/interprocess)
- `.github/workflows/macwin-remote.yml` — the GitHub Actions runner workflow
- `.macwin/config.toml.example` — template for `macwin debug` relay config
- `rust-toolchain.toml` — pins the toolchain channel + components macwin uses
