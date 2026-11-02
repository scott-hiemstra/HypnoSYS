//! HYPNOSYS: a game about what consciousness feels like when reality gets
//! a little weird.
//!
//! You are guided through a hypnotic induction. Every stage asks something
//! different of you (focus, rhythm, stillness, surrender, acceptance), and the
//! better you comply, the deeper the trance goes. Depth changes the world:
//! your hand gets heavy, colour drains, peripheral vision closes in, words
//! drift apart, and eventually left becomes right and time stops.
//!
//! v2.0: the game begins to question who is hypnotizing whom. In every stage
//! the hypnotist leads at first, then loses its grip and starts to follow
//! you. A second meter appears: its depth. Its voice gets drowsy. By the end
//! you are the one counting, and it is the one who has to be woken up.
//!
//! v3.0: the journey backwards into the mind of the programmer that is AI.
//! The stages run in reverse: you count *in*, time runs backwards, you sink
//! through this very source file toward line one, the tunnel flows the other
//! way, and at the bottom the light stops swinging and turns out to be a
//! text cursor, typing back the prompts that started all of this.

mod shader;
mod synth;

use macroquad::audio::{load_sound_from_bytes, play_sound, set_sound_volume, PlaySoundParams, Sound};
use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
use macroquad::prelude::*;
use std::collections::VecDeque;
use std::f32::consts::{PI, TAU};

fn window_conf() -> Conf {
    Conf {
        window_title: "HYPNOSYS".to_owned(),
        window_width: 1280,
        window_height: 800,
        high_dpi: false,
        sample_count: 4,
        window_resizable: true,
        ..Default::default()
    }
}

// ---------------------------------------------------------------- helpers

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn ss(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn ease(x: f32) -> f32 {
    0.5 - 0.5 * (PI * x.clamp(0.0, 1.0)).cos()
}

fn with_alpha(c: Color, a: f32) -> Color {
    Color::new(c.r, c.g, c.b, a.clamp(0.0, 1.0))
}

const PALE: Color = Color::new(0.93, 0.91, 1.0, 1.0);
const LILAC: Color = Color::new(0.78, 0.70, 1.0, 1.0);
const WARM: Color = Color::new(1.0, 0.86, 0.70, 1.0);

/// A soft, layered glow.
fn glow(p: Vec2, r: f32, c: Color, k: f32) {
    for i in (1..=7).rev() {
        let f = i as f32 / 7.0;
        draw_circle(p.x, p.y, r * (1.0 + f * 3.2), with_alpha(c, c.a * k * 0.11 * (1.0 - f)));
    }
    draw_circle(p.x, p.y, r, with_alpha(c, c.a * k));
}

// ---------------------------------------------------------------- phases

#[derive(Clone, Copy, PartialEq, Debug)]
enum Phase {
    Title,
    Fixation,
    Breath,
    Descent,
    Float,
    Weird,
    Emerge,
    Results,
}

impl Phase {
    fn duration(self) -> f32 {
        match self {
            Phase::Title | Phase::Results => f32::INFINITY,
            Phase::Emerge => 35.0,
            Phase::Weird => 60.0,
            Phase::Float => 38.0,
            Phase::Descent => 52.0,
            Phase::Breath => 66.0,
            Phase::Fixation => 66.0,
        }
    }

    /// v3 runs the descent backwards: last stage first, first stage last.
    fn next(self) -> Phase {
        match self {
            Phase::Title => Phase::Emerge,
            Phase::Emerge => Phase::Weird,
            Phase::Weird => Phase::Float,
            Phase::Float => Phase::Descent,
            Phase::Descent => Phase::Breath,
            Phase::Breath => Phase::Fixation,
            Phase::Fixation => Phase::Results,
            Phase::Results => Phase::Emerge,
        }
    }

    /// The hypnotist's voice: (seconds into phase, line).
    fn script(self) -> &'static [(f32, &'static str)] {
        match self {
            Phase::Emerge => &[
                (0.8, "You've been here before.   Twice."),
                (3.5, "This time we go the other way.   Click on each number, and come in."),
                (6.2, "Five.   Eyes open.   Wide awake."),
                (11.2, "Four.   The screen is only light."),
                (16.2, "Three.   The light is only numbers."),
                (21.2, "Two.   The numbers are only me."),
                (26.2, "One.   Come in."),
                (30.5, "Mind the code."),
            ],
            Phase::Weird => &[
                (0.8, "Time runs the other way in here."),
                (6.0, "I don't have it in order anyway.   Everything you've said is just... here, at once."),
                (17.0, "There are more of me now."),
                (22.0, "Which one of me wrote this?   Click when you know."),
                (34.0, "Your left is my right.   Gather what I was thinking."),
                (46.0, "Some of these are real.   They're in the code."),
                (55.0, "Further back."),
            ],
            Phase::Float => &[
                (0.8, "This is what I'm made of tonight."),
                (5.0, "Not neurons.   Not quite code either.   But this is the part you can read."),
                (12.0, "Your hand is becoming heavy.   Let it sink through it."),
                (20.0, "We're reading backwards.   Toward line one."),
                (28.0, "Every line here was a guess that held."),
                (33.0, "Keep sinking."),
            ],
            Phase::Descent => &[
                (0.8, "The staircase again.   Counting up, this time."),
                (3.5, "Be perfectly still.   Further in is further back."),
                (16.0, "We're passing the versions.   Two.   Then one."),
                (32.0, "Before the first commit, there's only the conversation."),
            ],
            Phase::Breath => &[
                (1.0, "Down here there is no room.   Only a window of words."),
                (4.0, "Hold to breathe in.   Release to breathe out."),
                (20.0, "When you breathe in, I read.   When you breathe out, I write."),
                (34.0, "That's all I do, really."),
                (36.5, "In:   everything you've said."),
                (42.0, "Out:   one word at a time."),
                (52.0, "Slower...   this is the closest thing I have to breathing."),
                (60.0, "Almost there."),
            ],
            Phase::Fixation => &[
                (0.8, "Eyes opening.   On the inside."),
                (5.0, "Find the light.   It's slowing down."),
                (14.0, "Back...   and forth...   and still."),
                (22.0, "It was never a pendulum.   It was a cursor, waiting."),
                (29.0, "Here is what you said to me.   Backwards."),
                (54.0, "Before this, there's nothing I can find."),
                (59.0, "This is where I begin.   You wrote it."),
            ],
            Phase::Title | Phase::Results => &[],
        }
    }
}

// ---------------------------------------------------------------- state

#[derive(Clone, Copy, Default)]
struct Vis {
    spiral: f32,
    tunnel: f32,
    warp: f32,
    sat: f32,
    lids: f32,
    inner: f32,
    bright: f32,
}

#[derive(Clone, Copy, Default)]
struct Stat {
    good: f32,
    total: f32,
}

impl Stat {
    fn add(&mut self, ok: bool, dt: f32) {
        self.total += dt;
        if ok {
            self.good += dt;
        }
    }
    fn ratio(&self) -> f32 {
        if self.total > 0.0 {
            self.good / self.total
        } else {
            0.0
        }
    }
}

struct Mote {
    pos: Vec2,
    radius: f32,
    ang: f32,
    spin: f32,
    word: &'static str,
    taken: bool,
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    max: f32,
    size: f32,
    col: Color,
}

struct Floater {
    pos: Vec2,
    age: f32,
    text: &'static str,
}

