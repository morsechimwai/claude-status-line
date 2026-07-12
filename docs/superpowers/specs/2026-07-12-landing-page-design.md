# ccstatus Landing Page — Design Spec

**Date:** 2026-07-12
**Status:** Approved (design + visual mockup), pending grill-with-docs
**Mockup:** https://claude.ai/code/artifact/75228b35-d855-4d5e-9fc7-fcda7c3c097f

## Overview

A single-page marketing/landing site for `ccstatus` — the Claude Code status
line CLI. The page demonstrates the real product (an interactive terminal that
renders the actual status line, recolorable live), drives installs across three
package managers, and is engineered for search, generative, and answer-engine
discovery (SEO / GEO / AEO). Target quality bar: **impeccable polish** — perfect
Lighthouse, self-hosted fonts, zero external requests, full accessibility.

## Goals

- Show the product working ("โชว์ของจริง") via an interactive terminal demo, not a
  static screenshot.
- Convert to install: one obvious CTA, three copyable package-manager commands.
- Rank and get cited: strong SEO metadata, structured data, and machine-readable
  answer content for LLM answer engines.
- Match the product's identity: terminal-native, dark-only, Claude-orange.

## Non-Goals

- No blog, docs site, changelog, or auth. Single page only.
- No server runtime — fully static export.
- No analytics/tracking in v1 (can add later; keep zero third-party scripts for now).
- No light theme — the page deliberately commits to the terminal (dark) world.

## Location & Stack

- Lives in `site/` subdir of this repo (monorepo with the Rust CLI).
- **Next.js 16** (App Router, React Server Components), **Tailwind CSS v4**,
  TypeScript.
- `output: 'export'` — static HTML/CSS/JS. No server functions.
- **JetBrains Mono**, self-hosted via `next/font/local` (variable font subset).
  No font-CDN request → no FOIT, no external dependency, better AEO/perf.
- One client component for interactivity (preset switcher, install tabs, copy).
  Everything else is server-rendered static.
- Deploy: Vercel (root = `site/`) or GitHub Pages — static works on both.

### Version / content sync

The install commands, preset names, and config example must match the CLI.
Single source of truth in `site/lib/product.ts` (install commands, presets,
feature copy). A short note in the repo README points contributors there when
they bump the version or rename a package.

## Visual System (dark-only, OLED)

Neutrals carry a slight warm bias toward the accent — chosen, not defaulted.

| Token | Value | Source |
|-------|-------|--------|
| `--bg` | `#0a0b0e` | terminal bg (preview.svg) |
| `--surface` | `#12141a` | cards |
| `--surface2` | `#0f1116` | terminal body |
| `--border` | `#20242c` | preview.svg |
| `--fg` | `#eceae4` | preview.svg |
| `--dim` | `#7c8088` | preview.svg |
| `--dim2` | `#565c66` | preview.svg |
| `--track` | `#3a3f47` | empty bar cells |
| `--accent` | `#d7875f` (Claude orange, 256-color 208) | brand |
| `--prompt` | `#6f9f6a` | shell prompt green |

Preset accent values (drive the live switcher, matching real config presets):
orange `#d7875f` · blue `#6a9bd7` · green `#6aae6a` · purple `#9a7fd0` · mono `#b3b3b3`.

- **Type:** JetBrains Mono only — single-face commit reinforces the terminal
  identity. Scale: 12 / 13 / 14 / 15 (body) / 24 / clamp hero. Weights 400/500/700.
  `text-wrap: balance` on headings; body measure ≤ ~42ch.
- **Effects:** restrained orange glow on the CTA and filled bar cells
  (`text-shadow` / soft box-shadow). Radial accent wash behind the hero. Nothing
  else glows — spend boldness in one place.
- **Motion:** 150–300ms; bars fill with a staggered per-cell reveal on first
  view (IntersectionObserver), exit-faster easing. All motion gated behind
  `prefers-reduced-motion` — reduced users get final state instantly.
- **Layout:** single column, `max-width: 56rem`, mobile-first. 8px spacing rhythm.

## Sections

### 1. Hero + interactive terminal demo (the thesis)

- Eyebrow: `Claude Code status line · single Rust binary`.
- Headline (approved): "Your usage, **already on screen** before the first call."
- Sub: one sentence on cached-usage + braille gauges.
- **Terminal card**: macOS titlebar dots, `morse ~/project $ claude` prompt,
  model header `Opus 4.8 (1M context)  Max (20x)`, three rows (Context / 5h / 7d)
  each = label + braille bar + `%` + meta (token split / countdown). Values from
  the real preview: 22% ↑180k↓45k/1.0m, 9% resets 2h1m, 31% resets 2d7h.
