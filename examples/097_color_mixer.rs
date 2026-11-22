//! 097 — Color Mixer: three horizontal sliders (R/G/B) with draggable
//! handles — click or drag anywhere on a track. A large preview rect shows
//! the mixed color with its #RRGGBB hex, each slider carries a small
//! gradient strip beneath it, and RANDOM (or `R`) rolls a new color.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 600.0;
const H: f64 = 560.0;

struct Mixer {
    rgb: [f32; 3],
    drag: Option<usize>,
    rng: gui::Rng,
}

impl Mixer {
    fn new() -> Mixer {
        Mixer {
            rgb: [0.85, 0.45, 0.25],
            drag: None,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn color(&self) -> gui::Color {
        gui::Color::rgb(self.rgb[0], self.rgb[1], self.rgb[2])
    }

    fn hex(&self) -> String {
        let [r, g, b] = self.rgb;
        format!(
            "#{:02X}{:02X}{:02X}",
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8
        )
    }

    fn random(&mut self) {
        for ch in &mut self.rgb {
            *ch = 0.1 + self.rng.f32() * 0.9;
        }
    }
}

fn main() {
    let mut mixer = Mixer::new();
    let app = gui::App::new("097 - Color Mixer", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.09, 0.09, 0.12));
        let s = f.scale;

        // ------------------------------------------------------------- layout
        let margin = 30.0 * s;
        let preview = (margin, 70.0 * s, f.width - 2.0 * margin, 170.0 * s);
        let track_x0 = margin + 44.0 * s;
        let track_x1 = f.width - margin - 70.0 * s;
        let track_w = track_x1 - track_x0;
        let rows_y = preview.1 + preview.3 + 34.0 * s;
        let row_pitch = 62.0 * s;
        let track_h = 12.0 * s;

        // ------------------------------------------------------------- input
        if f.input.key_pressed(gui::key::R) {
            mixer.random();
        }
        let (mx, my) = f.mouse();
        let mut random_rect = (margin, 0.0, 120.0 * s, 40.0 * s);
        random_rect.1 = rows_y + 3.0 * row_pitch;
        let in_random = mx >= random_rect.0
            && mx <= random_rect.0 + random_rect.2
            && my >= random_rect.1
            && my <= random_rect.1 + random_rect.3;

        for i in 0..3 {
            let y = rows_y + i as f32 * row_pitch;
            let on_track = my >= y - 14.0 * s
                && my <= y + track_h + 26.0 * s
                && mx >= track_x0
                && mx <= track_x1;
            if f.input.mouse_pressed() && on_track {
                mixer.drag = Some(i);
            }
        }
        if let Some(i) = mixer.drag {
            let t = ((mx - track_x0) / track_w).clamp(0.0, 1.0);
            mixer.rgb[i] = t;
            if f.input.mouse_released() {
                mixer.drag = None;
            }
        }
        if f.input.mouse_pressed() && in_random {
            mixer.random();
        }

        // ------------------------------------------------------------- preview
        f.rect(preview.0, preview.1, preview.2, preview.3, mixer.color());
        f.frame_rect(
            preview.0,
            preview.1,
            preview.2,
            preview.3,
            2.0 * s,
            gui::Color::rgb(0.30, 0.32, 0.42),
        );
        let luma = 0.299 * mixer.rgb[0] + 0.587 * mixer.rgb[1] + 0.114 * mixer.rgb[2];
        let text_color = if luma > 0.55 {
            gui::Color::BLACK
        } else {
            gui::Color::WHITE
        };
        let hex = mixer.hex();
        let hex_size = 8.0 * s;
        f.text_center(
            preview.0 + preview.2 * 0.5,
            preview.1 + preview.3 * 0.5 - 3.5 * hex_size,
            hex_size,
            &hex,
            text_color,
        );
        let sub = "MIXED COLOR";
        f.text_center(
            preview.0 + preview.2 * 0.5,
            preview.1 + preview.3 * 0.5 + hex_size + 14.0 * s,
            2.5 * s,
            sub,
            gui::Color::lerp(text_color, gui::Color::GRAY, 0.4),
        );

        // ------------------------------------------------------------- sliders
        let labels = ["R", "G", "B"];
        let channel_colors = [
            gui::Color::rgb(1.0, 0.25, 0.25),
            gui::Color::rgb(0.3, 1.0, 0.35),
            gui::Color::rgb(0.3, 0.5, 1.0),
        ];
        for i in 0..3 {
            let y = rows_y + i as f32 * row_pitch;
            let v = mixer.rgb[i];

            // Label + value.
            f.text(
                margin,
                y + track_h * 0.5 - 3.5 * 4.0 * s,
                4.0 * s,
                labels[i],
                channel_colors[i],
            );
            let val = format!("{:3}", (v * 255.0).round() as u8);
            f.text(
                track_x1 + 14.0 * s,
                y + track_h * 0.5 - 3.5 * 3.0 * s,
                3.0 * s,
                &val,
                gui::Color::WHITE,
            );

            // Track.
            let active = mixer.drag == Some(i);
            let base = if active {
                gui::Color::rgb(0.26, 0.27, 0.34)
            } else {
                gui::Color::rgb(0.19, 0.19, 0.24)
            };
            f.rect(track_x0, y, track_w, track_h, base);
            // Filled portion up to the handle.
            f.rect(track_x0, y, track_w * v, track_h, channel_colors[i]);
            f.frame_rect(
                track_x0,
                y,
                track_w,
                track_h,
                1.5 * s,
                gui::Color::rgb(0.32, 0.34, 0.44),
            );

            // Handle.
            let hx = track_x0 + track_w * v;
            let hy = y + track_h * 0.5;
            let handle_r = (10.0 * s).max(6.0);
            let handle_color = if active
                || (f.input.mouse_down()
                    && (mx - hx).abs() < 16.0 * s
                    && (my - hy).abs() < 16.0 * s)
            {
                gui::Color::WHITE
            } else {
                gui::Color::lerp(gui::Color::rgb(0.85, 0.86, 0.92), channel_colors[i], 0.35)
            };
            f.circle(hx, hy, handle_r, handle_color);
            f.ring(
                hx,
                hy,
                handle_r,
                handle_r - 2.5 * s,
                gui::Color::rgb(0.12, 0.12, 0.16),
            );

            // Gradient strip under the track: 0 -> full channel.
            let strip_y = y + track_h + 10.0 * s;
            let segs = 40;
            let seg_w = track_w / segs as f32;
            for k in 0..segs {
                let t = (k as f32 + 0.5) / segs as f32;
                let (r, g, b) = match i {
                    0 => (t, 0.0, 0.0),
                    1 => (0.0, t, 0.0),
                    _ => (0.0, 0.0, t),
                };
                f.rect(
                    track_x0 + k as f32 * seg_w,
                    strip_y,
                    seg_w + 1.0,
                    8.0 * s,
                    gui::Color::rgb(r, g, b),
                );
            }
        }

        // ------------------------------------------------------------- random
        let hover_fill =
            gui::Color::lerp(gui::Color::rgb(0.55, 0.35, 0.15), gui::Color::WHITE, 0.2);
        f.rect(
            random_rect.0,
            random_rect.1,
            random_rect.2,
            random_rect.3,
            if in_random {
                hover_fill
            } else {
                gui::Color::rgb(0.55, 0.35, 0.15)
            },
        );
        f.text_center(
            random_rect.0 + random_rect.2 * 0.5,
            random_rect.1 + random_rect.3 * 0.5 - 3.5 * 3.0 * s,
            3.0 * s,
            "RANDOM",
            gui::Color::WHITE,
        );

        f.text(
            margin,
            f.height - 30.0 * s,
            2.0 * s,
            "CLICK/DRAG SLIDERS - R OR RANDOM = NEW COLOR",
            gui::Color::GRAY,
        );
    });
}
