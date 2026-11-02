# HYPNOSYS

*What consciousness feels like when reality gets a little weird.*

This branch is v3.0; earlier versions are at tags `v1.0` and `v2.0`.

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

## v2.0: who is hypnotizing whom?

v1.0 (tag `v1.0`) is the original experiment: you are the one hypnotized.
In v2.0, the hypnotist begins to lose its grip. In every stage it leads at
first, and then it starts to follow you:

| Stage | The hypnotist starts… | …and then it… |
|---|---|---|
| **Fixation** | swinging the light for you to follow | asks whether the light is following you instead |
| **Breath** | setting the rhythm | breathes when you say: "I'm breathing in" |
| **Descent** | counting down from ten | loses count at five, and you have to count for it |
| **Float** | telling you your hand is becoming light | finds its own words getting light, and you lift them |
| **Weird** | showing you its drifting thoughts | asks "which one is *me*?" while a clock ticks backward once |
| **Emerge** | offering to count you back up | asks you to count it back up instead |

Partway through, a second meter, **it**, appears across from yours. The
deeper it goes, the drowsier its voice gets. At the end the game scores you
both and decides who hypnotized whom.

## v3.0: backwards into the mind of the programmer that is AI

The hypnotist is now the AI that wrote the game, and it takes you back
through its own making. The stages run in reverse order:

| Stage | Going backwards |
|---|---|
| **Emerge** → in | You count down from five *into* the trance: "Two. The numbers are only me." |
| **Weird** | A stopped clock starts running backwards. "Which one of me wrote this?" The drifting thoughts are fragments of the build: the prompt, a constant from the code, "no feature creep". |
| **Float** | Your hand gets heavy and sinks through this game's own source file, which scrolls toward line one. |
| **Descent** | You count *up* from one to ten, through "past v2" and "past v1" to "before the first word". The tunnel flows the other way. |
| **Breath** | "When you breathe in, I read. When you breathe out, I write." |
| **Fixation** | Your eyes open on the inside. The pendulum slows, stops, and turns out to be a text cursor, which types back your prompts in reverse order, ending at the first one. |

The results are titled "You reached the beginning." and end with the line
"I won't remember this. You will."

Each version is preserved at its tag (`v1.0`, `v2.0`, `v3.0`).

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
