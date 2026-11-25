//! 072 — Newton Fractal: a fragment shader running Newton's method for
//! z^3 = 1 (eleven iterations max). Hue encodes which of the three cube
//! roots the pixel converged to; brightness fades with the iteration count,
//! so basins glow near their roots and darken along the boundaries.

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

fragment float4 fragment_main(GuiOut in [[stage_in]],
                              constant float4* u [[buffer(0)]]) {
    const float TAU = 6.28318530718;
    float top = u[1].x;
    float bot = u[1].y;
    float rh = u[0].y - top - bot;
    float2 rel = in.pos - float2(u[0].x * 0.5, top + rh * 0.5);
    float2 z = rel / (rh * 0.5) * 1.35;
    z.y = -z.y;
    float2 roots[3] = {float2(1.0, 0.0),
                       float2(-0.5, 0.8660254),
                       float2(-0.5, -0.8660254)};
    float it = 11.0;
    for (int i = 0; i < 11; i++) {
        float2 z2 = float2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
        float2 z3 = float2(z2.x * z.x - z2.y * z.y, z2.x * z.y + z2.y * z.x);
        float2 num = float2(z3.x - 1.0, z3.y);
        float2 den = float2(3.0 * z2.x, 3.0 * z2.y);
        float dd = dot(den, den) + 1e-12;
        z -= float2(num.x * den.x + num.y * den.y, num.y * den.x - num.x * den.y) / dd;
        float m = min(min(distance(z, roots[0]), distance(z, roots[1])), distance(z, roots[2]));
        if (m < 0.004) {
            it = float(i);
            break;
        }
    }
    float d0 = distance(z, roots[0]);
    float d1 = distance(z, roots[1]);
    float d2 = distance(z, roots[2]);
    float hue;
    if (d0 <= d1 && d0 <= d2) {
        hue = 0.0;
    } else if (d1 <= d2) {
        hue = 1.0 / 3.0;
    } else {
        hue = 2.0 / 3.0;
    }
    float v = clamp(1.0 - it / 11.0, 0.12, 1.0);
    float3 col = 0.55 + 0.45 * cos(TAU * (hue + float3(0.0, 0.33, 0.66)));
    col *= 0.18 + 0.82 * v;
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("072 - Newton Fractal", W, H);
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
            "NEWTON FRACTAL - NEWTON'S METHOD FOR Z^3 = 1",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 16.0 - f.text_width(2.5, "11 ITERATIONS MAX"),
            18.0,
            2.5,
            "11 ITERATIONS MAX",
            gui::Color::MAGENTA,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "HUE = WHICH ROOT CONVERGED - BRIGHTNESS = FEWER ITERATIONS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
