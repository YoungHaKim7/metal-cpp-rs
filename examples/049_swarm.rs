//! 049 — Swarm: 350 particles chase the mouse, each with its own top speed
//! plus a touch of wander noise. Once the mouse sits still for 2 s they
//! abandon it and orbit the window center in a slow carousel instead.
//! Color maps to speed — cool blue is lazy, hot red is flat-out.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const SWARM: usize = 350;
const IDLE_T: f32 = 2.0;
const GAIN: f32 = 3.2;
const MAX_FORCE: f32 = 1300.0;
const TOP_MAX: f32 = 320.0;

struct Bug {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    top: f32,
    phase: f32,
    orbit: f32,
}

struct Swarm {
    bugs: Vec<Bug>,
    last: (f32, f32),
    last_move: f32,
    rng: gui::Rng,
}

impl Swarm {
    fn new() -> Swarm {
        Swarm {
            bugs: Vec::new(),
            last: (-1000.0, -1000.0),
            last_move: -100.0,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn seed(&mut self, w: f32, h: f32) {
        self.bugs.clear();
        for _ in 0..SWARM {
            let a = self.rng.f32() * std::f32::consts::TAU;
            self.bugs.push(Bug {
                x: self.rng.f32() * w,
                y: self.rng.f32() * h,
                vx: a.cos() * 60.0,
                vy: a.sin() * 60.0,
                top: 90.0 + self.rng.f32() * (TOP_MAX - 90.0),
                phase: self.rng.f32() * std::f32::consts::TAU,
                orbit: 0.5 + self.rng.f32() * 0.6,
            });
        }
    }
}

fn main() {
    let mut swarm = Swarm::new();
    let app = gui::App::new("049 - Swarm", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.05));

        if swarm.bugs.is_empty() {
            swarm.seed(f.width, f.height);
        }

        let mp = f.mouse();
        if swarm.last.0 < -900.0 {
            swarm.last = mp;
        }
        let moved = (mp.0 - swarm.last.0).abs() + (mp.1 - swarm.last.1).abs() > 2.0;
        if moved {
            swarm.last = mp;
            swarm.last_move = f.time;
        }
        let idle = f.time - swarm.last_move > IDLE_T;

        let cx = f.width * 0.5;
        let cy = f.height * 0.46;
        let radius = (f.width.min(f.height) * 0.33).max(60.0);

        for b in &mut swarm.bugs {
            b.phase += 2.6 * f.dt;
            let (tx, ty) = if idle {
                let a = f.time * 1.35 + b.phase;
                (
                    cx + a.cos() * radius * b.orbit,
                    cy + a.sin() * radius * b.orbit * 0.8,
                )
            } else {
                (mp.0, mp.1)
            };
            let dx = tx - b.x;
            let dy = ty - b.y;
            let d = (dx * dx + dy * dy).sqrt().max(1.0);
            let dvx = dx / d * b.top;
            let dvy = dy / d * b.top;
            let mut ax = (dvx - b.vx) * GAIN + b.phase.cos() * 55.0;
            let mut ay = (dvy - b.vy) * GAIN + b.phase.sin() * 55.0;
            let am = (ax * ax + ay * ay).sqrt();
            if am > MAX_FORCE {
                ax *= MAX_FORCE / am;
                ay *= MAX_FORCE / am;
            }
            b.vx += ax * f.dt;
            b.vy += ay * f.dt;
            let sp = (b.vx * b.vx + b.vy * b.vy).sqrt();
            if sp > b.top {
                let k = b.top / sp;
                b.vx *= k;
                b.vy *= k;
            }
            b.x += b.vx * f.dt;
            b.y += b.vy * f.dt;
            // Soft walls so nobody escapes the frame.
            if b.x < 8.0 {
                b.vx += 500.0 * f.dt;
            }
            if b.x > f.width - 8.0 {
                b.vx -= 500.0 * f.dt;
            }
            if b.y < 8.0 {
                b.vy += 500.0 * f.dt;
            }
            if b.y > f.height - 8.0 {
                b.vy -= 500.0 * f.dt;
            }
        }

        if idle {
            f.ring(
                cx,
                cy,
                radius * 1.06,
                radius * 1.02,
                gui::Color::rgba(0.4, 0.5, 0.8, 0.25),
            );
            // Below the title's band (y 20..48), not across it.
            f.text_center(f.width * 0.5, 54.0, 2.5, "ORBIT MODE", gui::Color::GRAY);
        } else {
            f.text_center(f.width * 0.5, 54.0, 2.5, "SEEKING MOUSE", gui::Color::CYAN);
        }

        for b in &swarm.bugs {
            let sp = (b.vx * b.vx + b.vy * b.vy).sqrt();
            let t = (sp / TOP_MAX).clamp(0.0, 1.0);
            let c = gui::Color::hsv(t, 0.85, 1.0);
            // 4 units: two device pixels across on a non-Retina display,
            // where the 2.6-unit seekers were single faint samples.
            f.rect(b.x - 2.0, b.y - 2.0, 4.0, 4.0, c);
        }

        f.text_center(f.width * 0.5, 20.0, 4.0, "049 - SWARM", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "350 SEEKERS CHASE THE MOUSE - STILL FOR 2S = ORBIT MODE - HUE BY SPEED - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
