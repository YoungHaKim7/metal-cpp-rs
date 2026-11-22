//! 070 — Julia Animated: a fragment shader drawing the Julia set of
//! z^2 + c with c = 0.7885 * e^(i*theta), theta = 0.3 * time. The escape
//! count is smoothed (n + 1 - log2 log2 |z|) and mapped through a cosine
//! palette while c slowly orbits the circle, so the set breathes, splits
//! and swirls.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 640.0;
const TOP_STRIP: f32 = 48.0;
const BOT_STRIP: f32 = 52.0;

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
    const float TAU = 6.28318530718;
    float top = u[1].x;
    float bot = u[1].y;
    float rh = u[0].y - top - bot;
    float2 rel = frag.xy - float2(u[0].x * 0.5, top + rh * 0.5);
    float2 z = rel / (rh * 0.5) * 1.1;
    z.y = -z.y;
    float theta = u[0].z * 0.3;
    float2 c = 0.7885 * float2(cos(theta), sin(theta));
    float sn = 0.0;
    bool escaped = false;
    for (int i = 0; i < 300; i++) {
        z = float2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        float m2 = dot(z, z);
        if (m2 > 64.0) {
            sn = float(i) + 1.0 - log2(0.5 * log2(m2));
            escaped = true;
            break;
        }
    }
    float3 col = float3(0.01, 0.01, 0.03);
    if (escaped) {
        float t = pow(clamp(sn / 48.0, 0.0, 1.0), 0.45);
        col = 0.55 + 0.45 * cos(TAU * (t * 1.6 + float3(0.0, 0.18, 0.36)));
        col = col * (0.35 + 0.65 * smoothstep(0.0, 0.25, t));
    }
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("070 - Julia Animated", W, H);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::BLACK);

        // Fullscreen quad for the fractal, minus the caption strips.
        let y0 = TOP_STRIP;
        let y1 = f.height - BOT_STRIP;
        let black = gui::Color::BLACK;
        let verts = [
            gui::Vertex::new(0.0, y0, black),
            gui::Vertex::new(f.width, y0, black),
            gui::Vertex::new(0.0, y1, black),
            gui::Vertex::new(f.width, y0, black),
            gui::Vertex::new(f.width, y1, black),
            gui::Vertex::new(0.0, y1, black),
        ];
        f.draw_custom(&pipeline, &[TOP_STRIP, BOT_STRIP], &verts);

        // Caption strips (the custom quad never covers them).
        f.text(
            16.0,
            14.0,
            3.0,
            "JULIA SET - C = 0.7885 E^(I THETA)",
            gui::Color::WHITE,
        );
        let theta = format!("THETA = {:.3}", f.time * 0.3);
        f.text(
            f.width - 16.0 - f.text_width(2.5, &theta),
            18.0,
            2.5,
            &theta,
            gui::Color::CYAN,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "SMOOTH ITERATION COLORING - C ORBITS EVERY ~21S - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
