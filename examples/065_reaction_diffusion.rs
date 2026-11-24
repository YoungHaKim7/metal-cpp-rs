//! 065 — Reaction-Diffusion: the Gray-Scott model on a 140x90 torus —
//! nine-point Laplacian, dt = 1.0, feed = 0.037, kill = 0.06, dA = 1.0,
//! dB = 0.5. Chemical B paints the picture: dark for famine, cyan for
//! bloom, white at saturation. R reseeds with a handful of B-blobs; click
//! injects fresh B under the cursor.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1060.0;
const H: f64 = 780.0;
const COLS: usize = 140;
const ROWS: usize = 90;
const CELL: f32 = 7.0;
const BOARD_Y: f32 = 84.0;
const FEED: f32 = 0.037;
const KILL: f32 = 0.06;
const DA: f32 = 1.0;
const DB: f32 = 0.5;
const DT: f32 = 1.0;
const TICK: f32 = 1.0 / 30.0;
const SUBSTEPS: usize = 6;

struct GrayScott {
    a: Vec<f32>,
    b: Vec<f32>,
    na: Vec<f32>,
    nb: Vec<f32>,
    acc: f32,
}

impl GrayScott {
    fn blob(&mut self, cx: i32, cy: i32, r: i32) {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let x = (cx + dx).rem_euclid(COLS as i32) as usize;
                    let y = (cy + dy).rem_euclid(ROWS as i32) as usize;
                    let i = y * COLS + x;
                    self.a[i] = 0.0;
                    self.b[i] = 1.0;
                }
            }
        }
    }

    fn reseed(&mut self) {
        let mut rng = gui::Rng::from_entropy();
        self.a.fill(1.0);
        self.b.fill(0.0);
        for _ in 0..5 {
            let cx = rng.range(10, COLS as i64 - 10) as i32;
            let cy = rng.range(10, ROWS as i64 - 10) as i32;
            let r = rng.range(3, 5) as i32;
            self.blob(cx, cy, r);
        }
    }

    fn step(&mut self) {
        let Self { a, b, na, nb, .. } = self;
        for y in 0..ROWS {
            let ym = ((y as i32 - 1).rem_euclid(ROWS as i32) as usize) * COLS;
            let yp = ((y + 1) % ROWS) * COLS;
            let yc = y * COLS;
            for x in 0..COLS {
                let xm = (x as i32 - 1).rem_euclid(COLS as i32) as usize;
                let xp = (x + 1) % COLS;
                let c = yc + x;
                let lap_a = 0.2 * (a[yc + xm] + a[yc + xp] + a[ym + x] + a[yp + x])
                    + 0.05 * (a[ym + xm] + a[ym + xp] + a[yp + xm] + a[yp + xp])
                    - a[c];
                let lap_b = 0.2 * (b[yc + xm] + b[yc + xp] + b[ym + x] + b[yp + x])
                    + 0.05 * (b[ym + xm] + b[ym + xp] + b[yp + xm] + b[yp + xp])
                    - b[c];
                let abb = a[c] * b[c] * b[c];
                na[c] = (a[c] + (DA * lap_a - abb + FEED * (1.0 - a[c])) * DT).clamp(0.0, 1.0);
                nb[c] = (b[c] + (DB * lap_b + abb - (FEED + KILL) * b[c]) * DT).clamp(0.0, 1.0);
            }
        }
        std::mem::swap(a, na);
        std::mem::swap(b, nb);
    }
}

fn main() {
    let mut rd = GrayScott {
        a: vec![1.0; COLS * ROWS],
        b: vec![0.0; COLS * ROWS],
        na: vec![0.0; COLS * ROWS],
        nb: vec![0.0; COLS * ROWS],
        acc: 0.0,
    };
    rd.reseed();
    let app = gui::App::new_points("065 - Reaction Diffusion", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.06, 0.06, 0.09));

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let bx = ((f.width - bw) * 0.5).max(16.0);

        // Input: click injects a B blob, R reseeds.
        let (mx, my) = f.mouse();
        if f.input.mouse_pressed() && mx >= bx && mx < bx + bw && my >= BOARD_Y && my < BOARD_Y + bh
        {
            let cx = ((mx - bx) / CELL) as i32;
            let cy = ((my - BOARD_Y) / CELL) as i32;
            rd.blob(cx, cy, 4);
        }
        if f.input.key_pressed(gui::key::R) {
            rd.reseed();
        }
        rd.acc += f.dt;
        if rd.acc >= TICK {
            rd.acc = 0.0;
            for _ in 0..SUBSTEPS {
                rd.step();
            }
        }

        // Header.
        f.text(
            bx,
            18.0,
            4.0,
            "GRAY-SCOTT REACTION-DIFFUSION",
            gui::Color::WHITE,
        );
        f.text(
            bx + bw - f.text_width(2.5, "9-POINT LAPLACIAN - DT 1.0"),
            26.0,
            2.5,
            "9-POINT LAPLACIAN - DT 1.0",
            gui::Color::GRAY,
        );

        // Board: B concentration mapped dark -> cyan -> white.
        f.rect(bx, BOARD_Y, bw, bh, gui::Color::rgb(0.02, 0.04, 0.07));
        let cyan = gui::Color::CYAN;
        let dark = gui::Color::rgb(0.02, 0.04, 0.07);
        for y in 0..ROWS {
            for x in 0..COLS {
                let t = (rd.b[y * COLS + x] / 0.35).clamp(0.0, 1.0);
                if t < 0.02 {
                    continue;
                }
                let ink = if t < 0.5 {
                    gui::Color::lerp(dark, cyan, t * 2.0)
                } else {
                    gui::Color::lerp(cyan, gui::Color::WHITE, (t - 0.5) * 2.0)
                };
                f.rect(
                    bx + x as f32 * CELL,
                    BOARD_Y + y as f32 * CELL,
                    CELL - 1.0,
                    CELL - 1.0,
                    ink,
                );
            }
        }
        f.frame_rect(bx, BOARD_Y, bw, bh, 2.0, gui::Color::rgb(0.3, 0.3, 0.4));

        f.text(
            bx,
            H as f32 - 40.0,
            2.0,
            "R RESEED - CLICK INJECTS CHEMICAL B - FEED 0.037 KILL 0.06 - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
