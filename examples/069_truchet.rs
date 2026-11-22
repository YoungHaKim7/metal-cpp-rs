//! 069 — Truchet Tiles: the window tiled with quarter-circle arcs in two
//! orientations, hue drifting across the lattice. Click flips the tile under
//! the cursor; SPACE randomly flips about ten percent of the wall.

use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 640.0;
const TILE: f32 = 40.0;
const ARC_SEGMENTS: usize = 10;
const FLIP_CHANCE: f32 = 0.10;
const MAX_COLS: usize = 80;
const MAX_ROWS: usize = 60;

struct Truchet {
    tiles: HashMap<(usize, usize), u8>,
    rng: gui::Rng,
}

impl Truchet {
    fn flip(&mut self, col: usize, row: usize) {
        let o = self.tiles.get(&(col, row)).copied().unwrap_or(0) ^ 1;
        self.tiles.insert((col, row), o);
    }

    fn flip_scatter(&mut self, cols: usize, rows: usize) {
        for row in 0..rows {
            for col in 0..cols {
                if self.rng.f32() < FLIP_CHANCE {
                    self.flip(col, row);
                }
            }
        }
    }
}

/// An annular sector from angle `a0` to `a1` — the same triangle pair
/// `ring_segments` builds per segment, restricted to an arc.
fn arc_band(
    f: &mut gui::Frame,
    cx: f32,
    cy: f32,
    r_out: f32,
    r_in: f32,
    a0: f32,
    a1: f32,
    color: gui::Color,
) {
    for i in 0..ARC_SEGMENTS {
        let t0 = a0 + (a1 - a0) * (i as f32) / ARC_SEGMENTS as f32;
        let t1 = a0 + (a1 - a0) * ((i + 1) as f32) / ARC_SEGMENTS as f32;
        let (sx, sy) = (t0.cos(), t0.sin());
        let (ex, ey) = (t1.cos(), t1.sin());
        let o0 = (cx + sx * r_out, cy + sy * r_out);
        let o1 = (cx + ex * r_out, cy + ey * r_out);
        let i0 = (cx + sx * r_in, cy + sy * r_in);
        let i1 = (cx + ex * r_in, cy + ey * r_in);
        f.tri(o0, o1, i0, color);
        f.tri(o1, i1, i0, color);
    }
}

fn main() {
    let mut truchet = Truchet {
        tiles: HashMap::new(),
        rng: gui::Rng::from_entropy(),
    };
    let app = gui::App::new("069 - Truchet Tiles", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.05, 0.05, 0.07));

        let cols = ((f.width / TILE).ceil() as usize).min(MAX_COLS);
        let rows = ((f.height / TILE).ceil() as usize).min(MAX_ROWS);

        // Input: click flips one tile, SPACE scatters flips over ~10%.
        let (mx, my) = f.mouse();
        if f.input.mouse_pressed() {
            let col = (mx / TILE) as usize;
            let row = (my / TILE) as usize;
            if col < cols && row < rows {
                truchet.flip(col, row);
            }
        }
        if f.input.key_pressed(gui::key::SPACE) {
            truchet.flip_scatter(cols, rows);
        }

        // Tile wall: two quarter arcs per tile, hue drifting by position.
        let half = TILE * 0.5;
        let width = TILE * 0.24;
        let (r_out, r_in) = (half + width * 0.5, half - width * 0.5);
        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * TILE;
                let y = row as f32 * TILE;
                f.rect(x, y, TILE, TILE, gui::Color::rgb(0.07, 0.07, 0.10));
                let hue = (col as f32 * 0.049 + row as f32 * 0.083).fract();
                let ink = gui::Color::hsv(hue, 0.75, 0.95);
                match truchet.tiles.get(&(col, row)).copied().unwrap_or(0) {
                    0 => {
                        arc_band(f, x, y, r_out, r_in, 0.0, FRAC_PI_2, ink);
                        arc_band(f, x + TILE, y + TILE, r_out, r_in, PI, PI + FRAC_PI_2, ink);
                    }
                    _ => {
                        arc_band(f, x + TILE, y, r_out, r_in, FRAC_PI_2, PI, ink);
                        arc_band(f, x, y + TILE, r_out, r_in, PI + FRAC_PI_2, TAU, ink);
                    }
                }
            }
        }

        f.text(16.0, 10.0, 3.0, "TRUCHET TILES", gui::Color::WHITE);
        f.text(
            f.width - 16.0 - f.text_width(2.0, "QUARTER-CIRCLE ARCS - 2 ORIENTATIONS"),
            16.0,
            2.0,
            "QUARTER-CIRCLE ARCS - 2 ORIENTATIONS",
            gui::Color::GRAY,
        );
        f.text(
            16.0,
            f.height - 34.0,
            2.0,
            "CLICK FLIPS A TILE - SPACE RANDOMLY FLIPS ~10% - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
