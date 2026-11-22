//! 086 — Minesweeper: the classic. 12x10 cells with 20 mines; the first
//! reveal is always safe (mines are placed after it, excluding that cell).
//! LEFT click reveals (zeros flood-fill), `F` flags the hovered cell,
//! `R` restarts. A timer runs while playing; win when every safe cell is
//! revealed, lose when you hit a mine (all mines revealed + banner).

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 620.0;
const H: f64 = 580.0;
const COLS: usize = 12;
const ROWS: usize = 10;
const MINES: usize = 20;
const CELL: f32 = 36.0;

/// Classic per-number colors for adjacency counts 1..=8.
const NUM_COLORS: [gui::Color; 8] = [
    gui::Color::rgb(0.25, 0.50, 1.00), // 1 blue
    gui::Color::rgb(0.20, 0.80, 0.35), // 2 green
    gui::Color::rgb(1.00, 0.30, 0.30), // 3 red
    gui::Color::rgb(0.55, 0.30, 0.95), // 4 purple
    gui::Color::rgb(0.85, 0.25, 0.40), // 5 maroon
    gui::Color::rgb(0.15, 0.75, 0.75), // 6 teal
    gui::Color::rgb(0.90, 0.90, 0.95), // 7 near-white
    gui::Color::rgb(0.65, 0.65, 0.72), // 8 gray
];

struct Cell {
    mine: bool,
    revealed: bool,
    flagged: bool,
    adjacent: u8,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            mine: false,
            revealed: false,
            flagged: false,
            adjacent: 0,
        }
    }
}

struct Game {
    cells: Vec<Cell>,
    placed: bool,
    over: bool,
    won: bool,
    started: bool,
    time: f32,
    flags: usize,
    rng: gui::Rng,
}

