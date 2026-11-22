//! 052 — Paint Splatter: each click slaps ~30 dots around the cursor in a
//! gaussian cluster (one hue per splat, jittered per dot). Dots persist for
//! 20 s while their alpha fades linearly to zero; the pool caps at 4000 and
//! dead dots recycle. C wipes the canvas instantly.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const DOTS_PER_SPLAT: usize = 30;
const CAP: usize = 4000;
const FADE_S: f32 = 20.0;

struct Dot {
    x: f32,
    y: f32,
    r: f32,
    col: gui::Color,
    birth: f32,
}

struct Canvas {
    dots: Vec<Dot>,
    rng: gui::Rng,
}

/// Cheap gaussian: the sum of four uniforms is bell-shaped enough.
fn gauss(rng: &mut gui::Rng) -> f32 {
    (rng.f32() + rng.f32() + rng.f32() + rng.f32() - 2.0) * 0.5
}

impl Canvas {
    fn new() -> Canvas {
        Canvas {
            dots: Vec::new(),
            rng: gui::Rng::from_entropy(),
        }
    }

    fn splat(&mut self, x: f32, y: f32, now: f32) {
        let base_hue = self.rng.f32();
        for _ in 0..DOTS_PER_SPLAT {
            let dx = gauss(&mut self.rng) * 30.0;
            let dy = gauss(&mut self.rng) * 30.0;
            let hue = (base_hue + (self.rng.f32() - 0.5) * 0.10).rem_euclid(1.0);
            let c = gui::Color::hsv(hue, 0.82, 0.72 + self.rng.f32() * 0.28);
            self.dots.push(Dot {
                x: x + dx,
                y: y + dy,
                r: 2.0 + self.rng.f32() * 4.6,
                col: c,
                birth: now,
            });
        }
        if self.dots.len() > CAP {
            self.dots.drain(0..self.dots.len() - CAP);
        }
    }
}

fn main() {
    let mut canvas = Canvas::new();
    let app = gui::App::new("052 - Paint Splatter", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.05, 0.05, 0.08));

        if f.input.key_pressed(gui::key::C) {
            canvas.dots.clear();
        }
        if f.input.mouse_pressed() {
            let (mx, my) = f.mouse();
            canvas.splat(mx, my, f.time);
        }
        // Recycle dead dots (alpha has reached zero).
        canvas.dots.retain(|d| f.time - d.birth < FADE_S);

        for d in &canvas.dots {
            let a = (1.0 - (f.time - d.birth) / FADE_S).max(0.0);
            let segs = if d.r < 3.4 { 6 } else { 8 };
            f.circle_segments(
                d.x,
                d.y,
                d.r,
                segs,
                gui::Color::rgba(d.col.r, d.col.g, d.col.b, a),
            );
        }

        // Brush cursor.
        let (mx, my) = f.mouse();
        f.ring(mx, my, 12.0, 10.5, gui::Color::rgba(1.0, 1.0, 1.0, 0.35));

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "052 - PAINT SPLATTER",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 130.0,
            26.0,
            2.5,
            &format!("DOTS: {}", canvas.dots.len()),
            gui::Color::GRAY,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "CLICK SPLATS 30 GAUSSIAN DOTS - FADES AFTER 20S - C CLEARS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
