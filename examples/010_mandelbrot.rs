//! 010 — Mandelbrot: a fullscreen custom fragment shader. The view is
//! centered at (-0.6, 0) with a scale showing the full set; coloring is
//! smooth (continuous) in the escape iteration count with a slowly drifting
//! cosine palette. Caption strips stay dark for the bitmap text.

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
        return float4(0.03, 0.03, 0.06, 1.0);
    }
    // Complex plane: 3.0 units of height, centered at (-0.6, 0).
    float s = 3.0 / size.y;
    float2 c = float2(-0.6, 0.0) + (frag.xy - size * 0.5) * s;
    float2 z = float2(0.0, 0.0);
    float mu = -1.0;
    for (int i = 0; i < 300; i++) {
        z = float2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        float z2 = dot(z, z);
        if (z2 > 16.0) {
            mu = float(i) + 1.0 - log2(max(log2(z2) * 0.5, 1e-6));
            break;
        }
    }
    if (mu < 0.0) {
        return float4(0.01, 0.01, 0.04, 1.0);
    }
    float t = mu * 0.016 + u[0].z * 0.008;
    float3 col = 0.5 + 0.5 * cos(6.2831853 * (t + float3(0.0, 0.15, 0.35)));
    col *= 0.35 + 0.65 * clamp(mu * 0.02, 0.0, 1.0);
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("010 - Mandelbrot", 640.0, 440.0);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));

        let (w, h) = (f.width, f.height);
        let verts = [
            gui::Vertex::new(0.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(w * 2.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(0.0, h * 2.0, gui::Color::BLACK),
        ];
        f.draw_custom(&pipeline, &[], &verts);

        f.text_center(
            f.width * 0.5,
            14.0,
            4.0,
            "MANDELBROT SET",
            gui::Color::WHITE,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "SMOOTH COLORING BY ESCAPE ITERATIONS - CENTER (-0.6, 0) - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