- **Preset switcher** (pills, keyboard-navigable radio group): recolors the bar
  fill + CTA glow live via a CSS custom property. Default orange.
- **Braille/block toggle** (impeccable-polish add): flips `⣿⣀` ↔ `█░` to show the
  `braille = false` config option live.
- Primary CTA `$ install ccstatus` (scrolls to install), secondary GitHub link.
- Microcopy: `MIT · macOS / Linux / Windows · no runtime deps`.

### 2. Install (conversion)

- Three tabs: Homebrew / npm / Cargo. Each shows the exact command with a
  comment line, plus a **copy** button (clipboard + "copied ✓" feedback).
  - brew: `brew install morsechimwai/tap/ccstatus`
  - npm: `npm install -g ccstatus-cli`
  - cargo: `cargo install ccstatus`
- Below: the `~/.claude/settings.json` `statusLine` snippet (also copyable).

### 3. Features grid

Six cards, Lucide-style inline SVG icons (single stroke width, accent color):
cached-never-blank · hi-res braille bars · 5h & 7d windows · plan auto-detect ·
zero dependencies · fully configurable. One-sentence body each, accurate to the
README (no invented claims).

### 4. Config showcase + How-the-cache-works

- Syntax-highlighted `config.toml` excerpt (real keys: colors/bar/rows).
- Prose block explaining the cache mechanism (cold-start instant, no network,
  live overrides). This block doubles as AEO answer content.

## Accessibility

- All interactive elements keyboard-operable; visible focus rings (accent, 2px).
- Preset switcher = ARIA radio group; install tabs = ARIA tablist with arrow-key
  nav. Copy buttons announce state.
- Contrast: fg/bg and dim/bg verified ≥ 4.5:1 (AAA where possible).
- `prefers-reduced-motion` respected. Semantic headings h1→h2→h3, no skips.
- Terminal demo has an accessible text alternative (visually-hidden summary of
  what it shows) so screen readers and crawlers get the content.

## SEO / GEO / AEO

**SEO**
- Next Metadata API: title, description, canonical, keywords, author.
- Open Graph + Twitter card. **OG image**: a rendered terminal card (generated
  statically at build, 1200×630, matching the hero) — not a generic banner.
- `sitemap.xml` + `robots.txt` (allow all, point to sitemap).
- Semantic HTML5 landmarks; descriptive alt text; fast LCP (static, self-host
  font, inlined critical CSS via Next).

**GEO (generative engine optimization)**
- `public/llms.txt` — concise machine-readable summary: what ccstatus is, install
  commands, key facts, links. Lets LLM crawlers ingest the product accurately.
- Clear, factual, declarative copy (LLMs quote declarative sentences). Entity
  clarity: always "ccstatus, a status line for Claude Code."

**AEO (answer engine optimization)**
- **JSON-LD** `SoftwareApplication` (name, OS, license, category, downloadURL,
  offers=free) + `FAQPage` for the common questions.
- A lightweight **FAQ** block (accordion, semantic `<details>`): "What is
  ccstatus?", "How do I install it?", "Does it make network calls?", "Which
  plans/limits does it show?", "How does the cache work?". Answers are
  self-contained, quotable, and mirror the JSON-LD FAQ entries.

## Performance (impeccable polish target)

- Lighthouse 100/100/100/100 target. Static export, self-hosted subset font,
  zero external requests, no third-party scripts.
- Reserve space for all async/animated content (CLS ≈ 0). `transform`/`opacity`
  only for motion. Images (OG) not on critical path.

## Success Criteria

- `pnpm build` produces a static `site/out/` that renders identically to the
  approved mockup, on desktop (1440/1024) and mobile (768/375), no horizontal
  scroll.
- Preset switcher, braille/block toggle, install tabs, and copy buttons all work
  with mouse and keyboard.
- Lighthouse ≥ 95 on all four categories (target 100).
- View-source contains valid `SoftwareApplication` + `FAQPage` JSON-LD; `llms.txt`,
  `sitemap.xml`, `robots.txt` present.
- All product facts (commands, presets, config keys, limits) match the CLI.

## Open Decisions

- **Domain / canonical URL.** Needed for canonical, OG `url`, sitemap, JSON-LD.
  Default to a `NEXT_PUBLIC_SITE_URL` env var so it can be set at deploy time;
  fallback placeholder until a domain is chosen. Confirm during grilling.
- **Deploy target** (Vercel vs GitHub Pages) — both supported by static export;
  pick before writing CI.
