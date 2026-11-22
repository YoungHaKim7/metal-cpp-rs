//! 008 — Starfield: 300 stars in three depth layers radiating outward from
//! the center. Holding UP spins up the warp factor — stars accelerate and
//! stretch into radial streaks — and stars that leave the screen recycle
//! back near the center.

use std::f32::consts::TAU;

#[path = "gui/mod.rs"]
mod gui;

const PER_LAYER: usize = 100;
/// Radial speed per layer, as a fraction of the half-diagonal per second.
const SPEEDS: [f32; 3] = [0.20, 0.42, 0.80];
const SIZES: [f32; 3] = [1.3, 2.1, 3.2];
const BG: gui::Color = gui::Color::rgb(0.02, 0.02, 0.05);

struct Star {
    angle: f32,
    dist: f32,
    layer: usize,
}

struct Field {
    stars: Vec<Star>,
    warp: f32,
}

impl Field {
    fn new(rng: &mut gui::Rng) -> Field {
        let mut stars = Vec::with_capacity(PER_LAYER * 3);
        for layer in 0..3 {
            for _ in 0..PER_LAYER {
                stars.push(Star {
                    angle: rng.f32() * TAU,
                    dist: 4.0 + rng.f32() * rng.f32() * 700.0,
                    layer,
                });
            }
        }
        Field { stars, warp: 1.0 }
    }

    fn update(&mut self, f: &mut gui::Frame, rng: &mut gui::Rng) {
        let target = if f.input.key(gui::key::UP) { 4.5 } else { 1.0 };
        self.warp += (target - self.warp) * (f.dt * 2.5).min(1.0);
        let max_dist = (f.width * f.width + f.height * f.height).sqrt() * 0.5 + 40.0;
        for star in &mut self.stars {
            star.dist *= 1.0 + SPEEDS[star.layer] * self.warp * f.dt;
            if star.dist > max_dist {
                // Recycle near the center with a fresh direction.
                star.angle = rng.f32() * TAU;
                star.dist = 4.0 + rng.f32() * 30.0;
            }
        }
    }
}

fn main() {
    let mut rng = gui::Rng::from_entropy();
    let mut field = Field::new(&mut rng);
    let app = gui::App::new("008 - Starfield", 720.0, 440.0);
    app.run(move |f| {
        f.set_clear(BG);
        field.update(f, &mut rng);

        let (cx, cy) = (f.width * 0.5, f.height * 0.5);
        let streaking = field.warp > 1.35;
        let stretch = ((field.warp - 1.0) * 0.22).clamp(0.0, 0.6);
        let tones = [
            gui::Color::rgb(0.45, 0.55, 0.85),
            gui::Color::rgb(0.65, 0.78, 1.0),
            gui::Color::rgb(0.95, 0.97, 1.0),
        ];
        for star in &field.stars {
            let (dx, dy) = star.angle.sin_cos();
            let x = cx + dx * star.dist;
            let y = cy + dy * star.dist;
            let col = gui::Color::lerp(BG, tones[star.layer], 0.4 + 0.2 * star.layer as f32);
            if streaking {
                // Radial streak: the head trails back toward the center.
                let tx = x - dx * star.dist * stretch;
                let ty = y - dy * star.dist * stretch;
                f.line(x, y, tx, ty, SIZES[star.layer] * 1.6, col);
            } else {
                f.circle_segments(x, y, SIZES[star.layer], 8, col);
            }
        }

        f.text_center(f.width * 0.5, 14.0, 4.0, "STARFIELD", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "300 STARS IN 3 DEPTH LAYERS - HOLD UP FOR WARP DRIVE - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
