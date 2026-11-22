//! 023 — Superformula: Gielis' superformula r(phi) = (|cos(m*phi/4)|^n2 +
//! |sin(m*phi/4)|^n3)^(-1/n1) with m oscillating 3..12 and n1..n3 drifting;
//! the closed 250-segment shape is drawn as a hue-cycling filled fan with a
//! bright outline, auto-scaled to fit the window.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 880.0;
const H: f64 = 640.0;
const TAU: f32 = std::f32::consts::TAU;
const SEGS: usize = 250;

/// Superformula radius with a = b = 1 at angle `phi`.
fn super_radius(phi: f32, m: f32, n1: f32, n2: f32, n3: f32) -> f32 {
    let t = m * phi * 0.25;
    let a = t.cos().abs().powf(n2);
    let b = t.sin().abs().powf(n3);
    (a + b).powf(-1.0 / n1)
}

fn main() {
    let app = gui::App::new("023 - Superformula", W, H);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        let (cx, cy) = (f.width * 0.5, f.height * 0.53);
        let r_fit = (f.width.min(f.height) * 0.36).max(60.0);

        let m = 7.5 + 4.5 * (f.time * 0.31).sin();
        let n1 = 0.6 + 0.3 * (f.time * 0.19).sin();
        let n2 = 1.7 + 1.2 * (f.time * 0.23).sin();
        let n3 = 1.7 + 1.2 * (f.time * 0.27 + 1.3).sin();

        // Sample the closed curve and track the peak radius for fit-to-window.
        let mut pts = [(0.0f32, 0.0f32); SEGS];
        let mut r_peak = 1e-6f32;
        for (i, pt) in pts.iter_mut().enumerate() {
            let phi = i as f32 / SEGS as f32 * TAU;
            let r = super_radius(phi, m, n1, n2, n3);
            r_peak = r_peak.max(r);
            *pt = (phi, r);
        }
        let scale = r_fit / r_peak;

        let mut prev = (cx, cy);
        for i in 0..=SEGS {
            let (phi, r) = pts[i % SEGS];
            let p = (
                cx + (phi + 0.15).cos() * r * scale,
                cy + (phi + 0.15).sin() * r * scale,
            );
            let hue = i as f32 / SEGS as f32 * 0.22 + f.time * 0.06;
            if i > 0 {
                f.tri((cx, cy), prev, p, gui::Color::hsv(hue, 0.6, 0.3));
                f.line(
                    prev.0,
                    prev.1,
                    p.0,
                    p.1,
                    1.8,
                    gui::Color::hsv(hue, 0.8, 1.0),
                );
            }
            prev = p;
        }

        f.circle(cx, cy, 3.0, gui::Color::WHITE);
        f.text(24.0, 20.0, 4.0, "023 - SUPERFORMULA", gui::Color::WHITE);
        let label = format!("M = {m:.2}   N1 = {n1:.2}   N2 = {n2:.2}   N3 = {n3:.2}");
        f.text(24.0, 54.0, 3.0, &label, gui::Color::YELLOW);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "M OSCILLATES 3..12 - CLOSED SHAPE - HUE CYCLES WITH TIME - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
