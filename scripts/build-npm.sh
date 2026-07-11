#!/usr/bin/env bash
# Download the prebuilt binaries for release v$VERSION from GitHub and drop them
# into each npm/packages/<platform> directory. The binaries themselves are
# git-ignored — this script regenerates them before packing/publishing.
#
# Usage: scripts/build-npm.sh [VERSION]   (VERSION defaults to npm/ccstatus/package.json)
set -euo pipefail

REPO="morsechimwai/claude-status-line"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VER="${1:-$(node -p "require('$ROOT/npm/ccstatus/package.json').version")}"

echo "Building npm platform packages for v$VER"

fetch() { # dir triple ext binname
  local dir=$1 triple=$2 ext=$3 binname=$4
  local pdir="$ROOT/npm/packages/$dir"
  local tmp="/tmp/ccs-$triple.$ext"
  gh release download "v$VER" -R "$REPO" -p "ccstatus-$triple.$ext" -O "$tmp" --clobber
  if [ "$ext" = "zip" ]; then tar -xf "$tmp" -C "$pdir"; else tar -xzf "$tmp" -C "$pdir"; fi
  [ "$binname" = "ccstatus" ] && chmod +x "$pdir/$binname"
  rm -f "$tmp"
  echo "  $dir/$binname ready"
}

fetch darwin-arm64 aarch64-apple-darwin      tar.gz ccstatus
fetch darwin-x64   x86_64-apple-darwin        tar.gz ccstatus
fetch linux-arm64  aarch64-unknown-linux-gnu  tar.gz ccstatus
fetch linux-x64    x86_64-unknown-linux-gnu   tar.gz ccstatus
fetch win32-x64    x86_64-pc-windows-msvc      zip    ccstatus.exe

echo "Done. Binaries staged in npm/packages/*/"
