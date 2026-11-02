# HYPNOSYS

*What consciousness feels like when reality gets a little weird.*

> **[Download journey 1 (v1.0)](https://github.com/scott-hiemstra/HypnoSYS/releases/tag/v1.0)**: packages for Debian/Ubuntu, RHEL, Arch, Windows and macOS.  
> [All three journeys](https://github.com/scott-hiemstra/HypnoSYS) · ⚠️ [Read the warning before you play](https://github.com/scott-hiemstra/HypnoSYS/blob/main/WARNING.md)

A short (about five minutes) guided-trance game written in Rust with
[macroquad](https://macroquad.rs). The game guides you down into a hypnotic
state. The better you follow each suggestion, the deeper you go, and the
deeper you go, the stranger the world becomes.

```
cargo run --release
```

Headphones are recommended because the drone is binaural: 100 Hz in the left
ear and 106 Hz in the right.

## The descent

| Stage | You are asked to… | Reality starts to… |
|---|---|---|
| **Fixation** | follow a swinging light | close in, as your eyelids get heavy and shut |
| **Breath** | hold to breathe in, release to breathe out | breathe with you, so the whole room swells |
| **Descent** | stay perfectly still as you count down from 10 | drop away into a tunnel, losing its colour |
| **Float** | let your hand rise on its own, without fighting it | lose its weight |
| **Weird** | gather drifting thoughts, find yourself among your echoes | mirror (left becomes right), split you into copies, and stop time |
| **Emerge** | click on each count from 1 to 5 | come back, solid and bright |

Depth also changes how things feel. Your cursor gets heavier and leaves
longer afterimages, your peripheral vision narrows, and words start to drift
apart. At the end you get a trance-depth score.

## Keys

- **Mouse**: follow, hold, release, or stay still
- **Esc**: wakes you gently (it skips to the count-up instead of cutting out)
- **F**: fullscreen
- **M**: mute
- **Tab**: skip a stage

All visuals move slowly and nothing flashes. Even so, skip it if you're prone
to seizures, and never play while driving.

## Code

- `src/main.rs`: the game, its stages, and the hypnotist's script
- `src/shader.rs`: GLSL for the spiral, tunnel, warp, colour, and eyelids
- `src/synth.rs`: all audio, synthesized at startup (no asset files)
