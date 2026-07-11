# ccstatus

Fast Claude Code status line with usage bars. Renders the model, your context
window, and both rate-limit windows (5-hour and 7-day) as hi-res braille gauges —
styled to match Claude Code's own usage panel — with **cached last-known usage
shown instantly on session start** instead of blanks.

Single Rust binary, no runtime dependencies.

```
Opus 4.8 (1M context)  Max (20x)
Context  ⣿⣿⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀   22%  ↑180k ↓45k / 1.0m
5h       ⣿⣿⣿⣇⣀⣀⣀⣀⣀⣀⣀⣀   28%  resets in 2h 23m
7d       ⣿⣿⣿⣿⣀⣀⣀⣀⣀⣀⣀⣀   33%  resets in 2d 7h
```

Each row shows usage percent and a hi-res braille bar in the Claude blue; the
context row adds the input/output token split, and the rate-limit rows count down
to their reset. An optional plan label (e.g. `Max (20x)`) sits next to the model.
Colors, the braille/block bar, labels, the plan label, and which rows show are all
configurable — see [`config.example.toml`](config.example.toml).

> The plan tier and per-model ("Fable") limits aren't in the status-line JSON that
> Claude Code provides, so the plan is an optional label you set in config, and
> only the aggregate 5-hour and 7-day windows are shown.

## Install

**Homebrew** (macOS / Linux):

```bash
brew install morsechimwai/tap/ccstatus
```

**npm / pnpm / yarn / bun** (downloads the prebuilt binary for your platform):

```bash
npm  install -g ccstatus
pnpm add     -g ccstatus
```

**Cargo** (builds from source):

```bash
cargo install ccstatus
```

Or download a prebuilt binary from the [Releases](https://github.com/morsechimwai/claude-status-line/releases) page and put it on your `PATH`.

> The Homebrew and Cargo installs are pure native binaries. The npm package wraps
> the same binary in a thin Node launcher (~tens of ms startup) for people who
> prefer the npm toolchain.

## Configure Claude Code

Add to `~/.claude/settings.json`:

```json
"statusLine": { "type": "command", "command": "ccstatus", "padding": 1 }
```

## Configuration (optional)

Copy `config.example.toml` to `~/.config/ccstatus/config.toml` and edit. Every
key is optional and falls back to the defaults that reproduce the look above.

## How the cache works

Claude Code only knows your rate-limit usage after its first API call, so a
fresh session would otherwise show `--`. `ccstatus` writes the last-known
usage to `~/.cache/ccstatus/usage.json` and reads it back on cold start — no
network calls, no auth. Live data always overrides the cache and refreshes it.

## License

MIT
