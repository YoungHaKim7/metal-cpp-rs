//! 022 — Rose Curve: r = cos(k*theta) with integer k cycling 2..9 every four
//! seconds, drawn as a 700-segment polyline that slowly rotates; the hue
//! sweeps along theta and even k blooms into 2k petals.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 880.0;
const H: f64 = 640.0;
const TAU: f32 = std::f32::consts::TAU;
const SEGS: usize = 700;
const K_MIN: i64 = 2;
const K_MAX: i64 = 9;
const K_SLOT: f32 = 4.0;

fn main() {
    let app = gui::App::new("022 - Rose Curve", W, H);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        let (cx, cy) = (f.width * 0.5, f.height * 0.53);
        let r_fit = (f.width.min(f.height) * 0.36).max(60.0);

        let k = K_MIN + ((f.time / K_SLOT) as i64 % (K_MAX - K_MIN + 1));
        let rot = f.time * 0.25;

        // Short grow-in ease whenever k changes, so the pop reads as a bloom.
        let age = (f.time / K_SLOT).fract();
        let grow = (age * 4.0).min(1.0);
        let scale = r_fit * (0.55 + 0.45 * grow);

        let mut prev = (cx, cy);
        for i in 0..=SEGS {
            let theta = i as f32 / SEGS as f32 * TAU;
            let r = (k as f32 * theta + rot).cos() * scale;
            let p = (cx + theta.cos() * r, cy + theta.sin() * r);
            if i > 0 {
                let hue = i as f32 / SEGS as f32 * 0.55 + 0.52 + f.time * 0.03;
                let dim = gui::Color::hsv(hue, 0.85, 0.32);
                let hot = gui::Color::hsv(hue, 0.85, 1.0);
                f.line(prev.0, prev.1, p.0, p.1, 4.5, dim);
                f.line(prev.0, prev.1, p.0, p.1, 1.6, hot);
            }
            prev = p;
        }

        f.circle(cx, cy, 4.0, gui::Color::WHITE);
        f.text(24.0, 20.0, 4.0, "022 - ROSE CURVE", gui::Color::WHITE);
        let petals = if k % 2 == 0 { k * 2 } else { k };
        let label = format!("K = {k}   PETALS = {petals}");
        f.text(24.0, 54.0, 3.0, &label, gui::Color::MAGENTA);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "R = COS(K*THETA) - K CYCLES 2..9 EVERY 4S - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
