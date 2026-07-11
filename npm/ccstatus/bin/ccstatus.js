#!/usr/bin/env node
// Launcher: resolve the prebuilt native binary from the matching
// @ccstatus/<platform> optional dependency and exec it, passing stdin/stdout
// straight through. npm/pnpm/yarn/bun install only the one platform package
// that matches this machine's os+cpu, so no install scripts are needed.

const { spawnSync } = require("child_process");

const key = `${process.platform}-${process.arch}`;
const pkg = `@morsechimwai/ccstatus-${key}`;
const binName = process.platform === "win32" ? "ccstatus.exe" : "ccstatus";

let binPath;
try {
  binPath = require.resolve(`${pkg}/${binName}`);
} catch (_) {
  console.error(
    `[ccstatus] no prebuilt binary for ${key} (${pkg} is not installed).\n` +
      `[ccstatus] install from source instead: cargo install ccstatus`
  );
  process.exit(1);
}

const result = spawnSync(binPath, process.argv.slice(2), { stdio: "inherit" });

if (result.error) {
  console.error(`[ccstatus] failed to run binary: ${result.error.message}`);
  process.exit(1);
}
process.exit(result.status === null ? 1 : result.status);
