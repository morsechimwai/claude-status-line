import { describe, it, expect } from "vitest";
import { readFileSync } from "node:fs";
import { VERSION, INSTALL, PRESETS } from "./product";

describe("product data", () => {
  it("VERSION matches Cargo.toml (guards drift)", () => {
    const cargo = readFileSync(new URL("../../Cargo.toml", import.meta.url), "utf8");
    const m = cargo.match(/^version\s*=\s*"([^"]+)"/m);
    expect(m).not.toBeNull();
    expect(VERSION).toBe(m![1]);
  });
  it("has the three canonical install commands", () => {
    const byId = Object.fromEntries(INSTALL.map((i) => [i.id, i.command]));
    expect(byId.brew).toBe("brew install morsechimwai/tap/ccstatus");
    expect(byId.npm).toBe("npm install -g ccstatus-cli");
    expect(byId.cargo).toBe("cargo install ccstatus");
  });
  it("has the five config presets", () => {
    expect(PRESETS.map((p) => p.id)).toEqual(["orange", "blue", "green", "purple", "mono"]);
  });
});
