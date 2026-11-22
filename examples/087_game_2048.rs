//! 087 — 2048: the sliding-tile classic on a 4x4 board. Arrow keys slide
//! and merge (one move per key press), each successful move spawns a new
//! tile (2 90% of the time, 4 otherwise) and scores the merge values.
//! Tile colors ramp from cool dark blues up to hot oranges with value.
//! Game-over is detected when no move is possible; reaching 2048 raises a
//! banner that can be dismissed to keep playing. `R` restarts.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 560.0;
const H: f64 = 660.0;
const N: usize = 4;

struct Game {
    grid: [u32; N * N],
    score: u32,
    best: u32,
    over: bool,
    banner: bool,
    rng: gui::Rng,
}

impl Game {
    fn new() -> Game {
        let mut g = Game {
            grid: [0; N * N],
            score: 0,
            best: 0,
            over: false,
            banner: false,
            rng: gui::Rng::from_entropy(),
        };
        g.spawn();
        g.spawn();
        g
    }

    fn spawn(&mut self) {
        let empty: Vec<usize> = (0..N * N).filter(|&i| self.grid[i] == 0).collect();
        if empty.is_empty() {
            return;
        }
        let i = empty[self.rng.index(empty.len())];
        self.grid[i] = if self.rng.f32() < 0.9 { 2 } else { 4 };
    }

    /// Slide + merge one line of 4 tiles toward index 0.
    /// Returns `(moved, merge_gain)`.
    fn slide(line: &mut [u32; N]) -> (bool, u32) {
        let mut out = [0u32; N];
        let mut n = 0;
        for &v in line.iter() {
            if v != 0 {
                out[n] = v;
                n += 1;
            }
        }
        let mut gained = 0;
        let mut i = 0;
        while i + 1 < n {
            if out[i] == out[i + 1] {
                out[i] *= 2;
                gained += out[i];
                let mut j = i + 1;
                while j + 1 < n {
                    out[j] = out[j + 1];
                    j += 1;
                }
                out[n - 1] = 0;
                n -= 1;
            }
            i += 1;
        }
        let moved = out != *line;
        *line = out;
        (moved, gained)
    }

    /// Gather the tiles of line `k` for direction `dir` (0 left, 1 right,
    /// 2 up, 3 down) into a 4-array ordered from the slide target.
    fn gather(&self, dir: usize, k: usize) -> [u32; N] {
        let mut line = [0u32; N];
        for j in 0..N {
            let (x, y) = match dir {
                0 => (j, k),
                1 => (N - 1 - j, k),
                2 => (k, j),
                _ => (k, N - 1 - j),
            };
            line[j] = self.grid[y * N + x];
        }
        line
    }

    fn scatter(&mut self, dir: usize, k: usize, line: &[u32; N]) {
        for j in 0..N {
            let (x, y) = match dir {
                0 => (j, k),
                1 => (N - 1 - j, k),
                2 => (k, j),
                _ => (k, N - 1 - j),
            };
            self.grid[y * N + x] = line[j];
        }
    }

    fn move_tiles(&mut self, dir: usize) {
        if self.over {
            return;
        }
        let mut moved_any = false;
        for k in 0..N {
            let mut line = self.gather(dir, k);
            let (moved, gained) = Game::slide(&mut line);
            if moved {
                self.scatter(dir, k, &line);
                moved_any = true;
                self.score += gained;
                if self.grid.iter().any(|&v| v == 2048) && !self.banner {
                    self.banner = true;
                }
            }
        }
        if moved_any {
            self.spawn();
            self.best = self.best.max(self.score);
            if !self.can_move() {
                self.over = true;
            }
        }
    }

    fn can_move(&self) -> bool {
        if self.grid.iter().any(|&v| v == 0) {
            return true;
        }
        for y in 0..N {
            for x in 0..N {
                let v = self.grid[y * N + x];
                if x + 1 < N && self.grid[y * N + x + 1] == v {
                    return true;
                }
                if y + 1 < N && self.grid[(y + 1) * N + x] == v {
                    return true;
                }
            }
        }
        false
    }
}

fn tile_color(v: u32) -> gui::Color {
    if v == 0 {
        return gui::Color::rgb(0.17, 0.17, 0.21);
    }
    // Cool blue -> warm orange as the exponent grows.
    let t = (((v as f32).log2() - 1.0) / 10.0).clamp(0.0, 1.0);
    gui::Color::hsv(0.58 - 0.50 * t, 0.55 + 0.25 * t, 0.92 - 0.10 * t)
}

