// Single source of truth for user-facing product facts.
// VERSION is asserted equal to Cargo.toml by product.test.ts.
// PRESETS / CONFIG_TOML mirror src/config.rs — update together.
export const VERSION = "0.6.0";

export const REPO = "https://github.com/morsechimwai/claude-status-line";

export const INSTALL = [
  { id: "brew", label: "Homebrew", comment: "# macOS / Linux",
    command: "brew install morsechimwai/tap/ccstatus" },
  { id: "npm", label: "npm", comment: "# any node package manager",
    command: "npm install -g ccstatus-cli" },
  { id: "cargo", label: "Cargo", comment: "# builds from source",
    command: "cargo install ccstatus" },
] as const;

export const PRESETS = [
  { id: "orange", label: "orange", color: "#d7875f" },
  { id: "blue", label: "blue", color: "#6a9bd7" },
  { id: "green", label: "green", color: "#6aae6a" },
  { id: "purple", label: "purple", color: "#9a7fd0" },
  { id: "mono", label: "mono", color: "#b3b3b3" },
] as const;

export const SETTINGS_SNIPPET =
  '"statusLine": { "type": "command", "command": "ccstatus", "padding": 1 }';

export const FEATURES = [
  { icon: "sparkle", title: "Cached, never blank",
    body: "Last-known usage written to disk and read on cold start. No blanks, no waiting for the first API call." },
  { icon: "bars", title: "Hi-res braille bars",
    body: "Sub-cell braille dots pack 8× the resolution of block bars. Toggle to solid blocks if you prefer." },
  { icon: "clock", title: "5h & 7d windows",
    body: "Both rate-limit windows with live countdowns to reset — the numbers Claude Code actually limits on." },
  { icon: "check", title: "Plan auto-detect",
    body: "Reads your account's plan and maps it to a short plan label like Max (20x)." },
  { icon: "shield", title: "Zero dependencies",
    body: "One static Rust binary. No Node, no Python, no network calls, no auth. It just reads and draws." },
  { icon: "sliders", title: "Fully configurable",
    body: "Colors, bar glyph, labels, plan text, and which rows show — one optional config.toml." },
] as const;

export const FAQ = [
  { q: "What is ccstatus?",
    a: "ccstatus is a status line for Claude Code: a single Rust binary that renders your model, context window, and both rate-limit windows as hi-res braille bars." },
  { q: "How do I install ccstatus?",
    a: "Install with Homebrew (brew install morsechimwai/tap/ccstatus), npm (npm install -g ccstatus-cli), or Cargo (cargo install ccstatus), then point Claude Code's statusLine setting at the ccstatus command." },
  { q: "Does ccstatus make network calls?",
    a: "No. ccstatus makes no network calls and needs no auth. It only reads local status-line data and a small on-disk cache, then draws the bars." },
  { q: "Which limits does ccstatus show?",
    a: "It shows the aggregate 5-hour and 7-day rate-limit windows with live countdowns, plus your context-window usage. Per-model limits are not exposed to status lines, so they are not shown." },
  { q: "How does the cold-start cache work?",
    a: "Claude Code only knows your usage after its first API call, so a fresh session would show blanks. ccstatus writes last-known usage to disk and reads it back instantly on cold start; live data always overrides and refreshes it." },
] as const;

export const CONFIG_TOML = `# ~/.config/ccstatus/config.toml — every key optional
[colors]
preset = "orange"   # blue · green · purple · mono

[bar]
width   = 12
braille = true       # false = solid block bar

[rows]
context = true
current = true       # the 5-hour window
weekly  = true       # the 7-day window`;
