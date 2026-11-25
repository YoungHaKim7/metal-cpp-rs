//! 031 — Phyllotaxis: 800 seeds placed at the golden angle with radius
//! c*sqrt(i); hue follows the seed index, dot size grows with radius and the
//! whole head slowly rotates. SPACE rebuilds the head at a new random scale.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 800.0;
const H: f64 = 800.0;
const SEEDS: usize = 800;
/// Golden angle in radians (~137.5 degrees).
const GOLDEN: f32 = 2.399_963_2;

struct Head {
    scale: f32,
    rng: gui::Rng,
}

fn main() {
    let mut head = Head {
        scale: 1.0,
        rng: gui::Rng::from_entropy(),
    };
    let app = gui::App::new("031 - Phyllotaxis", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        if f.input.key_pressed(gui::key::SPACE) {
            head.scale = head.rng.range(70, 140) as f32 / 100.0;
        }

        let (cx, cy) = (f.width * 0.5, f.height * 0.53);
        let r_max = (f.width.min(f.height) * 0.42).max(60.0) * head.scale;
        let c = r_max / (SEEDS as f32).sqrt();
        let rot = f.time * 0.15;

        for i in 0..SEEDS {
            let fi = i as f32;
            let r = c * fi.sqrt();
            let a = fi * GOLDEN + rot;
            // 4..9 units: at least two device pixels across on a non-Retina
            // display, where 1-2 pixel seeds left the spirals unreadable.
            let size = 4.0 + 5.0 * (r / r_max);
            f.rect(
                cx + a.cos() * r - size * 0.5,
                cy + a.sin() * r - size * 0.5,
                size,
                size,
                gui::Color::hsv(fi / SEEDS as f32 * 0.85 + 0.04, 0.8, 1.0),
            );
        }

        f.circle(cx, cy, 6.0, gui::Color::WHITE);
        f.text(24.0, 20.0, 4.0, "031 - PHYLLOTAXIS", gui::Color::WHITE);
        let label = format!(
            "SEEDS = {SEEDS}   SCALE = {:.2}   GOLDEN ANGLE = 137.5 DEG",
            head.scale
        );
        f.text(24.0, 54.0, 3.0, &label, gui::Color::YELLOW);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "RADIUS = C*SQRT(I) - PATTERN ROTATES - SPACE REBUILDS SCALE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
