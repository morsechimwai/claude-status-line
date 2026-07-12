# Context

Glossary of the language used across `ccstatus` — the CLI and its landing page.
Terms here are canonical; use them exactly in code, copy, and docs.

## Glossary

- **ccstatus** — the tool. The single Rust binary and its distribution packages.
  Never "cc-status" or "ccstatusline".

- **status line** (two words) — the bottom strip of Claude Code that ccstatus
  renders. `statusLine` (one word, camelCase) refers *only* to the Claude Code
  `settings.json` key. Do not write "statusline" as prose.

- **braille bar** — the hi-res usage gauge drawn from braille glyphs. Marketing
  copy may call it a "gauge", but "braille bar" is canonical (the code calls it
  `braille_bar`). A **block bar** is the same bar rendered with solid block
  glyphs instead, selectable in config.

- **rate-limit window** — one of the two usage windows ccstatus shows: the
  **5-hour window** and the **7-day window**. Not "quota" or a bare "limit".

- **preset** — a named color scheme (orange, blue, green, purple, mono). Not a
  "theme". A preset sets the fill/track/dim colors of the bars and labels.

- **plan label** — the short text beside the model (e.g. `Max (20x)`),
  auto-detected from the account's rate-limit tier. Not "tier" or "subscription"
  in user-facing copy.

- **cold-start cache** — the last-known usage ccstatus writes to disk and shows
  instantly when a new session starts, before any API call returns. Not "offline
  mode" — live data always overrides it.
