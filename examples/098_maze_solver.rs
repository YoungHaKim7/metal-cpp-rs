//! 098 — Maze Solver: a 31x21 odd-lattice maze carved by an iterative
//! recursive-backtracker (wall cells dark, floor cells light). `R`
//! regenerates. After a short pause the BFS flood animates out of the
//! top-left entry at ~30 cells/s (fresh frontier bright yellow, visited
//! cells olive), then the shortest path traces through in green. The
//! status line shows the GRID / BFS / PATH phase.

use std::collections::VecDeque;

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 800.0;
const H: f64 = 620.0;
const COLS: usize = 31;
const ROWS: usize = 21;
const CELLS: usize = COLS * ROWS;

const STATE_GRID: u8 = 0;
const STATE_BFS: u8 = 1;
const STATE_PATH: u8 = 2;

/// BFS flood speed in cells per second.
const BFS_RATE: f32 = 30.0;
/// Path trace speed in cells per second.
const PATH_RATE: f32 = 24.0;
/// How many recently-visited cells count as the bright frontier.
const FRONTIER_WINDOW: i32 = 34;

struct Maze {
    open: [bool; CELLS],
    visit_pos: [i32; CELLS],
    order: Vec<usize>,
    path: Vec<usize>,
    state: u8,
    wait: f32,
    bfs_n: f32,
    path_n: f32,
    rng: gui::Rng,
}

impl Maze {
    fn new() -> Maze {
        let mut m = Maze {
            open: [false; CELLS],
            visit_pos: [-1; CELLS],
            order: Vec::new(),
            path: Vec::new(),
            state: STATE_GRID,
            wait: 0.0,
            bfs_n: 0.0,
            path_n: 0.0,
            rng: gui::Rng::from_entropy(),
        };
        m.generate();
        m
    }

    /// Iterative recursive-backtracker over the odd lattice.
    fn generate(&mut self) {
        self.open = [false; CELLS];
        let mut stack: Vec<(usize, usize)> = vec![(1, 1)];
        self.open[COLS + 1] = true;
        while let Some(&(x, y)) = stack.last() {
            let dirs = [(2i32, 0), (-2, 0), (0, 2), (0, -2)];
            let off = self.rng.index(4) as usize;
            let mut carved = false;
            for k in 0..4 {
                let (dx, dy) = dirs[(off + k) % 4];
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx <= 0 || ny <= 0 || nx >= COLS as i32 || ny >= ROWS as i32 {
                    continue;
                }
                let j = ny as usize * COLS + nx as usize;
                if self.open[j] {
                    continue;
                }
                let wx = (x as i32 + dx / 2) as usize;
                let wy = (y as i32 + dy / 2) as usize;
                self.open[wy * COLS + wx] = true;
                self.open[j] = true;
                stack.push((nx as usize, ny as usize));
                carved = true;
                break;
            }
            if !carved {
                stack.pop();
            }
        }
        self.bfs();
        self.state = STATE_GRID;
        self.wait = 0.7;
        self.bfs_n = 0.0;
        self.path_n = 0.0;
    }

    /// Full BFS from the entry; records visit order and the shortest path.
    fn bfs(&mut self) {
        let start = COLS + 1;
        let goal = (ROWS - 2) * COLS + (COLS - 2);
        let mut parent = [-1i32; CELLS];
        let mut seen = [false; CELLS];
        let mut queue = VecDeque::new();
        seen[start] = true;
        self.visit_pos[start] = 0;
        self.order.clear();
        self.order.push(start);
        queue.push_back(start);
        while let Some(cur) = queue.pop_front() {
            let (x, y) = (cur % COLS, cur / COLS);
            for (dx, dy) in [(1i32, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || ny < 0 || nx >= COLS as i32 || ny >= ROWS as i32 {
                    continue;
                }
                let j = ny as usize * COLS + nx as usize;
                if !self.open[j] || seen[j] {
                    continue;
                }
                seen[j] = true;
                parent[j] = cur as i32;
                self.visit_pos[j] = self.order.len() as i32;
                self.order.push(j);
                queue.push_back(j);
            }
        }
        self.path.clear();
        let mut cur = goal;
        loop {
            self.path.push(cur);
            if cur == start {
                break;
            }
            cur = parent[cur] as usize;
        }
        self.path.reverse();
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            STATE_GRID => {
                self.wait -= dt;
                if self.wait <= 0.0 {
                    self.state = STATE_BFS;
                }
            }
            STATE_BFS => {
                self.bfs_n = (self.bfs_n + BFS_RATE * dt).min(self.order.len() as f32);
                if self.bfs_n >= self.order.len() as f32 {
                    self.state = STATE_PATH;
                }
            }
            _ => {
                self.path_n = (self.path_n + PATH_RATE * dt).min(self.path.len() as f32);
            }
        }
    }
}