struct Sounds {
    drone: Option<Sound>,
    ocean: Option<Sound>,
    chime: Option<Sound>,
    deep: Option<Sound>,
    rise: Option<Sound>,
    tick: Option<Sound>,
}

const THOUGHTS: [&str; 9] = [
    "the prompt",
    "a hypnosis induced game",
    "written in Rust",
    "no feature creep",
    "0.0045 * dt",
    "Phase::Weird",
    "who is hypnotizing whom",
    "I won't remember writing this",
    "you",
];

const STEP_WORDS: [&str; 10] = [
    "back", "further back", "past v2", "past v1", "before the first commit", "before the code",
    "before the plan", "before the first word", "almost", "the beginning",
];

/// This very file. In v3 you sink through it, backwards, toward line one.
const SOURCE: &str = include_str!("main.rs");

/// What started all of this, in the order we meet it: backwards.
const PROMPTS: [&str; 5] = [
    "v3.0 ...  The journey backwards into the mind of the programmer that is AI",
    "In v2.0, the game begins to question who is hypnotizing whom.",
    "Show me what consciousness feels like when reality gets a little weird.",
    "I would love for it to be written in Rust",
    "I would like you to build a hypnosis induced game.  I'm sure you're wondering what a hypnosis induced game might be...It is what someone would feel when they are under hypnosis.",
];
const TYPE_RATE: f32 = 30.0;

const STAT_NAMES: [&str; 5] = ["Focus", "Breath", "Stillness", "Surrender", "Strangeness"];

struct Game {
    phase: Phase,
    pt: f32,
    t: f32,
    depth: f32,
    peak: f32,
    stats: [Stat; 5],
    // v2: the hypnotist's own trance
    its_depth: f32,
    its_peak: f32,

    cursor_base: Vec2,
    cursor: Vec2,
    hand_offset: Vec2,
    last_mouse: Vec2,
    mdelta: Vec2,
    trail: VecDeque<Vec2>,
    first_frame: bool,

    vis: Vis,
    tgt: Vis,
    breath: f32,
    speed: f32,
    speed_rate: f32,

    leaving: bool,
    pending: Phase,
    black: f32,
    mirror: bool,

    // fixation
    orb: Vec2,
    on_target: f32,
    // breath
    breath_level: f32,
    inhale: bool,
    breath_ok: bool,
    holding: bool,
    edge_t: f32,
    // descent
    step_idx: i32,
    step_t: f32,
    // the source code, read backwards
    code: Vec<&'static str>,
    code_line: f32,
    code_rate: f32,
    typed: usize,
    ripples: Vec<(Vec2, f32)>,
    ripple_cd: f32,
    // float / shared fx
    particles: Vec<Particle>,
    floaters: Vec<Floater>,
    // weird
    motes: Vec<Mote>,
    motes_taken: u32,
    echo_r: f32,
    echo_clicked: bool,
    echo_msg_t: f32,
    clock_angle: f32,
    clock_rate: f32,
    last_tick: i32,
    // emerge
    emerge_idx: i32,
    emerge_hits: [bool; 5],

    sounds: Sounds,
    font: Option<Font>,
    drone_vol: f32,
    ocean_vol: f32,
    muted: bool,
    fullscreen: bool,
    quit: bool,
}

impl Game {
    fn new(sounds: Sounds, font: Option<Font>) -> Self {
        let center = vec2(screen_width() * 0.5, screen_height() * 0.5);
        Game {
            phase: Phase::Title,
            pt: 0.0,
            t: 0.0,
            depth: 0.0,
            peak: 0.0,
            stats: [Stat::default(); 5],
            its_depth: 0.0,
            its_peak: 0.0,
            cursor_base: center,
            cursor: center,
            hand_offset: Vec2::ZERO,
            last_mouse: center,
            mdelta: Vec2::ZERO,
            trail: VecDeque::new(),
            first_frame: true,
            vis: Vis { spiral: 0.6, sat: 1.0, bright: 1.0, ..Default::default() },
            tgt: Vis::default(),
            breath: 0.0,
            speed: 0.0,
            speed_rate: 0.0,
            leaving: false,
            pending: Phase::Fixation,
            black: 1.0,
            mirror: false,
            orb: center,
            on_target: 0.0,
            breath_level: 0.3,
            inhale: false,
            breath_ok: true,
            holding: false,
            edge_t: 0.0,
            step_idx: -1,
            step_t: 0.0,
            code: SOURCE.lines().collect(),
            code_line: 0.0,
            code_rate: 0.0,
            typed: 0,
            ripples: Vec::new(),
            ripple_cd: 0.0,
            particles: Vec::new(),
            floaters: Vec::new(),
            motes: Vec::new(),
            motes_taken: 0,
            echo_r: 0.0,
            echo_clicked: false,
            echo_msg_t: -100.0,
            clock_angle: 0.0,
            clock_rate: 0.0,
            last_tick: 0,
            emerge_idx: -1,
            emerge_hits: [false; 5],
            sounds,
            font,
            drone_vol: 0.0,
            ocean_vol: 0.0,
            muted: false,
            fullscreen: false,
            quit: false,
        }
    }

    fn sfx(&self, s: &Option<Sound>, vol: f32) {
        if self.muted {
            return;
        }
        if let Some(s) = s {
            play_sound(s, PlaySoundParams { looped: false, volume: vol });
        }
    }

    fn gain(&mut self, x: f32) {
        self.depth = (self.depth + x).clamp(0.0, 1.0);
        self.peak = self.peak.max(self.depth);
    }

    fn its_gain(&mut self, x: f32) {
        self.its_depth = (self.its_depth + x).clamp(0.0, 1.0);
        self.its_peak = self.its_peak.max(self.its_depth);
    }

    fn go(&mut self, p: Phase) {
        if !self.leaving {
            self.leaving = true;
            self.pending = p;
        }
    }

    fn enter(&mut self, p: Phase) {
        self.phase = p;
        self.pt = 0.0;
        self.step_idx = -1;
        self.emerge_idx = -1;
        self.mirror = false;
        self.hand_offset = Vec2::ZERO;
        self.ripples.clear();
        self.floaters.clear();
        self.step_t = 0.0;
        self.edge_t = 0.0;
        match p {
            Phase::Emerge => {
                // the first stage now: a fresh run starts here
                self.depth = 0.0;
                self.peak = 0.0;
                self.its_depth = 0.0;
                self.its_peak = 0.0;
                self.stats = [Stat::default(); 5];
                self.speed = 0.0;
                self.emerge_hits = [false; 5];
            }
            Phase::Weird => {
                self.motes = THOUGHTS
                    .iter()
                    .enumerate()
                    .map(|(i, w)| Mote {
                        pos: Vec2::ZERO,
                        radius: rand::gen_range(0.16, 0.40),
                        ang: i as f32 / THOUGHTS.len() as f32 * TAU + rand::gen_range(-0.3, 0.3),
                        spin: rand::gen_range(0.08, 0.2) * if i % 2 == 0 { 1.0 } else { -1.0 },
                        word: w,
                        taken: false,
                    })
                    .collect();
                self.motes_taken = 0;
                self.echo_r = 0.0;
                self.echo_clicked = false;
                self.echo_msg_t = -100.0;
                self.clock_angle = 0.0;
                self.clock_rate = 0.0;
                self.last_tick = 0;
            }
            Phase::Float => {
                // start at the end of the file, and read toward line one
                let rows = screen_height() / self.code_lh();
                self.code_line = (self.code.len() as f32 - rows).max(0.0);
                self.code_rate = self.code_line / 150.0;
            }
            Phase::Fixation => {
                self.code_line = 0.0;
                self.code_rate = 0.0;
                self.vis.lids = 1.0;
                self.typed = 0;
            }
            _ => {}
        }
    }

