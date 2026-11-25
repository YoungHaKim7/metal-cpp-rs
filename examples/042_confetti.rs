//! 042 — Confetti: 300 paper pieces rain down, each swaying on its own
//! sine phase, spinning and flashing a rainbow hue. Holding the mouse
//! button sprays 20 fresh pieces per frame at the cursor; pieces that
//! leave the bottom recycle back to the top of the window.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 800.0;
const H: f64 = 600.0;
const BASE_COUNT: usize = 300;
const EMIT_PER_FRAME: usize = 20;
const CAP: usize = 1000;

struct Piece {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    amp: f32,
    freq: f32,
    phase: f32,
    spin: f32,
    angle: f32,
    size: f32,
    hue: f32,
}

struct Confetti {
    pieces: Vec<Piece>,
    rng: gui::Rng,
}

impl Confetti {
    fn new() -> Confetti {
        Confetti {
            pieces: Vec::with_capacity(CAP),
            rng: gui::Rng::from_entropy(),
        }
    }

    fn spawn(&mut self, x: f32, y: f32, vx: f32) {
        if self.pieces.len() >= CAP {
            self.pieces.remove(0);
        }
        self.pieces.push(Piece {
            x,
            y,
            vx,
            vy: 45.0 + self.rng.f32() * 85.0,
            amp: 10.0 + self.rng.f32() * 26.0,
            freq: 0.8 + self.rng.f32() * 1.6,
            phase: self.rng.f32() * std::f32::consts::TAU,
            spin: (self.rng.f32() - 0.5) * 9.0,
            angle: self.rng.f32() * std::f32::consts::TAU,
            // 5..12 units (2.5..6 device pixels on a non-Retina display):
            // small enough to be confetti, large enough to read as paper.
            size: 5.0 + self.rng.f32() * 7.0,
            hue: self.rng.f32(),
        });
    }

    fn fill(&mut self, w: f32, h: f32) {
        for _ in 0..BASE_COUNT {
            let x = self.rng.f32() * w;
            let y = self.rng.f32() * h;
            self.spawn(x, y, 0.0);
        }
    }
}

fn main() {
    let mut confetti = Confetti::new();
    let app = gui::App::new("042 - Confetti", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.04, 0.04, 0.08));

        if confetti.pieces.is_empty() {
            confetti.fill(f.width, f.height);
        }

        // Mouse spray: 20 new pieces per frame while the button is held.
        if f.input.mouse_down() {
            let (mx, my) = f.mouse();
            for _ in 0..EMIT_PER_FRAME {
                let vx = (confetti.rng.f32() - 0.5) * 360.0;
                confetti.spawn(mx, my, vx);
            }
        }

        // Integrate sway, spin and fall; recycle pieces leaving the bottom.
        let rng = &mut confetti.rng;
        confetti.pieces.retain_mut(|p| {
            p.phase += p.freq * f.dt;
            p.angle += p.spin * f.dt;
            p.vx *= (-2.4 * f.dt).exp();
            p.x += p.vx * f.dt + p.amp * p.freq * p.phase.cos() * f.dt;
            p.y += p.vy * f.dt;
            if p.x < -26.0 {
                p.x += f.width + 52.0;
            }
            if p.x > f.width + 26.0 {
                p.x -= f.width + 52.0;
            }
            if p.y > f.height + 12.0 {
                p.y = -12.0;
                p.x = rng.f32() * f.width;
                p.vx = 0.0;
            }
            true
        });

        // Draw each piece as a spinning rectangle (two triangles).
        for p in &confetti.pieces {
            let c = gui::Color::hsv(p.hue, 0.9, 1.0);
            let dark = gui::Color::lerp(c, gui::Color::BLACK, 0.25);
            let hx = p.size * 0.5 * p.angle.cos();
            let hy = p.size * 0.5 * p.angle.sin();
            let c1 = (p.x + hx, p.y + hy);
            let c2 = (p.x - hy, p.y + hx);
            let c3 = (p.x - hx, p.y - hy);
            let c4 = (p.x + hy, p.y - hx);
            f.tri(c1, c2, c3, c);
            f.tri(c1, c3, c4, dark);
        }

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "042 - CONFETTI",
            gui::Color::WHITE,
        );
        f.text(
            20.0,
            26.0,
            2.5,
            &format!("PIECES: {}", confetti.pieces.len()),
            gui::Color::GRAY,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "300 PIECES FALL WITH SWAY AND SPIN - HOLD MOUSE TO EMIT - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
