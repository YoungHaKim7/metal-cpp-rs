//! 048 — Boids: 90 agents steer with the classic three rules — separation
//! (weight 1.5), alignment (1.0) and cohesion (1.0) — inside a 70 px
//! perception radius, wrapping at the edges with speed clamped to
//! 40..160 px/s. Hold the mouse button to attract the flock to the cursor.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 600.0;
const BOIDS: usize = 90;
const PERCEPTION: f32 = 70.0;
const SEP_RADIUS: f32 = 26.0;
const W_SEP: f32 = 1.5;
const W_ALI: f32 = 1.0;
const W_COH: f32 = 1.0;
const MIN_SPD: f32 = 40.0;
const MAX_SPD: f32 = 160.0;

struct Boid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

struct Flock {
    boids: Vec<Boid>,
    rng: gui::Rng,
}

impl Flock {
    fn new() -> Flock {
        Flock {
            boids: Vec::new(),
            rng: gui::Rng::from_entropy(),
        }
    }

    fn seed(&mut self, w: f32, h: f32) {
        self.boids.clear();
        for _ in 0..BOIDS {
            let a = self.rng.f32() * std::f32::consts::TAU;
            self.boids.push(Boid {
                x: self.rng.f32() * w,
                y: self.rng.f32() * h,
                vx: a.cos() * 100.0,
                vy: a.sin() * 100.0,
            });
        }
    }

    fn update(&mut self, f: &mut gui::Frame) {
        let dt = f.dt;
        let (mx, my) = f.mouse();
        let attract = f.input.mouse_down();
        let per2 = PERCEPTION * PERCEPTION;
        let sep2 = SEP_RADIUS * SEP_RADIUS;
        for i in 0..self.boids.len() {
            let (px, py, pvx, pvy) = {
                let b = &self.boids[i];
                (b.x, b.y, b.vx, b.vy)
            };
            let (mut sx, mut sy) = (0.0f32, 0.0f32);
            let (mut ax, mut ay) = (0.0f32, 0.0f32);
            let (mut cx, mut cy) = (0.0f32, 0.0f32);
            let mut n = 0usize;
            for j in 0..self.boids.len() {
                if j == i {
                    continue;
                }
                let o = &self.boids[j];
                let dx = o.x - px;
                let dy = o.y - py;
                let d2 = dx * dx + dy * dy;
                if d2 < per2 {
                    n += 1;
                    ax += o.vx;
                    ay += o.vy;
                    cx += o.x;
                    cy += o.y;
                    if d2 < sep2 && d2 > 1e-6 {
                        let d = d2.sqrt();
                        let wgt = (1.0 - d / SEP_RADIUS) / d;
                        sx -= dx * wgt;
                        sy -= dy * wgt;
                    }
                }
            }
            let mut fx = sx * 340.0 * W_SEP;
            let mut fy = sy * 340.0 * W_SEP;
            if n > 0 {
                let inv = 1.0 / n as f32;
                fx += (ax * inv - pvx) * 2.4 * W_ALI;
                fy += (ay * inv - pvy) * 2.4 * W_ALI;
                fx += (cx * inv - px) * 1.6 * W_COH;
                fy += (cy * inv - py) * 1.6 * W_COH;
            }
            if attract {
                let dx = mx - px;
                let dy = my - py;
                let d = (dx * dx + dy * dy).sqrt().max(1.0);
                let pull = 760.0 * (1.0 - d / 520.0).clamp(0.0, 1.0);
                fx += dx / d * pull;
                fy += dy / d * pull;
            }
            let b = &mut self.boids[i];
            b.vx += fx * dt;
            b.vy += fy * dt;
            let sp = (b.vx * b.vx + b.vy * b.vy).sqrt();
            if sp > MAX_SPD {
                let k = MAX_SPD / sp;
                b.vx *= k;
                b.vy *= k;
            } else if sp < MIN_SPD && sp > 1e-3 {
                let k = MIN_SPD / sp;
                b.vx *= k;
                b.vy *= k;
            } else if sp <= 1e-3 {
                b.vx = MIN_SPD;
                b.vy = 0.0;
            }
            b.x += b.vx * dt;
            b.y += b.vy * dt;
            const MARGIN: f32 = 14.0;
            if b.x < -MARGIN {
                b.x += f.width + 2.0 * MARGIN;
            }
            if b.x > f.width + MARGIN {
                b.x -= f.width + 2.0 * MARGIN;
            }
            if b.y < -MARGIN {
                b.y += f.height + 2.0 * MARGIN;
            }
            if b.y > f.height + MARGIN {
                b.y -= f.height + 2.0 * MARGIN;
            }
        }
    }
}

fn main() {
    let mut flock = Flock::new();
    let app = gui::App::new("048 - Boids", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));

        if flock.boids.is_empty() {
            flock.seed(f.width, f.height);
        }
        flock.update(f);

        // Oriented triangles, tinted by heading.
        for b in &flock.boids {
            let sp = (b.vx * b.vx + b.vy * b.vy).sqrt().max(1e-4);
            let (ux, uy) = (b.vx / sp, b.vy / sp);
            let hue = ((b.vy.atan2(b.vx) / std::f32::consts::TAU) + 1.0).rem_euclid(1.0);
            let c = gui::Color::hsv(hue, 0.55, 0.95);
            let tip = (b.x + ux * 11.0, b.y + uy * 11.0);
            let left = (b.x - ux * 5.0 - uy * 4.5, b.y - uy * 5.0 + ux * 4.5);
            let right = (b.x - ux * 5.0 + uy * 4.5, b.y - uy * 5.0 - ux * 4.5);
            f.tri(tip, left, right, c);
        }

        if f.input.mouse_down() {
            let (mx, my) = f.mouse();
            f.ring(mx, my, 26.0, 23.0, gui::Color::rgba(1.0, 0.9, 0.4, 0.5));
        }

        f.text_center(f.width * 0.5, 20.0, 4.0, "048 - BOIDS", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "SEPARATION 1.5 + ALIGNMENT 1.0 + COHESION 1.0 - HOLD MOUSE TO ATTRACT - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
