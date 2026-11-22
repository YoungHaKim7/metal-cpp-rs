//! 019 — Star Polygons: {5/2}, {7/3} and {9/4} rendered as closed line
//! loops — connect every p-th vertex of an n-gon and keep going until back
//! at the start — each spinning at a different speed, with an uppercase
//! label underneath.

use std::f32::consts::TAU;

#[path = "gui/mod.rs"]
mod gui;

const STARS: [(usize, usize); 3] = [(5, 2), (7, 3), (9, 4)];
const SPEEDS: [f32; 3] = [0.35, -0.50, 0.65];
const COLORS: [gui::Color; 3] = [
    gui::Color::rgb(1.0, 0.85, 0.30),
    gui::Color::rgb(0.30, 0.90, 1.0),
    gui::Color::rgb(1.0, 0.40, 0.85),
];

fn main() {
    let app = gui::App::new("019 - Star Polygons", 720.0, 420.0);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.05, 0.05, 0.09));

        for (i, &(n, p)) in STARS.iter().enumerate() {
            let cx = f.width * (i as f32 + 0.5) / 3.0;
            let cy = f.height * 0.45;
            let r = (f.width / 3.0).min(f.height) * 0.30;
            let rot = f.time * SPEEDS[i];
            let vertex = |k: usize| -> (f32, f32) {
                let a = rot + k as f32 * TAU / n as f32;
                (cx + a.cos() * r, cy + a.sin() * r)
            };
            let mut prev = vertex(0);
            for j in 1..=n {
                let cur = vertex((j * p) % n);
                f.line(prev.0, prev.1, cur.0, cur.1, 3.0, COLORS[i]);
                prev = cur;
            }
            // The rotation hub.
            f.circle_segments(cx, cy, 3.0, 12, gui::Color::rgb(0.35, 0.36, 0.45));
            let label = format!("{{{}/{}}}", n, p);
            f.text_center(cx, cy + r + 24.0, 3.0, &label, COLORS[i]);
        }

        f.text_center(f.width * 0.5, 14.0, 4.0, "STAR POLYGONS", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "CLOSED LOOPS THROUGH EVERY P-TH VERTEX OF AN N-GON - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