impl Game {
    fn new() -> Game {
        Game {
            cells: (0..COLS * ROWS).map(|_| Cell::new()).collect(),
            placed: false,
            over: false,
            won: false,
            started: false,
            time: 0.0,
            flags: 0,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= COLS as i32 || y >= ROWS as i32 {
            None
        } else {
            Some(y as usize * COLS + x as usize)
        }
    }

    fn place_mines(&mut self, safe: usize) {
        let mut placed = 0;
        while placed < MINES {
            let i = self.rng.index(COLS * ROWS);
            if i == safe || self.cells[i].mine {
                continue;
            }
            self.cells[i].mine = true;
            placed += 1;
        }
        for y in 0..ROWS as i32 {
            for x in 0..COLS as i32 {
                let mut n = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if let Some(j) = self.idx(x + dx, y + dy) {
                            if self.cells[j].mine {
                                n += 1;
                            }
                        }
                    }
                }
                self.cells[y as usize * COLS + x as usize].adjacent = n;
            }
        }
        self.placed = true;
    }

    fn reveal(&mut self, x: usize, y: usize) {
        if self.over || self.won {
            return;
        }
        let i0 = self.idx(x as i32, y as i32).unwrap();
        if self.cells[i0].flagged || self.cells[i0].revealed {
            return;
        }
        if !self.placed {
            self.place_mines(i0);
            self.started = true;
        }
        let mut stack = vec![(x, y)];
        while let Some((cx, cy)) = stack.pop() {
            let Some(i) = self.idx(cx as i32, cy as i32) else {
                continue;
            };
            let (revealed, flagged, adjacent) = {
                let c = &self.cells[i];
                (c.revealed, c.flagged, c.adjacent)
            };
            if revealed || flagged {
                continue;
            }
            self.cells[i].revealed = true;
            if self.cells[i].mine {
                self.over = true;
                continue;
            }
            if adjacent == 0 {
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        stack.push(((cx as i32 + dx) as usize, (cy as i32 + dy) as usize));
                    }
                }
            }
        }
        if self.over {
            for c in &mut self.cells {
                if c.mine {
                    c.revealed = true;
                }
            }
            return;
        }
        let safe_left = self.cells.iter().filter(|c| !c.mine && !c.revealed).count();
        if safe_left == 0 {
            self.won = true;
        }
    }

    fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.over || self.won {
            return;
        }
        if let Some(i) = self.idx(x as i32, y as i32) {
            let (revealed, flagged) = {
                let c = &self.cells[i];
                (c.revealed, c.flagged)
            };
            if !revealed {
                self.cells[i].flagged = !flagged;
                if flagged {
                    self.flags -= 1;
                } else {
                    self.flags += 1;
                }
                if !self.started {
                    self.started = true;
                }
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    let app = gui::App::new("086 - Minesweeper", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.09, 0.09, 0.12));
        let s = f.scale;
        let cell = CELL * s;

        // Board layout.
        let bw = COLS as f32 * cell;
        let bh = ROWS as f32 * cell;
        let bx = (f.width - bw) * 0.5;
        let by = 96.0 * s;

        // Timer: runs while a game is in progress.
        if game.started && !game.over && !game.won {
            game.time += f.dt;
        }

        // ------------------------------------------------------------- input
        let (mx, my) = f.mouse();
        let hx = ((mx - bx) / cell).floor();
        let hy = ((my - by) / cell).floor();
        let hovered = hx >= 0.0 && hy >= 0.0 && hx < COLS as f32 && hy < ROWS as f32;
        if hovered && !game.over && !game.won {
            let (ux, uy) = (hx as usize, hy as usize);
            if f.input.mouse_pressed() {
                game.reveal(ux, uy);
            }
            if f.input.key_pressed(gui::key::F) {
                game.toggle_flag(ux, uy);
            }
        }
        if f.input.key_pressed(gui::key::R) {
            game = Game::new();
        }

        // ------------------------------------------------------------- board
        for y in 0..ROWS {
            for x in 0..COLS {
                let c = &game.cells[y * COLS + x];
                let px = bx + x as f32 * cell;
                let py = by + y as f32 * cell;
                let hovered_here = hovered && x == hx as usize && y == hy as usize;
                if c.revealed {
                    let bg = if c.mine {
                        gui::Color::rgb(0.55, 0.15, 0.15)
                    } else {
                        gui::Color::rgb(0.17, 0.18, 0.22)
                    };
                    f.rect(px, py, cell - 1.0, cell - 1.0, bg);
                    if c.mine {
                        let cx = px + cell * 0.5;
                        let cy = py + cell * 0.5;
                        let r = cell * 0.22;
                        f.circle_segments(cx, cy, r, 16, gui::Color::BLACK);
                        for k in 0..8 {
                            let a = k as f32 / 8.0 * std::f32::consts::TAU;
                            f.line(
                                cx - a.cos() * r,
                                cy - a.sin() * r,
                                cx + a.cos() * r * 1.7,
                                cy + a.sin() * r * 1.7,
                                2.0 * s,
                                gui::Color::BLACK,
                            );
                        }
                    } else if c.adjacent > 0 {
                        let label = format!("{}", c.adjacent);
                        f.text_center(
                            px + (cell - 1.0) * 0.5,
                            py + (cell - 1.0) * 0.5 - 3.5 * 4.0 * s,
                            4.0 * s,
                            &label,
                            NUM_COLORS[(c.adjacent - 1) as usize],
                        );
                    }
                } else {
                    let base = if hovered_here {
                        gui::Color::rgb(0.50, 0.54, 0.64)
                    } else {
                        gui::Color::rgb(0.40, 0.43, 0.52)
                    };
                    f.rect(px, py, cell - 1.0, cell - 1.0, base);
                    // Bevel for the raised look.
                    f.rect(
                        px,
                        py,
                        cell - 1.0,
                        3.0 * s,
                        gui::Color::rgb(0.55, 0.58, 0.68),
                    );
                    f.rect(
                        px,
                        py,
                        3.0 * s,
                        cell - 1.0,
                        gui::Color::rgb(0.52, 0.55, 0.65),
                    );
                    if c.flagged {
                        let fx = px + cell * 0.5;
                        let fy = py + cell * 0.30;
                        f.line(
                            fx,
                            fy,
                            fx,
                            py + cell * 0.72,
                            2.0 * s,
                            gui::Color::rgb(0.2, 0.2, 0.25),
                        );
                        f.tri(
                            (fx, fy),
                            (fx + cell * 0.30, fy + cell * 0.11),
                            (fx, fy + cell * 0.24),
                            gui::Color::RED,
                        );
                    }
                }
            }
        }
        f.frame_rect(
            bx - 2.0 * s,
            by - 2.0 * s,
            bw + 4.0 * s,
            bh + 4.0 * s,
            2.0 * s,
            gui::Color::rgb(0.30, 0.32, 0.42),
        );

        // ------------------------------------------------------------- header
        let flags_left = MINES.saturating_sub(game.flags);
        let mines_left = format!("MINES {}", flags_left);
        f.text(bx, 24.0 * s, 4.0 * s, "MINESWEEPER", gui::Color::WHITE);
        f.text(bx, 62.0 * s, 3.0 * s, &mines_left, gui::Color::RED);
        let clock = format!(
            "{:02}:{:02}",
            (game.time / 60.0) as u32,
            game.time as u32 % 60
        );
        f.text(
            bx + 160.0 * s,
            62.0 * s,
            3.0 * s,
            &clock,
            gui::Color::YELLOW,
        );
        let status = if game.won {
            "CLEARED"
        } else if game.over {
            "DETONATED"
        } else if game.started {
            "SWEEPING"
        } else {
            "READY"
        };
        let status_color = if game.won {
            gui::Color::GREEN
        } else if game.over {
            gui::Color::RED
        } else {
            gui::Color::CYAN
        };
        f.text(bx + 260.0 * s, 62.0 * s, 3.0 * s, status, status_color);

        // ------------------------------------------------------------- banner
        if game.won || game.over {
            let banner = if game.won { "ALL CLEAR!" } else { "GAME OVER" };
            let color = if game.won {
                gui::Color::GREEN
            } else {
                gui::Color::RED
            };
            let bh2 = 74.0 * s;
            let byy = by + bh * 0.5 - bh2 * 0.5;
            f.rect(bx, byy, bw, bh2, gui::Color::rgb(0.05, 0.05, 0.07));
            f.frame_rect(bx, byy, bw, bh2, 2.0 * s, color);
            f.text_center(bx + bw * 0.5, byy + 14.0 * s, 5.0 * s, banner, color);
            f.text_center(
                bx + bw * 0.5,
                byy + 46.0 * s,
                2.5 * s,
                "PRESS R TO PLAY AGAIN",
                gui::Color::WHITE,
            );
        }

        f.text(
            bx,
            f.height - 50.0 * s,
            2.0 * s,
            "FIRST CLICK ALWAYS SAFE - 20 MINES",
            gui::Color::GRAY,
        );
        f.text(
            bx,
            f.height - 26.0 * s,
            2.0 * s,
            "CLICK REVEAL - F FLAG - R RESTART",
            gui::Color::GRAY,
        );
    });
}