fn main() {
    let mut game = Game::new();
    let app = gui::App::new("087 - 2048", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.10, 0.10, 0.13));
        let s = f.scale;

        // ------------------------------------------------------------- input
        if f.input.key_pressed(gui::key::LEFT) {
            game.move_tiles(0);
        }
        if f.input.key_pressed(gui::key::RIGHT) {
            game.move_tiles(1);
        }
        if f.input.key_pressed(gui::key::UP) {
            game.move_tiles(2);
        }
        if f.input.key_pressed(gui::key::DOWN) {
            game.move_tiles(3);
        }
        if f.input.key_pressed(gui::key::R) {
            game = Game::new();
        }
        if game.banner && f.input.key_pressed(gui::key::SPACE) {
            game.banner = false;
        }

        // ------------------------------------------------------------- header
        let margin = 30.0 * s;
        f.text(
            margin,
            26.0 * s,
            10.0 * s,
            "2048",
            gui::Color::rgb(1.0, 0.75, 0.3),
        );
        let score = format!("SCORE {}", game.score);
        f.text(
            f.width - margin - f.text_width(4.0 * s, &score),
            26.0 * s,
            4.0 * s,
            &score,
            gui::Color::WHITE,
        );
        let best = format!("BEST {}", game.best);
        f.text(
            f.width - margin - f.text_width(2.5 * s, &best),
            58.0 * s,
            2.5 * s,
            &best,
            gui::Color::GRAY,
        );

        // ------------------------------------------------------------- board
        let board = 440.0 * s;
        let cell = board / N as f32;
        let gap = 8.0 * s;
        let bx = (f.width - board) * 0.5;
        let by = 96.0 * s;
        f.rect(
            bx - gap,
            by - gap,
            board + 2.0 * gap,
            board + 2.0 * gap,
            gui::Color::rgb(0.15, 0.15, 0.19),
        );
        for y in 0..N {
            for x in 0..N {
                let v = game.grid[y * N + x];
                let px = bx + x as f32 * cell + gap * 0.5;
                let py = by + y as f32 * cell + gap * 0.5;
                let inner = cell - gap;
                f.rect(px, py, inner, inner, tile_color(v));
                if v > 0 {
                    let label = format!("{v}");
                    let size = if v < 100 {
                        6.0 * s
                    } else if v < 1000 {
                        5.0 * s
                    } else {
                        4.0 * s
                    };
                    f.text_center(
                        px + inner * 0.5,
                        py + inner * 0.5 - 3.5 * size,
                        size,
                        &label,
                        gui::Color::WHITE,
                    );
                }
            }
        }

        // ------------------------------------------------------------- overlays
        if game.banner {
            let bw = board;
            let bh = 110.0 * s;
            let byy = by + board * 0.5 - bh * 0.5;
            f.rect(bx, byy, bw, bh, gui::Color::rgb(0.07, 0.07, 0.09));
            f.frame_rect(bx, byy, bw, bh, 2.0 * s, gui::Color::rgb(1.0, 0.75, 0.3));
            f.text_center(
                bx + bw * 0.5,
                byy + 22.0 * s,
                5.0 * s,
                "YOU MADE 2048",
                gui::Color::rgb(1.0, 0.75, 0.3),
            );
            f.text_center(
                bx + bw * 0.5,
                byy + 68.0 * s,
                2.5 * s,
                "SPACE TO KEEP GOING",
                gui::Color::WHITE,
            );
        }
        if game.over {
            let bw = board;
            let bh = 110.0 * s;
            let byy = by + board * 0.5 - bh * 0.5;
            f.rect(bx, byy, bw, bh, gui::Color::rgb(0.07, 0.07, 0.09));
            f.frame_rect(bx, byy, bw, bh, 2.0 * s, gui::Color::RED);
            f.text_center(
                bx + bw * 0.5,
                byy + 22.0 * s,
                5.0 * s,
                "NO MOVES LEFT",
                gui::Color::RED,
            );
            f.text_center(
                bx + bw * 0.5,
                byy + 68.0 * s,
                2.5 * s,
                "PRESS R TO RESTART",
                gui::Color::WHITE,
            );
        }

        f.text(
            margin,
            f.height - 34.0 * s,
            2.0 * s,
            "ARROWS MOVE+MERGE - SPACE OK - R RESTART",
            gui::Color::GRAY,
        );
    });
}
