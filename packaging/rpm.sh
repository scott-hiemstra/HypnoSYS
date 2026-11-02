#!/usr/bin/env bash
# Build an .rpm for RHEL/Fedora/Rocky/Alma.  Usage: rpm.sh <journey-tag> <binary> <out-dir>
set -euo pipefail
source "$(dirname "$0")/journeys.sh"
journey_meta "$1"; bin="$(realpath "$2")"; out="$3"
mkdir -p "$out"
top="$(mktemp -d)"
desktop_entry > "$top/${PKG}.desktop"
cat > "$top/${PKG}.spec" <<SPEC
%global debug_package %{nil}
Name:      ${PKG}
Version:   ${VERSION}
Release:   1
Summary:   ${SUMMARY}
License:   MIT
URL:       ${HOMEPAGE}
Requires:  alsa-lib libX11 libXi mesa-libGL

%description
${NAME}. ${SUMMARY}

WARNING: ${WARN_SHORT}

%install
install -Dm755 ${bin} %{buildroot}/usr/bin/${PKG}
install -Dm644 ${top}/${PKG}.desktop %{buildroot}/usr/share/applications/${PKG}.desktop
install -Dm644 ${ROOT}/LICENSE %{buildroot}/usr/share/licenses/${PKG}/LICENSE
install -Dm644 ${ROOT}/WARNING.md %{buildroot}/usr/share/doc/${PKG}/WARNING.md

%files
%license /usr/share/licenses/${PKG}/LICENSE
%doc /usr/share/doc/${PKG}/WARNING.md
/usr/bin/${PKG}
/usr/share/applications/${PKG}.desktop
SPEC
rpmbuild --quiet --define "_topdir $top/rpmbuild" -bb "$top/${PKG}.spec"
find "$top/rpmbuild/RPMS" -name '*.rpm' -exec cp {} "$out/" \;
