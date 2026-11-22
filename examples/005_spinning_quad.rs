//! 005 — Spinning Quad: a square rotating about its center. The side length
//! pulses with `sin(time)`, the fill hue follows the rotation angle, and a
//! thin white outline tracks the corners.

#[path = "gui/mod.rs"]
mod gui;

fn main() {
    let app = gui::App::new("005 - Spinning Quad", 640.0, 400.0);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.05, 0.05, 0.09));

        let (cx, cy) = (f.width * 0.5, f.height * 0.5);
        let angle = f.time * 0.9;
        let side = f.width.min(f.height) * 0.42 * (1.0 + 0.18 * (f.time * 1.4).sin());
        let half = side * 0.5;
        let (sin, cos) = angle.sin_cos();

        // Corners of the unrotated square, then rotated about the center.
        let mut pts = [(0.0f32, 0.0f32); 4];
        for (i, (px, py)) in [(-half, -half), (half, -half), (half, half), (-half, half)]
            .into_iter()
            .enumerate()
        {
            pts[i] = (cx + px * cos - py * sin, cy + px * sin + py * cos);
        }

        let fill = gui::Color::hsv(angle / std::f32::consts::TAU, 0.65, 0.92);
        f.tri(pts[0], pts[1], pts[2], fill);
        f.tri(pts[0], pts[2], pts[3], fill);
        for i in 0..4 {
            let a = pts[i];
            let b = pts[(i + 1) % 4];
            f.line(a.0, a.1, b.0, b.1, 2.0, gui::Color::WHITE);
        }

        // Soft orbit ring that frames the pulsing square.
        f.ring_segments(
            cx,
            cy,
            half * 2.0,
            half * 1.94,
            64,
            gui::Color::rgb(0.15, 0.16, 0.26),
        );

        f.text_center(f.width * 0.5, 14.0, 4.0, "SPINNING QUAD", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "SIDE PULSES WITH SIN(TIME) - FILL HUE FOLLOWS ROTATION - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
