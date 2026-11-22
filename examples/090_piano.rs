//! 090 — Piano: two octaves of playable keys (14 white + 10 black) along
//! the bottom of the window. Keyboard map: A S D F G H J K = C4..C5 white
//! keys (L adds D5), W E T Y U = C#4 D#4 F#4 G#4 A#4; the mouse plays too.
//! Held keys light up and their note names (C4, F#4, ...) stack above the
//! keyboard. Purely visual — no audio.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 920.0;
const H: f64 = 480.0;

const WHITE_NAMES: [&str; 14] = [
    "C4", "D4", "E4", "F4", "G4", "A4", "B4", "C5", "D5", "E5", "F5", "G5", "A5", "B5",
];
const BLACK_NAMES: [&str; 10] = [
    "C#4", "D#4", "F#4", "G#4", "A#4", "C#5", "D#5", "F#5", "G#5", "A#5",
];
/// White keys that carry a black key on their right edge.
const BLACK_AFTER: [usize; 10] = [0, 1, 3, 4, 5, 7, 8, 10, 11, 12];
/// Keyboard letters mapped to the first nine white keys.
const WHITE_KEYS: [u16; 9] = [
    gui::key::A,
    gui::key::S,
    gui::key::D,
    gui::key::F,
    gui::key::G,
    gui::key::H,
    gui::key::J,
    gui::key::K,
    gui::key::L,
];
/// Keyboard letters mapped to the first five black keys.
const BLACK_KEYS: [u16; 5] = [
    gui::key::W,
    gui::key::E,
    gui::key::T,
    gui::key::Y,
    gui::key::U,
];

