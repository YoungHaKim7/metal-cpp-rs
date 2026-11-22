//! 094 — Sine Plotter: an animated function plot over x in [-10, 10] with
//! axes and faint gridlines. Four curves — sin(x+t), cos(x-t), a damped
//! sin(x)*exp(-0.15x), and sin(3x)*0.4 + sin(x+t)*0.6 — are toggled with
//! keys 1-4 or by clicking their legend rows; the legend shows each
//! curve's color and ON/OFF state.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;

const X_MIN: f32 = -10.0;
const X_MAX: f32 = 10.0;
const Y_MIN: f32 = -2.0;
const Y_MAX: f32 = 2.0;
const SAMPLES: usize = 400;

const CURVE_NAMES: [&str; 4] = [
    "SIN(X + T)",
    "COS(X - T)",
    "SIN(X)*EXP(-0.15X)",
    "SIN(3X)*0.4 + SIN(X+T)*0.6",
];
const CURVE_COLORS: [gui::Color; 4] = [
    gui::Color::CYAN,
    gui::Color::MAGENTA,
    gui::Color::YELLOW,
    gui::Color::GREEN,
];

/// Evaluate curve `i` at `x` with animation time `t`.
fn curve(i: usize, x: f32, t: f32) -> f32 {
    match i {
        0 => (x + t).sin(),
        1 => (x - t).cos(),
        2 => x.sin() * (-0.15 * x).exp(),
        _ => (3.0 * x).sin() * 0.4 + (x + t).sin() * 0.6,
    }
}

fn main() {
    let mut on = [true, false, false, false];
    let app = gui::App::new("094 - Sine Plotter", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.08, 0.08, 0.11));
        let s = f.scale;

        // ------------------------------------------------------------- layout
        let margin = 26.0 * s;
        let left = 54.0 * s;
        let bottom = 40.0 * s;
        let px0 = margin + left;
        let py0 = margin + 26.0 * s;
        let pw = f.width - px0 - margin;
        let ph = f.height - py0 - margin - bottom;

        let map_x = |x: f32| px0 + (x - X_MIN) / (X_MAX - X_MIN) * pw;
        let map_y = |y: f32| py0 + (Y_MAX - y) / (Y_MAX - Y_MIN) * ph;

        // ------------------------------------------------------------- toggles
        let num_keys = [
            gui::key::NUM_1,
            gui::key::NUM_2,
            gui::key::NUM_3,
            gui::key::NUM_4,
        ];
        for (i, k) in num_keys.iter().enumerate() {
            if f.input.key_pressed(*k) {
                on[i] = !on[i];
            }
        }

        // ------------------------------------------------------------- grid
        f.rect(px0, py0, pw, ph, gui::Color::rgb(0.11, 0.11, 0.15));
        let mut xi = X_MIN as i32;
        while xi <= X_MAX as i32 {
            let x = xi as f32;
            let color = if x == 0.0 {
                gui::Color::rgb(0.42, 0.44, 0.54)
            } else {
                gui::Color::rgb(0.17, 0.18, 0.23)
            };
            f.line(map_x(x), py0, map_x(x), py0 + ph, 1.0 * s, color);
            if x.abs() % 5.0 == 0.0 {
                let label = format!("{x}");
                f.text_center(
                    map_x(x),
                    py0 + ph + 6.0 * s,
                    2.0 * s,
                    &label,
                    gui::Color::GRAY,
                );
            }
            xi += 1;
        }
        let mut yi = (Y_MIN * 2.0) as i32;
        while yi <= (Y_MAX * 2.0) as i32 {
            let y = yi as f32 / 2.0;
            let color = if y == 0.0 {
                gui::Color::rgb(0.42, 0.44, 0.54)
            } else {
                gui::Color::rgb(0.17, 0.18, 0.23)
            };
            f.line(px0, map_y(y), px0 + pw, map_y(y), 1.0 * s, color);
            let label = format!("{y:.1}");
            f.text(
                px0 - f.text_width(2.0 * s, &label) - 8.0 * s,
                map_y(y) - 3.5 * 2.0 * s,
                2.0 * s,
                &label,
                gui::Color::GRAY,
            );
            yi += 1;
        }
        f.frame_rect(px0, py0, pw, ph, 2.0 * s, gui::Color::rgb(0.30, 0.32, 0.42));

        // ------------------------------------------------------------- curves
        for i in 0..4 {
            if !on[i] {
                continue;
            }
            let mut prev: Option<(f32, f32)> = None;
            for k in 0..=SAMPLES {
                let x = X_MIN + (X_MAX - X_MIN) * k as f32 / SAMPLES as f32;
                let y = curve(i, x, f.time).clamp(Y_MIN - 0.2, Y_MAX + 0.2);
                let (sx, sy) = (map_x(x), map_y(y));
                if let Some((lx, ly)) = prev {
                    f.line(lx, ly, sx, sy, 2.5 * s, CURVE_COLORS[i]);
                }
                prev = Some((sx, sy));
            }
        }

        // ------------------------------------------------------------- legend
        let lg_x = margin + 8.0 * s;
        let lg_y = py0 + 12.0 * s;
        let row_h = 26.0 * s;
        let (mx, my) = f.mouse();
        for i in 0..4 {
            let y = lg_y + i as f32 * row_h;
            let label = format!("{}  [{}]", CURVE_NAMES[i], if on[i] { "ON" } else { "OFF" });
            let lw = f.text_width(2.5 * s, &label) + 34.0 * s;
            let hover = mx >= lg_x && mx <= lg_x + lw && my >= y && my <= y + row_h - 6.0 * s;
            if hover {
                // Only paint a legend backing plate on hover (alpha blending
                // is off in this runtime, so a low-alpha fill is not an option).
                f.rect(
                    lg_x,
                    y,
                    lw,
                    row_h - 6.0 * s,
                    gui::Color::rgb(0.20, 0.21, 0.28),
                );
            }
            let swatch_color = if on[i] {
                CURVE_COLORS[i]
            } else {
                gui::Color::rgb(0.35, 0.36, 0.42)
            };
            f.rect(
                lg_x + 6.0 * s,
                y + 6.0 * s,
                18.0 * s,
                row_h - 18.0 * s,
                swatch_color,
            );
            let text_color = if on[i] {
                CURVE_COLORS[i]
            } else {
                gui::Color::GRAY
            };
            f.text(lg_x + 32.0 * s, y + 5.0 * s, 2.5 * s, &label, text_color);
            if f.input.mouse_pressed() && hover {
                on[i] = !on[i];
            }
        }

        // ------------------------------------------------------------- header
        f.text(margin, 22.0 * s, 5.0 * s, "SINE PLOTTER", gui::Color::WHITE);
        let visible = on.iter().filter(|&&v| v).count();
        let tag = format!("{} / 4 CURVES", visible);
        f.text(
            f.width - margin - f.text_width(2.5 * s, &tag),
            30.0 * s,
            2.5 * s,
            &tag,
            gui::Color::GRAY,
        );

        f.text(
            margin,
            f.height - 26.0 * s,
            2.0 * s,
            "1-4 TOGGLE CURVES - CLICK LEGEND TO TOGGLE - X IN [-10,10]",
            gui::Color::GRAY,
        );
    });
}
