//! 024 — Fourier Square: a square wave synthesized live from its odd
//! harmonics (N grows 1..15 and loops), drawn across the window over the
//! ideal square wave; the Gibbs overshoot appears as the harmonics stack up.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 560.0;
const TAU: f32 = std::f32::consts::TAU;
const PI: f32 = std::f32::consts::PI;
const SAMPLES: usize = 440;
const N_MAX: f32 = 15.0;
const PERIODS: f32 = 2.0;

/// Sum of the first `n` odd harmonics of a square wave, amplitude-normalized.
fn square_partial(t: f32, n: usize) -> f32 {
    let mut sum = 0.0f32;
    for j in 0..n {
        let k = (2 * j + 1) as f32;
        sum += (k * t).sin() / k;
    }
    sum * (4.0 / PI)
}

fn main() {
    let app = gui::App::new("024 - Fourier Square", W, H);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        let cy = f.height * 0.56;
        let amp = f.height * 0.3;
        let x0 = f.width * 0.06;
        let x1 = f.width * 0.94;

        let n = (1.0 + (f.time * 0.6) % N_MAX) as usize;

        // Center axis.
        f.line(x0, cy, x1, cy, 1.0, gui::Color::rgb(0.15, 0.15, 0.22));

        // Ideal square wave, faint underneath.
        let mut prev = (x0, cy);
        for i in 0..=SAMPLES {
            let u = i as f32 / SAMPLES as f32;
            let t = u * TAU * PERIODS;
            let p = (x0 + u * (x1 - x0), cy - t.sin().signum() * amp);
            if i > 0 {
                f.line(
                    prev.0,
                    prev.1,
                    p.0,
                    p.1,
                    2.0,
                    gui::Color::rgb(0.3, 0.3, 0.4),
                );
            }
            prev = p;
        }

        // Synthesized partial sum on top.
        let mut prev = (x0, cy);
        for i in 0..=SAMPLES {
            let u = i as f32 / SAMPLES as f32;
            let t = u * TAU * PERIODS;
            let p = (x0 + u * (x1 - x0), cy - square_partial(t, n) * amp);
            if i > 0 {
                let hue = 0.5 + u * 0.25 + f.time * 0.05;
                f.line(
                    prev.0,
                    prev.1,
                    p.0,
                    p.1,
                    2.5,
                    gui::Color::hsv(hue, 0.7, 1.0),
                );
            }
            prev = p;
        }

        f.text(24.0, 20.0, 4.0, "024 - FOURIER SQUARE", gui::Color::WHITE);
        let label = format!("N = {n} / 15 ODD HARMONICS   K = 1..{}", 2 * n - 1);
        f.text(24.0, 54.0, 3.0, &label, gui::Color::CYAN);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "SUM OF 4/(PI*K) * SIN(K*T) OVER ODD K - GRAY = IDEAL WAVE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
