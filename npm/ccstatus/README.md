# ccstatus (npm)

Fast Claude Code status line with cached usage bars. This package downloads the
prebuilt native binary for your platform (via an `@ccstatus/<platform>` optional
dependency — no install scripts, no build step).

```bash
npm  install -g ccstatus
pnpm add     -g ccstatus
```

Then point Claude Code at it in `~/.claude/settings.json`:

```json
"statusLine": { "type": "command", "command": "ccstatus", "padding": 1 }
```

Prefer a pure native binary (no Node launcher)? Use Homebrew or Cargo:

```bash
brew install morsechimwai/tap/ccstatus
cargo install ccstatus
```

Source & docs: https://github.com/morsechimwai/claude-status-line
