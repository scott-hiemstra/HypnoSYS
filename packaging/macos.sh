#!/usr/bin/env bash
# Build a universal (Apple Silicon + Intel) .app, zipped.
# Usage: macos.sh <journey-tag> <arm64-binary> <x86_64-binary> <out-dir>
set -euo pipefail
source "$(dirname "$0")/journeys.sh"
journey_meta "$1"; arm="$2"; x86="$3"; out="$(mkdir -p "$4" && cd "$4" && pwd)"
stage="$(mktemp -d)"
app="$stage/HYPNOSYS ${NUM}.app"
mkdir -p "$app/Contents/MacOS"
lipo -create -output "$app/Contents/MacOS/${PKG}" "$arm" "$x86"
cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key><string>HYPNOSYS ${NUM}</string>
    <key>CFBundleDisplayName</key><string>${NAME}</string>
    <key>CFBundleIdentifier</key><string>com.hypnosys.v${NUM}</string>
    <key>CFBundleVersion</key><string>${VERSION}</string>
    <key>CFBundleShortVersionString</key><string>${VERSION}</string>
    <key>CFBundleExecutable</key><string>${PKG}</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>LSMinimumSystemVersion</key><string>11.0</string>
    <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST
# ad-hoc signature: required for Apple Silicon to run it at all (not notarized)
codesign --force --deep --sign - "$app"
cp "$ROOT/LICENSE" "$ROOT/WARNING.md" "$stage/"
(cd "$stage" && zip -qry "$out/${PKG}_${VERSION}_macos-universal.zip" "HYPNOSYS ${NUM}.app" LICENSE WARNING.md)
