//! 028 — Sierpinski Triangle: the chaos game — every frame 300 points hop
//! halfway toward a random triangle vertex and land in a persistent ring
//! buffer capped at 20 000 points, drawn as 1.5px rects; the palette slot
//! cycles by point index and R clears the cloud.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 820.0;
const H: f64 = 700.0;
const PER_FRAME: usize = 300;
const CAP: usize = 20_000;

/// Triangle corners in unit space: top, bottom-left, bottom-right.
const CORNERS: [(f32, f32); 3] = [(0.5, 0.02), (0.02, 0.98), (0.98, 0.98)];

/// Three-way palette picked by point index.
const PALETTE: [gui::Color; 3] = [
    gui::Color::rgb(0.2, 0.85, 1.0),
    gui::Color::rgb(1.0, 0.3, 0.75),
    gui::Color::rgb(0.95, 0.85, 0.25),
];

struct Chaos {
    rng: gui::Rng,
    pts: Vec<(f32, f32)>,
    slots: Vec<u8>,
    cursor: usize,
    px: f32,
    py: f32,
}

impl Chaos {
    fn new() -> Chaos {
        let mut game = Chaos {
            rng: gui::Rng::from_entropy(),
            pts: Vec::with_capacity(CAP),
            slots: Vec::with_capacity(CAP),
            cursor: 0,
            px: 0.5,
            py: 0.5,
        };
        game.seed();
        game
    }

    fn seed(&mut self) {
        self.pts.clear();
        self.slots.clear();
        self.cursor = 0;
        self.px = self.rng.f32();
        self.py = self.rng.f32();
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n {
            let v = self.rng.index(3);
            self.px = (self.px + CORNERS[v].0) * 0.5;
            self.py = (self.py + CORNERS[v].1) * 0.5;
            let slot = (self.cursor % 3) as u8;
            if self.pts.len() < CAP {
                self.pts.push((self.px, self.py));
                self.slots.push(slot);
            } else {
                let i = self.cursor % CAP;
                self.pts[i] = (self.px, self.py);
                self.slots[i] = slot;
            }
            self.cursor += 1;
        }
    }
}

fn main() {
    let mut game = Chaos::new();
    let app = gui::App::new("028 - Sierpinski", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        if f.input.key_pressed(gui::key::R) {
            game.seed();
        }
        game.step(PER_FRAME);

        let side = (f.height - 130.0).min(f.width - 60.0).max(60.0);
        let (ox, oy) = ((f.width - side) * 0.5, 70.0);

        // Faint triangle skeleton.
        for i in 0..3 {
            let a = CORNERS[i];
            let b = CORNERS[(i + 1) % 3];
            f.line(
                ox + a.0 * side,
                oy + a.1 * side,
                ox + b.0 * side,
                oy + b.1 * side,
                1.0,
                gui::Color::rgb(0.14, 0.14, 0.2),
            );
        }

        for (i, (u, v)) in game.pts.iter().enumerate() {
            f.rect(
                ox + u * side,
                oy + v * side,
                1.5,
                1.5,
                PALETTE[game.slots[i] as usize],
            );
        }

        f.text(24.0, 20.0, 4.0, "028 - SIERPINSKI", gui::Color::WHITE);
        let label = format!("POINTS = {} / {CAP}   CHAOS GAME", game.pts.len());
        f.text(24.0, 54.0, 3.0, &label, gui::Color::CYAN);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "+300 POINTS PER FRAME - 1.5PX RECTS - R CLEARS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
