//! 032 — Superellipse: three |x/a|^n + |y/b|^n = 1 outlines centered in a
//! row, each with n morphing 2..8 on a phase-shifted sine; 200 stroke
//! segments per shape.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 980.0;
const H: f64 = 560.0;
const TAU: f32 = std::f32::consts::TAU;
const SEGS: usize = 200;
/// Column centers as fractions of the width.
const COLS: [f32; 3] = [0.22, 0.5, 0.78];
/// Phase offsets so the three shapes morph out of step.
const PHASES: [f32; 3] = [0.0, 2.1, 4.2];
const COLORS: [gui::Color; 3] = [
    gui::Color::rgb(0.25, 0.9, 1.0),
    gui::Color::rgb(1.0, 0.9, 0.25),
    gui::Color::rgb(1.0, 0.35, 0.9),
];

fn main() {
    let app = gui::App::new("032 - Superellipse", W, H);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        let cy = f.height * 0.54;
        let r = (f.width * 0.15).min(f.height * 0.32).max(40.0);

        for e in 0..3 {
            let n = 5.0 + 3.0 * (f.time * 0.5 + PHASES[e]).sin();
            let cx = f.width * COLS[e];

            let mut prev = (cx + r, cy);
            for i in 0..=SEGS {
                let t = i as f32 / SEGS as f32 * TAU;
                let (s, c) = t.sin_cos();
                let x = cx + r * c.signum() * c.abs().powf(2.0 / n);
                let y = cy + r * s.signum() * s.abs().powf(2.0 / n);
                if i > 0 {
                    f.line(prev.0, prev.1, x, y, 2.2, COLORS[e]);
                }
                prev = (x, y);
            }

            let label = format!("N = {n:.2}");
            let lw = f.text_width(3.0, &label);
            f.text(cx - lw * 0.5, cy + r + 18.0, 3.0, &label, COLORS[e]);
        }

        f.text(24.0, 20.0, 4.0, "032 - SUPERELLIPSE", gui::Color::WHITE);
        f.text(
            24.0,
            54.0,
            3.0,
            "|X/A|^N + |Y/B|^N = 1   N MORPHS 2..8",
            gui::Color::GRAY,
        );
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "200 SEGMENTS PER OUTLINE - PHASE-SHIFTED MORPH - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
