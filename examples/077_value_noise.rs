//! 077 — Value Noise: a fragment shader evaluating three octaves of
//! lattice-hashed, smoothly interpolated value noise with z = time, so the
//! heightfield boils gently. Height maps through a palette: deep darks up
//! to green slopes, rock gray, then a white snowline on the peaks.

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

float hash31(float3 p) {
    p = fract(p * 0.3183099 + 0.1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

float vnoise(float3 x) {
    float3 i = floor(x);
    float3 f = fract(x);
    f = f * f * (3.0 - 2.0 * f);
    float n000 = hash31(i + float3(0.0, 0.0, 0.0));
    float n100 = hash31(i + float3(1.0, 0.0, 0.0));
    float n010 = hash31(i + float3(0.0, 1.0, 0.0));
    float n110 = hash31(i + float3(1.0, 1.0, 0.0));
    float n001 = hash31(i + float3(0.0, 0.0, 1.0));
    float n101 = hash31(i + float3(1.0, 0.0, 1.0));
    float n011 = hash31(i + float3(0.0, 1.0, 1.0));
    float n111 = hash31(i + float3(1.0, 1.0, 1.0));
    float nx00 = mix(n000, n100, f.x);
    float nx10 = mix(n010, n110, f.x);
    float nx01 = mix(n001, n101, f.x);
    float nx11 = mix(n011, n111, f.x);
    float nxy0 = mix(nx00, nx10, f.y);
    float nxy1 = mix(nx01, nx11, f.y);
    return mix(nxy0, nxy1, f.z);
}

float fbm3(float3 p) {
    float v = 0.0;
    float a = 0.5;
    for (int o = 0; o < 3; o++) {
        v += a * vnoise(p);
        p = p * 2.03 + float3(11.7, 5.3, 7.9);
        a *= 0.5;
    }
    return v / 0.875;
}

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
    float top = u[1].x;
    float bot = u[1].y;
    float rh = u[0].y - top - bot;
    float2 rel = frag.xy - float2(u[0].x * 0.5, top + rh * 0.5);
    float2 uv = rel / rh;
    float3 p = float3(uv.x * 2.4, -uv.y * 2.4, u[0].z * 0.4);
    float h = fbm3(p);
    float3 col = mix(float3(0.03, 0.05, 0.08), float3(0.10, 0.38, 0.16), smoothstep(0.15, 0.45, h));
    col = mix(col, float3(0.45, 0.42, 0.36), smoothstep(0.52, 0.62, h));
    col = mix(col, float3(0.93, 0.95, 0.98), smoothstep(0.62, 0.74, h));
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("077 - Value Noise", W, H);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::BLACK);

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

        f.text(
            16.0,
            14.0,
            3.0,
            "ANIMATED 3-OCTAVE VALUE NOISE",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 16.0 - f.text_width(2.5, "Z = TIME"),
            18.0,
            2.5,
            "Z = TIME",
            gui::Color::GREEN,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "HASHED LATTICE - DARK VALLEYS, GREEN SLOPES, WHITE SNOWLINE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
