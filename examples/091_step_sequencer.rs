//! 091 — Step Sequencer: an 8-track x 16-step drum grid. Click cells to
//! toggle steps, SPACE or the PLAY button starts the transport, and the
//! playhead sweeps the 16 columns at 8 steps per second, lighting up any
//! active cells under it. `C` (or CLEAR) wipes the pattern. Tracks are
//! labeled BD / SD / HH / OH / LT / MT / RM / CP.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 880.0;
const H: f64 = 600.0;
const TRACKS: usize = 8;
const STEPS: usize = 16;
/// Seconds per step: 8 steps per second.
const STEP_TIME: f32 = 0.125;

const ROW_LABELS: [&str; 8] = ["BD", "SD", "HH", "OH", "LT", "MT", "RM", "CP"];
const ROW_COLORS: [gui::Color; 8] = [
    gui::Color::rgb(0.30, 0.55, 1.00),
    gui::Color::rgb(1.00, 0.60, 0.20),
    gui::Color::rgb(1.00, 0.90, 0.25),
    gui::Color::rgb(0.75, 0.90, 0.25),
    gui::Color::rgb(0.30, 0.85, 0.40),
    gui::Color::rgb(0.25, 0.90, 1.00),
    gui::Color::rgb(1.00, 0.35, 0.90),
    gui::Color::rgb(1.00, 0.30, 0.30),
];

struct Seq {
    grid: [[bool; STEPS]; TRACKS],
    playing: bool,
    acc: f32,
    active: usize,
}

impl Seq {
    fn step(&mut self, dt: f32) {
        if !self.playing {
            return;
        }
        self.acc += dt;
        if self.acc >= STEP_TIME {
            self.acc -= STEP_TIME;
            self.active = (self.active + 1) % STEPS;
        }
    }
}

