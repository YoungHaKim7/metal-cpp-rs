//! 033 — Grid Waves: a 24px grid of dots, each displaced vertically by
//! sin(x*0.02 + t) * cos(y*0.02 + t*0.5) * 10; the dot color maps the
//! displacement from blue (down) through purple to red (up).

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 600.0;
const SPACING: f32 = 24.0;
const AMP: f32 = 10.0;

fn main() {
    let app = gui::App::new("033 - Grid Waves", W, H);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));

        let top = 76.0;
        let bottom = 52.0;
        let plot_h = f.height - top - bottom;
        let nx = ((f.width - 40.0) / SPACING).floor().max(1.0) as usize;
        let ny = (plot_h / SPACING).floor().max(1.0) as usize;
        let ox = (f.width - (nx as f32 - 1.0) * SPACING) * 0.5;
        let oy = top + (plot_h - (ny as f32 - 1.0) * SPACING) * 0.5;

        for gy in 0..ny {
            for gx in 0..nx {
                let x = ox + gx as f32 * SPACING;
                let y = oy + gy as f32 * SPACING;
                let disp = (x * 0.02 + f.time).sin() * (y * 0.02 + f.time * 0.5).cos() * AMP;
                let tt = (disp + AMP) / (2.0 * AMP);
                let base = gui::Color::lerp(gui::Color::BLUE, gui::Color::RED, tt);
                let bright = 0.55 + 0.45 * (disp.abs() / AMP);
                f.circle_segments(
                    x,
                    y + disp,
                    3.4,
                    6,
                    gui::Color::rgb(base.r * bright, base.g * bright, base.b * bright),
                );
            }
        }

        f.text(24.0, 20.0, 4.0, "033 - GRID WAVES", gui::Color::WHITE);
        f.text(
            24.0,
            54.0,
            3.0,
            "DY = SIN(X*0.02+T) * COS(Y*0.02+T*0.5) * 10",
            gui::Color::CYAN,
        );
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "BLUE = DISPLACED DOWN - RED = DISPLACED UP - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