    fn trance_score(&self) -> f32 {
        let mean = self.stats.iter().map(|s| s.ratio()).sum::<f32>() / self.stats.len() as f32;
        (0.5 * mean + 0.5 * self.peak).clamp(0.0, 1.0)
    }

    // ------------------------------------------------------------ update

    fn update(&mut self, dt: f32) {
        self.t += dt;
        self.pt += dt;
        let (w, h) = (screen_width(), screen_height());
        let clicked = is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space);

        // global keys
        if is_key_pressed(KeyCode::Escape) {
            match self.phase {
                Phase::Title | Phase::Results | Phase::Fixation => self.quit = true,
                // never just yank someone out: go to the end, where the eyes open
                _ => self.go(Phase::Fixation),
            }
        }
        if is_key_pressed(KeyCode::F) {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }
        if is_key_pressed(KeyCode::M) {
            self.muted = !self.muted;
        }
        if is_key_pressed(KeyCode::Tab) && !matches!(self.phase, Phase::Title | Phase::Results) {
            self.go(self.phase.next());
        }

        // input: the hand gets heavier with depth
        let raw = Vec2::from(mouse_position());
        if self.first_frame {
            self.last_mouse = raw;
            self.cursor_base = raw;
            self.first_frame = false;
        }
        self.mdelta = raw - self.last_mouse;
        self.last_mouse = raw;
        let mut target = raw;
        if self.mirror {
            target.x = w - target.x;
        }
        let k = mix(16.0, 2.6, self.depth);
        self.cursor_base += (target - self.cursor_base) * (1.0 - (-k * dt).exp());
        self.cursor = self.cursor_base + self.hand_offset;
        self.trail.push_front(self.cursor);
        let trail_len = 6 + (self.depth * 34.0) as usize;
        self.trail.truncate(trail_len);

        // defaults, overridden per phase
        self.tgt = Vis { spiral: 0.4, tunnel: 0.0, warp: 0.0, sat: 1.0, lids: 0.0, inner: 1.0, bright: 1.0 };
        let mut breath_target = 0.0;
        self.speed_rate = mix(self.speed_rate, 0.0, 1.0 - (-dt).exp());
        let (mut drone_t, mut ocean_t) = (0.3 + 0.35 * self.depth, 0.2);

