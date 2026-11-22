//! 030 — Dragon IFS: the Heighway dragon as its two-map iterated function
//! system — map1 = (0.5x-0.5y, 0.5x+0.5y), map2 = (1-0.5x-0.5y, 0.5x-0.5y).
//! That second map (the complex form 1-(1-i)z/2) was chosen over the
//! tentative translated variant after plotting both numerically; only this
//! pair reproduces the dragon. 400 points per frame accumulate into a
//! 25 000-point ring buffer, hue by point index; R clears.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 820.0;
const H: f64 = 700.0;
const PER_FRAME: usize = 400;
const CAP: usize = 25_000;

struct Dragon {
    rng: gui::Rng,
    pts: Vec<(f32, f32, f32)>, // IFS-space x, y and the point's hue
    cursor: usize,
    x: f32,
    y: f32,
}

impl Dragon {
    fn new() -> Dragon {
        let mut dragon = Dragon {
            rng: gui::Rng::from_entropy(),
            pts: Vec::with_capacity(CAP),
            cursor: 0,
            x: 0.5,
            y: 0.5,
        };
        dragon.seed();
        dragon
    }

    fn seed(&mut self) {
        self.pts.clear();
        self.cursor = 0;
        self.x = 0.5;
        self.y = 0.5;
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n {
            let (nx, ny) = if self.rng.f32() < 0.5 {
                (0.5 * self.x - 0.5 * self.y, 0.5 * self.x + 0.5 * self.y)
            } else {
                (
                    1.0 - 0.5 * self.x - 0.5 * self.y,
                    0.5 * self.x - 0.5 * self.y,
                )
            };
            self.x = nx;
            self.y = ny;
            let hue = (self.cursor as f32 * 0.618_034) % 1.0;
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
    let mut dragon = Dragon::new();
    let app = gui::App::new("030 - Dragon IFS", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        if f.input.key_pressed(gui::key::R) {
            dragon.seed();
        }
        dragon.step(PER_FRAME);

        // Attractor bbox: x in [-0.333, 1.166], y in [-0.333, 0.666].
        let side = (f.height - 140.0).min(f.width - 80.0).max(60.0);
        let scale = side / 1.5;
        let (ox, oy) = ((f.width - side) * 0.5, 70.0 + (side - scale) * 0.5);

        for (x, y, hue) in &dragon.pts {
            f.rect(
                ox + (x + 0.333) * scale,
                oy + (0.666 - y) * scale,
                2.0,
                2.0,
                gui::Color::hsv(*hue, 0.8, 1.0),
            );
        }

        f.text(24.0, 20.0, 4.0, "030 - DRAGON IFS", gui::Color::WHITE);
        let label = format!(
            "POINTS = {} / {CAP}   HEIGHWAY DRAGON, 2 MAPS",
            dragon.pts.len()
        );
        f.text(24.0, 54.0, 3.0, &label, gui::Color::MAGENTA);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "MAP1 = (.5X-.5Y, .5X+.5Y)  MAP2 = (1-.5X-.5Y, .5X-.5Y) - R CLEARS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
