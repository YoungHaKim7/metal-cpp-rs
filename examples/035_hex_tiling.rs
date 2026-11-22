//! 035 — Hex Tiling: a pointy-top hexagon grid (side ~18px) covering the
//! window; every hexagon is six center triangles filled with a hue that
//! sweeps across (col + row) as time advances, with a 1px visual gap.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 640.0;
const TAU: f32 = std::f32::consts::TAU;
const PI: f32 = std::f32::consts::PI;
const SIDE: f32 = 18.0;

fn main() {
    let app = gui::App::new("035 - Hex Tiling", W, H);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.04));

        let hstep = 3.0f32.sqrt() * SIDE;
        let vstep = 1.5 * SIDE;
        let ncols = ((f.width / hstep).ceil() as i32) + 2;
        let nrows = ((f.height / vstep).ceil() as i32) + 2;
        let radius = SIDE - 1.2; // 1px visual gap between hexagons

        for row in 0..nrows {
            for col in 0..ncols {
                let cx = hstep * (col as f32 + 0.5) + (row % 2) as f32 * hstep * 0.5;
                let cy = radius + row as f32 * vstep;
                let hue = ((col + row) as f32 * 0.045 + f.time * 0.06).rem_euclid(1.0);
                let color = gui::Color::hsv(hue, 0.62, 0.9);

                // Six triangles fan out from the center; first corner is the
                // top point (angle -90 degrees), so the hexagons are pointy-top.
                let mut prev = (cx, cy - radius);
                for k in 1..=6 {
                    let a = -PI / 2.0 + k as f32 / 6.0 * TAU;
                    let p = (cx + a.cos() * radius, cy + a.sin() * radius);
                    f.tri((cx, cy), prev, p, color);
                    prev = p;
                }
            }
        }

        f.text(24.0, 20.0, 4.0, "035 - HEX TILING", gui::Color::WHITE);
        let label = format!("SIDE = {SIDE} PX   POINTY-TOP");
        f.text(24.0, 54.0, 3.0, &label, gui::Color::CYAN);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "HUE SWEEPS ACROSS COL+ROW WITH TIME - 1PX GAP - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
