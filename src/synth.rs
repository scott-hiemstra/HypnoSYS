//! Procedural audio: every sound in the game is synthesized here at startup
//! and handed to macroquad as an in-memory WAV file. No asset files needed.

use std::f64::consts::TAU;

const RATE: u32 = 44_100;

/// Tiny deterministic noise source.
struct Lcg(u32);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (self.0 >> 8) as f64 / (1u32 << 24) as f64 * 2.0 - 1.0
    }
}

fn frames(secs: f64) -> usize {
    (RATE as f64 * secs) as usize
}

/// Normalize a stereo buffer and encode it as a 16-bit PCM WAV.
fn to_wav(l: &[f64], r: &[f64]) -> Vec<u8> {
    let peak = l.iter().chain(r).fold(1e-9f64, |m, &x| m.max(x.abs()));
    let g = 0.85 / peak;
    let n = l.len();
    let data_len = (n * 4) as u32;
    let mut v = Vec::with_capacity(44 + n * 4);
    v.extend_from_slice(b"RIFF");
    v.extend_from_slice(&(36 + data_len).to_le_bytes());
    v.extend_from_slice(b"WAVE");
    v.extend_from_slice(b"fmt ");
    v.extend_from_slice(&16u32.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes()); // PCM
    v.extend_from_slice(&2u16.to_le_bytes()); // stereo
    v.extend_from_slice(&RATE.to_le_bytes());
    v.extend_from_slice(&(RATE * 4).to_le_bytes());
    v.extend_from_slice(&4u16.to_le_bytes());
    v.extend_from_slice(&16u16.to_le_bytes());
    v.extend_from_slice(b"data");
    v.extend_from_slice(&data_len.to_le_bytes());
    for i in 0..n {
        for s in [l[i], r[i]] {
            let q = (s * g * i16::MAX as f64).clamp(i16::MIN as f64, i16::MAX as f64) as i16;
            v.extend_from_slice(&q.to_le_bytes());
        }
    }
    v
}

fn sine(f: f64, t: f64) -> f64 {
    (TAU * f * t).sin()
}

/// A seamless 24 s loop. Left ear hears 100 Hz, right ear 106 Hz: the brain
/// perceives a slow 6 Hz "beat" (theta range) that exists in neither channel.
pub fn drone() -> Vec<u8> {
    let n = frames(24.0);
    let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
    for i in 0..n {
        let t = i as f64 / RATE as f64;
        let lfo = 0.7 + 0.3 * (TAU * t / 12.0).sin();
        let shimmer = 0.5 - 0.5 * (TAU * t / 8.0).cos();
        let common = 0.10 * sine(150.0, t) * lfo
            + 0.03 * sine(300.0, t) * shimmer
            + 0.02 * sine(450.0, t) * (1.0 - shimmer);
        l[i] = 0.30 * sine(100.0, t) + 0.08 * sine(200.0, t) + common;
        r[i] = 0.30 * sine(106.0, t) + 0.08 * sine(212.0, t) + common;
    }
    to_wav(&l, &r)
}

/// Filtered noise swelling like distant waves; silent at the loop seam.
pub fn ocean() -> Vec<u8> {
    let n = frames(24.0);
    let mut out = [vec![0.0; n], vec![0.0; n]];
    for (ch, buf) in out.iter_mut().enumerate() {
        let mut rng = Lcg(0x9E37 + ch as u32 * 7919);
        let (mut y1, mut y2) = (0.0, 0.0);
        for (i, s) in buf.iter_mut().enumerate() {
            let t = i as f64 / RATE as f64;
            // three swells per loop; zero at both ends so the loop is seamless.
            // Each ear gets its own noise so the wash feels wide.
            let env = 0.5 - 0.5 * (TAU * t / 8.0).cos();
            let a = 0.01 + 0.05 * env;
            y1 += a * (rng.next() - y1);
            y2 += a * (y1 - y2);
            *s = y2 * env;
        }
    }
    let [l, r] = out;
    to_wav(&l, &r)
}

fn bell(base: f64, secs: f64, partials: &[(f64, f64, f64)]) -> Vec<u8> {
    let n = frames(secs);
    let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
    for i in 0..n {
        let t = i as f64 / RATE as f64;
        let attack = (t / 0.006).min(1.0);
        let mut s = 0.0;
        for &(ratio, amp, decay) in partials {
            s += amp * sine(base * ratio, t) * (-t * decay).exp();
        }
        let tail = (1.0 - t / secs).max(0.0);
        l[i] = s * attack * tail;
        // tiny detune between ears gives it width
        r[i] = s * attack * tail * (1.0 + 0.02 * sine(0.7, t));
    }
    to_wav(&l, &r)
}

/// Glassy chime for each step of the descent.
pub fn chime() -> Vec<u8> {
    bell(
        528.0,
        4.0,
        &[(1.0, 1.0, 1.1), (2.76, 0.35, 2.4), (5.4, 0.12, 4.0), (0.5, 0.45, 0.8)],
    )
}

/// Bright rising chime for waking up.
pub fn rise() -> Vec<u8> {
    bell(784.0, 2.5, &[(1.0, 1.0, 1.8), (1.5, 0.5, 2.2), (3.0, 0.2, 3.5)])
}

/// A deep, slow "whoom" that sinks under each number.
pub fn deep() -> Vec<u8> {
    let secs = 3.0;
    let n = frames(secs);
    let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
    for i in 0..n {
        let t = i as f64 / RATE as f64;
        let env = (std::f64::consts::PI * t / secs).sin().powi(2);
        let f = 70.0 - 12.0 * (t / secs); // slight downward glide
        let s = (sine(f, t) + 0.3 * sine(f * 2.0, t)) * env;
        l[i] = s;
        r[i] = s;
    }
    to_wav(&l, &r)
}

/// Soft clock tick.
pub fn tick() -> Vec<u8> {
    let n = frames(0.12);
    let mut rng = Lcg(42);
    let (mut l, mut r) = (vec![0.0; n], vec![0.0; n]);
    for i in 0..n {
        let t = i as f64 / RATE as f64;
        let s = (0.6 * rng.next() + sine(1900.0, t)) * (-t * 70.0).exp();
        l[i] = s;
        r[i] = s;
    }
    to_wav(&l, &r)
}
