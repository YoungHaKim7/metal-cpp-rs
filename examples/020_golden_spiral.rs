//! 020 — Golden Spiral: fibonacci squares (sides 1, 1, 2, 3, 5, 8, 13, 21)
//! scaled to fit the window, quarter-circle arcs drawn inside each square
//! with [`gui::Frame::ring_segments`], and a slowly rotating golden-angle
//! phyllotaxis dot spiral overlaid on top.
//!
//! Each arc is a full `ring_segments` annulus centered on its square's inner
//! corner; the three non-arc quadrants of the annulus are painted back over
//! with the background color (drawn in spiral order so later squares always
//! repaint whatever an earlier erase touched), leaving exactly the quarter
//! arc inside its square.

use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

#[path = "gui/mod.rs"]
mod gui;

const SIDES: [f32; 8] = [1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0];
const GOLDEN_ANGLE: f32 = 2.39996323;
const BG: gui::Color = gui::Color::rgb(0.04, 0.04, 0.08);

fn main() {
    let app = gui::App::new("020 - Golden Spiral", 560.0, 620.0);
    app.run(|f| {
        f.set_clear(BG);

        // Build the spiral in "fib units" (y-up): arc k is a quarter circle
        // of radius side_k centered at c, spanning ang..ang+90; the next
        // center sits one radius further along the arc's end radial.
        let mut squares: Vec<((f32, f32), f32, f32)> = Vec::new();
        let mut c = (0.0f32, 0.0f32);
        let mut ang = 0.0f32;
        for (k, &side) in SIDES.iter().enumerate() {
            squares.push((c, ang, side));
            let a2 = ang + FRAC_PI_2;
            let end = (c.0 + side * a2.cos(), c.1 + side * a2.sin());
            let next = SIDES.get(k + 1).copied().unwrap_or(side);
            c = (end.0 + next * a2.cos(), end.1 + next * a2.sin());
            ang = a2 + PI;
        }

        // Bounding box over every square.
        let mut minx = f32::MAX;
        let mut maxx = f32::MIN;
        let mut miny = f32::MAX;
        let mut maxy = f32::MIN;
        for &(cc, a, r) in &squares {
            let a2 = a + FRAC_PI_2;
            let p = (cc.0 + r * a.cos(), cc.1 + r * a.sin());
            let q = (cc.0 + r * a2.cos(), cc.1 + r * a2.sin());
            let d = (p.0 + q.0 - cc.0, p.1 + q.1 - cc.1);
            for &(x, y) in &[cc, p, q, d] {
                minx = minx.min(x);
                maxx = maxx.max(x);
                miny = miny.min(y);
                maxy = maxy.max(y);
            }
        }

        // Fit between the caption strips, centered.
        let bw = maxx - minx;
        let bh = maxy - miny;
        let top = 64.0;
        let bottom = 56.0;
        let s = ((f.width - 60.0) / bw).min((f.height - top - bottom) / bh);
        let ox = (f.width - bw * s) * 0.5 - minx * s;
        let oy = top + ((f.height - top - bottom) - bh * s) * 0.5 + maxy * s;
        let scr = |x: f32, y: f32| -> (f32, f32) { (ox + x * s, oy - y * s) };
        let square_rect = |cc: (f32, f32), a: f32, r: f32| -> (f32, f32, f32, f32) {
            let a2 = a + FRAC_PI_2;
            let p = (cc.0 + r * a.cos(), cc.1 + r * a.sin());
            let q = (cc.0 + r * a2.cos(), cc.1 + r * a2.sin());
            let d = (p.0 + q.0 - cc.0, p.1 + q.1 - cc.1);
            let rx0 = cc.0.min(p.0).min(q.0).min(d.0);
            let rx1 = cc.0.max(p.0).max(q.0).max(d.0);
            let ry0 = cc.1.min(p.1).min(q.1).min(d.1);
            let ry1 = cc.1.max(p.1).max(q.1).max(d.1);
            let (x0, y0) = scr(rx0, ry1);
            (x0, y0, (rx1 - rx0) * s, (ry1 - ry0) * s)
        };

        // Squares + quarter arcs, innermost first.
        let arc_w = (s * 0.16).max(2.5);
        for (k, &(cc, a, r)) in squares.iter().enumerate() {
            let (x0, y0, w, h) = square_rect(cc, a, r);
            let tint = gui::Color::lerp(
                gui::Color::rgb(0.07, 0.08, 0.13),
                gui::Color::rgb(0.15, 0.13, 0.22),
                k as f32 / 7.0,
            );
            f.rect(x0, y0, w, h, tint);

            // Full annulus at the arc corner; only the arc quadrant may live.
            let (cx, cy) = scr(cc.0, cc.1);
            let col = gui::Color::hsv(0.105 - k as f32 * 0.006, 0.62, 1.0);
            f.ring_segments(cx, cy, r * s, (r * s - arc_w).max(0.5), 48, col);

            // Erase the three off-quadrants with the background color.
            for step in 1..4u32 {
                let b = a + FRAC_PI_2 * step as f32;
                let b2 = b + FRAC_PI_2;
                let c1 = (cc.0 + r * b.cos(), cc.1 + r * b.sin());
                let c2 = (cc.0 + r * b2.cos(), cc.1 + r * b2.sin());
                let c3 = (c1.0 + c2.0 - cc.0, c1.1 + c2.1 - cc.1);
                let ex0 = cc.0.min(c1.0).min(c2.0).min(c3.0);
                let ex1 = cc.0.max(c1.0).max(c2.0).max(c3.0);
                let ey0 = cc.1.min(c1.1).min(c2.1).min(c3.1);
                let ey1 = cc.1.max(c1.1).max(c2.1).max(c3.1);
                let (sx0, sy0) = scr(ex0, ey1);
                f.rect(sx0, sy0, (ex1 - ex0) * s, (ey1 - ey0) * s, BG);
            }
        }

        // Square outlines on top of everything geometric.
        for &(cc, a, r) in &squares {
            let (x0, y0, w, h) = square_rect(cc, a, r);
            f.frame_rect(x0, y0, w, h, 2.0, gui::Color::rgb(0.56, 0.50, 0.34));
        }

        // Golden-angle phyllotaxis overlay, rotating slowly.
        let (pcx, pcy) = (f.width * 0.5, f.height * 0.5);
        let rmax = f.width.min(f.height) * 0.42;
        let n_dots = 240;
        let norm = (n_dots as f32).sqrt();
        for i in 1..=n_dots {
            let fi = i as f32;
            let rad = rmax * (fi.sqrt() / norm);
            let a = fi * GOLDEN_ANGLE + f.time * 0.15;
            let col = gui::Color::hsv(0.13 - 0.05 * (fi / n_dots as f32), 0.60, 0.95);
            f.circle_segments(pcx + a.cos() * rad, pcy + a.sin() * rad, 2.5, 10, col);
        }

        f.text_center(f.width * 0.5, 14.0, 4.0, "GOLDEN SPIRAL", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "FIBONACCI SQUARES 1 1 2 3 5 8 13 21 + GOLDEN ANGLE DOTS - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
