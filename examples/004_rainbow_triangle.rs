//! 004 — Rainbow Triangle: one triangle spinning slowly around the window
//! center. Its three vertex colors cycle through hue space with different
//! phase offsets, and the radius pulses gently with `sin(time)`. A true
//! per-vertex color gradient needs the stock vertex-color shader rebuilt
//! with [`gui::make_pipeline`] (the GUI primitives paint one color per
//! primitive), so the triangle goes through [`Frame::draw_custom`].

use std::f32::consts::TAU;

#[path = "gui/mod.rs"]
mod gui;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct GuiVertex {
    float2 pos;
    float4 color;
};

struct GuiOut {
    float4 position [[position]];
    float4 color;
};

vertex GuiOut vertex_main(const device GuiVertex* v [[buffer(0)]],
                          constant float4* u [[buffer(1)]],
                          uint vid [[vertex_id]]) {
    float2 size = u[0].xy;
    float2 ndc = float2(v[vid].pos.x / size.x * 2.0 - 1.0,
                        1.0 - v[vid].pos.y / size.y * 2.0);
    GuiOut out;
    out.position = float4(ndc, 0.0, 1.0);
    out.color = v[vid].color;
    return out;
}

fragment float4 fragment_main(GuiOut in [[stage_in]]) {
    return in.color;
}
"#;

fn main() {
    let app = gui::App::new("004 - Rainbow Triangle", 640.0, 400.0);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.04, 0.04, 0.08));

        let (cx, cy) = (f.width * 0.5, f.height * 0.5);
        let radius = f.width.min(f.height) * 0.30 * (1.0 + 0.15 * (f.time * 1.1).sin());
        let spin = f.time * 0.7;

        // Halo ring around the triangle, drawn under it.
        f.ring_segments(
            cx,
            cy,
            radius * 1.22,
            radius * 1.17,
            64,
            gui::Color::rgb(0.16, 0.18, 0.30),
        );

        let mut verts = Vec::with_capacity(3);
        for i in 0..3 {
            let angle = spin + i as f32 * TAU / 3.0;
            let hue = f.time * 0.10 + i as f32 / 3.0;
            verts.push(gui::Vertex::new(
                cx + angle.cos() * radius,
                cy + angle.sin() * radius,
                gui::Color::hsv(hue, 0.85, 1.0),
            ));
        }
        f.draw_custom(&pipeline, &[], &verts);

        f.text_center(
            f.width * 0.5,
            14.0,
            4.0,
            "RAINBOW TRIANGLE",
            gui::Color::WHITE,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "VERTEX HUES CYCLE AT OFFSET PHASES - RADIUS PULSES WITH SIN(TIME)",
            gui::Color::GRAY,
        );
    });
}
