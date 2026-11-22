//! 003 — Gradient Rect: bringing your own Metal shader. Builds a pipeline
//! with [`gui::make_pipeline`] (MSL functions named `vertex_main` /
//! `fragment_main`) and submits an oversized fullscreen triangle through
//! [`Frame::draw_custom`].
//!
//! Conventions for custom shaders (fixed by the GUI runtime):
//! * vertex stage: `const device GuiVertex* v [[buffer(0)]]` (24 bytes:
//!   `float2 pos` in pixels, `float4 color`) and
//!   `constant float4* u [[buffer(1)]]` with `u[0] = (w, h, time, scale)`;
//! * fragment stage: `float4 frag [[position]]` (pixel coords) and
//!   `constant float4* u [[buffer(0)]]` with the same `u[0]`.

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

fragment float4 fragment_main(float4 frag [[position]],
                              constant float4* u [[buffer(0)]]) {
    float2 uv = frag.xy / u[0].xy;
    float3 top = float3(0.10, 0.35, 0.85);
    float3 bottom = float3(0.85, 0.15, 0.45);
    float3 col = mix(top, bottom, uv.y);
    // Soft animated banding so it is visibly alive.
    col += 0.05 * sin(uv.y * 40.0 + u[0].z * 2.0);
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("003 - Gradient Rect", 640.0, 400.0);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        // One oversized triangle covers the whole drawable.
        let (w, h) = (f.width, f.height);
        let verts = [
            gui::Vertex::new(0.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(w * 2.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(0.0, h * 2.0, gui::Color::BLACK),
        ];
        f.draw_custom(&pipeline, &[], &verts);

        f.text_center(
            f.width * 0.5,
            f.height * 0.10,
            3.0,
            "CUSTOM FRAGMENT SHADER - GRADIENT + BANDS",
            gui::Color::WHITE,
        );
    });
}
