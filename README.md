# ccstatus

Fast Claude Code status line with usage bars. Renders three rows — model /
context tokens, the 5-hour rate-limit window, and the 7-day window — and shows
**cached last-known usage instantly on session start** instead of blanks.

Single Rust binary, no runtime dependencies.

```
Opus 4.8 (1M context)  ████░░░░░░░░  0% | 0/1.0m
Current                █████░░░░░░░ 42% | 3:30 PM
Weekly                 ██░░░░░░░░░░ 18% | Jul 14, 3:30 PM
```

## Install

```bash
cargo install ccstatus
```

Or download a prebuilt binary from the [Releases](https://github.com/morsechimwai/claude-status-line/releases) page and put it on your `PATH`.

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
