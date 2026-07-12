#!/usr/bin/env bash
# Publish all six npm packages (5 platform packages + the meta package) for the
# version in npm/ccstatus/package.json. Run `npm login` first — this reads your
# local ~/.npmrc credentials; no token is passed on the command line.
#
# Usage: scripts/publish-npm.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VER="$(node -p "require('$ROOT/npm/ccstatus/package.json').version")"

# Ensure the native binaries are present before publishing the platform packages.
"$ROOT/scripts/build-npm.sh" "$VER"

echo "Publishing @ccstatus platform packages…"
for dir in darwin-arm64 darwin-x64 linux-arm64 linux-x64 win32-x64; do
  ( cd "$ROOT/npm/packages/$dir" && npm publish --access public )
done

echo "Publishing meta package ccstatus-cli@${VER}..."
( cd "$ROOT/npm/ccstatus" && npm publish --access public )

echo "Published ccstatus-cli@${VER} and all platform packages."
