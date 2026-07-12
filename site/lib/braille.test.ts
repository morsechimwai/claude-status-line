import { describe, it, expect } from "vitest";
import { brailleCells, blockCells, filledSubcols } from "./braille";

const glyphs = (pct: number) => brailleCells(pct).map((c) => c.glyph).join("");

describe("brailleCells matches src/render.rs output", () => {
  it("9% (5h row) → filled 2 → ⣿⣀⣀…", () => {
    expect(filledSubcols(9, 12)).toBe(2);
    expect(glyphs(9)).toBe("⣿⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀");
  });
  it("22% (Context row) → filled 5 → ⣿⣿⣇⣀…", () => {
    expect(filledSubcols(22, 12)).toBe(5);
    expect(glyphs(22)).toBe("⣿⣿⣇⣀⣀⣀⣀⣀⣀⣀⣀⣀");
  });
  it("31% (7d row) → filled 7 → ⣿⣿⣿⣇⣀…", () => {
    expect(filledSubcols(31, 12)).toBe(7);
    expect(glyphs(31)).toBe("⣿⣿⣿⣇⣀⣀⣀⣀⣀⣀⣀⣀");
  });
  it("0% → all empty, 100% → all full", () => {
    expect(glyphs(0)).toBe("⣀".repeat(12));
    expect(glyphs(100)).toBe("⣿".repeat(12));
  });
  it("marks cells filled when any sub-column is lit", () => {
    const cells = brailleCells(22);
    expect(cells.slice(0, 3).every((c) => c.filled)).toBe(true);
    expect(cells[3].filled).toBe(false);
  });
});

describe("blockCells", () => {
  it("22% → filled 3 of 12 (rounded)", () => {
    const cells = blockCells(22);
    expect(cells.filter((c) => c.filled).length).toBe(3);
    expect(cells.map((c) => c.glyph).join("")).toBe("███░░░░░░░░░");
  });
});
