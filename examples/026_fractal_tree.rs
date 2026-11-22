//! 026 — Fractal Tree: a recursive binary tree whose depth grows to 12 over
//! ten seconds and holds (R replays the growth); the branch spread sways
//! with time, bark darkens at the trunk and brightens toward the tips, and
//! line width shrinks with depth.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 820.0;
const H: f64 = 680.0;
const PI: f32 = std::f32::consts::PI;
const MAX_DEPTH: usize = 12;
const GROW_TIME: f32 = 10.0;

fn branch(
    f: &mut gui::Frame,
    x: f32,
    y: f32,
    angle: f32,
    len: f32,
    depth: usize,
    max_depth: usize,
    spread: f32,
    sway: f32,
) {
    let wobble = (sway * 1.7 + depth as f32 * 0.6).sin() * 0.05;
    let a = angle + wobble;
    let (nx, ny) = (x + a.cos() * len, y + a.sin() * len);
    let shade = depth as f32 / max_depth as f32;
    let tip = gui::Color::hsv(0.29 - 0.06 * shade, 0.8, 0.3 + 0.7 * shade);
    let color = gui::Color::lerp(gui::Color::rgb(0.3, 0.19, 0.11), tip, shade);
    let width = (11.0 * 0.78f32.powi(depth as i32)).max(1.2);
    f.line(x, y, nx, ny, width, color);
    if depth < max_depth {
        branch(
            f,
            nx,
            ny,
            a - spread,
            len * 0.74,
            depth + 1,
            max_depth,
            spread,
            sway,
        );
        branch(
            f,
            nx,
            ny,
            a + spread,
            len * 0.74,
            depth + 1,
            max_depth,
            spread,
            sway,
        );
    }
}

fn main() {
    let mut t0 = 0.0f32;
    let app = gui::App::new("026 - Fractal Tree", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        if f.input.key_pressed(gui::key::R) {
            t0 = f.time;
        }
        let eff = f.time - t0;

        let depth = (1.0 + (eff / GROW_TIME).min(1.0) * (MAX_DEPTH - 1) as f32) as usize;
        let spread = 0.42 + 0.2 * (eff * 0.8).sin();
        let len0 = f.height * 0.15;

        branch(
            f,
            f.width * 0.5,
            f.height - 46.0,
            -PI / 2.0,
            len0,
            0,
            depth,
            spread,
            eff,
        );

        f.text(24.0, 20.0, 4.0, "026 - FRACTAL TREE", gui::Color::WHITE);
        let label = format!("DEPTH = {depth} / {MAX_DEPTH}");
        f.text(24.0, 54.0, 3.0, &label, gui::Color::GREEN);
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "DEPTH GROWS TO 12 OVER 10S THEN HOLDS - ANGLE SWAYS - R REPLAYS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