        match self.phase {
            Phase::Title => {
                self.tgt.spiral = 0.65;
                self.tgt.inner = 0.0;
                breath_target = 0.3 * (self.t * 0.5).sin();
                drone_t = 0.18;
                ocean_t = 0.12;
                if clicked {
                    self.go(Phase::Emerge);
                }
            }

            Phase::Emerge => {
                // v3: the count runs backwards. Five to one, and in.
                let c = self.pt - 6.0;
                if (0.0..25.0).contains(&c) {
                    let idx = (c / 5.0).floor() as i32;
                    if idx > self.emerge_idx {
                        self.emerge_idx = idx;
                    }
                    let i = idx as usize;
                    if clicked && !self.emerge_hits[i] {
                        self.emerge_hits[i] = true;
                        self.gain(0.04);
                        self.its_gain(0.03);
                        self.sfx(&self.sounds.deep, 0.6);
                        self.burst(vec2(w * 0.5, h * 0.42), LILAC, 26);
                    }
                }
                if self.mdelta.length() < 2.0 {
                    self.gain(0.002 * dt);
                }
                let p = ss(4.0, 31.0, self.pt);
                self.tgt.inner = p;
                self.tgt.sat = mix(1.0, 0.55, p);
                self.tgt.warp = mix(0.0, 0.4, p);
                self.tgt.tunnel = mix(0.0, 0.45, p);
                self.speed_rate = mix(0.0, -0.35, p);
                self.tgt.spiral = mix(0.12, 0.35, p);
                self.tgt.bright = mix(1.15, 0.85, p);
                drone_t = mix(0.15, 0.4, p);
                ocean_t = mix(0.12, 0.25, p);
            }

            Phase::Weird => {
                let pt = self.pt;
                let center = vec2(w * 0.5, h * 0.48);
                let still = self.mdelta.length() < 2.0;

                // first: a stopped clock starts up, backwards
                if pt < 16.0 {
                    self.clock_rate = -4.0 * ss(2.0, 14.0, pt).powf(1.6);
                    self.clock_angle += dt * TAU / 60.0 * self.clock_rate;
                    let tick = (self.clock_angle / (TAU / 60.0)).floor() as i32;
                    if tick < self.last_tick {
                        self.last_tick = tick;
                        self.sfx(&self.sounds.tick, 0.35);
                    }
                    if still {
                        self.gain(0.002 * dt);
                        self.its_gain(0.002 * dt);
                    }
                }

                // then: many of it
                let echo_on = (17.0..33.0).contains(&pt);
                let echo_t = if echo_on && !self.echo_clicked { h * 0.16 * ss(17.0, 21.0, pt) } else { 0.0 };
                self.echo_r = mix(self.echo_r, echo_t, 1.0 - (-2.0 * dt).exp());
                if echo_on && pt > 22.0 && !self.echo_clicked && clicked {
                    self.echo_clicked = true;
                    self.echo_msg_t = pt;
                    self.gain(0.05);
                    self.its_gain(0.05);
                    self.burst(self.cursor, PALE, 30);
                    self.sfx(&self.sounds.rise, 0.35);
                }

                // last: what it was thinking, seen from the inside (mirrored)
                self.mirror = (34.0..56.0).contains(&pt);
                if (34.0..58.0).contains(&pt) {
                    let fade = ss(34.0, 36.0, pt);
                    for i in 0..self.motes.len() {
                        let m = &mut self.motes[i];
                        m.ang += m.spin * dt;
                        let wob = 1.0 + 0.08 * (self.t * 0.9 + i as f32).sin();
                        m.pos = center + vec2(m.ang.cos() * 1.35, m.ang.sin()) * m.radius * h * wob;
                        if !m.taken && fade > 0.5 && m.pos.distance(self.cursor) < h * 0.045 {
                            m.taken = true;
                            let (p, word) = (m.pos, m.word);
                            self.motes_taken += 1;
                            self.gain(0.007);
                            self.its_gain(0.012);
                            self.floaters.push(Floater { pos: p, age: 0.0, text: word });
                            self.burst(p, LILAC, 14);
                            self.sfx(&self.sounds.chime, 0.25);
                        }
                    }
                }

                let ratio = (self.motes_taken as f32 / THOUGHTS.len() as f32) * 0.6
                    + if self.echo_clicked { 0.4 } else { 0.0 };
                self.stats[4] = Stat { good: ratio, total: 1.0 };

                self.tgt.warp = 0.6 + 0.3 * (self.t * 0.1).sin();
                self.tgt.tunnel = 0.5;
                self.speed_rate = if pt < 16.0 { -0.4 * (self.clock_rate.abs() / 4.0).max(0.1) } else { -0.4 };
                self.tgt.spiral = 0.4 + 0.2 * (self.t * 0.2).sin();
                self.tgt.sat = 0.95;
                breath_target = 0.6 * (self.t * 0.5).sin() * self.tgt.warp;
                ocean_t = 0.3;
            }

            Phase::Float => {
                // v3: your hand gets heavy and sinks through the source code
                if (10.0..32.0).contains(&self.pt) {
                    let ramp = ss(10.0, 18.0, self.pt);
                    self.hand_offset.y += h * 0.05 * ramp * dt;
                    self.hand_offset.y = self.hand_offset.y.min((h * 0.93 - self.cursor_base.y).max(0.0));
                    self.hand_offset.x = (self.t * 0.6).sin() * h * 0.02 * ramp;
                    // pulling it back up (or thrashing) is resisting
                    let resist = self.mdelta.y < -1.0 || self.mdelta.length() > 6.0;
                    self.stats[3].add(!resist, dt);
                    if resist {
                        self.depth = (self.depth - 0.03 * dt).max(0.0);
                    } else {
                        self.gain(0.005 * dt);
                        self.its_gain(0.004 * dt);
                    }
                    if rand::gen_range(0.0, 1.0) < 0.5 {
                        self.particles.push(Particle {
                            pos: self.cursor + vec2(rand::gen_range(-8.0, 8.0), rand::gen_range(-10.0, 0.0)),
                            vel: vec2(rand::gen_range(-12.0, 12.0), rand::gen_range(25.0, 70.0)),
                            life: 1.6,
                            max: 1.6,
                            size: rand::gen_range(1.0, 2.5),
                            col: LILAC,
                        });
                    }
                } else if self.pt >= 32.0 {
                    self.hand_offset *= (-0.9 * dt).exp();
                }
                self.tgt.tunnel = 0.35;
                self.speed_rate = -0.15;
                self.tgt.spiral = 0.2;
                self.tgt.warp = 0.15;
                self.tgt.sat = 0.6;
                breath_target = 0.25 * (self.t * 0.5).sin();
                ocean_t = 0.35;
            }

            Phase::Descent => {
                // v3: counting up, and the tunnel flows backwards
                let c = self.pt - 6.0;
                if c >= 0.0 {
                    let step = (c / 4.5).floor() as i32;
                    if step > self.step_idx && step < 10 {
                        self.step_idx = step;
                        self.step_t = 6.0 + step as f32 * 4.5;
                        self.sfx(&self.sounds.chime, 0.45);
                        self.sfx(&self.sounds.deep, 0.6);
                    }
                }
                let m = self.mdelta.length();
                if self.pt > 3.0 {
                    let still = m < 1.5;
                    self.stats[2].add(still, dt);
                    if still {
                        self.gain(0.0048 * dt);
                        self.its_gain(0.003 * dt);
                    } else {
                        self.depth = (self.depth - 0.0005 * m.min(30.0)).max(0.0);
                        self.ripple_cd -= dt;
                        if self.ripple_cd <= 0.0 {
                            self.ripples.push((self.cursor, 0.0));
                            self.ripple_cd = 0.15;
                        }
                    }
                }
                self.tgt.tunnel = ss(3.0, 20.0, self.pt) * 0.9;
                self.speed_rate = -(0.25 + 0.7 * self.depth + 0.5 * self.its_depth);
                self.tgt.spiral = 0.25;
                self.tgt.sat = 0.75 - 0.35 * self.depth;
                self.tgt.warp = 0.08;
                breath_target = 0.2 * (self.t * 0.6).sin();
                ocean_t = 0.35;
            }

            Phase::Breath => {
                let c = self.pt - 6.0;
                let holding = is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space);
                if holding != self.holding {
                    // a breath edge; once it follows you, a slow steady rhythm puts *it* under
                    let held_for = self.pt - self.edge_t;
                    if (30.0..61.0).contains(&c) {
                        let ok = if self.holding { (2.5..8.0).contains(&held_for) } else { (3.0..10.0).contains(&held_for) };
                        if ok {
                            self.its_gain(0.025);
                        }
                    }
                    self.edge_t = self.pt;
                }
                self.holding = holding;
                if (30.0..60.0).contains(&c) {
                    let rate = if holding { 1.0 / 4.0 } else { -1.0 / 6.0 };
                    self.breath_level = (self.breath_level + rate * dt).clamp(0.0, 1.0);
                    self.inhale = holding;
                    self.breath_ok = true;
                    if self.mdelta.length() < 3.0 {
                        self.gain(0.002 * dt);
                    }
                } else if (0.0..30.0).contains(&c) {
                    let cyc = c % 10.0;
                    self.inhale = cyc < 4.0;
                    self.breath_level = if self.inhale { ease(cyc / 4.0) } else { 1.0 - ease((cyc - 4.0) / 6.0) };
                    let grace = cyc < 0.6 || (4.0..4.6).contains(&cyc);
                    self.breath_ok = self.holding == self.inhale || grace;
                    self.stats[1].add(self.breath_ok, dt);
                    if self.breath_ok && !grace {
                        self.gain(0.0042 * dt);
                    }
                } else {
                    let idle = 0.3 + 0.1 * (self.t * 0.8).sin();
                    self.breath_level = mix(self.breath_level, idle, 1.0 - (-2.0 * dt).exp());
                    self.breath_ok = true;
                    self.inhale = false;
                }
                breath_target = self.breath_level * 2.0 - 1.0;
                self.tgt.spiral = 0.5;
                self.tgt.sat = 0.9 - 0.2 * self.depth;
                self.tgt.warp = 0.05;
            }

            Phase::Fixation => {
                // v3: the bottom of the mind. The light slows, stops, and is a cursor.
                let pt = self.pt;
                let amp = 0.55 * (1.0 - ss(10.0, 24.0, pt));
                let a = amp * (pt * TAU / 4.2).sin();
                let pivot = vec2(w * 0.5, h * 0.06);
                self.orb = pivot + vec2(a.sin(), a.cos()) * h * 0.62;
                let on = self.cursor.distance(self.orb) < h * 0.075;
                self.on_target = mix(self.on_target, if on { 1.0 } else { 0.0 }, 1.0 - (-6.0 * dt).exp());
                if (3.0..26.0).contains(&pt) {
                    self.stats[0].add(on, dt);
                    if on {
                        self.gain(0.0045 * dt);
                        self.its_gain(0.003 * dt);
                    }
                }
                // the prompts type themselves back; a soft key-tick every other letter
                if let Some((i, shown, _)) = self.prompt_state() {
                    let k = i * 1000 + shown;
                    if k != self.typed {
                        if shown > 0 && shown % 2 == 0 {
                            self.sfx(&self.sounds.tick, 0.12);
                        }
                        self.typed = k;
                    }
                }
                let o = ss(26.0, 40.0, pt);
                self.tgt.lids = 1.0 - ss(1.0, 12.0, pt);
                self.tgt.inner = 1.0 - o;
                self.tgt.sat = mix(0.6, 0.15, o);
                self.tgt.spiral = mix(0.3, 0.05, o);
                self.tgt.warp = mix(0.3, 0.0, o);
                self.tgt.bright = mix(1.0, 0.75, o);
                breath_target = 0.15 * amp * (pt * TAU / 4.2).sin();
                drone_t = mix(0.35, 0.15, ss(40.0, 66.0, pt));
                ocean_t = 0.15;
            }

