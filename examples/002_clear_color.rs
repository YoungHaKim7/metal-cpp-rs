//! 002 — Clear Color: the simplest animated background. The render-pass
//! clear color cycles through hue space while a white square orbits the
//! center — demonstrates [`Frame::set_clear`], [`Frame::rect`] and the
//! per-frame `time`/`dt` values.

#[path = "gui/mod.rs"]
mod gui;

fn main() {
    let app = gui::App::new("002 - Clear Color", 640.0, 400.0);
    app.run(|f| {
        // Slowly cycle the clear color through hue space.
        f.set_clear(gui::Color::hsv(f.time * 0.05, 0.5, 0.20));

        // A square orbiting the window center.
        let cx = f.width * 0.5;
        let cy = f.height * 0.5;
        let orbit = (f.height * 0.25).min(140.0);
        let x = cx + (f.time * 1.2).cos() * orbit;
        let y = cy + (f.time * 1.2).sin() * orbit;
        let s = 48.0;
        f.rect(x - s * 0.5, y - s * 0.5, s, s, gui::Color::WHITE);

        // Caption.
        f.text_center(
            cx,
            f.height - 40.0,
            2.0,
            "CLEAR COLOR CYCLES - SQUARE ORBITS",
            gui::Color::WHITE,
        );
    });
}
