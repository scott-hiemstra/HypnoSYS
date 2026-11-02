#!/usr/bin/env bash
# Zip the Windows build.  Usage: windows.sh <journey-tag> <exe> <out-dir>
set -euo pipefail
source "$(dirname "$0")/journeys.sh"
journey_meta "$1"; exe="$2"; out="$(mkdir -p "$3" && cd "$3" && pwd)"
stage="$(mktemp -d)/${PKG}"
mkdir -p "$stage"
cp "$exe" "$stage/${PKG}.exe"
cp "$ROOT/LICENSE" "$stage/LICENSE.txt"
cp "$ROOT/WARNING.md" "$stage/READ-ME-FIRST-WARNING.txt"
(cd "$(dirname "$stage")" && 7z a -tzip -bso0 "$out/${PKG}_${VERSION}_windows-x64.zip" "${PKG}")
