#!/usr/bin/env bash
# Build a .pkg.tar.zst for Arch.  Usage: arch.sh <journey-tag> <binary> <out-dir>
# makepkg refuses to run as root, so in CI this runs as an unprivileged user.
set -euo pipefail
source "$(dirname "$0")/journeys.sh"
journey_meta "$1"; bin="$2"; out="$(realpath -m "$3")"
mkdir -p "$out"
work="$(mktemp -d)"
cp "$bin" "$work/hypnosys"
cp "$ROOT/LICENSE" "$ROOT/WARNING.md" "$work/"
desktop_entry > "$work/${PKG}.desktop"
cat > "$work/PKGBUILD" <<PKGBUILD
pkgname=${PKG}
pkgver=${VERSION}
pkgrel=1
pkgdesc="${NAME}. ${SUMMARY}"
arch=('x86_64')
url="${HOMEPAGE}"
license=('MIT')
depends=('alsa-lib' 'libx11' 'libxi' 'libglvnd')
options=('!debug' '!strip')
source=('hypnosys' '${PKG}.desktop' 'LICENSE' 'WARNING.md')
sha256sums=('SKIP' 'SKIP' 'SKIP' 'SKIP')
install=${PKG}.install

package() {
    install -Dm755 "\$srcdir/hypnosys" "\$pkgdir/usr/bin/${PKG}"
    install -Dm644 "\$srcdir/LICENSE" "\$pkgdir/usr/share/licenses/${PKG}/LICENSE"
    install -Dm644 "\$srcdir/WARNING.md" "\$pkgdir/usr/share/doc/${PKG}/WARNING.md"
    install -Dm644 "\$srcdir/${PKG}.desktop" "\$pkgdir/usr/share/applications/${PKG}.desktop"
}
PKGBUILD
# show the warning when pacman installs it
printf 'post_install() {\n  echo "%s"\n}\n' "WARNING: ${WARN_SHORT}" > "$work/${PKG}.install"
(cd "$work" && PKGDEST="$out" makepkg --nodeps --force --noconfirm)
