# ccstatus — Design Spec

**Date:** 2026-07-11
**Status:** Approved
**Repo:** claude-status-line

## Summary

`ccstatus` is a fast, single-binary Rust CLI that renders the Claude Code
status line. It reads the session JSON that Claude Code pipes to the
`statusLine` command on stdin, and prints three usage rows to stdout:
model/context tokens, the 5-hour ("Current") rate-limit window, and the
7-day ("Weekly") rate-limit window.

It is an open-source, drop-in replacement for a personal bash script
(`usage-statusline.sh`). The key improvement over the bash version: usage
bars render **immediately on session start** by falling back to cached
last-known values, instead of showing `--` until Claude Code makes its
first API call.

## Goals

- **Speed:** binary startup in low single-digit milliseconds. The command
  is invoked on every status-line render, so an interpreter (Node/Python)
  boot cost is unacceptable.
- **Instant cold start:** on a fresh session (no live rate-limit data yet),
  show cached last-known usage instead of blanks.
- **Never break Claude Code:** any error path renders a best-effort line and
  exits 0. Never panic, never emit garbage.
- **Configurable:** consumers can override colors, bar width, glyphs, row
  visibility, and labels via a TOML file. Defaults reproduce the original
  look exactly.
- **Distributable:** `cargo install`, Homebrew, npm wrapper, GitHub Release
  binaries, and a `curl | sh` installer.

## Non-Goals

- No caveman badge. That was a personal, separate concern in the original
  script and is intentionally dropped.
- No live API fetching / auth. Cold-start data comes from the local cache
  only — never from a network call.
- No interactivity, TUI, or long-running process. One-shot: read stdin,
  print, exit.

## Input Contract

Claude Code pipes a JSON object to the command on stdin. The fields
consumed (all optional; missing → sensible default):

| JSON path | Type | Default | Use |
|---|---|---|---|
| `.model.display_name` | string | `"Claude"` | Row 1 label |
| `.context_window.used_percentage` | number | `0` | Row 1 bar % |
| `.context_window.context_window_size` | number | `0` | Row 1 value (denominator) |
| `.context_window.total_input_tokens` | number | `0` | Row 1 value (numerator part) |
| `.context_window.total_output_tokens` | number | `0` | Row 1 value (numerator part) |
| `.rate_limits.five_hour.used_percentage` | number | absent → `--` | Row 2 bar % |
| `.rate_limits.five_hour.resets_at` | epoch seconds | absent → `--` | Row 2 value |
| `.rate_limits.seven_day.used_percentage` | number | absent → `--` | Row 3 bar % |
| `.rate_limits.seven_day.resets_at` | epoch seconds | absent → `--` | Row 3 value |

Percentages may arrive as floats; they are floored to integers for display.
"absent" means the field is null or missing — distinct from a present `0`.

## Output Contract

Exactly the visible rows (default config), ANSI-colored:

```
Opus 4.8   ████░░░░░░░░  0% | 0/1.0m
Current    █████░░░░░░░ 42% | 3:30 PM
Weekly     ██░░░░░░░░░░ 18% | Jul 14
```

Note: `░` above is a **visual stand-in** for the empty track. The default
`empty` glyph is a solid `█` rendered in the `track` color (matching the
original script and the reference screenshot — a solid gray track, not a
light-shade character). Users who prefer a lighter track can set
`bar.empty = "░"` in config.

Each row: `<bold label, left-padded to 8> <bar> <dim right-aligned pct> | <dim value>`.

- **Row 1 value:** `<used tokens>/<context size>`, each formatted by
  `fmt_tokens` (`0` / `15k` / `1.0m`). Used tokens = input + output.
- **Row 2/3 value:** `resets_at` formatted by `fmt_reset`, or `--` when the
  value is absent.
- **pct display:** absent → `--`; present → integer + `%`.

## Architecture

Single crate, small focused modules. Each module has one purpose, a narrow
interface, and is unit-testable in isolation.

```
src/
  main.rs      // orchestration: stdin -> parse -> merge cache -> persist -> render -> stdout
  input.rs     // serde structs + parse of the Claude Code JSON
  cache.rs     // read/write last-known usage
  config.rs    // load TOML config + defaults
  render.rs    // bar(), fmt_tokens(), fmt_reset(), row(); builds the output lines
```

### `input.rs`
- Serde structs mirroring the Input Contract. All numeric fields tolerant of
  int-or-float and of missing keys (default to `None`/`0`).
- `parse(stdin: &str) -> Input`. On malformed JSON, returns an empty `Input`
  (all defaults) rather than erroring — the caller still renders a line.
- Distinguishes "rate-limit window absent" (`Option<Window>` is `None`) from
  "window present with 0%".

