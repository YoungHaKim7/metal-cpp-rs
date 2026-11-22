//! 079 — Spirograph: a family of hypotrochoids. The ring radius R stays
//! fixed at 9 while the rolling radius r sweeps continuously through 1..9
//! and the pen rides at d = 1.6 r. Twelve hundred parametric points per
//! frame with hue riding the parameter; the current (r, d) shows in the
//! header.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 640.0;
const TOP_STRIP: f32 = 48.0;
const BOT_STRIP: f32 = 52.0;
const R_FIXED: f32 = 9.0;
const D_RATIO: f32 = 1.6;
const POINTS: usize = 1200;
const TURNS: f32 = 9.0;

fn main() {
    let app = gui::App::new("079 - Spirograph", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.04, 0.04, 0.07));

        let top = TOP_STRIP;
        let bot = f.height - BOT_STRIP;
        let rh = bot - top;
        let cx = f.width * 0.5;
        let cy = top + rh * 0.5;

        // r morphs smoothly through 1..9, d follows at 1.6 r.
        let morph = f.time * 0.35;
        let r = 5.0 + 4.0 * morph.sin();
        let d = D_RATIO * r;
        let ratio = (R_FIXED - r) / r;
        let scale = (f.width.min(rh) * 0.5 - 24.0) / (R_FIXED + 0.6 * R_FIXED);

        // The hypotrochoid itself, one 1.5px dot per sample.
        for i in 0..POINTS {
            let th = i as f32 / POINTS as f32 * std::f32::consts::TAU * TURNS;
            let x = (R_FIXED - r) * th.cos() + d * (ratio * th).cos();
            let y = (R_FIXED - r) * th.sin() - d * (ratio * th).sin();
            let hue = i as f32 / POINTS as f32;
            f.rect(
                cx + x * scale - 0.75,
                cy + y * scale - 0.75,
                1.5,
                1.5,
                gui::Color::hsv(hue, 0.8, 1.0),
            );
        }

        // Header with the live parameters.
        f.text(
            16.0,
            14.0,
            3.0,
            "SPIROGRAPH - HYPOTROCHOID FAMILY",
            gui::Color::WHITE,
        );
        let label = format!("RING R {:.2} - ROLLER r {:.2} - PEN d {:.2}", R_FIXED, r, d);
        f.text(
            f.width - 16.0 - f.text_width(2.5, &label),
            18.0,
            2.5,
            &label,
            gui::Color::CYAN,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "R FIXED AT 9 - r MORPHS 1..9 - D = 1.6 r - 1200 POINTS PER FRAME - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
