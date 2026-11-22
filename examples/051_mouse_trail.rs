//! 051 — Mouse Trail: a glowing ribbon follows the cursor — the last 60
//! positions are stroked as connected thick lines whose hue slides along
//! the trail, over a wider low-alpha glow pass underneath. Leave the mouse
//! still for a moment and the ribbon heads off on its own orbit around the
//! window center, so the demo never goes dark.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const TRAIL_LEN: usize = 60;
const IDLE_T: f32 = 0.9;

struct Ribbon {
    pts: Vec<(f32, f32)>,
    last: (f32, f32),
    last_move: f32,
    orbit_ang: f32,
}

impl Ribbon {
    fn new() -> Ribbon {
        Ribbon {
            pts: Vec::new(),
            last: (-1000.0, -1000.0),
            last_move: -100.0,
            orbit_ang: 0.0,
        }
    }

    fn push(&mut self, x: f32, y: f32) {
        self.pts.push((x, y));
        if self.pts.len() > TRAIL_LEN {
            self.pts.remove(0);
        }
    }
}

fn main() {
    let mut ribbon = Ribbon::new();
    let app = gui::App::new("051 - Mouse Trail", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.05));

        let mp = f.mouse();
        if ribbon.last.0 < -900.0 {
            ribbon.last = mp;
        }
        let moved = (mp.0 - ribbon.last.0).abs() + (mp.1 - ribbon.last.1).abs() > 1.5;
        let idle = f.time - ribbon.last_move > IDLE_T;
        if moved {
            ribbon.last = mp;
            ribbon.last_move = f.time;
            ribbon.push(mp.0, mp.1);
        } else if idle {
            // Idle: the virtual cursor rides a lazy orbit around the center.
            ribbon.orbit_ang += f.dt * 1.8;
            let r = 150.0 + 42.0 * (f.time * 0.6).sin();
            let cx = f.width * 0.5;
            let cy = f.height * 0.5;
            let px = cx + ribbon.orbit_ang.cos() * r;
            let py = cy + ribbon.orbit_ang.sin() * r * 0.72;
            ribbon.last = (px, py);
            ribbon.push(px, py);
        }

        let n = ribbon.pts.len();
        if n >= 2 {
            let denom = (n - 1) as f32;
            // Wide glow pass underneath.
            for k in 1..n {
                let t = k as f32 / denom;
                let (x0, y0) = ribbon.pts[k - 1];
                let (x1, y1) = ribbon.pts[k];
                let hue = (f.time * 0.10 + t * 0.45).rem_euclid(1.0);
                let g = gui::Color::hsv(hue, 0.7, 0.7);
                f.line(
                    x0,
                    y0,
                    x1,
                    y1,
                    3.0 + 15.0 * t,
                    gui::Color::rgba(g.r, g.g, g.b, 0.10),
                );
            }
            // Bright core ribbon.
            for k in 1..n {
                let t = k as f32 / denom;
                let (x0, y0) = ribbon.pts[k - 1];
                let (x1, y1) = ribbon.pts[k];
                let hue = (f.time * 0.10 + t * 0.45).rem_euclid(1.0);
                let c = gui::Color::hsv(hue, 0.55, 1.0);
                f.line(
                    x0,
                    y0,
                    x1,
                    y1,
                    1.6 + 5.0 * t,
                    gui::Color::rgba(c.r, c.g, c.b, 0.25 + 0.75 * t),
                );
            }
            // Head dot.
            let (hx, hy) = ribbon.pts[n - 1];
            f.circle(hx, hy, 4.5, gui::Color::WHITE);
        }

        if idle {
            f.text_center(f.width * 0.5, 30.0, 2.5, "AUTO-ORBIT", gui::Color::GRAY);
        }

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "051 - MOUSE TRAIL",
            gui::Color::WHITE,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "60-POINT GLOWING RIBBON - IDLE RIDES AN ORBIT - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
