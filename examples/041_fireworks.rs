//! 041 — Fireworks: every ~1.2 s a rocket lifts off from the bottom of the
//! window and bursts at its apex into ~80 sparks that arc under gravity,
//! feel air drag and fade out. Each burst is tinted with one random hue.
//! Click anywhere to launch an extra rocket aimed at the click's x.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 600.0;
const BURST: usize = 80;
const SPARK_CAP: usize = 1500;
const INTERVAL: f32 = 1.2;
const GRAVITY: f32 = 340.0;
const DRAG: f32 = 0.85;

#[derive(Clone, Copy)]
struct Rocket {
    x: f32,
    y: f32,
    vy: f32,
    burst_y: f32,
    hue: f32,
}

#[derive(Clone, Copy)]
struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    life: f32,
    hue: f32,
}

struct Show {
    rockets: Vec<Rocket>,
    sparks: Vec<Spark>,
    timer: f32,
    rng: gui::Rng,
}

impl Show {
    fn new() -> Show {
        Show {
            rockets: Vec::new(),
            sparks: Vec::new(),
            timer: 0.5,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn launch(&mut self, x: f32, ground_y: f32) {
        let vy = -(430.0 + self.rng.f32() * 150.0);
        let burst_y = (ground_y + vy * vy / (2.0 * GRAVITY)).max(70.0);
        self.rockets.push(Rocket {
            x,
            y: ground_y,
            vy,
            burst_y,
            hue: self.rng.f32(),
        });
    }

    fn burst(&mut self, r: &Rocket) {
        let keep = SPARK_CAP.saturating_sub(BURST);
        if self.sparks.len() > keep {
            self.sparks.drain(0..self.sparks.len() - keep);
        }
        for i in 0..BURST {
            let a = (i as f32 / BURST as f32) * std::f32::consts::TAU + self.rng.f32() * 0.25;
            let sp = 60.0 + self.rng.f32() * 270.0;
            self.sparks.push(Spark {
                x: r.x,
                y: r.y,
                vx: a.cos() * sp,
                vy: a.sin() * sp + r.vy * 0.2,
                age: 0.0,
                life: 1.0 + self.rng.f32() * 0.8,
                hue: r.hue,
            });
        }
    }
}

fn main() {
    let mut show = Show::new();
    let app = gui::App::new("041 - Fireworks", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.05));
        let ground_y = f.height - 26.0;

        // Scheduled + clicked launches.
        show.timer -= f.dt;
        if show.timer <= 0.0 {
            let x = 60.0 + show.rng.f32() * (f.width - 120.0);
            show.launch(x, ground_y);
            show.timer = INTERVAL * (0.75 + show.rng.f32() * 0.5);
        }
        if f.input.mouse_pressed() {
            let (mx, _) = f.mouse();
            show.launch(mx.clamp(40.0, f.width - 40.0), ground_y);
        }

        // Rockets rise until they reach the burst altitude.
        let mut done: Vec<Rocket> = Vec::new();
        show.rockets.retain_mut(|r| {
            r.y += r.vy * f.dt;
            r.vy += GRAVITY * f.dt;
            if r.y <= r.burst_y {
                done.push(*r);
                false
            } else {
                true
            }
        });
        for r in &done {
            show.burst(r);
        }

        // Sparks: gravity + exponential drag + fade.
        show.sparks.retain_mut(|s| {
            s.age += f.dt;
            let drag = (-DRAG * f.dt).exp();
            s.vx *= drag;
            s.vy = s.vy * drag + GRAVITY * f.dt;
            s.x += s.vx * f.dt;
            s.y += s.vy * f.dt;
            s.age < s.life
        });

        // Rockets: flickering head + short exhaust trail.
        for r in &show.rockets {
            let flicker = 0.7 + 0.3 * (f.time * 30.0).sin();
            f.line(
                r.x,
                r.y,
                r.x + 2.0,
                r.y + 20.0,
                2.5,
                gui::Color::rgba(1.0, 0.6, 0.25, 0.55),
            );
            f.rect(
                r.x - 2.0,
                r.y - 3.0,
                4.0,
                7.0,
                gui::Color::rgba(1.0, 0.95, 0.8, flicker),
            );
        }

        // Sparks.
        for s in &show.sparks {
            let t = (s.age / s.life).clamp(0.0, 1.0);
            let a = (1.0 - t) * (1.0 - t);
            let c = gui::Color::hsv(s.hue, 0.7, 1.0);
            let sz = 2.6 - 1.4 * t;
            f.rect(
                s.x - sz * 0.5,
                s.y - sz * 0.5,
                sz,
                sz,
                gui::Color::rgba(c.r, c.g, c.b, a),
            );
        }

        // Ground line.
        f.rect(
            0.0,
            ground_y,
            f.width,
            3.0,
            gui::Color::rgb(0.14, 0.15, 0.22),
        );

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "041 - FIREWORKS",
            gui::Color::WHITE,
        );
        f.text(
            20.0,
            26.0,
            2.5,
            &format!("SPARKS: {}", show.sparks.len()),
            gui::Color::GRAY,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "A ROCKET EVERY 1.2S - CLICK TO LAUNCH ONE - 80 SPARKS PER BURST - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
