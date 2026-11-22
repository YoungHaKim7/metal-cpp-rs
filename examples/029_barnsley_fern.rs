//! 029 — Barnsley Fern: the classic four affine maps with weights
//! 0.01/0.85/0.07/0.07; 500 points land per frame in a ring buffer capped at
//! 30 000, drawn as 2px rects in green hues; R clears and reseeds.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 760.0;
const H: f64 = 720.0;
const PER_FRAME: usize = 500;
const CAP: usize = 30_000;

struct Fern {
    rng: gui::Rng,
    pts: Vec<(f32, f32, f32)>, // fern-space x, y and the point's green hue
    cursor: usize,
    x: f32,
    y: f32,
}

impl Fern {
    fn new() -> Fern {
        let mut fern = Fern {
            rng: gui::Rng::from_entropy(),
            pts: Vec::with_capacity(CAP),
            cursor: 0,
            x: 0.0,
            y: 0.0,
        };
        fern.seed();
        fern
    }

    fn seed(&mut self) {
        self.pts.clear();
        self.cursor = 0;
        self.x = 0.0;
        self.y = 0.0;
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n {
            // Weights 0.01 / 0.85 / 0.07 / 0.07.
            let r = self.rng.f32();
            let (nx, ny) = if r < 0.01 {
                (0.0, 0.16 * self.y)
            } else if r < 0.86 {
                (
                    0.85 * self.x + 0.04 * self.y,
                    -0.04 * self.x + 0.85 * self.y + 1.6,
                )
            } else if r < 0.93 {
                (
                    0.2 * self.x - 0.26 * self.y,
                    0.23 * self.x + 0.22 * self.y + 1.6,
                )
            } else {
                (
                    -0.15 * self.x + 0.28 * self.y,
                    0.26 * self.x + 0.24 * self.y + 0.44,
                )
            };
            self.x = nx;
            self.y = ny;
            let hue = 0.26 + (self.cursor % 89) as f32 * 0.0022;
            if self.pts.len() < CAP {
                self.pts.push((self.x, self.y, hue));
            } else {
                let i = self.cursor % CAP;
                self.pts[i] = (self.x, self.y, hue);
            }
            self.cursor += 1;
        }
    }
}

fn main() {
    let mut fern = Fern::new();
    let app = gui::App::new("029 - Barnsley Fern", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.04, 0.03));
        if f.input.key_pressed(gui::key::R) {
            fern.seed();
        }
        fern.step(PER_FRAME);

        // Fern math space: x in [-2.2, 2.7], y in [0, 10]; center x on 0.24.
        let plot_h = f.height - 150.0;
        let s = (plot_h / 10.0).min((f.width - 80.0) / 5.0).max(10.0);
        let (cx, base) = (f.width * 0.5 - 0.24 * s, f.height - 70.0);

        for (x, y, hue) in &fern.pts {
            f.rect(
                cx + x * s,
                base - y * s,
                2.0,
                2.0,
                gui::Color::hsv(*hue, 0.75, 0.95),
            );
        }

        f.text(24.0, 20.0, 4.0, "029 - BARNSLEY FERN", gui::Color::WHITE);
        let label = format!(
            "POINTS = {} / {CAP}   WEIGHTS 0.01/0.85/0.07/0.07",
            fern.pts.len()
        );
        f.text(24.0, 54.0, 3.0, &label, gui::Color::GREEN);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "+500 POINTS PER FRAME - 2PX RECTS - R CLEARS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
