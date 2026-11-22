//! 015 — Kaleidoscope: a fullscreen custom fragment shader. The fragment is
//! folded into one-eighth of the disc by mirroring the angle, then samples a
//! blob field of three moving points — giving 8-fold mirror symmetry. Caption
//! strips stay dark for the bitmap text.

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
        return float4(0.03, 0.03, 0.07, 1.0);
    }
    float t = u[0].z;
    float m = min(size.x, size.y);
    float2 p = frag.xy - size * 0.5;
    float r = length(p);
    float a = atan2(p.y, p.x);
    // Fold the angle into 0..seg/2 — 8-fold mirror symmetry.
    float seg = 6.2831853 / 8.0;
    a = abs(a);
    a = fmod(a, seg);
    a = min(a, seg - a);
    float2 q = float2(cos(a), sin(a)) * r;
    // Three drifting blobs sampled in the folded wedge.
    float2 c1 = float2(sin(t * 0.80), cos(t * 1.10)) * m * 0.26;
    float2 c2 = float2(cos(t * 0.50 + 2.1), sin(t * 0.70 + 1.3)) * m * 0.30;
    float2 c3 = float2(sin(t * 0.90 + 4.2), sin(t * 0.40 + 2.8)) * m * 0.15;
    float g = exp(-pow(length(q - c1) / (m * 0.10), 2.0))
            + exp(-pow(length(q - c2) / (m * 0.12), 2.0))
            + exp(-pow(length(q - c3) / (m * 0.08), 2.0));
    float3 col = float3(0.03, 0.03, 0.07);
    col = mix(col, float3(0.55, 0.12, 0.70), smoothstep(0.10, 0.75, g));
    col = mix(col, float3(0.20, 0.75, 0.85), smoothstep(0.70, 1.35, g));
    col = mix(col, float3(1.00, 0.95, 0.90), smoothstep(1.30, 2.20, g));
    // Faint mirrored rays for structure.
    col += float3(0.05, 0.05, 0.10) * pow(max(cos(a * 8.0), 0.0), 3.0);
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("015 - Kaleidoscope", 640.0, 440.0);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.07));

        let (w, h) = (f.width, f.height);
        let verts = [
            gui::Vertex::new(0.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(w * 2.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(0.0, h * 2.0, gui::Color::BLACK),
        ];
        f.draw_custom(&pipeline, &[], &verts);

        f.text_center(f.width * 0.5, 14.0, 4.0, "KALEIDOSCOPE", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "8-FOLD MIRROR SYMMETRY OF THREE MOVING BLOBS - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
