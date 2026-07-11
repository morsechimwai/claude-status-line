# ccstatus v0.2.0–0.3.0 — UI redesign

**Date:** 2026-07-11
**Status:** Approved
**Supersedes the presentation layer of** `2026-07-11-ccstatus-design.md` (data model, cache,
input contract, and distribution are unchanged).

## Chosen combination

From the design-exploration gallery, the selected treatment is **B1 + C1 + L2 + I3**:

- **B1 — light track:** filled `█`, empty `░`.
- **C1 — threshold color:** each bar is colored by that row's own percentage.
- **L2 — aligned columns:** short fixed-width labels so bars, percentages, and values line up.
- **I3 — token detail:** the context row shows the input/output token split.

## Rendered target

```
Opus 4.8 (1M context)
Context  ████░░░░░░░░  34%  ↑280k ↓60k / 1.0m
5h       █████░░░░░░░  42%  resets in 2h 15m
7d       ██░░░░░░░░░░  18%  resets in 4d 6h
```

Four lines: a bold model-name header, then three aligned gauge rows.

## Layout

Each gauge row: `<bold label padded to 8> <bar> <two spaces> <dim right-aligned pct>%  <two spaces> <dim value>`.

- Labels default to `Context`, `5h`, `7d` (short, so the columns align). Configurable.
- Percentage is right-aligned in a 3-wide field: `{:>3}%` → `  8%`, ` 34%`, `100%`.
- The model header is the bold model display name alone on line 1. Toggle with `[layout] model_header`.

## Color — threshold

The bar's **fill** color is chosen from the row's percentage; the empty track stays gray.

| Band | Condition | Default xterm-256 |
|---|---|---|
| good | `pct < warn_at` | `71` (green) |
| warn | `warn_at <= pct < crit_at` | `179` (amber) |
| crit | `pct >= crit_at` | `167` (red) |
| track | (empty cells) | `240` (gray) |
| dim | pct/value text | `245` |

Defaults: `warn_at = 50`, `crit_at = 80`. All colors and both cutoffs are configurable. A solid
single-color bar is achieved by setting `good = warn = crit`.

## Information per row

- **Context row value:** `↑{input} ↓{output} / {size}`, each count via `fmt_tokens`
  (e.g. `↑280k ↓60k / 1.0m`). Data comes from `context_window.{total_input_tokens,
  total_output_tokens, context_window_size}`.
- **5h / 7d row value:** `resets in {countdown}` where `countdown` is `fmt_countdown` of
  `resets_at - now`. If `resets_at` is absent **or already in the past** (rolled-over cache),
  show `--` instead.

  **Data constraint:** the Claude Code status-line JSON exposes only `used_percentage` and
  `resets_at` for each rate-limit window — there is **no per-window token count**. So the
  "token detail" (I3) applies to the context row only; the rate-limit rows carry the countdown.

### `fmt_countdown(seconds)`

- `>= 1 day`: `"{d}d {h}h"` (e.g. `4d 6h`)
- `>= 1 hour`: `"{h}h {m}m"` (e.g. `2h 15m`)
- `>= 1 minute`: `"{m}m"` (e.g. `45m`)
- `< 1 minute`: `"<1m"`

The caller passes only positive durations; non-positive or absent resets render `--`.

## Config additions

```toml
[colors]
track = 240
dim   = 245
good  = 71
warn  = 179
crit  = 167

[thresholds]
warn_at = 50
crit_at = 80

[bar]
width  = 12
filled = "█"
empty  = "░"     # light track is now the default

[layout]
model_header = true

[labels]
context = "Context"
current = "5h"
weekly  = "7d"
```

- The old `[colors] fill = 173` key is removed (replaced by `good/warn/crit`).
- `[labels] context` is new; `current`/`weekly` defaults change from `Current`/`Weekly` to `5h`/`7d`.
- `bar.empty` default changes from `█` to `░`.

## `render::Style` shape

`Style { track: u8, dim: u8, good: u8, warn: u8, crit: u8, warn_at: u8, crit_at: u8, width: usize, filled: String, empty: String }`.

