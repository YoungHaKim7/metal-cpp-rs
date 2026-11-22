//! 011 — Rings: eight annuli expanding from the center, launched one every
//! eighth of the cycle. Ring i's radius is `fract(time*0.5 - i*0.125) *
//! max_radius` and its color fades toward the background as it grows (the
//! stock pipeline has no alpha blending, so fading is a color lerp). A small
//! static dot marks the launcher.

#[path = "gui/mod.rs"]
mod gui;

const BG: gui::Color = gui::Color::rgb(0.04, 0.04, 0.08);

fn main() {
    let app = gui::App::new("011 - Rings", 640.0, 400.0);
    app.run(|f| {
        f.set_clear(BG);

        let (cx, cy) = (f.width * 0.5, f.height * 0.5);
        let max_radius = (f.width * f.width + f.height * f.height).sqrt() * 0.5;
        for i in 0..8 {
            let ph = (f.time * 0.5 - i as f32 * 0.125).rem_euclid(1.0);
            let radius = ph * max_radius;
            if radius > 1.0 {
                let thickness = 4.0 + 26.0 * ph;
                let inner = (radius - thickness).max(0.0);
                let base = gui::Color::hsv(i as f32 / 8.0 + f.time * 0.03, 0.70, 0.95);
                let col = gui::Color::lerp(BG, base, 1.0 - ph);
                f.ring(cx, cy, radius, inner, col);
            }
        }

        // The static launcher dot.
        f.circle(cx, cy, 6.0, gui::Color::WHITE);

        f.text_center(f.width * 0.5, 14.0, 4.0, "RINGS", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "8 RINGS, ONE LAUNCH EVERY QUARTER SECOND - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
