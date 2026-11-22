//! 055 — Ripples: every click drops an expanding, fading ring on the pond.
//! Rings overlap freely (alpha blending is free) and at most 30 are alive —
//! the oldest pops when the pond is crowded. Rings run at ~200 px/s and die
//! at ~2.5 s. A lazy ambient ripple keeps the pond moving between clicks.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const MAX_LIVE: usize = 30;
const SPEED: f32 = 200.0;
const LIFE: f32 = 2.5;

struct Ripple {
    x: f32,
    y: f32,
    birth: f32,
    hue: f32,
}

struct Pond {
    rings: Vec<Ripple>,
    ambient: f32,
    rng: gui::Rng,
}

impl Pond {
    fn new() -> Pond {
        Pond {
            rings: Vec::new(),
            ambient: 0.6,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn drop(&mut self, x: f32, y: f32, now: f32) {
        if self.rings.len() >= MAX_LIVE {
            let mut oldest = 0usize;
            for i in 1..self.rings.len() {
                if self.rings[i].birth < self.rings[oldest].birth {
                    oldest = i;
                }
            }
            self.rings.remove(oldest);
        }
        self.rings.push(Ripple {
            x,
            y,
            birth: now,
            hue: self.rng.f32(),
        });
    }
}

fn main() {
    let mut pond = Pond::new();
    let app = gui::App::new("055 - Ripples", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.04, 0.07));

        if f.input.mouse_pressed() {
            let (mx, my) = f.mouse();
            pond.drop(mx, my, f.time);
        }
        // Ambient ripple so the pond never sits still.
        pond.ambient -= f.dt;
        if pond.ambient <= 0.0 {
            let x = 50.0 + pond.rng.f32() * (f.width - 100.0);
            let y = 50.0 + pond.rng.f32() * (f.height - 100.0);
            pond.drop(x, y, f.time);
            pond.ambient = 2.6;
        }
        pond.rings.retain(|r| f.time - r.birth < LIFE);

        for r in &pond.rings {
            let age = f.time - r.birth;
            let t = age / LIFE;
            let rad = SPEED * age;
            if rad <= 1.0 {
                continue;
            }
            let thick = 2.5 + 5.5 * (1.0 - t);
            let a = (1.0 - t) * 0.85;
            let c = gui::Color::hsv(r.hue, 0.45, 1.0);
            let inner = (rad - thick).max(0.8);
            f.ring(r.x, r.y, rad, inner, gui::Color::rgba(c.r, c.g, c.b, a));
            f.circle(r.x, r.y, 2.5, gui::Color::rgba(c.r, c.g, c.b, a * 0.8));
        }

        // Cursor target.
        let (mx, my) = f.mouse();
        f.ring(mx, my, 10.0, 8.5, gui::Color::rgba(0.7, 0.85, 1.0, 0.35));

        f.text_center(f.width * 0.5, 20.0, 4.0, "055 - RIPPLES", gui::Color::WHITE);
        f.text(
            f.width - 130.0,
            26.0,
            2.5,
            &format!("RINGS: {}", pond.rings.len()),
            gui::Color::GRAY,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "CLICK DROPS RINGS AT ~200 PX/S - MAX 30 LIVE - RINGS OVERLAP - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
