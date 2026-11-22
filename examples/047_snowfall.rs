//! 047 — Snowfall: 300 flakes drift down on individual sinusoidal paths
//! while the mouse shoves them aside within an 80 px radius. Flakes that
//! slip below the bottom edge vanish — like a snow globe settling — and R
//! shakes it: every flake is reseeded at a fresh random position. The live
//! count is shown so you can watch the flurry thin out.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const FLAKES: usize = 300;
const REPEL_R: f32 = 80.0;
const REPEL_F: f32 = 240.0;

struct Flake {
    x: f32,
    y: f32,
    r: f32,
    vy: f32,
    amp: f32,
    freq: f32,
    phase: f32,
}

struct Snow {
    flakes: Vec<Flake>,
    rng: gui::Rng,
}

impl Snow {
    fn new() -> Snow {
        Snow {
            flakes: Vec::new(),
            rng: gui::Rng::from_entropy(),
        }
    }

    fn seed(&mut self, w: f32, h: f32) {
        self.flakes.clear();
        for _ in 0..FLAKES {
            self.flakes.push(Flake {
                x: self.rng.f32() * w,
                y: self.rng.f32() * h,
                r: 1.0 + self.rng.f32() * 2.0,
                vy: 22.0 + self.rng.f32() * 34.0,
                amp: 8.0 + self.rng.f32() * 26.0,
                freq: 0.5 + self.rng.f32() * 1.2,
                phase: self.rng.f32() * std::f32::consts::TAU,
            });
        }
    }
}

fn main() {
    let mut snow = Snow::new();
    let app = gui::App::new("047 - Snowfall", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.04, 0.08));

        if snow.flakes.is_empty() || f.input.key_pressed(gui::key::R) {
            snow.seed(f.width, f.height);
        }

        let (mx, my) = f.mouse();
        let mut i = 0;
        while i < snow.flakes.len() {
            let fl = &mut snow.flakes[i];
            fl.phase += fl.freq * f.dt;
            fl.x += fl.amp * fl.freq * fl.phase.cos() * f.dt;
            fl.y += fl.vy * f.dt;
            let dx = fl.x - mx;
            let dy = fl.y - my;
            let d2 = dx * dx + dy * dy;
            if d2 < REPEL_R * REPEL_R && d2 > 1e-6 {
                let d = d2.sqrt();
                let push = (1.0 - d / REPEL_R) * REPEL_F * f.dt;
                fl.x += dx / d * push;
                fl.y += dy / d * push;
            }
            let fell = fl.y > f.height + fl.r;
            if fell {
                snow.flakes.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // Flakes: small soft circles, dimmer when tiny.
        for fl in &snow.flakes {
            let a = 0.45 + 0.40 * (fl.r - 1.0) / 2.0;
            f.circle_segments(fl.x, fl.y, fl.r, 8, gui::Color::rgba(0.92, 0.95, 1.0, a));
        }

        // Repulsion field ring around the cursor.
        f.ring(
            mx,
            my,
            REPEL_R,
            REPEL_R - 1.5,
            gui::Color::rgba(0.5, 0.65, 0.9, 0.18),
        );

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "047 - SNOWFALL",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 150.0,
            26.0,
            2.5,
            &format!("FLAKES: {}", snow.flakes.len()),
            gui::Color::GRAY,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "MOUSE REPELS WITHIN 80PX - FLAKES VANISH BELOW THE BOTTOM - R RESEEDS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
