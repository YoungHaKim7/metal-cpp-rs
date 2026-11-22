//! 043 — Galaxy: a 2000-star two-armed spiral. Stars are seeded along
//! logarithmic arms with gaussian scatter, then orbit with differential
//! rotation — inner stars sweep around faster than the rim. The core glows
//! yellow-white while the arms cool toward blue, and the whole disk drifts
//! slowly over a scatter of distant backdrop stars.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const STARS: usize = 2000;
const BACKDROP: usize = 140;
const ARM_SEED_R: f32 = 0.12;
const ARM_TWIST: f32 = 0.30;
const DISH_SQUASH: f32 = 0.55;
const SPIN: f32 = 0.05;

struct Star {
    rn: f32,
    ang: f32,
    sz: f32,
    bright: f32,
    twinkle: f32,
    col: gui::Color,
}

struct Galaxy {
    stars: Vec<Star>,
    backdrop: Vec<(f32, f32)>,
    rng: gui::Rng,
}

/// Cheap gaussian: the sum of four uniforms is bell-shaped enough.
fn gauss(rng: &mut gui::Rng) -> f32 {
    (rng.f32() + rng.f32() + rng.f32() + rng.f32() - 2.0) * 0.5
}

impl Galaxy {
    fn new() -> Galaxy {
        let mut g = Galaxy {
            stars: Vec::with_capacity(STARS),
            backdrop: Vec::new(),
            rng: gui::Rng::from_entropy(),
        };
        g.seed();
        g
    }

    fn seed(&mut self) {
        self.stars.clear();
        for i in 0..STARS {
            let (rn, ang) = if i % 5 == 0 {
                // Central bulge: 20% of the stars in a tight random cloud.
                (
                    gauss(&mut self.rng).abs() * 0.14,
                    self.rng.f32() * std::f32::consts::TAU,
                )
            } else {
                let mut r = 0.10 + 0.90 * self.rng.f32().powf(0.72) + gauss(&mut self.rng) * 0.04;
                r = r.clamp(0.06, 1.0);
                let arm = (i % 2) as f32;
                let base = (r / ARM_SEED_R).ln() / ARM_TWIST + arm * std::f32::consts::PI;
                (r, base + gauss(&mut self.rng) * 0.22)
            };
            let t = ((rn - 0.10) / 0.45).clamp(0.0, 1.0);
            let col = gui::Color::lerp(
                gui::Color::rgb(1.0, 0.93, 0.74),
                gui::Color::rgb(0.55, 0.70, 1.0),
                t,
            );
            self.stars.push(Star {
                rn,
                ang,
                sz: 1.0 + self.rng.f32() * 1.5,
                bright: 0.45 + self.rng.f32() * 0.55,
                twinkle: self.rng.f32() * std::f32::consts::TAU,
                col,
            });
        }
        self.backdrop.clear();
        for _ in 0..BACKDROP {
            self.backdrop.push((self.rng.f32(), self.rng.f32()));
        }
    }
}

fn main() {
    let mut galaxy = Galaxy::new();
    let app = gui::App::new("043 - Galaxy", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.015, 0.02, 0.045));

        let cx = f.width * 0.5;
        let cy = f.height * 0.52;
        let radius = (f.width.min(f.height) * 0.46).max(80.0);

        // Differential rotation: omega falls off with radius.
        for st in &mut galaxy.stars {
            st.ang += SPIN / (st.rn + 0.14) * f.dt;
        }

        // Distant backdrop stars.
        for (nx, ny) in &galaxy.backdrop {
            f.rect(
                nx * f.width,
                ny * f.height,
                1.4,
                1.4,
                gui::Color::rgba(0.8, 0.85, 1.0, 0.35),
            );
        }

        // Core glow, then the disk (squashed on y like an inclined galaxy).
        f.circle(cx, cy, radius * 0.16, gui::Color::rgba(1.0, 0.9, 0.6, 0.07));
        f.circle(
            cx,
            cy,
            radius * 0.07,
            gui::Color::rgba(1.0, 0.95, 0.8, 0.22),
        );
        for st in &galaxy.stars {
            let r = st.rn * radius;
            let x = cx + st.ang.cos() * r;
            let y = cy + st.ang.sin() * r * DISH_SQUASH;
            let b = st.bright * (0.82 + 0.18 * (f.time * 2.0 + st.twinkle).sin());
            f.rect(
                x - st.sz * 0.5,
                y - st.sz * 0.5,
                st.sz,
                st.sz,
                gui::Color::rgba(st.col.r, st.col.g, st.col.b, b),
            );
        }

        f.text_center(f.width * 0.5, 20.0, 4.0, "043 - GALAXY", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "2000 STARS - TWO LOG-SPIRAL ARMS - DIFFERENTIAL ROTATION - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
