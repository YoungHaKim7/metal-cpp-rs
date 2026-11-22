//! 016 — Synthwave Grid: a fullscreen custom fragment shader. A big sun with
//! horizontal scanlines sits on the horizon, a perspective grid scrolls
//! toward the viewer below it, and the sky is dark with twinkling stars —
//! all in purple/pink. Caption strips stay dark for the bitmap text.

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
        return float4(0.03, 0.02, 0.06, 1.0);
    }
    float t = u[0].z;
    float horizon = size.y * 0.46;
    float3 col;
    if (frag.y < horizon) {
        // Sky: near-black at the top, purple toward the horizon, plus stars.
        float k = frag.y / horizon;
        col = mix(float3(0.02, 0.02, 0.05), float3(0.12, 0.04, 0.22), k);
        float2 cell = floor(frag.xy / 4.0);
        float h = fract(sin(dot(cell, float2(127.1, 311.7))) * 43758.5453);
        float star = step(0.986, h) * max(sin(t * 2.0 + h * 60.0), 0.0);
        col += float3(0.80, 0.85, 1.00) * star * 0.6;
        // Sun with horizontal scanlines that thicken toward its base.
        float2 sc = float2(size.x * 0.5, horizon - size.y * 0.10);
        float sr = size.y * 0.17;
        float d = distance(frag.xy, sc);
        float inside = 1.0 - smoothstep(sr - 1.5, sr + 1.5, d);
        float kk = clamp(0.5 - (sc.y - frag.y) / sr * 0.5, 0.0, 1.0);
        float stripe = fract(frag.y / max(sr * 0.13, 1.0));
        float vis = step(mix(0.12, 0.80, kk), stripe);
        float3 sun_col = mix(float3(1.00, 0.85, 0.30), float3(1.00, 0.28, 0.55), kk);
        col = mix(col, sun_col, inside * vis);
        col += float3(0.90, 0.25, 0.50) * 0.30 * exp(-d * d / (sr * sr * 3.0));
    } else {
        // Ground: rows and columns in true perspective, scrolling forward.
        float g = (frag.y - horizon) / max(size.y - horizon, 1.0);
        col = mix(float3(0.10, 0.03, 0.18), float3(0.03, 0.02, 0.07), g);
        float row = 1.0 / max(g, 1e-4) + t * 1.6;
        float drow = abs(fract(row) - 0.5);
        float hline = smoothstep(0.38, 0.49, drow) * smoothstep(0.02, 0.10, g);
        float xw = (frag.x - size.x * 0.5) / (max(size.y - horizon, 1.0) * max(g, 1e-4));
        float fx = fract(abs(xw) * 4.0);
        float vline = (1.0 - smoothstep(0.015, 0.05, min(fx, 1.0 - fx))) * smoothstep(0.02, 0.12, g);
        col += float3(1.00, 0.30, 0.65) * hline * 0.85;
        col += float3(0.20, 0.80, 1.00) * vline * 0.50;
    }
    // Hot pink glow line along the horizon.
    col += float3(1.00, 0.30, 0.60) * 0.40 * exp(-abs(frag.y - horizon) * 0.15);
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("016 - Synthwave Grid", 640.0, 440.0);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.02, 0.06));

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
            "SYNTHWAVE GRID",
            gui::Color::WHITE,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "SCANLINED SUN + SCROLLING PERSPECTIVE GRID - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
