//! 013 — Voronoi: a fullscreen custom fragment shader. Sixteen seed points
//! drift on hash-seeded lissajous paths; cells are shaded by seed id, the
//! F2-F1 borders are highlighted, and the seeds themselves glow. Caption
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
    // Canvas coordinate in drawing units, interpolated from the vertices:
    // independent of the display's pixel density, unlike `position`.
    float2 pos;
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
    out.pos = v[vid].pos;
    out.color = v[vid].color;
    return out;
}

float hash(float n) {
    return fract(sin(n) * 43758.5453123);
}

fragment float4 fragment_main(GuiOut in [[stage_in]],
                              constant float4* u [[buffer(0)]]) {
    float2 size = u[0].xy;
    if (in.pos.y < 56.0 || in.pos.y > size.y - 56.0) {
        return float4(0.03, 0.03, 0.06, 1.0);
    }
    float aspect = size.x / size.y;
    float2 st = in.pos / size.y;
    float t = u[0].z;
    float d1 = 1e9;
    float d2 = 1e9;
    float id = 0.0;
    for (int i = 0; i < 16; i++) {
        float fi = float(i);
        float h1 = hash(fi * 127.1 + 11.0);
        float h2 = hash(fi * 311.7 + 17.0);
        float s1 = 0.10 + hash(fi * 74.7 + 3.0) * 0.22;
        float s2 = 0.10 + hash(fi * 91.3 + 7.0) * 0.22;
        float px = 0.10 + h1 * (aspect - 0.20) + 0.07 * sin(t * s1 * 6.2831853 + h2 * 40.0);
        float py = 0.10 + h2 * 0.80 + 0.07 * cos(t * s2 * 6.2831853 + h1 * 40.0);
        float d = distance(st, float2(px, py));
        if (d < d1) {
            d2 = d1;
            d1 = d;
            id = fi;
        } else if (d < d2) {
            d2 = d;
        }
    }
    // Cell body shaded by seed id, dimmed toward the seed center.
    float hue = hash(id * 91.7 + 3.0);
    float3 col = 0.30 + 0.22 * cos(6.2831853 * (hue + float3(0.0, 0.33, 0.66)));
    col *= clamp(1.15 - d1 * 1.5, 0.55, 1.15);
    // Bright borders where the two nearest seeds are almost equidistant.
    float edge = d2 - d1;
    col = mix(float3(0.90, 0.86, 0.95), col, smoothstep(0.008, 0.030, edge));
    // Glowing seed dot.
    col = mix(col, float3(1.00, 0.85, 0.35), 1.0 - smoothstep(0.006, 0.016, d1));
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("013 - Voronoi", 640.0, 440.0);
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

        f.text_center(f.width * 0.5, 14.0, 4.0, "VORONOI", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "16 DRIFTING SEEDS - SHADE BY CELL ID - HIGHLIGHTED BORDERS - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
