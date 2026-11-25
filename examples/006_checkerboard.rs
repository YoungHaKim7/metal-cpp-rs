//! 006 — Checkerboard: an 8x8 board sized from the window (70% of the
//! canvas height), centered. Click a cell to toggle it between the two
//! palette entries, SPACE randomizes roughly 30% of the cells, and a faint
//! grid keeps the board readable.

#[path = "gui/mod.rs"]
mod gui;

const N: usize = 8;

const PALETTE: [gui::Color; 2] = [
    gui::Color::rgb(0.13, 0.15, 0.21),
    gui::Color::rgb(0.55, 0.60, 0.68),
];

struct Board {
    cells: [[u8; N]; N],
    rng: gui::Rng,
}

impl Board {
    fn new() -> Board {
        let mut board = Board {
            cells: [[0; N]; N],
            rng: gui::Rng::from_entropy(),
        };
        for y in 0..N {
            for x in 0..N {
                board.cells[y][x] = ((x + y) % 2) as u8;
            }
        }
        board
    }

    fn toggle(&mut self, x: usize, y: usize) {
        self.cells[y][x] ^= 1;
    }

    fn randomize(&mut self) {
        for y in 0..N {
            for x in 0..N {
                if self.rng.f32() < 0.3 {
                    self.cells[y][x] ^= 1;
                }
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    let app = gui::App::new("006 - Checkerboard", 640.0, 400.0);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.07, 0.07, 0.11));

        // Cell edge in whole units, so the board follows the window size.
        let cell = ((f.height * 0.70 / N as f32).floor()).max(8.0);
        let bw = N as f32 * cell;
        let bx = (f.width - bw) * 0.5;
        let by = (f.height - bw) * 0.5;

        if f.input.key_pressed(gui::key::SPACE) {
            board.randomize();
        }

        // Cell under the cursor drives both the click and the hover tint.
        let (mx, my) = f.mouse();
        let hit = (
            ((mx - bx) / cell).floor() as i64,
            ((my - by) / cell).floor() as i64,
        );
        let inside = hit.0 >= 0 && hit.0 < N as i64 && hit.1 >= 0 && hit.1 < N as i64;

        if f.input.mouse_pressed() && inside {
            board.toggle(hit.0 as usize, hit.1 as usize);
        }

        for y in 0..N {
            for x in 0..N {
                let mut col = PALETTE[board.cells[y][x] as usize];
                if inside && hit.0 == x as i64 && hit.1 == y as i64 {
                    col = gui::Color::lerp(col, gui::Color::WHITE, 0.12);
                }
                f.rect(bx + x as f32 * cell, by + y as f32 * cell, cell, cell, col);
            }
        }

        // Faint grid over the internal cell boundaries.
        let grid = gui::Color::rgb(0.30, 0.33, 0.42);
        for i in 1..N {
            let o = i as f32 * cell;
            f.line(bx + o, by, bx + o, by + bw, 1.0, grid);
            f.line(bx, by + o, bx + bw, by + o, 1.0, grid);
        }
        f.frame_rect(bx, by, bw, bw, 2.0, gui::Color::rgb(0.42, 0.45, 0.55));

        f.text_center(f.width * 0.5, 14.0, 4.0, "CHECKERBOARD", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "CLICK A CELL TO TOGGLE - SPACE RANDOMIZES 30% - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
