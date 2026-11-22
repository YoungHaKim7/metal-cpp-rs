//! 037 — Pendulum Wave: 15 pendulums hung from a top bar with periods
//! T(i) = 60/(60+i) seconds and angle = A*cos(2*pi*t/T(i)); the phases drift
//! apart into the classic traveling wave and reunite every 60 seconds.
//! R restarts the phase.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 980.0;
const H: f64 = 640.0;
const TAU: f32 = std::f32::consts::TAU;
const N: usize = 15;
const CYCLE: f32 = 60.0;
const AMPLITUDE: f32 = 0.55;

fn main() {
    let mut t0 = 0.0f32;
    let app = gui::App::new("037 - Pendulum Wave", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        if f.input.key_pressed(gui::key::R) {
            t0 = f.time;
        }
        let t = f.time - t0;

        let (cx, bar_y) = (f.width * 0.5, 84.0);
        let margin = f.width * 0.09;
        let bar_w = f.width - 2.0 * margin;
        let colw = bar_w / (N - 1) as f32;
        let l_max = f.height - bar_y - 90.0;

        // Top bar with a bright top edge.
        f.rect(
            margin - 18.0,
            bar_y - 5.0,
            bar_w + 36.0,
            10.0,
            gui::Color::rgb(0.16, 0.17, 0.24),
        );
        f.rect(
            margin - 18.0,
            bar_y - 5.0,
            bar_w + 36.0,
            3.0,
            gui::Color::rgb(0.42, 0.45, 0.6),
        );

        for i in 0..N {
            let fi = i as f32;
            let period = CYCLE / (CYCLE + fi);
            let angle = AMPLITUDE * (TAU * t / period).cos();
            let px = cx + (fi - (N - 1) as f32 * 0.5) * colw;
            let len = l_max * period * period;
            let bx = px + angle.sin() * len;
            let by = bar_y + angle.cos() * len;

            f.line(px, bar_y, bx, by, 1.5, gui::Color::rgb(0.3, 0.32, 0.42));
            let hue = 0.55 + fi / N as f32 * 0.45;
            f.circle_segments(bx, by, 11.0, 16, gui::Color::hsv(hue, 0.7, 1.0));
            f.circle(bx, by, 7.0 - fi * 0.15, gui::Color::hsv(hue, 0.55, 1.0));
        }

        // Progress through the 60s reunion cycle.
        let prog = (t % CYCLE) / CYCLE;
        f.rect(
            margin,
            f.height - 52.0,
            bar_w,
            1.0,
            gui::Color::rgb(0.2, 0.2, 0.28),
        );
        f.rect(
            margin,
            f.height - 53.0,
            bar_w * prog,
            3.0,
            gui::Color::rgb(0.35, 0.6, 0.9),
        );

        f.text(24.0, 20.0, 4.0, "037 - PENDULUM WAVE", gui::Color::WHITE);
        let label = format!(
            "T(I) = 60/(60+I) S   REUNION EVERY {CYCLE} S   T = {:.1} S",
            t % CYCLE
        );
        f.text(24.0, 54.0, 3.0, &label, gui::Color::YELLOW);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "15 BOBS - PHASES DRIFT AND REUNITE - R RESTARTS PHASE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
