//! 088 — Analog Clock: a live clock driven straight off the system clock.
//! The face carries 60 tick marks (hour ticks bolder and longer), smooth
//! hour/minute/second hands in different lengths and colors, and a center
//! dot. Time is computed from `SystemTime` in UTC and echoed below as a
//! digital HH:MM:SS UTC readout.

use std::time::SystemTime;

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 520.0;
const H: f64 = 640.0;

/// Seconds since the Unix epoch split into (hours, minutes, seconds+frac),
/// all in UTC.
fn utc_now() -> (u32, u32, f64) {
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let day_secs = d.as_secs() % 86400;
    let frac = d.subsec_nanos() as f64 / 1e9;
    (
        (day_secs / 3600) as u32,
        ((day_secs % 3600) / 60) as u32,
        (day_secs % 60) as f64 + frac,
    )
}

/// Clock hand endpoint: `a` is the fraction of a turn from 12 o'clock.
fn hand_point(cx: f32, cy: f32, len: f32, a: f32) -> (f32, f32) {
    let ang = a * std::f32::consts::TAU;
    (cx + ang.sin() * len, cy - ang.cos() * len)
}

fn main() {
    let app = gui::App::new("088 - Analog Clock", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.08, 0.08, 0.11));
        let s = f.scale;

        let (h, m, sec) = utc_now();
        let (cx, cy) = (f.width * 0.5, f.height * 0.40);
        let r = (f.width.min(f.height) * 0.34).min(210.0 * s);

        // Face + rim.
        f.circle(cx, cy, r, gui::Color::rgb(0.13, 0.13, 0.17));
        f.ring(cx, cy, r, r - 5.0 * s, gui::Color::rgb(0.32, 0.34, 0.44));

        // 60 tick marks; hour ticks bolder and longer.
        for i in 0..60usize {
            let a = i as f32 / 60.0;
            let hour = i % 5 == 0;
            let inner = if hour { r * 0.82 } else { r * 0.89 };
            let outer = r * 0.94;
            let (x0, y0) = hand_point(cx, cy, inner, a);
            let (x1, y1) = hand_point(cx, cy, outer, a);
            let (color, width) = if hour {
                (gui::Color::WHITE, 3.0 * s)
            } else {
                (gui::Color::rgb(0.45, 0.47, 0.56), 1.5 * s)
            };
            f.line(x0, y0, x1, y1, width, color);
        }

        // Cardinal numbers.
        for (label, a) in [("12", 0.0f32), ("3", 0.25), ("6", 0.5), ("9", 0.75)] {
            let (x, y) = hand_point(cx, cy, r * 0.70, a);
            let size = 4.0 * s;
            f.text_center(
                x,
                y - 3.5 * size,
                size,
                label,
                gui::Color::rgb(0.75, 0.78, 0.88),
            );
        }

        // Hands. Angles are fractions of a full turn from 12 o'clock.
        let sec_a = (sec / 60.0) as f32;
        let min_a = (m as f64 + sec / 60.0) as f32 / 60.0;
        let hour_a = ((h % 12) as f64 + m as f64 / 60.0 + sec / 3600.0) as f32 / 12.0;

        // Hour hand: short and stout, white.
        let (hx, hy) = hand_point(cx, cy, r * 0.50, hour_a);
        f.line(cx, cy, hx, hy, 7.0 * s, gui::Color::WHITE);
        // Minute hand: long, blue.
        let (mx, my) = hand_point(cx, cy, r * 0.72, min_a);
        f.line(cx, cy, mx, my, 4.5 * s, gui::Color::rgb(0.35, 0.55, 1.0));
        // Second hand: longest and thin, red, with a counterweight tail.
        let (sx, sy) = hand_point(cx, cy, r * 0.84, sec_a);
        let (tx, ty) = hand_point(cx, cy, -r * 0.14, sec_a);
        f.line(tx, ty, sx, sy, 2.0 * s, gui::Color::RED);
        f.circle(sx, sy, 4.0 * s, gui::Color::RED);

        // Center dot.
        f.circle(cx, cy, 8.0 * s, gui::Color::rgb(0.32, 0.34, 0.44));
        f.circle(cx, cy, 5.0 * s, gui::Color::WHITE);

        // Digital readout (UTC).
        let digital = format!("{:02}:{:02}:{:02} UTC", h, m, sec as u32);
        let size = 6.0 * s;
        f.text_center(
            f.width * 0.5,
            cy + r + 34.0 * s,
            size,
            &digital,
            gui::Color::YELLOW,
        );
        let frac = (sec.fract() * 10.0) as u32;
        let sub = format!("TENTHS 0.{frac}");
        f.text_center(
            f.width * 0.5,
            cy + r + 34.0 * s + size + 10.0 * s,
            2.5 * s,
            &sub,
            gui::Color::GRAY,
        );

        f.text_center(
            f.width * 0.5,
            f.height - 30.0 * s,
            2.0 * s,
            "SMOOTH HANDS - SYSTEM CLOCK - UTC",
            gui::Color::GRAY,
        );
    });
}