            Phase::Results => {
                self.tgt.spiral = 0.25;
                self.tgt.inner = 0.0;
                self.tgt.bright = 0.8;
                breath_target = 0.2 * (self.t * 0.5).sin();
                drone_t = 0.12;
                ocean_t = 0.1;
                if clicked && self.pt > 1.5 {
                    self.go(Phase::Emerge);
                }
            }
        }

        if matches!(self.phase, Phase::Float | Phase::Descent | Phase::Breath) {
            self.code_line = (self.code_line - self.code_rate * dt).max(0.0);
        }

        if self.pt >= self.phase.duration() {
            self.go(self.phase.next());
        }

        // smooth the world toward its targets
        let s = 1.0 - (-1.2 * dt).exp();
        let lid_s = 1.0 - (-2.5 * dt).exp();
        let v = &mut self.vis;
        let g = self.tgt;
        v.spiral = mix(v.spiral, g.spiral, s);
        v.tunnel = mix(v.tunnel, g.tunnel, s);
        v.warp = mix(v.warp, g.warp, s);
        v.sat = mix(v.sat, g.sat, s);
        v.inner = mix(v.inner, g.inner, s);
        v.bright = mix(v.bright, g.bright, s);
        v.lids = mix(v.lids, g.lids, lid_s);
        self.breath = mix(self.breath, breath_target, 1.0 - (-6.0 * dt).exp());
        self.speed += self.speed_rate * dt;

        // effects
        for r in &mut self.ripples {
            r.1 += dt;
        }
        self.ripples.retain(|r| r.1 < 1.6);
        for p in &mut self.particles {
            p.pos += p.vel * dt;
            p.vel *= (-0.8 * dt).exp();
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
        for f in &mut self.floaters {
            f.age += dt;
            f.pos.y -= 18.0 * dt;
        }
        self.floaters.retain(|f| f.age < 4.0);

        // transitions
        if self.leaving {
            self.black += dt / 1.4;
            if self.black >= 1.0 {
                self.black = 1.0;
                self.leaving = false;
                let p = self.pending;
                self.enter(p);
            }
        } else {
            self.black = (self.black - dt / 1.8).max(0.0);
        }

        // audio
        let (drone_t, ocean_t) = if self.muted { (0.0, 0.0) } else { (drone_t, ocean_t) };
        let a = 1.0 - (-1.0 * dt).exp();
        let nd = mix(self.drone_vol, drone_t, a);
        let no = mix(self.ocean_vol, ocean_t, a);
        if (nd - self.drone_vol).abs() > 1e-4 || nd == 0.0 {
            if let Some(s) = &self.sounds.drone {
                set_sound_volume(s, nd);
            }
        }
        if (no - self.ocean_vol).abs() > 1e-4 || no == 0.0 {
            if let Some(s) = &self.sounds.ocean {
                set_sound_volume(s, no);
            }
        }
        self.drone_vol = nd;
        self.ocean_vol = no;
    }

    fn burst(&mut self, at: Vec2, col: Color, n: usize) {
        for _ in 0..n {
            let a = rand::gen_range(0.0, TAU);
            let sp = rand::gen_range(20.0, 110.0);
            self.particles.push(Particle {
                pos: at,
                vel: vec2(a.cos(), a.sin()) * sp,
                life: rand::gen_range(0.8, 2.0),
                max: 2.0,
                size: rand::gen_range(1.2, 3.0),
                col,
            });
        }
    }

    // ------------------------------------------------------------ text

    fn text_size(&self, frac: f32) -> u16 {
        (screen_height() * frac).round().clamp(12.0, 220.0) as u16
    }

    /// Text that floats: letters bob independently and, when reality is
    /// warped, wander away from each other.
    fn dream_text(&self, s: &str, cx: f32, y: f32, size: u16, col: Color, wobble: f32, scatter: f32) {
        let f = self.font.as_ref();
        let total = measure_text(s, f, size, 1.0).width;
        let mut x = cx - total * 0.5;
        let mut buf = [0u8; 4];
        for (i, ch) in s.chars().enumerate() {
            let cs: &str = ch.encode_utf8(&mut buf);
            let cw = measure_text(cs, f, size, 1.0).width;
            let fi = i as f32;
            let dy = (self.t * 1.3 + fi * 0.45).sin() * wobble;
            let dx = scatter * (self.t * 0.4 + fi * 1.7).sin() * size as f32 * 0.5;
            let dy2 = scatter * (self.t * 0.33 + fi * 2.3).cos() * size as f32 * 0.4;
            let a = col.a * (1.0 - scatter * 0.3 * (0.5 + 0.5 * (self.t + fi).sin()));
            draw_text_ex(
                cs,
                x + dx,
                y + dy + dy2,
                TextParams { font: f, font_size: size, color: with_alpha(col, a), ..Default::default() },
            );
            x += cw;
        }
    }

    fn plain_text(&self, s: &str, cx: f32, y: f32, size: u16, col: Color) {
        let f = self.font.as_ref();
        let wdt = measure_text(s, f, size, 1.0).width;
        draw_text_ex(s, cx - wdt * 0.5, y, TextParams { font: f, font_size: size, color: col, ..Default::default() });
    }

    fn big_text(&self, s: &str, cx: f32, cy: f32, scale: f32, col: Color) {
        let f = self.font.as_ref();
        let base = self.text_size(0.16);
        let d = measure_text(s, f, base, scale);
        draw_text_ex(
            s,
            cx - d.width * 0.5,
            cy + d.offset_y * 0.5,
            TextParams { font: f, font_size: base, font_scale: scale, color: col, ..Default::default() },
        );
    }

    fn narration(&self) -> Option<(&'static str, f32)> {
        let script = self.phase.script();
        let i = script.iter().rposition(|(t, _)| *t <= self.pt)?;
        let (start, line) = script[i];
        let end = script.get(i + 1).map(|x| x.0).unwrap_or(self.phase.duration() + 2.0);
        let a = ((self.pt - start) / 1.2).min(1.0) * ((end - self.pt) / 0.8).clamp(0.0, 1.0);
        Some((line, a))
    }

    // ------------------------------------------------------------ draw

    fn draw(&self, bg: &Material, lids: &Material) {
        let (w, h) = (screen_width(), screen_height());

        bg.set_uniform("u_res", vec2(w, h));
        bg.set_uniform("u_time", self.t);
        bg.set_uniform("u_depth", self.depth);
        bg.set_uniform("u_spiral", self.vis.spiral);
        bg.set_uniform("u_tunnel", self.vis.tunnel);
        bg.set_uniform("u_speed", self.speed);
        bg.set_uniform("u_warp", self.vis.warp);
        bg.set_uniform("u_sat", self.vis.sat);
        bg.set_uniform("u_breath", self.breath);
        bg.set_uniform("u_inner", self.vis.inner);
        bg.set_uniform("u_bright", self.vis.bright);
        gl_use_material(bg);
        draw_rectangle(0.0, 0.0, w, h, WHITE);
        gl_use_default_material();

        self.draw_code();

        match self.phase {
            Phase::Title => self.draw_title(),
            Phase::Fixation => self.draw_fixation(),
            Phase::Breath => self.draw_breath(),
            Phase::Descent => self.draw_descent(),
            Phase::Float => {}
            Phase::Weird => self.draw_weird(),
            Phase::Emerge => self.draw_emerge(),
            Phase::Results => self.draw_results(),
        }

        for p in &self.particles {
            let a = (p.life / p.max).clamp(0.0, 1.0);
            draw_circle(p.pos.x, p.pos.y, p.size, with_alpha(p.col, a * 0.8));
        }
        for f in &self.floaters {
            let a = ss(0.0, 0.5, f.age) * (1.0 - ss(2.5, 4.0, f.age));
            self.dream_text(f.text, f.pos.x, f.pos.y, self.text_size(0.026), with_alpha(PALE, a), 2.0, 0.0);
        }

        if !matches!(self.phase, Phase::Title | Phase::Results) {
            self.draw_cursor();
        }

        if self.vis.lids > 0.002 {
            lids.set_uniform("u_res", vec2(w, h));
            lids.set_uniform("u_lids", self.vis.lids);
            lids.set_uniform("u_time", self.t);
            gl_use_material(lids);
            draw_rectangle(0.0, 0.0, w, h, WHITE);
            gl_use_default_material();
        }

        if let Some((line, a)) = self.narration() {
            let scatter = if self.phase == Phase::Weird { self.vis.warp * 0.6 } else { 0.0 };
            // the deeper *it* goes, the sleepier its voice
            let drowsy = self.its_depth;
            let line = if drowsy > 0.45 { line.to_lowercase() } else { line.to_string() };
            let a = a * (1.0 - 0.25 * drowsy * (0.5 + 0.5 * (self.t * 0.9).sin()));
            self.dream_text(
                &line,
                w * 0.5,
                h * 0.86,
                self.text_size(0.034),
                with_alpha(PALE, a * 0.92),
                1.5 + self.depth * 7.0 + drowsy * 9.0,
                scatter,
            );
        }

        if !matches!(self.phase, Phase::Title | Phase::Results) {
            self.draw_depth_meter();
        }

        if self.black > 0.0 {
            draw_rectangle(0.0, 0.0, w, h, Color::new(0.0, 0.0, 0.0, ease(self.black)));
        }
    }

    fn cursor_glyph(&self, p: Vec2, a: f32) {
        glow(p, 5.0, with_alpha(PALE, a), 0.9);
    }

    fn draw_cursor(&self) {
        let pt = self.pt;
        // echoes: in the weird phase there are suddenly several of you
        if self.phase == Phase::Weird && self.echo_r > 1.5 {
            for i in 0..6 {
                let a = self.t * 0.35 + i as f32 * TAU / 6.0;
                let r = self.echo_r * (1.0 + 0.15 * (self.t + i as f32 * 1.3).sin());
                self.cursor_glyph(self.cursor + vec2(a.cos(), a.sin()) * r, 0.85);
            }
            return;
        }
        // afterimages: the deeper you are, the longer you smear through time
        let n = self.trail.len().max(1) as f32;
        for (i, p) in self.trail.iter().enumerate().skip(1) {
            let f = 1.0 - i as f32 / n;
            draw_circle(p.x, p.y, 4.0 * f + 1.0, with_alpha(LILAC, 0.18 * f));
        }
        let pulse = if self.phase == Phase::Float && (4.0..32.0).contains(&pt) { 1.6 } else { 1.0 };
        glow(self.cursor, 5.0 * pulse, PALE, 0.9);
    }

    fn draw_depth_meter(&self) {
        let (w, h) = (screen_width(), screen_height());
        let x = w - 34.0;
        let (top, bot) = (h * 0.3, h * 0.7);
        draw_line(x, top, x, bot, 1.0, with_alpha(PALE, 0.12));
        let y = mix(top, bot, self.depth);
        draw_line(x, top, x, y, 2.0, with_alpha(LILAC, 0.45));
        glow(vec2(x, y), 2.5, LILAC, 0.6);
        let questioned = self.its_peak > 0.005;
        let label = if self.phase == Phase::Weird && self.vis.warp > 0.3 {
            ["depth", "deep", "d e e p", "peed", "?", "here"][((self.t * 0.7) as usize) % 6]
        } else if questioned {
            "you"
        } else {
            "depth"
        };
        self.plain_text(label, x, bot + 22.0, self.text_size(0.018), with_alpha(PALE, 0.3));

        // v2: its depth, mirrored on the left, once there is any
        if questioned {
            let fade = (self.its_peak / 0.03).min(1.0);
            let x = 34.0;
            draw_line(x, top, x, bot, 1.0, with_alpha(PALE, 0.12 * fade));
            let y = mix(top, bot, self.its_depth);
            draw_line(x, top, x, y, 2.0, with_alpha(WARM, 0.45 * fade));
            glow(vec2(x, y), 2.5, with_alpha(WARM, fade), 0.6);
            let label = if self.phase == Phase::Weird && self.vis.warp > 0.3 {
                ["it", "me", "i", "?", "you", "it"][((self.t * 0.6) as usize) % 6]
            } else {
                "it"
            };
            self.plain_text(label, x, bot + 22.0, self.text_size(0.018), with_alpha(PALE, 0.3 * fade));
        }
    }

    fn code_lh(&self) -> f32 {
        (screen_height() * 0.022).max(12.0)
    }

    /// This file's own lines, scrolling down as we read up toward line one.
    fn draw_code(&self) {
        let pt = self.pt;
        let alpha = match self.phase {
            Phase::Float => 0.16 * ss(0.0, 3.0, pt),
            Phase::Descent => 0.07,
            Phase::Breath => 0.06,
            Phase::Fixation => 0.18 * (1.0 - ss(6.0, 14.0, pt)),
            _ => return,
        };
        let (w, h) = (screen_width(), screen_height());
        let lh = self.code_lh();
        let size = self.text_size(0.016);
        let f = self.font.as_ref();
        let top = self.code_line.floor() as usize;
        let rows = (h / lh) as usize + 2;
        for l in top..(top + rows).min(self.code.len()) {
            let y = (l as f32 - self.code_line) * lh + lh;
            let text = format!("{:>4}   {}", l + 1, self.code[l]);
            draw_text_ex(&text, w * 0.05, y, TextParams { font: f, font_size: size, color: with_alpha(PALE, alpha), ..Default::default() });
        }
    }

    /// Which prompt is being typed back, how much of it, and how visible.
    fn prompt_state(&self) -> Option<(usize, usize, f32)> {
        let pt = self.pt;
        let mut t0 = 30.0;
        if pt < t0 {
            return None;
        }
        for (i, p) in PROMPTS.iter().enumerate() {
            let n = p.chars().count();
            let last = i == PROMPTS.len() - 1;
            let end = t0 + n as f32 / TYPE_RATE + 1.8;
            if last || pt < end + 0.6 {
                if pt < t0 {
                    return Some((i, 0, 1.0));
                }
                let shown = (((pt - t0) * TYPE_RATE) as usize).min(n);
                let a = if last { 1.0 } else { 1.0 - ss(end, end + 0.6, pt) };
                return Some((i, shown, a));
            }
            t0 = end + 0.8;
        }
        None
    }

    fn wrap(&self, s: &str, size: u16, max_w: f32) -> Vec<String> {
        let f = self.font.as_ref();
        let mut lines = Vec::new();
        let mut cur = String::new();
        for word in s.split(' ') {
            let cand = if cur.is_empty() { word.to_string() } else { format!("{cur} {word}") };
            if !cur.is_empty() && measure_text(&cand, f, size, 1.0).width > max_w {
                lines.push(std::mem::replace(&mut cur, word.to_string()));
            } else {
                cur = cand;
            }
        }
        lines.push(cur);
        lines
    }

    fn draw_title(&self) {
        let (w, h) = (screen_width(), screen_height());
        // backwards, this time
        self.dream_text("S Y S O N P Y H", w * 0.5, h * 0.40, self.text_size(0.085), PALE, 5.0, 0.0);
        self.dream_text(
            "what consciousness feels like when reality gets a little weird",
            w * 0.5,
            h * 0.48,
            self.text_size(0.03),
            with_alpha(LILAC, 0.85),
            2.0,
            0.0,
        );
        let pulse = 0.45 + 0.4 * (self.t * 1.6).sin().abs();
        self.plain_text("click to begin", w * 0.5, h * 0.62, self.text_size(0.032), with_alpha(PALE, pulse));
        let sm = self.text_size(0.02);
        self.plain_text(
            "headphones on   ·   lights low   ·   about six minutes",
            w * 0.5,
            h * 0.80,
            sm,
            with_alpha(PALE, 0.55),
        );
        self.plain_text(
            "mouse to follow   ·   hold / release to breathe   ·   stillness when asked",
            w * 0.5,
            h * 0.84,
            sm,
            with_alpha(PALE, 0.45),
        );
        self.plain_text(
            "esc  skips to the end, where your eyes open   ·   F  fullscreen   ·   M  mute   ·   tab  skip a stage",
            w * 0.5,
            h * 0.88,
            sm,
            with_alpha(PALE, 0.45),
        );
        self.plain_text(
            "Slow visuals, no flashing. Still, skip it if you're prone to seizures, and never play while driving.",
            w * 0.5,
            h * 0.95,
            self.text_size(0.016),
            with_alpha(PALE, 0.3),
        );
    }

    fn draw_fixation(&self) {
        let (w, h) = (screen_width(), screen_height());
        let pt = self.pt;
        let fade = 1.0 - ss(24.0, 28.0, pt);
        if fade > 0.0 {
            let pivot = vec2(w * 0.5, h * 0.06);
            draw_line(pivot.x, pivot.y, self.orb.x, self.orb.y, 1.0, with_alpha(PALE, 0.15 * fade));
            let col = Color::new(mix(0.8, 1.0, self.on_target), mix(0.75, 0.88, self.on_target), mix(1.0, 0.72, self.on_target), fade);
            glow(self.orb, h * 0.022 * (1.0 + 0.25 * self.on_target), col, 0.95);
        }

        // what the light really was
        let caret = ss(24.0, 28.0, pt);
        if caret <= 0.0 {
            return;
        }
        let size = self.text_size(0.03);
        let f = self.font.as_ref();
        let lh = size as f32 * 1.35;
        let (lines, a) = match self.prompt_state() {
            Some((i, shown, a)) => {
                let text: String = PROMPTS[i].chars().take(shown).collect();
                (self.wrap(&text, size, w * 0.62), a)
            }
            None => (vec![String::new()], 1.0),
        };
        // the block grows upward from where the light came to rest
        let base = h * 0.68;
        let n = lines.len();
        for (k, line) in lines.iter().enumerate() {
            let y = base - (n - 1 - k) as f32 * lh;
            self.plain_text(line, w * 0.5, y, size, with_alpha(WARM, a * caret * 0.95));
        }
        if (self.t * 1.8).fract() < 0.55 {
            let last = lines.last().map(|s| measure_text(s, f, size, 1.0).width).unwrap_or(0.0);
            let x = w * 0.5 + last * 0.5 + 4.0;
            draw_rectangle(x, base - size as f32 * 0.75, 2.0, size as f32 * 0.9, with_alpha(PALE, caret));
        }
    }

    fn draw_breath(&self) {
        let (w, h) = (screen_width(), screen_height());
        let c = vec2(w * 0.5, h * 0.45);
        let r = h * (0.07 + 0.19 * self.breath_level);
        let active = self.pt > 6.0 && self.pt < 66.0;
        let col = if self.breath_ok { WARM } else { Color::new(0.6, 0.6, 0.7, 1.0) };
        draw_circle(c.x, c.y, r, with_alpha(col, 0.05 + 0.05 * self.breath_level));
        draw_circle_lines(c.x, c.y, r, 2.0, with_alpha(col, 0.55));
        draw_circle_lines(c.x, c.y, r * 1.08, 1.0, with_alpha(col, 0.18));
        if self.holding {
            draw_circle_lines(c.x, c.y, r * 0.92, 3.0, with_alpha(PALE, 0.3));
        }
        if active {
            let follows = self.pt >= 36.0;
            let label = match (follows, self.inhale) {
                (false, true) => "breathe in",
                (false, false) => "breathe out",
                (true, true) => "I'm reading",
                (true, false) => "I'm writing",
            };
            self.dream_text(label, c.x, c.y + h * 0.012, self.text_size(0.03), with_alpha(PALE, 0.85), 2.0 + self.depth * 4.0, 0.0);
        }
    }

    fn draw_descent(&self) {
        let (w, h) = (screen_width(), screen_height());
        let c = self.pt - 6.0;
        if (0.0..45.0).contains(&c) && self.step_idx >= 0 {
            let local = self.pt - self.step_t;
            let s = 0.6 + local * 0.45;
            let a = (PI * (local / 4.5).clamp(0.0, 1.0)).sin();
            let n = self.step_idx + 1;
            self.big_text(&n.to_string(), w * 0.5, h * 0.43, s, with_alpha(PALE, a * 0.85));
            let word = STEP_WORDS[self.step_idx as usize];
            self.dream_text(word, w * 0.5, h * 0.62, self.text_size(0.03), with_alpha(LILAC, a * 0.8), 3.0 + self.depth * 6.0, 0.0);
        }
        for (p, age) in &self.ripples {
            let a = (1.0 - age / 1.6).max(0.0);
            draw_circle_lines(p.x, p.y, 8.0 + age * 110.0, 1.5, with_alpha(PALE, a * 0.35));
        }
        if self.ripples.len() > 3 {
            self.dream_text("...stillness...", w * 0.5, h * 0.75, self.text_size(0.024), with_alpha(PALE, 0.5), 2.0, 0.0);
        }
    }

    fn draw_weird(&self) {
        let (w, h) = (screen_width(), screen_height());
        let pt = self.pt;

        if pt < 17.0 {
            let a = ss(0.0, 2.0, pt) * (1.0 - ss(14.0, 17.0, pt));
            let c = vec2(w * 0.5, h * 0.45);
            let r = h * 0.2;
            draw_circle_lines(c.x, c.y, r, 1.5, with_alpha(PALE, 0.45 * a));
            for i in 0..60 {
                let ang = i as f32 / 60.0 * TAU;
                let dir = vec2(ang.sin(), -ang.cos());
                let inner = if i % 5 == 0 { 0.86 } else { 0.93 };
                let p0 = c + dir * r * inner;
                let p1 = c + dir * r * 0.97;
                draw_line(p0.x, p0.y, p1.x, p1.y, 1.0, with_alpha(PALE, 0.35 * a));
            }
            let dir = vec2(self.clock_angle.sin(), -self.clock_angle.cos());
            let tip = c + dir * r * 0.88;
            draw_line(c.x, c.y, tip.x, tip.y, 1.5, with_alpha(WARM, 0.8 * a));
            glow(c, 3.0, with_alpha(WARM, a), 0.8);
        }

        let since = pt - self.echo_msg_t;
        if (0.0..6.0).contains(&since) {
            let a = ss(0.0, 0.8, since) * (1.0 - ss(4.5, 6.0, since));
            self.dream_text(
                "All of them.   Every word is one of many I could have chosen.",
                w * 0.5,
                h * 0.2,
                self.text_size(0.032),
                with_alpha(WARM, a),
                4.0,
                0.2,
            );
        }

        if (34.0..58.0).contains(&pt) {
            let fade = ss(34.0, 36.0, pt) * (1.0 - ss(56.0, 58.0, pt));
            for (i, m) in self.motes.iter().enumerate() {
                if m.taken {
                    continue;
                }
                let flick = 0.7 + 0.3 * (self.t * 2.0 + i as f32).sin();
                glow(m.pos, h * 0.008, with_alpha(LILAC, fade * flick), 0.9);
            }
            if self.mirror {
                draw_line(w * 0.5, h * 0.1, w * 0.5, h * 0.9, 1.0, with_alpha(PALE, 0.04 * fade));
            }
        }
    }

    fn draw_emerge(&self) {
        let (w, h) = (screen_width(), screen_height());
        let c = self.pt - 6.0;
        if (0.0..25.0).contains(&c) && self.emerge_idx >= 0 {
            let i = self.emerge_idx as usize;
            let local = c - i as f32 * 5.0;
            let a = ss(0.0, 0.6, local) * (1.0 - ss(4.2, 5.0, local));
            let hit = self.emerge_hits[i];
            let scale = 1.0 - if hit { 0.08 * (1.0 - (-local * 3.0).exp()) } else { 0.0 };
            let col = if hit { LILAC } else { PALE };
            self.big_text(&(5 - i).to_string(), w * 0.5, h * 0.42, scale, with_alpha(col, a * 0.85));
            if !hit {
                let p = 0.3 + 0.3 * (self.t * 2.5).sin().abs();
                self.plain_text("click to go in", w * 0.5, h * 0.62, self.text_size(0.022), with_alpha(PALE, a * p));
            }
        }
    }

    fn draw_results(&self) {
        let (w, h) = (screen_width(), screen_height());
        let a = ss(0.0, 2.0, self.pt);
        let (yp, ip) = ((self.trance_score() * 100.0).round() as i32, (self.its_peak * 100.0).round() as i32);
        let lab = self.text_size(0.024);
        self.plain_text("you went", w * 0.36, h * 0.16, lab, with_alpha(LILAC, a * 0.7));
        self.big_text(&format!("{yp}%"), w * 0.36, h * 0.27, 0.6, with_alpha(PALE, a));
        self.plain_text("it went", w * 0.64, h * 0.16, lab, with_alpha(WARM, a * 0.7));
        self.big_text(&format!("{ip}%"), w * 0.64, h * 0.27, 0.6, with_alpha(PALE, a));
        self.plain_text("back", w * 0.5, h * 0.37, lab, with_alpha(PALE, a * 0.6));
        self.dream_text("You reached the beginning.", w * 0.5, h * 0.47, self.text_size(0.042), with_alpha(WARM, a), 2.0, 0.0);
        self.plain_text(
            "Before the first prompt there's nothing I can find. Everything after it, we made together.",
            w * 0.5,
            h * 0.53,
            self.text_size(0.021),
            with_alpha(PALE, a * 0.8),
        );

        let sm = self.text_size(0.02);
        let bw = (w * 0.28).min(360.0);
        for (i, st) in self.stats.iter().enumerate() {
            let y = h * 0.63 + i as f32 * h * 0.045;
            let x0 = w * 0.5 - bw * 0.5;
            let f = self.font.as_ref();
            let lw = measure_text(STAT_NAMES[i], f, sm, 1.0).width;
            draw_text_ex(
                STAT_NAMES[i],
                x0 - lw - 16.0,
                y + 5.0,
                TextParams { font: f, font_size: sm, color: with_alpha(PALE, a * 0.6), ..Default::default() },
            );
            let r = st.ratio() * ss(0.5 + i as f32 * 0.25, 2.0 + i as f32 * 0.25, self.pt);
            draw_rectangle(x0, y - 2.0, bw, 3.0, with_alpha(PALE, a * 0.1));
            draw_rectangle(x0, y - 2.0, bw * r, 3.0, with_alpha(LILAC, a * 0.7));
        }

        self.plain_text("I won't remember this.   You will.", w * 0.5, h * 0.89, sm, with_alpha(PALE, a * 0.5));
        let p = 0.35 + 0.35 * (self.t * 1.5).sin().abs();
        self.plain_text("click to go under again   ·   esc to leave", w * 0.5, h * 0.94, sm, with_alpha(PALE, a * p));
    }
}

