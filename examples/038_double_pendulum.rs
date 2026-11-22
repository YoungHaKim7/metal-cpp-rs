//! 038 — Double Pendulum: chaotic two-arm physics with semi-implicit Euler
//! (m1=m2=1, l1=l2=1, g=9.81, substeps clamped to 1/240 s); the second bob
//! leaves a 600-point trace whose oldest points fade; R resets with random
//! initial angles.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 680.0;
const PI: f32 = std::f32::consts::PI;
const G: f32 = 9.81;
const TRAIL_CAP: usize = 600;
const MAX_SUBSTEP_DT: f32 = 1.0 / 240.0;

/// Angular accelerations for the equal-mass, equal-length double pendulum.
fn accelerations(th1: f32, th2: f32, w1: f32, w2: f32) -> (f32, f32) {
    let d = th1 - th2;
    let den = 2.0 - (2.0 * d).cos();
    let a1 = (-3.0 * G * th1.sin()
        - G * (th1 - 2.0 * th2).sin()
        - 2.0 * d.sin() * (w2 * w2 + w1 * w1 * d.cos()))
        / den;
    let a2 = 2.0 * d.sin() * (2.0 * w1 * w1 + 2.0 * G * th1.cos() + w2 * w2 * d.cos()) / den;
    (a1, a2)
}

struct Pend {
    rng: gui::Rng,
    th1: f32,
    th2: f32,
    w1: f32,
    w2: f32,
    trail: Vec<(f32, f32)>,
}

impl Pend {
    fn new() -> Pend {
        let mut p = Pend {
            rng: gui::Rng::from_entropy(),
            th1: PI * 0.75,
            th2: PI * 0.6,
            w1: 0.0,
            w2: 0.0,
            trail: Vec::new(),
        };
        p.reset();
        p
    }

    fn reset(&mut self) {
        self.th1 = self.rng.range(60, 170) as f32 * PI / 180.0;
        self.th2 = self.rng.range(60, 170) as f32 * PI / 180.0;
        self.w1 = 0.0;
        self.w2 = 0.0;
        self.trail.clear();
    }

    fn step(&mut self, dt: f32) {
        if !(self.th1.is_finite() && self.th2.is_finite()) {
            self.reset();
            return;
        }
        let substeps = ((dt / MAX_SUBSTEP_DT).ceil() as usize).clamp(1, 32);
        let h = dt / substeps as f32;
        for _ in 0..substeps {
            let (a1, a2) = accelerations(self.th1, self.th2, self.w1, self.w2);
            self.w1 += a1 * h;
            self.w2 += a2 * h;
            self.th1 += self.w1 * h;
            self.th2 += self.w2 * h;
        }
    }
}

fn main() {
    let mut pend = Pend::new();
    let app = gui::App::new("038 - Double Pendulum", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        if f.input.key_pressed(gui::key::R) {
            pend.reset();
        }
        pend.step(f.dt);

        let (px, py) = (f.width * 0.5, f.height * 0.3);
        let len = ((f.height * 0.62).min(f.width * 0.8) * 0.5).max(80.0);

        let b1 = (px + pend.th1.sin() * len, py + pend.th1.cos() * len);
        let b2 = (b1.0 + pend.th2.sin() * len, b1.1 + pend.th2.cos() * len);

        // Trace of the second bob; the oldest points fade out.
        pend.trail.push(b2);
        if pend.trail.len() > TRAIL_CAP {
            pend.trail.remove(0);
        }
        let n = pend.trail.len();
        for i in 1..n {
            let t = i as f32 / n as f32;
            let a = pend.trail[i - 1];
            let b = pend.trail[i];
            f.line(
                a.0,
                a.1,
                b.0,
                b.1,
                1.5,
                gui::Color::rgba(1.0, 0.6, 0.25, t * 0.85),
            );
        }

        // Mount, arms and bobs.
        f.rect(
            px - 34.0,
            py - 7.0,
            68.0,
            8.0,
            gui::Color::rgb(0.2, 0.22, 0.3),
        );
        f.circle(px, py, 6.0, gui::Color::GRAY);
        f.line(px, py, b1.0, b1.1, 3.0, gui::Color::rgb(0.62, 0.68, 0.8));
        f.line(
            b1.0,
            b1.1,
            b2.0,
            b2.1,
            3.0,
            gui::Color::rgb(0.62, 0.68, 0.8),
        );
        f.circle_segments(b1.0, b1.1, 16.0, 20, gui::Color::rgba(0.3, 0.9, 1.0, 0.3));
        f.circle(b1.0, b1.1, 10.0, gui::Color::rgb(0.3, 0.9, 1.0));
        f.circle_segments(b2.0, b2.1, 15.0, 20, gui::Color::rgba(1.0, 0.6, 0.25, 0.3));
        f.circle(b2.0, b2.1, 9.0, gui::Color::rgb(1.0, 0.72, 0.35));

        f.text(24.0, 20.0, 4.0, "038 - DOUBLE PENDULUM", gui::Color::WHITE);
        let label = format!(
            "TH1 = {:+.2} RAD   TH2 = {:+.2} RAD   TRAIL = {} / {TRAIL_CAP}",
            pend.th1,
            pend.th2,
            pend.trail.len()
        );
        f.text(24.0, 54.0, 3.0, &label, gui::Color::CYAN);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "M1=M2=1 L1=L2=1 G=9.81 SEMI-IMPLICIT EULER - R RESETS RANDOM ANGLES - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
