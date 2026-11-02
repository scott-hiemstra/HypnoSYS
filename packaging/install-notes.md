### Install

Each journey installs under its own name (`hypnosys-v1`, `hypnosys-v2`,
`hypnosys-v3`), so you can install all three side by side.

| Platform | File | How |
|---|---|---|
| Debian / Ubuntu | `.deb` | `sudo apt install ./hypnosys-vN_*.deb` |
| RHEL / Fedora / Rocky / Alma | `.rpm` | `sudo dnf install ./hypnosys-vN-*.rpm` |
| Arch | `.pkg.tar.zst` | `sudo pacman -U hypnosys-vN-*.pkg.tar.zst` |
| Windows (x64) | `_windows-x64.zip` | Unzip and run `hypnosys-vN.exe`. SmartScreen may warn about an unrecognized app: choose *More info → Run anyway*. |
| macOS (Apple Silicon + Intel) | `_macos-universal.zip` | Unzip and move the app to Applications. It isn't notarized, so the first time, right-click it and choose *Open*. If macOS still refuses, run `xattr -dr com.apple.quarantine "/Applications/HYPNOSYS N.app"`. |

On Linux, start it from your app menu or run `hypnosys-vN`. Headphones are
recommended.
