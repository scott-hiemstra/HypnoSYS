#!/usr/bin/env bash
# Build a .deb for Debian/Ubuntu.  Usage: deb.sh <journey-tag> <binary> <out-dir>
set -euo pipefail
source "$(dirname "$0")/journeys.sh"
journey_meta "$1"; bin="$2"; out="$3"
mkdir -p "$out"
root="$(mktemp -d)/${PKG}"
install -Dm755 "$bin" "$root/usr/bin/${PKG}"
mkdir -p "$root/usr/share/applications" "$root/DEBIAN"
install -Dm644 "$ROOT/LICENSE" "$root/usr/share/doc/${PKG}/copyright"
install -Dm644 "$ROOT/WARNING.md" "$root/usr/share/doc/${PKG}/WARNING.md"
desktop_entry > "$root/usr/share/applications/${PKG}.desktop"
cat > "$root/DEBIAN/control" <<CONTROL
Package: ${PKG}
Version: ${VERSION}
Section: games
Priority: optional
Architecture: amd64
Depends: libasound2t64 | libasound2, libgl1, libx11-6, libxi6
Maintainer: ${MAINTAINER}
Homepage: ${HOMEPAGE}
Description: ${NAME}
 ${SUMMARY}
 .
 WARNING: ${WARN_SHORT}
 See /usr/share/doc/${PKG}/WARNING.md
CONTROL
dpkg-deb --build --root-owner-group "$root" "$out/${PKG}_${VERSION}_amd64.deb"
