# ccstatus

Fast Claude Code status line with usage bars. Renders the model, your context
window, and both rate-limit windows (5-hour and 7-day) — with bars that turn
**green → amber → red** as you approach a limit, and **cached last-known usage
shown instantly on session start** instead of blanks.

Single Rust binary, no runtime dependencies.

```
Opus 4.8 (1M context)
Context  ████░░░░░░░░   34%       ↑280k ↓60k / 1.0m
5h       ███████░░░░░   58% left  2h 15m
7d       █░░░░░░░░░░░   12% left  4d 5h
```

The **context** row fills as your context window fills, showing the input/output
token split. The **rate-limit** rows show headroom — how much you have *left*
before the 5-hour and 7-day limits, with the reset countdown beside it. Bars turn
red as a limit gets close (a short red bar means little left). Colors, thresholds,
bar glyphs, labels, and which rows show are all configurable — see
[`config.example.toml`](config.example.toml).

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
