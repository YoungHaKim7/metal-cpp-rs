//! 071 — Burning Ship: a fragment shader iterating z = (|Re z| + i|Im z|)^2
//! + c over the classic vantage point centered near (-1.75, -0.03) with a
//! half-height scale of about 0.045. Smooth iteration count drives a
//! heatmap palette: black hull, deep red flanks, orange fire, pale embers.

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
    float top = u[1].x;
    float bot = u[1].y;
    float rh = u[0].y - top - bot;
    float2 rel = in.pos - float2(u[0].x * 0.5, top + rh * 0.5);
    // Center (-1.75, -0.03), vertical span 0.09 (half-scale ~0.045).
    float2 c = float2(-1.75 + rel.x / rh * 0.09, -0.03 - rel.y / rh * 0.09);
    float2 z = float2(0.0, 0.0);
    float sn = 0.0;
    bool escaped = false;
    for (int i = 0; i < 200; i++) {
        z = float2(abs(z.x), abs(z.y));
        z = float2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        float m2 = dot(z, z);
        if (m2 > 64.0) {
            sn = float(i) + 1.0 - log2(0.5 * log2(m2));
            escaped = true;
            break;
        }
    }
    float3 col = float3(0.0, 0.0, 0.0);
    if (escaped) {
        float t = pow(clamp(sn / 90.0, 0.0, 1.0), 0.5);
        col = mix(float3(0.02, 0.0, 0.05), float3(0.55, 0.05, 0.10), smoothstep(0.0, 0.35, t));
        col = mix(col, float3(1.0, 0.55, 0.10), smoothstep(0.35, 0.65, t));
        col = mix(col, float3(1.0, 0.95, 0.75), smoothstep(0.65, 0.95, t));
    }
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("071 - Burning Ship", W, H);
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
            "BURNING SHIP - Z = (|RE Z| + I|IM Z|)^2 + C",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 16.0 - f.text_width(2.5, "CENTER (-1.75, -0.03)"),
            18.0,
            2.5,
            "CENTER (-1.75, -0.03)",
            gui::Color::ORANGE,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "VERTICAL SPAN 0.09 - HEATMAP PALETTE WITH SMOOTH ITERATIONS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
