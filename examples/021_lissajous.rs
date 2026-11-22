//! 021 — Lissajous: the parametric curve x = sin(a*t + phase), y = sin(b*t)
//! with the frequency ratio morphing through 3:2, 5:4 and 5:6 — each exact
//! ratio holds for a few seconds, then smoothsteps into the next. ~500
//! hue-coded dots trace the curve and a bright head dot sweeps the current t.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 920.0;
const H: f64 = 640.0;
const TAU: f32 = std::f32::consts::TAU;
const DOTS: usize = 500;
const PHASE: f32 = 0.35;
/// Seconds per (a, b) pair: hold the exact ratio, then morph to the next.
const SLOT: f32 = 8.0;
/// Fraction of each slot spent holding the exact ratio.
const HOLD: f32 = 0.6;

/// (a, b) frequency pairs the curve cycles through.
const PAIRS: [(f32, f32); 3] = [(3.0, 2.0), (5.0, 4.0), (5.0, 6.0)];

fn main() {
    let app = gui::App::new("021 - Lissajous", W, H);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        let (cx, cy) = (f.width * 0.5, f.height * 0.52);
        let (rx, ry) = (f.width * 0.38, f.height * 0.36);

        // Morph continuously between the pairs: hold, then smoothstep.
        let slot = f.time / SLOT;
        let idx = (slot as usize) % PAIRS.len();
        let frac = slot.fract();
        let next = (idx + 1) % PAIRS.len();
        let m = if frac < HOLD {
            0.0
        } else {
            let u = (frac - HOLD) / (1.0 - HOLD);
            u * u * (3.0 - 2.0 * u)
        };
        let a = PAIRS[idx].0 + (PAIRS[next].0 - PAIRS[idx].0) * m;
        let b = PAIRS[idx].1 + (PAIRS[next].1 - PAIRS[idx].1) * m;

        // Faint axes.
        f.line(
            cx,
            cy - ry,
            cx,
            cy + ry,
            1.0,
            gui::Color::rgb(0.13, 0.13, 0.2),
        );
        f.line(
            cx - rx,
            cy,
            cx + rx,
            cy,
            1.0,
            gui::Color::rgb(0.13, 0.13, 0.2),
        );

        for i in 0..DOTS {
            let t = i as f32 / DOTS as f32 * TAU;
            let x = cx + (a * t + PHASE).sin() * rx;
            let y = cy + (b * t).sin() * ry;
            let hue = i as f32 / DOTS as f32;
            f.circle_segments(x, y, 2.4, 8, gui::Color::hsv(hue, 0.75, 0.95));
        }

        // Bright head dot at the current t.
        let t_head = ((f.time * 0.35) % 1.0) * TAU;
        let hx = cx + (a * t_head + PHASE).sin() * rx;
        let hy = cy + (b * t_head).sin() * ry;
        f.circle_segments(hx, hy, 11.0, 24, gui::Color::rgba(1.0, 0.9, 0.55, 0.22));
        f.circle(hx, hy, 5.0, gui::Color::WHITE);

        f.text(24.0, 20.0, 4.0, "021 - LISSAJOUS", gui::Color::WHITE);
        let label = format!("A = {a:.2}   B = {b:.2}   PHASE = {PHASE}");
        f.text(24.0, 54.0, 3.0, &label, gui::Color::CYAN);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "RATIO MORPHS 3:2 - 5:4 - 5:6 EVERY 8S - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
