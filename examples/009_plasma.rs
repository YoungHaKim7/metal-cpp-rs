//! 009 — Plasma: a fullscreen custom fragment shader — the classic demoscene
//! sum of sines over uv and time, colored with a purple / green / orange
//! palette. The shader masks the caption strips so the bitmap text stays
//! readable on top of the field.

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
    float2 size = u[0].xy;
    if (frag.y < 56.0 || frag.y > size.y - 56.0) {
        return float4(0.04, 0.04, 0.08, 1.0);
    }
    float2 uv = frag.xy / size;
    float t = u[0].z;
    float v = sin(uv.x * 9.0 + t * 1.1)
            + sin((uv.x + uv.y) * 7.0 + t * 0.8)
            + sin(length(uv - float2(0.5, 0.5)) * 14.0 - t * 1.6)
            + sin(uv.y * 11.0 - t * 0.9)
            + sin(uv.x * uv.y * 30.0 + t * 0.5) * 0.5;
    float n = v * 0.5 / 4.5 + 0.5;
    float3 purple = float3(0.42, 0.10, 0.62);
    float3 green  = float3(0.12, 0.70, 0.38);
    float3 orange = float3(1.00, 0.55, 0.15);
    float3 col = n < 0.5 ? mix(purple, green, n * 2.0) : mix(green, orange, (n - 0.5) * 2.0);
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("009 - Plasma", 640.0, 400.0);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.04, 0.04, 0.08));

        // One oversized triangle covers the whole drawable.
        let (w, h) = (f.width, f.height);
        let verts = [
            gui::Vertex::new(0.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(w * 2.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(0.0, h * 2.0, gui::Color::BLACK),
        ];
        f.draw_custom(&pipeline, &[], &verts);

        f.text_center(f.width * 0.5, 14.0, 4.0, "PLASMA", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "SUM OF SINES OVER UV AND TIME - PURPLE GREEN ORANGE - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