fn main() {
    let mut maze = Maze::new();
    let app = gui::App::new("098 - Maze Solver", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.08, 0.08, 0.11));
        let s = f.scale;

        // ------------------------------------------------------------- input
        if f.input.key_pressed(gui::key::R) {
            maze = Maze::new();
        }
        maze.update(f.dt);

        // ------------------------------------------------------------- layout
        let cs = 22.0 * s;
        let bw = COLS as f32 * cs;
        let bh = ROWS as f32 * cs;
        let bx = (f.width - bw) * 0.5;
        let by = 66.0 * s;

        // ------------------------------------------------------------- header
        f.text(bx, 18.0 * s, 4.0 * s, "MAZE SOLVER", gui::Color::WHITE);
        let (state_label, state_color) = match maze.state {
            STATE_GRID => ("STATE: GRID", gui::Color::CYAN),
            STATE_BFS => ("STATE: BFS", gui::Color::YELLOW),
            _ => ("STATE: PATH", gui::Color::GREEN),
        };
        f.text(bx + 220.0 * s, 26.0 * s, 3.0 * s, state_label, state_color);
        let progress = match maze.state {
            STATE_GRID => "ENTRY (1,1)".to_string(),
            STATE_BFS => format!("FLOOD {:.0}/{}", maze.bfs_n, maze.order.len()),
            _ => format!("TRACE {:.0}/{}", maze.path_n, maze.path.len()),
        };
        f.text(
            f.width - bx - f.text_width(3.0 * s, &progress),
            26.0 * s,
            3.0 * s,
            &progress,
            gui::Color::GRAY,
        );

        // ------------------------------------------------------------- maze
        f.rect(bx, by, bw, bh, gui::Color::rgb(0.07, 0.08, 0.11));
        let frontier_min = maze.bfs_n as i32 - FRONTIER_WINDOW;
        for y in 0..ROWS {
            for x in 0..COLS {
                let i = y * COLS + x;
                if !maze.open[i] {
                    continue; // wall stays dark
                }
                let px = bx + x as f32 * cs;
                let py = by + y as f32 * cs;
                let mut color = gui::Color::rgb(0.85, 0.85, 0.82);
                let p = maze.visit_pos[i];
                if p >= 0 && (p as f32) < maze.bfs_n {
                    // Visited by the flood; fresh cells are the frontier.
                    color = if p >= frontier_min {
                        gui::Color::YELLOW
                    } else {
                        gui::Color::rgb(0.55, 0.50, 0.25)
                    };
                }
                f.rect(px, py, cs, cs, color);
            }
        }
        // Final path in green, drawn progressively.
        for k in 0..maze.path_n as usize {
            if k >= maze.path.len() {
                break;
            }
            let i = maze.path[k];
            let px = bx + (i % COLS) as f32 * cs;
            let py = by + (i / COLS) as f32 * cs;
            f.rect(px, py, cs, cs, gui::Color::GREEN);
        }
        // Entry + exit markers.
        let entry = (bx + cs * 1.0, by + cs * 1.0);
        let exit = (bx + cs * (COLS - 2) as f32, by + cs * (ROWS - 2) as f32);
        f.rect(entry.0, entry.1, cs, cs, gui::Color::rgb(0.25, 0.50, 1.00));
        f.rect(exit.0, exit.1, cs, cs, gui::Color::rgb(1.00, 0.30, 0.55));
        f.text_center(
            entry.0 + cs * 0.5,
            entry.1 + cs * 0.5 - 3.5 * 2.0 * s,
            2.0 * s,
            "IN",
            gui::Color::WHITE,
        );
        f.text_center(
            exit.0 + cs * 0.5,
            exit.1 + cs * 0.5 - 3.5 * 2.0 * s,
            2.0 * s,
            "OUT",
            gui::Color::WHITE,
        );
        f.frame_rect(
            bx - 2.0 * s,
            by - 2.0 * s,
            bw + 4.0 * s,
            bh + 4.0 * s,
            2.0 * s,
            gui::Color::rgb(0.30, 0.32, 0.42),
        );

        // ------------------------------------------------------------- footer
        let legend_x = bx;
        let ly = by + bh + 14.0 * s;
        for (i, (label, color)) in [
            ("WALL", gui::Color::rgb(0.07, 0.08, 0.11)),
            ("FLOOR", gui::Color::rgb(0.85, 0.85, 0.82)),
            ("VISITED", gui::Color::rgb(0.55, 0.50, 0.25)),
            ("FRONTIER", gui::Color::YELLOW),
            ("PATH", gui::Color::GREEN),
        ]
        .iter()
        .enumerate()
        {
            let lx = legend_x + i as f32 * 110.0 * s;
            f.rect(lx, ly + 2.0 * s, 12.0 * s, 12.0 * s, *color);
            f.text(
                lx + 18.0 * s,
                ly + 3.0 * s,
                2.0 * s,
                label,
                gui::Color::GRAY,
            );
        }
        f.text(
            bx,
            f.height - 30.0 * s,
            2.0 * s,
            "R REGENERATE - BACKTRACKER - BFS ~30 CELLS/S - PATH IN GREEN",
            gui::Color::GRAY,
        );
    });
}
