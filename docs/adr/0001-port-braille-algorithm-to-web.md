# 1. Port the braille bar algorithm to the web, don't run the binary

Date: 2026-07-12
Status: Accepted

## Context

The landing page's hero is an interactive terminal that renders the real
**braille bar** (see CONTEXT.md). The goal is fidelity — the bars on the page
must match, glyph for glyph, what the `ccstatus` binary emits in a terminal —
while staying interactive: switching **preset** colors and toggling braille vs
block glyphs live in the browser.

The bar geometry comes from a small, stable algorithm in `src/render.rs`
(`braille_bar`): for a given percent and width, `filled = (pct * 2*width + 50) /
100` sub-columns are lit; each cell is a glyph `U+2800 + bits` where bits combine
a baseline (`0xC0`) with the left (`0x07`) and/or right (`0x38`) sub-column.

Three ways to get faithful bars onto a static page:

1. **Run the binary at build time** — invoke `ccstatus` with sample JSON, capture
   its ANSI output, parse to HTML. Requires a Rust/cargo build step in the web
   pipeline, and produces static snapshots — one per preset — so live recoloring
   and the braille/block toggle would each need pre-rendered variants.
2. **Static snapshot image** — accurate but not interactive at all.
3. **Port the algorithm to JavaScript** — reimplement the ~10-line function in the
   site so bars are generated in the browser from percent + width.

## Decision

Port the algorithm (option 3). The site computes braille (and block) bars in
JavaScript from the same formula as `render.rs`. Presets only change color (a CSS
custom property); the braille/block toggle only swaps glyphs — geometry is always
computed by the ported function, so it stays binary-accurate for any percent.

## Consequences

- **The algorithm now lives in two places** — `src/render.rs` and the site. This
  duplication is deliberate. Do not "de-duplicate" it by deleting the JS copy; the
  web build has no Rust toolchain and must not gain one for a landing page.
- **If the braille algorithm changes in `render.rs`, the web copy must be updated
  in lockstep.** A code comment in each copy points at the other. The formula has
  been stable and is unlikely to change, which is what makes duplication cheap.
- No cargo/Rust dependency in the web build; static export stays pure Node.
- The port is covered by a test that checks a few known percent→glyph-string cases
  (e.g. 9% → `⣿⣀⣀…`, 22% → `⣿⣿⣇⣀…`) so drift from the Rust output is caught.
