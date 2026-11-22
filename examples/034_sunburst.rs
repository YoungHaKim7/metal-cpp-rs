//! 034 — Sunburst: 60 radial spokes alternating two warm colors, rotating
//! clockwise around a bright inner circle, plus a second 60-spoke set
//! counter-rotating through the outer ring.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 640.0;
const TAU: f32 = std::f32::consts::TAU;
const SPOKES: usize = 60;

fn main() {
    let app = gui::App::new("034 - Sunburst", W, H);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.04, 0.03, 0.05));
        let (cx, cy) = (f.width * 0.5, f.height * 0.53);
        let r = (f.width.min(f.height) * 0.44).max(60.0);

        let warm_a = gui::Color::rgb(1.0, 0.72, 0.2);
        let warm_b = gui::Color::rgb(0.78, 0.3, 0.12);
        let cool_a = gui::Color::rgb(0.25, 0.75, 0.9);
        let cool_b = gui::Color::rgb(0.16, 0.4, 0.55);

        // Outer ring spokes, counter-rotating.
        for i in 0..SPOKES {
            let a = i as f32 / SPOKES as f32 * TAU - f.time * 0.5;
            let (dx, dy) = a.sin_cos();
            let color = if i % 2 == 0 { cool_a } else { cool_b };
            f.line(
                cx + dx * r * 0.66,
                cy + dy * r * 0.66,
                cx + dx * r * 0.97,
                cy + dy * r * 0.97,
                5.0,
                color,
            );
        }

        // Divider ring between the two spoke sets.
        f.ring_segments(
            cx,
            cy,
            r * 0.625,
            r * 0.612,
            72,
            gui::Color::rgb(0.14, 0.12, 0.16),
        );

        // Inner spokes, rotating the other way.
        for i in 0..SPOKES {
            let a = i as f32 / SPOKES as f32 * TAU + f.time * 0.35;
            let (dx, dy) = a.sin_cos();
            let color = if i % 2 == 0 { warm_a } else { warm_b };
            f.line(
                cx + dx * r * 0.24,
                cy + dy * r * 0.24,
                cx + dx * r * 0.6,
                cy + dy * r * 0.6,
                7.0,
                color,
            );
        }

        // Bright inner circle with a soft halo.
        f.circle_segments(cx, cy, r * 0.2, 48, gui::Color::rgba(1.0, 0.72, 0.2, 0.25));
        f.circle(cx, cy, r * 0.15, warm_a);
        f.circle(cx, cy, r * 0.085, gui::Color::rgb(1.0, 0.95, 0.85));

        f.text(24.0, 20.0, 4.0, "034 - SUNBURST", gui::Color::WHITE);
        f.text(
            24.0,
            54.0,
            3.0,
            "60 + 60 SPOKES - COUNTER-ROTATING RING",
            gui::Color::YELLOW,
        );
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "INNER SET SPINS CW - OUTER RING SPINS CCW - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
