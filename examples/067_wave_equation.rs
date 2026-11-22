//! 067 — Wave Equation: u_tt = c^2 * lap(u) on a 120x80 field with double
//! buffering and mild damping. Click injects a Gaussian impulse; negative
//! amplitude renders blue, zero dark, positive red. D toggles extra
//! damping, SPACE pauses the ripples.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1040.0;
const H: f64 = 780.0;
const COLS: usize = 120;
const ROWS: usize = 80;
const CELL: f32 = 8.0;
const BOARD_Y: f32 = 84.0;
const STEP_EVERY: f32 = 1.0 / 90.0;
const C2: f32 = 0.4;
const DAMP: f32 = 0.995;
const DAMP_HEAVY: f32 = 0.94;

struct Wave {
    u: Vec<f32>,
    prev: Vec<f32>,
    next: Vec<f32>,
    acc: f32,
    paused: bool,
    heavy: bool,
}

impl Wave {
    fn step(&mut self) {
        let damp = if self.heavy { DAMP_HEAVY } else { DAMP };
        let Self { u, prev, next, .. } = self;
        for y in 0..ROWS {
            let ym = ((y as i32 - 1).rem_euclid(ROWS as i32) as usize) * COLS;
            let yp = ((y + 1) % ROWS) * COLS;
            let yc = y * COLS;
            for x in 0..COLS {
                let xm = (x as i32 - 1).rem_euclid(COLS as i32) as usize;
                let xp = (x + 1) % COLS;
                let c = yc + x;
                let lap = u[yc + xm] + u[yc + xp] + u[ym + x] + u[yp + x] - 4.0 * u[c];
                next[c] = damp * (2.0 * u[c] - prev[c]) + C2 * lap;
            }
        }
        std::mem::swap(prev, u);
        std::mem::swap(u, next);
    }

    fn impulse(&mut self, cx: i32, cy: i32) {
        for dy in -6i32..=6 {
            for dx in -6i32..=6 {
                let r2 = dx * dx + dy * dy;
                if r2 > 36 {
                    continue;
                }
                let x = (cx + dx).rem_euclid(COLS as i32) as usize;
                let y = (cy + dy).rem_euclid(ROWS as i32) as usize;
                self.u[y * COLS + x] += 1.6 * (-r2 as f32 / 8.0).exp();
            }
        }
    }
}

fn main() {
    let mut wave = Wave {
        u: vec![0.0; COLS * ROWS],
        prev: vec![0.0; COLS * ROWS],
        next: vec![0.0; COLS * ROWS],
        acc: 0.0,
        paused: false,
        heavy: false,
    };
    let app = gui::App::new("067 - Wave Equation", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.06, 0.06, 0.09));

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let bx = ((f.width - bw) * 0.5).max(16.0);

        // Input: click/drag injects impulses, D and SPACE toggle behavior.
        let (mx, my) = f.mouse();
        if (f.input.mouse_pressed() || f.input.mouse_down())
            && mx >= bx
            && mx < bx + bw
            && my >= BOARD_Y
            && my < BOARD_Y + bh
        {
            wave.impulse(((mx - bx) / CELL) as i32, ((my - BOARD_Y) / CELL) as i32);
        }
        if f.input.key_pressed(gui::key::D) {
            wave.heavy = !wave.heavy;
        }
        if f.input.key_pressed(gui::key::SPACE) {
            wave.paused = !wave.paused;
        }
        if !wave.paused {
            wave.acc += f.dt;
            while wave.acc >= STEP_EVERY {
                wave.acc -= STEP_EVERY;
                wave.step();
            }
        }

        // Header.
        f.text(bx, 18.0, 4.0, "WAVE EQUATION", gui::Color::WHITE);
        let status = format!(
            "DAMPING {}  {}",
            if wave.heavy { "HEAVY" } else { "MILD" },
            if wave.paused { "PAUSED" } else { "RUNNING" }
        );
        f.text(
            bx + bw - f.text_width(2.5, &status),
            26.0,
            2.5,
            &status,
            if wave.paused {
                gui::Color::YELLOW
            } else {
                gui::Color::CYAN
            },
        );

        // Board: signed amplitude — negative blue, zero dark, positive red.
        f.rect(bx, BOARD_Y, bw, bh, gui::Color::rgb(0.09, 0.09, 0.12));
        let dark = gui::Color::rgb(0.09, 0.09, 0.12);
        for y in 0..ROWS {
            for x in 0..COLS {
                let v = (wave.u[y * COLS + x] * 1.4).clamp(-1.0, 1.0);
                if v.abs() < 0.02 {
                    continue;
                }
                let ink = if v < 0.0 {
                    gui::Color::lerp(dark, gui::Color::BLUE, -v)
                } else {
                    gui::Color::lerp(dark, gui::Color::RED, v)
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
            "CLICK INJECTS IMPULSE - D EXTRA DAMPING - SPACE PAUSE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
