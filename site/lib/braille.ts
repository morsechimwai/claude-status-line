// Braille bar geometry, ported verbatim from src/render.rs `braille_bar`.
// KEEP IN SYNC with src/render.rs — see docs/adr/0001-port-braille-algorithm-to-web.md.
// filled = (pct.min(100) * 2*width + 50) / 100  (integer division)
// Each cell: baseline dots 7,8 (0xC0); left sub-col adds 0x07, right adds 0x38.

export type Cell = { glyph: string; filled: boolean };

const clampPct = (pct: number) => Math.max(0, Math.min(100, Math.trunc(pct)));

export function filledSubcols(pct: number, width: number): number {
  const subcols = 2 * width;
  return Math.floor((clampPct(pct) * subcols + 50) / 100);
}

export function brailleCells(pct: number, width = 12): Cell[] {
  const filled = filledSubcols(pct, width);
  const cells: Cell[] = [];
  for (let cell = 0; cell < width; cell++) {
    const left = 2 * cell < filled;
    const right = 2 * cell + 1 < filled;
    let bits = 0xc0;
    if (left) bits |= 0x07;
    if (right) bits |= 0x38;
    cells.push({ glyph: String.fromCodePoint(0x2800 + bits), filled: left || right });
  }
  return cells;
}

// Per-cell sub-column state, for drawing the bar as dots (dense, terminal-faithful)
// rather than relying on a web font's sparse braille glyphs. Same geometry as
// brailleCells: baseline dots always lit; left/right sub-columns light their 3 dots.
export type DotCell = { left: boolean; right: boolean; filled: boolean };

export function brailleDotCells(pct: number, width = 12): DotCell[] {
  const filled = filledSubcols(pct, width);
  return Array.from({ length: width }, (_, cell) => {
    const left = 2 * cell < filled;
    const right = 2 * cell + 1 < filled;
    return { left, right, filled: left || right };
  });
}

// Block-bar mode: filled = (pct * width + 50) / 100  (matches render.rs filled_cells).
export function blockCells(pct: number, width = 12): Cell[] {
  const filled = Math.floor((clampPct(pct) * width + 50) / 100);
  return Array.from({ length: width }, (_, i) => ({
    glyph: i < filled ? "█" : "░",
    filled: i < filled,
  }));
}
