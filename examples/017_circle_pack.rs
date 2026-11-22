//! 017 — Circle Pack: random circles placed by rejection sampling, no two
//! overlapping (radius 6..60px, up to 300 circles). Each frame fires one
//! burst of placement attempts until the window is packed; hue encodes
//! radius. SPACE clears and restarts.

#[path = "gui/mod.rs"]
mod gui;

const MAX_CIRCLES: usize = 300;
const ATTEMPTS_PER_FRAME: usize = 400;
const R_MIN: f32 = 6.0;
const R_MAX: f32 = 60.0;
const GAP: f32 = 2.0;

struct Packer {
    circles: Vec<(f32, f32, f32)>,
    rng: gui::Rng,
    done: bool,
}

impl Packer {
    fn new() -> Packer {
        Packer {
            circles: Vec::new(),
            rng: gui::Rng::from_entropy(),
            done: false,
        }
    }

    fn fits(&self, x: f32, y: f32, r: f32, w: f32, h: f32) -> bool {
        if x < r || y < r || x + r > w || y + r > h {
            return false;
        }
        self.circles.iter().all(|&(cx, cy, cr)| {
            let dx = x - cx;
            let dy = y - cy;
            let rr = r + cr + GAP;
            dx * dx + dy * dy >= rr * rr
        })
    }

    fn burst(&mut self, w: f32, h: f32) {
        if self.done || self.circles.len() >= MAX_CIRCLES {
            return;
        }
        let mut placed = 0;
        for _ in 0..ATTEMPTS_PER_FRAME {
            if self.circles.len() >= MAX_CIRCLES {
                break;
            }
            let r = R_MIN + self.rng.f32() * (R_MAX - R_MIN);
            let x = self.rng.f32() * w;
            let y = self.rng.f32() * h;
            if self.fits(x, y, r, w, h) {
                self.circles.push((x, y, r));
                placed += 1;
            }
        }
        if placed == 0 {
            self.done = true;
        }
    }
}

fn main() {
    let mut packer = Packer::new();
    let app = gui::App::new("017 - Circle Pack", 720.0, 480.0);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.05, 0.05, 0.08));

        if f.input.key_pressed(gui::key::SPACE) {
            packer = Packer::new();
        }
        packer.burst(f.width, f.height);

        for &(x, y, r) in &packer.circles {
            let t = (r - R_MIN) / (R_MAX - R_MIN);
            let col = gui::Color::hsv(0.60 - 0.55 * t, 0.70, 0.95);
            f.circle_segments(x, y, r, 24, col);
        }

        let status = if packer.done {
            format!("PACKING COMPLETE - {} CIRCLES", packer.circles.len())
        } else {
            format!("{} / {} CIRCLES", packer.circles.len(), MAX_CIRCLES)
        };
        f.text_center(f.width * 0.5, 14.0, 4.0, "CIRCLE PACK", gui::Color::WHITE);
        f.text_center(f.width * 0.5, 48.0, 2.5, &status, gui::Color::YELLOW);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "REJECTION SAMPLING, ONE ATTEMPT BURST PER FRAME - SPACE RESTARTS - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