### `cache.rs`
- Location: `~/.cache/ccstatus/usage.json` (respects `XDG_CACHE_HOME`).
- Stores the last-known rate-limit windows: for each of `five_hour` and
  `seven_day`, the `used_percentage` and `resets_at`.
- `load() -> Option<CachedUsage>` — returns `None` on any read/parse failure
  (missing file, corrupt) without erroring.
- `store(&CachedUsage)` — best-effort write; failure is ignored (creates the
  directory if needed).

### `config.rs`
- Location: `~/.config/ccstatus/config.toml` (respects `XDG_CONFIG_HOME`).
- All keys optional; missing file or missing keys → defaults that reproduce
  the original bash look.

```toml
[colors]
fill  = 173   # 256-color index for the filled portion of the bar
track = 240   # 256-color index for the empty track
dim   = 245   # 256-color index for pct/value text

[bar]
width  = 12
filled = "█"
empty  = "█"

[rows]
context = true
current = true
weekly  = true

[labels]
current = "Current"
weekly  = "Weekly"
```

- `load() -> Config` — returns defaults on any read/parse failure.

### `render.rs`
Pure functions, no I/O, fully unit-testable. A `now: i64` (epoch seconds) is
passed in explicitly so date-dependent formatting is deterministic in tests.

- `fmt_tokens(n: u64) -> String` — `0` / `15k` / `1.0m`. `>=1_000_000` →
  `{:.1}m`; `>=1_000` → `{}k`; else the integer.
- `fmt_reset(epoch: Option<i64>, now: i64) -> String` — `--` when `None`;
  `"3:30 PM"` when the reset is on the same local calendar day as `now`;
  `"Jul 14, 3:30 PM"` otherwise.
- `bar(pct: u8, cfg) -> String` — clamps to 0..=100, computes filled cells as
  `(pct * width + 50) / 100` (rounding), emits `fill` glyphs then `track`
  glyphs wrapped in the configured colors.
- `row(label, pct: Option<u8>, value, cfg) -> String` — assembles one line
  with padding and colors. `pct == None` renders `--` and an empty (0%) bar.

### `main.rs` — merge & persist logic
1. Read all of stdin (empty string if none).
2. `input = input::parse(stdin)`.
3. `config = config::load()`.
4. **Merge:** for each rate-limit window (`five_hour`, `seven_day`):
   - If the window is present in `input` → use it, and mark cache dirty.
   - Else → load from `cache`. If a cached window's `resets_at` is already in
     the past relative to `now`, treat its percentage as `0` (the window has
     rolled over) but keep showing it; if there is no cached value, render
     `--`.
5. If any live window was present, `cache::store` the merged live windows.
6. Render the enabled rows and print to stdout.
7. Always exit 0.

Context (row 1) always comes from live stdin — it is per-session and not
cached.

## Error Handling

Every failure mode degrades gracefully and exits 0:

- Empty/malformed stdin → `Input` defaults → row 1 shows `0% | 0/0`, rows 2/3
  fall back to cache or `--`.
- Missing/corrupt config → defaults.
- Missing/corrupt cache → treated as no cache.
- Cache write failure → ignored.
- No `panic!` reachable from normal operation; `main` returns `()` and prints
  what it has.

## Testing

- **Unit (`render.rs`):**
  - `fmt_tokens`: `0`, `999`, `1_000`→`1k`, `15_500`→`15k`, `1_000_000`→`1.0m`,
    `1_500_000`→`1.5m`.
  - `fmt_reset`: `None`→`--`; same-day epoch→`"h:MM AM/PM"`; other-day epoch→
    `"Mon D, h:MM AM/PM"` — using an injected fixed `now`.
  - `bar`: 0%→all track, 100%→all fill, rounding boundary (e.g. 50% of 12
    cells), clamp of out-of-range input.
  - `row`: `None` pct → `--` + empty bar; present pct formatting/padding.
- **Integration:** feed sample JSON fixtures on stdin, assert exact stdout
  lines — cases: full live data, missing rate_limits (cache hit), missing
  rate_limits (cache miss → `--`), malformed JSON, config overrides applied.
- **Cache round-trip:** `store` then `load` returns equal data; corrupt file
  → `None`.

## Distribution

- **crates.io:** `cargo install ccstatus` (name verified available).
- **GitHub Releases:** cross-compiled binaries for macOS (arm64/x64), Linux
  (arm64/x64), Windows (x64).
- **Homebrew tap:** formula pointing at the release binary.
- **npm wrapper:** postinstall downloads the matching release binary (name
  verified available), for users who prefer `npm i -g ccstatus`.
- **`curl | sh` installer** in the README.
- **README:** install steps + the `settings.json` snippet:
  ```json
  "statusLine": { "type": "command", "command": "ccstatus", "padding": 1 }
  ```

## Open Questions

None. Ready for implementation planning.