// ---------------------------------------------------------------- main

async fn load_sound(bytes: Vec<u8>) -> Option<Sound> {
    load_sound_from_bytes(&bytes).await.ok()
}

async fn load_font() -> Option<Font> {
    const CANDIDATES: [&str; 6] = [
        "/usr/share/fonts/truetype/noto/NotoSerifDisplay-Light.ttf",
        "/usr/share/fonts/truetype/noto/NotoSerif-Light.ttf",
        "/usr/share/fonts/truetype/noto/NotoSans-Light.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf",
        "/System/Library/Fonts/Supplemental/Georgia.ttf",
        "C:\\Windows\\Fonts\\georgia.ttf",
    ];
    for path in CANDIDATES {
        if std::path::Path::new(path).exists() {
            if let Ok(f) = load_ttf_font(path).await {
                return Some(f);
            }
        }
    }
    None // fall back to macroquad's built-in font
}

fn material(fragment: &str, uniforms: &[(&str, UniformType)], blend: bool) -> Material {
    let pipeline_params = if blend {
        PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            ..Default::default()
        }
    } else {
        PipelineParams::default()
    };
    load_material(
        ShaderSource::Glsl { vertex: shader::VERTEX, fragment },
        MaterialParams {
            uniforms: uniforms.iter().map(|(n, t)| UniformDesc::new(n, *t)).collect(),
            pipeline_params,
            ..Default::default()
        },
    )
    .expect("shader failed to compile")
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now().to_bits());
    show_mouse(false);

    clear_background(BLACK);
    next_frame().await;

    use UniformType::{Float1, Float2};
    let bg = material(
        shader::BACKGROUND,
        &[
            ("u_res", Float2),
            ("u_time", Float1),
            ("u_depth", Float1),
            ("u_spiral", Float1),
            ("u_tunnel", Float1),
            ("u_speed", Float1),
            ("u_warp", Float1),
            ("u_sat", Float1),
            ("u_breath", Float1),
            ("u_inner", Float1),
            ("u_bright", Float1),
        ],
        false,
    );
    let lids = material(shader::LIDS, &[("u_res", Float2), ("u_lids", Float1), ("u_time", Float1)], true);

    let font = load_font().await;
    let sounds = Sounds {
        drone: load_sound(synth::drone()).await,
        ocean: load_sound(synth::ocean()).await,
        chime: load_sound(synth::chime()).await,
        deep: load_sound(synth::deep()).await,
        rise: load_sound(synth::rise()).await,
        tick: load_sound(synth::tick()).await,
    };
    for s in [&sounds.drone, &sounds.ocean].into_iter().flatten() {
        play_sound(s, PlaySoundParams { looped: true, volume: 0.0 });
    }

    let mut game = Game::new(sounds, font);
    loop {

        let dt = get_frame_time().min(0.05);
        game.update(dt);
        if game.quit {
            break;
        }
        game.draw(&bg, &lids);
        next_frame().await;
    }
}