fn main() {
    let app = gui::App::new("090 - Piano", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.10, 0.09, 0.12));
        let s = f.scale;

        // ------------------------------------------------------------- layout
        let margin = 30.0 * s;
        let kb_x = margin;
        let kb_w = f.width - 2.0 * margin;
        let white_w = kb_w / 14.0;
        let white_h = 190.0 * s;
        let kb_y = f.height - white_h - 58.0 * s;
        let black_w = white_w * 0.62;
        let black_h = white_h * 0.58;

        // ------------------------------------------------------------- hit test
        let (mx, my) = f.mouse();
        let mouse_black: Option<usize> =
            if f.input.mouse_down() && my >= kb_y && my <= kb_y + black_h {
                BLACK_AFTER.iter().position(|&w| {
                    let bx = kb_x + (w + 1) as f32 * white_w - black_w * 0.5;
                    mx >= bx && mx <= bx + black_w
                })
            } else {
                None
            };
        let mouse_white: Option<usize> = if f.input.mouse_down()
            && mouse_black.is_none()
            && my >= kb_y
            && my <= kb_y + white_h
            && mx >= kb_x
            && mx <= kb_x + kb_w
        {
            Some((((mx - kb_x) / white_w) as usize).min(13))
        } else {
            None
        };

        // ------------------------------------------------------------- active set
        let mut white_active = [false; 14];
        let mut black_active = [false; 10];
        for (i, &k) in WHITE_KEYS.iter().enumerate() {
            white_active[i] = f.input.key(k);
        }
        for (i, &k) in BLACK_KEYS.iter().enumerate() {
            black_active[i] = f.input.key(k);
        }
        if let Some(i) = mouse_white {
            white_active[i] = true;
        }
        if let Some(i) = mouse_black {
            black_active[i] = true;
        }

        // ------------------------------------------------------------- white keys
        for (i, name) in WHITE_NAMES.iter().enumerate() {
            let x = kb_x + i as f32 * white_w;
            let active = white_active[i];
            let hover = f.input.mouse_down()
                && mouse_white == Some(i)
                && my >= kb_y
                && my <= kb_y + white_h;
            let fill = if active || hover {
                gui::Color::rgb(1.0, 0.78, 0.42)
            } else {
                gui::Color::rgb(0.92, 0.92, 0.90)
            };
            f.rect(x, kb_y, white_w - 2.0 * s, white_h, fill);
            f.frame_rect(
                x,
                kb_y,
                white_w - 2.0 * s,
                white_h,
                1.5 * s,
                gui::Color::rgb(0.35, 0.35, 0.40),
            );
            // Octave divider accents + C labels.
            if i == 0 || i == 7 {
                f.text_center(
                    x + white_w * 0.5 - s,
                    kb_y + white_h - 16.0 * s,
                    2.5 * s,
                    name,
                    gui::Color::rgb(0.3, 0.3, 0.36),
                );
            }
            if active {
                let label_y = kb_y + 12.0 * s;
                f.text_center(
                    x + white_w * 0.5 - s,
                    label_y,
                    2.5 * s,
                    name,
                    gui::Color::rgb(0.45, 0.25, 0.05),
                );
            }
        }

        // ------------------------------------------------------------- black keys
        for (j, &w) in BLACK_AFTER.iter().enumerate() {
            let bx = kb_x + (w + 1) as f32 * white_w - black_w * 0.5;
            let active = black_active[j];
            let hover = f.input.mouse_down() && mouse_black == Some(j);
            let fill = if active || hover {
                gui::Color::rgb(0.85, 0.45, 0.15)
            } else {
                gui::Color::rgb(0.13, 0.12, 0.15)
            };
            f.rect(bx, kb_y, black_w, black_h, fill);
            f.frame_rect(
                bx,
                kb_y,
                black_w,
                black_h,
                1.5 * s,
                gui::Color::rgb(0.30, 0.30, 0.34),
            );
            if active {
                f.text_center(
                    bx + black_w * 0.5,
                    kb_y + black_h - 16.0 * s,
                    2.0 * s,
                    BLACK_NAMES[j],
                    gui::Color::WHITE,
                );
            }
        }

        // ------------------------------------------------------------- note display
        let mut names: Vec<&str> = Vec::new();
        for (i, name) in WHITE_NAMES.iter().enumerate() {
            if white_active[i] {
                names.push(name);
            }
        }
        for (i, name) in BLACK_NAMES.iter().enumerate() {
            if black_active[i] {
                names.push(name);
            }
        }
        let top_y = kb_y - 46.0 * s;
        if names.is_empty() {
            f.text_center(
                f.width * 0.5,
                top_y,
                2.5 * s,
                "PLAY: A S D F G H J K L OR W E T Y U - OR CLICK THE KEYS",
                gui::Color::GRAY,
            );
        } else {
            // Keep the readout inside the window even when the player mashes
            // every key at once.
            if names.len() > 6 {
                names.truncate(6);
                names.push("+");
            }
            let joined = names.join("  ");
            let size = 5.0 * s;
            let tw = f.text_width(size, &joined);
            let color = gui::Color::hsv((f.time * 0.15) % 1.0, 0.6, 1.0);
            f.text_center(f.width * 0.5, top_y, size, &joined, color);
            // Small underlines beneath each sounded note name.
            let start = f.width * 0.5 - tw * 0.5;
            let mut cx = start;
            for name in &names {
                let w = f.text_width(size, name);
                f.line(
                    cx,
                    top_y + 9.0 * size + 4.0 * s,
                    cx + w,
                    top_y + 9.0 * size + 4.0 * s,
                    2.0 * s,
                    color,
                );
                cx += f.text_width(size, &format!("{name}  "));
            }
        }

        // ------------------------------------------------------------- header
        f.text(margin, 24.0 * s, 5.0 * s, "PIANO", gui::Color::WHITE);
        let tag = "2 OCTAVES - VISUAL ONLY, NO AUDIO";
        f.text(
            f.width - margin - f.text_width(2.5 * s, tag),
            32.0 * s,
            2.5 * s,
            tag,
            gui::Color::GRAY,
        );

        f.text(
            margin,
            f.height - 34.0 * s,
            2.0 * s,
            "A-K WHITES (L = D5) - W E T Y U BLACKS - HOLD TO SUSTAIN - MOUSE WORKS",
            gui::Color::GRAY,
        );
    });
}