fn main() {
    let mut seq = Seq {
        grid: [[false; STEPS]; TRACKS],
        playing: false,
        acc: 0.0,
        active: 0,
    };
    // Seed a starter groove so the grid is not empty on launch.
    for c in [0, 4, 8, 10] {
        seq.grid[0][c] = true;
    }
    for c in [4, 12] {
        seq.grid[1][c] = true;
    }
    for c in 0..STEPS {
        if c % 2 == 0 {
            seq.grid[2][c] = true;
        }
        if c % 4 == 2 {
            seq.grid[3][c] = true;
        }
    }

    let app = gui::App::new("091 - Step Sequencer", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.09, 0.09, 0.12));
        let s = f.scale;

        // ------------------------------------------------------------- layout
        let margin = 30.0 * s;
        let label_w = 58.0 * s;
        let cell = 30.0 * s;
        let pitch = 34.0 * s;
        let beat_gap = 8.0 * s;
        let gx = margin + label_w;
        let gy = 118.0 * s;
        let grid_w = STEPS as f32 * pitch + 3.0 * beat_gap;
        let grid_h = TRACKS as f32 * pitch;

        let col_x = |c: usize| gx + c as f32 * pitch + (c / 4) as f32 * beat_gap;
        let row_y = |r: usize| gy + r as f32 * pitch;

        // ------------------------------------------------------------- transport
        seq.step(f.dt);
        if f.input.key_pressed(gui::key::SPACE) {
            seq.playing = !seq.playing;
        }
        if f.input.key_pressed(gui::key::C) {
            for row in &mut seq.grid {
                row.fill(false);
            }
        }

        // PLAY / CLEAR buttons.
        let play_rect = (f.width - margin - 220.0 * s, 24.0 * s, 100.0 * s, 36.0 * s);
        let clear_rect = (f.width - margin - 110.0 * s, 24.0 * s, 100.0 * s, 36.0 * s);
        let (mx, my) = f.mouse();
        let in_play = mx >= play_rect.0
            && mx <= play_rect.0 + play_rect.2
            && my >= play_rect.1
            && my <= play_rect.1 + play_rect.3;
        let in_clear = mx >= clear_rect.0
            && mx <= clear_rect.0 + clear_rect.2
            && my >= clear_rect.1
            && my <= clear_rect.1 + clear_rect.3;
        let play_base = if seq.playing {
            gui::Color::rgb(0.95, 0.55, 0.15)
        } else {
            gui::Color::rgb(0.20, 0.62, 0.30)
        };
        f.rect(
            play_rect.0,
            play_rect.1,
            play_rect.2,
            play_rect.3,
            if in_play {
                gui::Color::lerp(play_base, gui::Color::WHITE, 0.2)
            } else {
                play_base
            },
        );
        f.text_center(
            play_rect.0 + play_rect.2 * 0.5,
            play_rect.1 + play_rect.3 * 0.5 - 3.5 * 3.0 * s,
            3.0 * s,
            if seq.playing { "PAUSE" } else { "PLAY" },
            gui::Color::WHITE,
        );
        f.rect(
            clear_rect.0,
            clear_rect.1,
            clear_rect.2,
            clear_rect.3,
            if in_clear {
                gui::Color::lerp(gui::Color::rgb(0.55, 0.22, 0.24), gui::Color::WHITE, 0.2)
            } else {
                gui::Color::rgb(0.55, 0.22, 0.24)
            },
        );
        f.text_center(
            clear_rect.0 + clear_rect.2 * 0.5,
            clear_rect.1 + clear_rect.3 * 0.5 - 3.5 * 3.0 * s,
            3.0 * s,
            "CLEAR",
            gui::Color::WHITE,
        );
        if f.input.mouse_pressed() && in_play {
            seq.playing = !seq.playing;
        }
        if f.input.mouse_pressed() && in_clear {
            for row in &mut seq.grid {
                row.fill(false);
            }
        }

        // ------------------------------------------------------------- header
        f.text(
            margin,
            26.0 * s,
            5.0 * s,
            "STEP SEQUENCER",
            gui::Color::WHITE,
        );
        let step_label = if seq.playing {
            format!("STEP {:02}/{}", seq.active + 1, STEPS)
        } else {
            format!("STEP --/{}", STEPS)
        };
        f.text(
            margin + 250.0 * s,
            34.0 * s,
            3.0 * s,
            &step_label,
            if seq.playing {
                gui::Color::GREEN
            } else {
                gui::Color::GRAY
            },
        );

        // ------------------------------------------------------------- grid
        // Column beat numbers.
        for c in 0..STEPS {
            let label = format!("{}", c + 1);
            let color = if seq.playing && c == seq.active {
                gui::Color::YELLOW
            } else {
                gui::Color::rgb(0.42, 0.44, 0.52)
            };
            f.text_center(col_x(c) + cell * 0.5, gy - 22.0 * s, 2.0 * s, &label, color);
        }
        // Row labels.
        for (r, label) in ROW_LABELS.iter().enumerate() {
            let y = row_y(r);
            f.text(
                margin,
                y + cell * 0.5 - 3.5 * 3.0 * s,
                3.0 * s,
                label,
                ROW_COLORS[r],
            );
        }
        // Cells.
        for r in 0..TRACKS {
            for c in 0..STEPS {
                let x = col_x(c);
                let y = row_y(r);
                let hover = mx >= x && mx <= x + cell && my >= y && my <= y + cell;
                let on = seq.grid[r][c];
                let playhead = seq.playing && c == seq.active;
                let base = if on {
                    ROW_COLORS[r]
                } else {
                    gui::Color::rgb(0.16, 0.16, 0.21)
                };
                let fill = if hover && !on {
                    gui::Color::rgb(0.26, 0.26, 0.33)
                } else if on && playhead {
                    gui::Color::lerp(ROW_COLORS[r], gui::Color::WHITE, 0.45)
                } else {
                    base
                };
                f.rect(x, y, cell, cell, fill);
                if on {
                    f.rect(
                        x,
                        y,
                        cell,
                        4.0 * s,
                        gui::Color::lerp(fill, gui::Color::BLACK, 0.35),
                    );
                }
                if playhead {
                    f.frame_rect(
                        x - 1.0 * s,
                        y - 1.0 * s,
                        cell + 2.0 * s,
                        cell + 2.0 * s,
                        1.5 * s,
                        gui::Color::YELLOW,
                    );
                }
                if f.input.mouse_pressed() && hover {
                    seq.grid[r][c] = !on;
                }
            }
        }
        // Playhead column wash: a warm tint strip behind the active column
        // markers (drawn per-cell above via the brightened fills + frames).
        if seq.playing {
            for r in 0..TRACKS {
                let x = col_x(seq.active);
                let y = row_y(r);
                if !seq.grid[r][seq.active] {
                    f.rect(x, y, cell, cell, gui::Color::rgb(0.30, 0.28, 0.14));
                }
            }
        }
        f.frame_rect(
            gx - 4.0 * s,
            gy - 4.0 * s,
            grid_w + 8.0 * s,
            grid_h + 8.0 * s,
            2.0 * s,
            gui::Color::rgb(0.28, 0.30, 0.40),
        );

        // ------------------------------------------------------------- footer
        let active_count = seq
            .grid
            .iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .sum::<usize>();
        let status = format!("{} ACTIVE STEPS", active_count);
        f.text(
            gx,
            gy + grid_h + 16.0 * s,
            2.5 * s,
            &status,
            gui::Color::GRAY,
        );
        f.text(
            margin,
            f.height - 32.0 * s,
            2.0 * s,
            "CLICK TOGGLES STEPS - SPACE PLAY/PAUSE - C CLEAR - 8 STEPS/SEC",
            gui::Color::GRAY,
        );
    });
}