- `fill_color(pct, &Style) -> u8` returns good/warn/crit by band.
- `bar(pct, &Style)` fills with `fill_color(pct)` glyphs then track glyphs.
- `row(label, pct: Option<u8>, value, &Style)` emits one aligned gauge line.
- `header(model) -> String` emits the bold model line (no style needed — always bold).

## Addendum (v0.3.0) — hybrid "remaining" framing for rate limits

UX research (API rate-limit convention is `X-RateLimit-Remaining`; GitHub/OpenAI
expose remaining; progress-bar HCI notes remaining-framing can raise anxiety,
mitigated here by threshold color + countdown) led to a **hybrid** framing:

- **Context row: unchanged** — used/fullness (`34%`, bar fills, `↑in ↓out / size`).
  A container filling toward compaction reads naturally as "how full".
- **5h / 7d rows: remaining** — the bar length and label show what's *left*
  (`58% left`), and the value is the bare countdown (`2h 15m`, no "resets in"
  prefix). The **color still reflects danger by the underlying used percentage**,
  so a nearly-exhausted window is a short *red* bar (little left = dangerous),
  while a fresh window is a long *green* bar.

```
Opus 4.8 (1M context)
Context  ████░░░░░░░░   34%       ↑280k ↓60k / 1.0m
5h       ███████░░░░░   58% left  2h 15m
7d       █░░░░░░░░░░░   12% left  4d 5h    ← red: little left
```

Render support: `bar_colored(bar_pct, color, &Style)` separates bar length from
fill color; `bar(pct, &Style)` is the used-mode convenience wrapper; and
`row_remaining(label, used, value, &Style)` renders the headroom rows (`used` is
the used percentage; it shows `100 - used`% left and colors by `used`).

## Addendum (v0.4.0) — braille bars, official-panel styling, plan label

This section **supersedes the v0.3.0 remaining/threshold addendum** above, per user
direction after iterating on the live output.

- **Bar: hi-res braille** (`[bar] braille = true`, default). Each cell is a braille
  glyph (base U+2800) with two horizontal sub-columns → `2*width` steps. Dots 7,8
  form an always-lit baseline (empty cells read as `⣀`); a lit left column adds
  dots 1,2,3, a lit right column adds 4,5,6. Cells with any lit column use the fill
  color, others the track color. `braille = false` falls back to the block bar
  (`filled`/`empty` glyphs).
- **Framing reverted to used** — every row shows the *used* percentage again
  (`28%`), not remaining. `row_remaining` and the threshold coloring are removed.
- **Single brand color, recolored to the official usage panel** — `[colors] fill`
  is one color (no good/warn/crit bands). Defaults changed to match Claude Code's
  usage panel: `fill = 68` (blue), `track = 17` (dark navy), `dim = 245`.
- **Optional plan label** — `[layout] plan` (default empty). When set, it is
  appended dim to the model header: `Opus 4.8 (1M context)  Max (20x)`. The
  status-line JSON does **not** expose the plan tier, so it is user-set config.
- **No Fable / per-model row** — the status-line JSON exposes only the aggregate
  `five_hour` and `seven_day` windows (confirmed against the Claude Code
  statusline docs); there is no per-model ("Fable") usage to render.

```
Opus 4.8 (1M context)  Max (20x)
Context  ⣿⣿⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀   22%  ↑180k ↓45k / 1.0m
5h       ⣿⣿⣿⣇⣀⣀⣀⣀⣀⣀⣀⣀   28%  resets in 2h 23m
7d       ⣿⣿⣿⣿⣀⣀⣀⣀⣀⣀⣀⣀   33%  resets in 2d 7h
```

`Style` becomes `{ track, dim, fill, width, filled, empty, braille }`;
`fill_color`/`row_remaining` are gone; `header(model, plan, &Style)` appends the
plan label.

## Out of scope / unchanged

Input parsing, the last-known-usage cache and its merge/persist logic, XDG paths, error-handling
(exit 0, no panics), and all distribution channels (brew/npm/cargo/release CI). Only the
presentation layer and config surface change. Version bumps to **0.2.0**.
