//! 089 — Stopwatch: big MM:SS.CC digits driven by accumulated frame deltas
//! while running. SPACE starts/stops, `R` resets, `L` records a lap (the
//! last five are kept, newest on top, listed on the right side). A status
//! readout shows RUNNING / STOPPED, plus lap count and total.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 780.0;
const H: f64 = 480.0;
const MAX_LAPS: usize = 5;

struct Watch {
    running: bool,
    elapsed: f32,
    laps: Vec<f32>,
}

impl Watch {
    fn new() -> Watch {
        Watch {
            running: false,
            elapsed: 0.0,
            laps: Vec::new(),
        }
    }

    /// MM:SS.CC for a duration in seconds.
    fn fmt(t: f32) -> String {
        let cs = (t * 100.0) as u64;
        format!("{:02}:{:02}.{:02}", cs / 6000, (cs / 100) % 60, cs % 100)
    }
}

fn main() {
    let mut watch = Watch::new();
    let app = gui::App::new("089 - Stopwatch", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.08, 0.08, 0.11));
        let s = f.scale;

        // ------------------------------------------------------------- input
        if f.input.key_pressed(gui::key::SPACE) {
            watch.running = !watch.running;
        }
        if f.input.key_pressed(gui::key::R) {
            watch = Watch::new();
        }
        if f.input.key_pressed(gui::key::L) && (watch.running || watch.elapsed > 0.0) {
            watch.laps.push(watch.elapsed);
            if watch.laps.len() > MAX_LAPS {
                watch.laps.remove(0);
            }
        }

        // ------------------------------------------------------------- update
        if watch.running {
            watch.elapsed += f.dt;
        }

        // ------------------------------------------------------------- dial
        let dial_x = 40.0 * s;
        let dial_w = f.width - 250.0 * s - 2.0 * dial_x;
        let dial_y = 80.0 * s;
        let dial_h = f.height - dial_y - 70.0 * s;
        f.rect(
            dial_x,
            dial_y,
            dial_w,
            dial_h,
            gui::Color::rgb(0.11, 0.11, 0.15),
        );
        f.frame_rect(
            dial_x,
            dial_y,
            dial_w,
            dial_h,
            2.0 * s,
            gui::Color::rgb(0.28, 0.30, 0.40),
        );

        // Progress bar: one full bar per minute.
        let minute_pos = (watch.elapsed % 60.0) / 60.0;
        let bar_w = dial_w - 60.0 * s;
        f.rect(
            dial_x + 30.0 * s,
            dial_y + dial_h - 34.0 * s,
            bar_w * minute_pos,
            10.0 * s,
            gui::Color::rgb(0.35, 0.55, 1.0),
        );
        f.frame_rect(
            dial_x + 30.0 * s,
            dial_y + dial_h - 34.0 * s,
            bar_w,
            10.0 * s,
            1.5 * s,
            gui::Color::rgb(0.30, 0.32, 0.42),
        );

        // Big digits, blinking colon while running.
        let text = Watch::fmt(watch.elapsed);
        let size = 9.0 * s;
        let tw = f.text_width(size, &text);
        f.text(
            dial_x + (dial_w - tw) * 0.5,
            dial_y + dial_h * 0.38 - 3.5 * size,
            size,
            &text,
            gui::Color::WHITE,
        );

        // Status.
        let (status, color) = if watch.running {
            ("RUNNING", gui::Color::GREEN)
        } else if watch.elapsed > 0.0 {
            ("STOPPED", gui::Color::YELLOW)
        } else {
            ("READY", gui::Color::GRAY)
        };
        let st_size = 4.0 * s;
        f.text_center(
            dial_x + dial_w * 0.5,
            dial_y + 24.0 * s,
            st_size,
            status,
            color,
        );
        if watch.running && (f.time % 1.0) < 0.5 {
            f.circle(
                dial_x + dial_w - 34.0 * s,
                dial_y + 24.0 * s + 3.5 * st_size,
                6.0 * s,
                color,
            );
        }
        let laps_info = format!("LAPS {}", watch.laps.len());
        f.text(
            dial_x + 30.0 * s,
            dial_y + 24.0 * s,
            2.5 * s,
            &laps_info,
            gui::Color::GRAY,
        );

        // ------------------------------------------------------------- laps
        let lap_x = f.width - 230.0 * s;
        f.text(lap_x, 40.0 * s, 4.0 * s, "LAPS", gui::Color::WHITE);
        f.line(
            lap_x,
            76.0 * s,
            lap_x + 200.0 * s,
            76.0 * s,
            2.0 * s,
            gui::Color::rgb(0.30, 0.32, 0.42),
        );
        let count = watch.laps.len();
        for (i, &t) in watch.laps.iter().enumerate().rev() {
            let row = count - 1 - i; // newest on top
            let y = 92.0 * s + row as f32 * 34.0 * s;
            let n = i + 1;
            let shade = if i == count - 1 && watch.running {
                gui::Color::rgb(0.24, 0.22, 0.16)
            } else {
                gui::Color::rgb(0.15, 0.15, 0.19)
            };
            f.rect(lap_x - 10.0 * s, y - 6.0 * s, 220.0 * s, 28.0 * s, shade);
            f.text(
                lap_x,
                y,
                3.0 * s,
                &format!("L{n:02}"),
                gui::Color::rgb(0.5, 0.7, 1.0),
            );
            f.text(
                lap_x + 60.0 * s,
                y,
                3.0 * s,
                &Watch::fmt(t),
                gui::Color::WHITE,
            );
        }
        if watch.laps.is_empty() {
            f.text(
                lap_x,
                96.0 * s,
                2.5 * s,
                "NO LAPS YET - PRESS L",
                gui::Color::GRAY,
            );
        }

        f.text(
            40.0 * s,
            f.height - 34.0 * s,
            2.0 * s,
            "SPACE START/STOP - R RESET - L LAP (LAST 5 KEPT, NEWEST ON TOP)",
            gui::Color::GRAY,
        );
    });
}
