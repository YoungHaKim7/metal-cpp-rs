//! 027 — Koch Snowflake: an equilateral triangle whose edges subdivide every
//! two seconds up to depth 5 (R replays the growth); every segment is a thin
//! line with a hue from its index and the whole flake slowly rotates.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 680.0;
const MAX_DEPTH: usize = 5;
const LEVEL_TIME: f32 = 2.0;
/// sqrt(3)/6 — equilateral bump height as a factor of the edge vector.
const BUMP: f32 = 0.288_675_14;

/// One Koch subdivision pass: every edge a->b becomes a->p1->peak->p2 (->b).
fn subdivide(poly: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut out = Vec::with_capacity(poly.len() * 4);
    for i in 0..poly.len() {
        let (ax, ay) = poly[i];
        let (bx, by) = poly[(i + 1) % poly.len()];
        let (vx, vy) = (bx - ax, by - ay);
        let p1 = (ax + vx / 3.0, ay + vy / 3.0);
        let p2 = (ax + vx * 2.0 / 3.0, ay + vy * 2.0 / 3.0);
        // Outward bump for this winding (screen y grows down).
        let peak = ((ax + bx) * 0.5 + vy * BUMP, (ay + by) * 0.5 - vx * BUMP);
        out.push((ax, ay));
        out.push(p1);
        out.push(peak);
        out.push(p2);
    }
    out
}

fn main() {
    let mut t0 = 0.0f32;
    let app = gui::App::new("027 - Koch Snowflake", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        if f.input.key_pressed(gui::key::R) {
            t0 = f.time;
        }
        let eff = f.time - t0;

        let depth = ((eff / LEVEL_TIME) as usize).min(MAX_DEPTH);
        // Grow-in ease per level so each subdivision blooms instead of popping.
        let age = (eff / LEVEL_TIME).fract();
        let grow = (age * 3.0).min(1.0);
        let grow = grow * grow * (3.0 - 2.0 * grow);

        let r = (f.width.min(f.height) * 0.33).max(60.0) * (0.8 + 0.2 * grow);
        let rot = eff * 0.22;
        let (cx, cy) = (f.width * 0.5, f.height * 0.54);

        // Unit triangle, clockwise on screen (y down): top, bottom-right, bottom-left.
        let mut poly = vec![(0.0, -1.0f32), (0.866_025_4, 0.5), (-0.866_025_4, 0.5)];
        for _ in 0..depth {
            poly = subdivide(&poly);
        }

        let total = poly.len();
        let (s, c) = rot.sin_cos();
        for i in 0..total {
            let (ax, ay) = poly[i];
            let (bx, by) = poly[(i + 1) % total];
            let p = (cx + (ax * c - ay * s) * r, cy + (ax * s + ay * c) * r);
            let q = (cx + (bx * c - by * s) * r, cy + (bx * s + by * c) * r);
            let hue = i as f32 / total as f32 * 0.5 + 0.52 + f.time * 0.03;
            // 2 units wide: a whole device pixel on a non-Retina display, where
            // a 1.4-unit stroke faded to a ghost line.
            f.line(p.0, p.1, q.0, q.1, 2.0, gui::Color::hsv(hue, 0.75, 1.0));
        }

        f.text(24.0, 20.0, 4.0, "027 - KOCH SNOWFLAKE", gui::Color::WHITE);
        let label = format!("DEPTH = {depth} / {MAX_DEPTH}   SEGMENTS = {total}");
        f.text(24.0, 54.0, 3.0, &label, gui::Color::CYAN);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "ONE LEVEL EVERY 2S TO DEPTH 5 - SHAPE ROTATES - R REPLAYS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
