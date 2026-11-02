# Shared metadata for packaging each HYPNOSYS journey.
# Usage: source packaging/journeys.sh; journey_meta v1.0
# Sets: NUM VERSION PKG NAME TITLE SUMMARY

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # LICENSE, WARNING.md live here
WARN_SHORT="Do not run if you have epilepsy, a seizure history, psychosis or a dissociative disorder, or are impaired. Read WARNING.md."
# Debian requires a maintainer address; this is GitHub's no-reply alias, not a real inbox.
MAINTAINER="Scott Hiemstra <scott-hiemstra@users.noreply.github.com>"
HOMEPAGE="https://github.com/scott-hiemstra/HypnoSYS"

journey_meta() {
    case "$1" in
        v1.0) NUM=1; TITLE="The Player Experiences Hypnosis"
              SUMMARY="A guided trance where reality bends the deeper you go." ;;
        v2.0) NUM=2; TITLE="Who Is Hypnotizing Whom"
              SUMMARY="The hypnotist loses its grip and begins to follow you." ;;
        v3.0) NUM=3; TITLE="Backwards Into the Mind of the Programmer"
              SUMMARY="A journey backwards through the AI that wrote it, to the first prompt." ;;
        *) echo "unknown journey: $1" >&2; return 1 ;;
    esac
    VERSION="${1#v}.0"          # v1.0 -> 1.0.0
    PKG="hypnosys-v${NUM}"      # journeys install side by side
    NAME="HYPNOSYS ${NUM}: ${TITLE}"
}

desktop_entry() {
    cat <<DESKTOP
[Desktop Entry]
Type=Application
Name=${NAME}
Comment=${SUMMARY}
Exec=${PKG}
Terminal=false
Categories=Game;
DESKTOP
}
