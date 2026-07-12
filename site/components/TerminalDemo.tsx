"use client";
import { useRef, useState } from "react";
import { brailleDotCells, blockCells, type Cell, type DotCell } from "@/lib/braille";
import { PRESETS } from "@/lib/product";

const ROWS = [
  { label: "Context", pct: 22, meta: "↑180k ↓45k / 1.0m" },
  { label: "5h", pct: 9, meta: "resets in 2h 1m" },
  { label: "7d", pct: 31, meta: "resets in 2d 7h" },
];

// Braille bar drawn as dots on a UNIFORM grid — every dot column is spaced by the
// same pitch as every other (no per-cell clustering), so the bar reads as one
// continuous dotted block like the real terminal font, not gapped cells. Layout
// matches src/render.rs geometry: baseline dots (bottom row, both sub-columns)
// always lit; each filled sub-column adds its 3 upper dots.
const PITCH = 3.4;              // uniform px pitch between adjacent dots (H and V)
const DOT_R = 1.15;            // fine dots, like the terminal
const ROWS_N = 4;
const SVG_H = ROWS_N * PITCH;

function BrailleBar({ dots }: { dots: DotCell[] }) {
  const cols = dots.length * 2;
  const w = cols * PITCH;
  const cx = (col: number) => (col + 0.5) * PITCH; // global column index → x
  const cy = (row: number) => (row + 0.5) * PITCH;
  const circles: React.ReactNode[] = [];
  dots.forEach((d, i) => {
    const xL = cx(2 * i), xR = cx(2 * i + 1);
    const color = d.filled ? "var(--accent)" : "var(--track)";
    circles.push(<circle key={`${i}b0`} cx={xL} cy={cy(3)} r={DOT_R} fill={color} />);
    circles.push(<circle key={`${i}b1`} cx={xR} cy={cy(3)} r={DOT_R} fill={color} />);
    if (d.left) for (let r = 0; r < 3; r++) circles.push(<circle key={`${i}l${r}`} cx={xL} cy={cy(r)} r={DOT_R} fill={color} />);
    if (d.right) for (let r = 0; r < 3; r++) circles.push(<circle key={`${i}r${r}`} cx={xR} cy={cy(r)} r={DOT_R} fill={color} />);
  });
  return (
    <svg width={w} height={SVG_H} viewBox={`0 0 ${w} ${SVG_H}`} className="inline-block shrink-0 align-middle" aria-hidden="true">
      {circles}
    </svg>
  );
}

// Block-bar mode keeps solid glyphs (they render dense in any monospace font).
function BlockBar({ cells }: { cells: Cell[] }) {
  return (
    <span className="text-[15px] align-middle" aria-hidden="true">
      {cells.map((c, i) => (
        <span key={i} style={{ color: c.filled ? "var(--accent)" : "var(--track)" }}>
          {c.glyph}
        </span>
      ))}
    </span>
  );
}

export default function TerminalDemo() {
  const [preset, setPreset] = useState<(typeof PRESETS)[number]>(PRESETS[0]);
  const [braille, setBraille] = useState(true);
  const presetRefs = useRef<(HTMLButtonElement | null)[]>([]);

  const selectPresetAt = (index: number) => {
    const wrapped = (index + PRESETS.length) % PRESETS.length;
    setPreset(PRESETS[wrapped]);
    presetRefs.current[wrapped]?.focus();
  };

  const handlePresetKeyDown = (e: React.KeyboardEvent<HTMLButtonElement>, index: number) => {
    if (e.key === "ArrowRight" || e.key === "ArrowDown") {
      e.preventDefault();
      selectPresetAt(index + 1);
    } else if (e.key === "ArrowLeft" || e.key === "ArrowUp") {
      e.preventDefault();
      selectPresetAt(index - 1);
    }
  };

  return (
    <div style={{ ["--accent" as string]: preset.color }}>
      <div className="mx-auto max-w-2xl overflow-hidden rounded-xl border border-border bg-[var(--surface2)] text-left shadow-2xl">
        <div className="flex items-center gap-2 border-b border-white/[0.06] px-3.5 py-2.5">
          <span className="h-[11px] w-[11px] rounded-full" style={{ background: "#ec6a5e" }} />
          <span className="h-[11px] w-[11px] rounded-full" style={{ background: "#f4bf4f" }} />
          <span className="h-[11px] w-[11px] rounded-full" style={{ background: "#61c554" }} />
          <span className="ml-1 text-[0.72rem] text-[var(--dim2)]">morse — claude</span>
        </div>
        <div className="px-[1.15rem] py-4 text-[14px] leading-[1.85]">
          <div><span className="text-[var(--prompt)]">morse</span><span className="text-[var(--dim2)]"> ~/project $ claude</span></div>
          <div className="font-bold text-[var(--fg)]">Opus 4.8 (1M context)<span className="ml-3 font-normal text-[#8a8a8a]">Max (20x)</span></div>
          {ROWS.map((r) => (
            <div key={r.label} className="flex items-baseline gap-2 whitespace-pre">
              <span className="inline-block w-[5.2em] font-bold text-[var(--fg)]">{r.label}</span>
              {braille ? <BrailleBar dots={brailleDotCells(r.pct)} /> : <BlockBar cells={blockCells(r.pct)} />}
              <span className="w-[3.2em] text-right tabular-nums text-[var(--dim)]">{r.pct}%</span>
              <span className="text-[13px] text-[var(--dim)]">{r.meta}</span>
            </div>
          ))}
        </div>
      </div>

      {/* preset switcher — radio group + braille/block toggle, same row */}
      <div className="mt-4 flex flex-wrap items-center justify-center gap-1.5">
        <div role="radiogroup" aria-label="Bar color preset" className="flex flex-wrap justify-center gap-1.5">
          {PRESETS.map((p, i) => (
            <button
              key={p.id}
              ref={(el) => { presetRefs.current[i] = el; }}
              role="radio"
              aria-checked={p.id === preset.id}
              tabIndex={p.id === preset.id ? 0 : -1}
              onClick={() => setPreset(p)}
              onKeyDown={(e) => handlePresetKeyDown(e, i)}
              className="cursor-pointer rounded-full border px-3 py-1.5 text-[0.78rem] transition-colors"
              style={{
                color: p.id === preset.id ? "var(--bg)" : "var(--dim)",
                background: p.id === preset.id ? p.color : "transparent",
                borderColor: p.id === preset.id ? p.color : "var(--border)",
                fontWeight: p.id === preset.id ? 700 : 400,
              }}
            >
              <span className="mr-1.5 inline-block h-2 w-2 rounded-full align-middle" style={{ background: p.color }} />
              {p.label}
            </button>
          ))}
        </div>
        <button
          onClick={() => setBraille((b) => !b)}
          aria-pressed={!braille}
          aria-label="Toggle bar style between braille and block"
          className="cursor-pointer rounded-full border border-border px-3 py-1.5 text-[0.78rem] text-[var(--dim)] transition-colors hover:text-[var(--fg)]"
        >
          {braille ? "braille" : "block"}
        </button>
      </div>
    </div>
  );
}
