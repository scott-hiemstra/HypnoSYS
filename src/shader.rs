//! GLSL shaders for the trance world.

pub const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying lowp vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
"#;

/// The world behind your eyes: a slow spiral, a tunnel you sink through,
/// reality that breathes and warps, colour that drains or bleeds with depth.
pub const BACKGROUND: &str = r#"#version 100
precision highp float;

uniform vec2  u_res;
uniform float u_time;
uniform float u_depth;
uniform float u_spiral;
uniform float u_tunnel;
uniform float u_speed;
uniform float u_warp;
uniform float u_sat;
uniform float u_breath;
uniform float u_inner;
uniform float u_bright;

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_res;
    vec2 p = (gl_FragCoord.xy - 0.5 * u_res) / u_res.y;
    float t = u_time;

    // the whole room breathes with you
    p *= 1.0 - 0.07 * u_breath;

    // reality goes soft
    float w = u_warp;
    p += w * 0.07 * vec2(sin(p.y * 5.0 + t * 0.6), sin(p.x * 4.0 - t * 0.5));
    p += w * 0.025 * vec2(sin(length(p) * 14.0 - t * 1.3));

    float r = length(p) + 1e-4;
    float a = atan(p.y, p.x);

    // three-armed logarithmic spiral, turning slower than you'd expect
    float sp = sin(3.0 * a + 10.0 * log(r) - t * (0.9 + 0.6 * u_depth));
    sp = smoothstep(-0.25, 0.65, sp);
    float spFade = smoothstep(0.0, 0.1, r) * (1.0 - 0.5 * smoothstep(0.5, 1.2, r));

    // the tunnel: rings rushing past as you sink
    float z = 0.25 / r + u_speed;
    float ring = smoothstep(0.40, 0.49, abs(fract(z) - 0.5));
    float stripes = 0.5 + 0.5 * sin(a * 8.0 + z * 2.0 + w * sin(t * 0.3) * 4.0);
    float near = smoothstep(0.03, 0.28, r);

    float hue = mix(0.58, 0.76, u_inner)
              + 0.04 * sin(t * 0.05 + r * 2.0)
              + w * 0.18 * sin(a * 2.0 + t * 0.15 + r * 3.0);

    vec3 col = hsv2rgb(vec3(hue, 0.65, 0.10 + 0.04 * sin(t * 0.3)));
    col = mix(col, hsv2rgb(vec3(hue + 0.07, 0.55, 0.55)), sp * u_spiral * 0.55 * spFade);

    vec3 tcol = hsv2rgb(vec3(hue - 0.1 + 0.06 * sin(z * 0.7), 0.6, 0.9));
    col += tcol * ring * u_tunnel * near * 0.55;
    col += tcol * stripes * 0.07 * u_tunnel * near;

    // a light at the very bottom of everything
    float glow = 0.015 / (r * r + 0.015);
    col += hsv2rgb(vec3(hue + 0.15, 0.3, 1.0)) * glow * (0.12 + 0.35 * u_tunnel);

    // peripheral vision narrows as you go deeper
    float vig = smoothstep(1.25, 0.15, length((uv - 0.5) * vec2(1.6, 1.2)) * (1.0 + u_depth * 0.7));
    col *= mix(0.3, 1.0, vig);

    float l = dot(col, vec3(0.299, 0.587, 0.114));
    col = mix(vec3(l), col, u_sat);
    col *= u_bright;

    // dither to kill gradient banding
    col += (fract(sin(dot(gl_FragCoord.xy, vec2(12.9898, 78.233))) * 43758.5453) - 0.5) / 255.0;

    gl_FragColor = vec4(col, 1.0);
}
"#;

/// Eyelids. Drawn over everything, with the warm red of light through skin.
pub const LIDS: &str = r#"#version 100
precision highp float;

uniform vec2  u_res;
uniform float u_lids;
uniform float u_time;

void main() {
    vec2 q = (gl_FragCoord.xy - 0.5 * u_res) / u_res.y;
    float ap = (1.0 - u_lids) * 0.75;
    float edge = ap * sqrt(max(0.0, 1.0 - (q.x * q.x) / 1.2));
    float visible = 1.0 - smoothstep(edge - 0.07, edge, abs(q.y));
    vec3 tint = vec3(0.04, 0.0, 0.015)
              + vec3(0.11, 0.015, 0.03) * (0.6 + 0.4 * sin(u_time * 0.4)) * max(0.0, 1.0 - length(q));
    gl_FragColor = vec4(tint, 1.0 - visible);
}
"#;
