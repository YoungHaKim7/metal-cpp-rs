//! 018 — Polygons: six regular polygons with 3 to 8 sides in a horizontal
//! row, each rendered stroke-only (every edge a thick `f.line` segment) and
//! spinning at its own speed.

use std::f32::consts::TAU;

#[path = "gui/mod.rs"]
mod gui;

const SIDES: [usize; 6] = [3, 4, 5, 6, 7, 8];
const SPEEDS: [f32; 6] = [0.50, -0.65, 0.80, -0.95, 1.10, -1.25];

fn poly_points(cx: f32, cy: f32, r: f32, n: usize, rot: f32) -> Vec<(f32, f32)> {
    (0..n)
        .map(|k| {
            let a = rot + k as f32 * TAU / n as f32;
            (cx + a.cos() * r, cy + a.sin() * r)
        })
        .collect()
}

fn main() {
    let app = gui::App::new("018 - Polygons", 760.0, 420.0);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.05, 0.05, 0.09));

        let slot = f.width / SIDES.len() as f32;
        let radius = slot.min(f.height) * 0.30;
        for (i, &n) in SIDES.iter().enumerate() {
            let cx = slot * (i as f32 + 0.5);
            let cy = f.height * 0.47;
            let pts = poly_points(cx, cy, radius, n, f.time * SPEEDS[i]);
            let col = gui::Color::hsv(0.52 - i as f32 * 0.085 + f.time * 0.02, 0.70, 0.95);
            for k in 0..n {
                let a = pts[k];
                let b = pts[(k + 1) % n];
                f.line(a.0, a.1, b.0, b.1, 3.0, col);
            }
            f.text_center(
                cx,
                cy + radius + 22.0,
                3.0,
                &n.to_string(),
                gui::Color::GRAY,
            );
        }

        f.text_center(
            f.width * 0.5,
            14.0,
            4.0,
            "REGULAR POLYGONS",
            gui::Color::WHITE,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "6 REGULAR POLYGONS FROM 3 TO 8 SIDES - STROKE ONLY - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
