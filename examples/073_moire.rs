//! 073 — Moire: a fragment shader multiplying two concentric ring families,
//! sin(dist(center1) * k) and sin(dist(center2) * k), with a drifting phase
//! on the first family so the interference pattern crawls. The second
//! center follows the mouse, bending the fringes as it moves.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 640.0;
const TOP_STRIP: f32 = 48.0;
const BOT_STRIP: f32 = 52.0;
const RING_K: f32 = 0.5;

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
    float2 c1 = float2(u[0].x * 0.5, top + rh * 0.5);
    float2 c2 = u[1].zw;
    float d1 = distance(in.pos, c1);
    float d2 = distance(in.pos, c2);
    float v = sin(d1 * u[2].x - u[0].z * 2.0) * sin(d2 * u[2].x);
    float t = v * 0.5 + 0.5;
    float3 col = mix(float3(0.02, 0.03, 0.09), float3(0.30, 0.85, 0.95), t);
    col += float3(0.85, 0.25, 0.55) * pow(abs(v), 8.0) * 0.55;
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("073 - Moire", W, H);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::BLACK);

        let y0 = TOP_STRIP;
        let y1 = f.height - BOT_STRIP;
        let black = gui::Color::BLACK;
        let (mx, my) = f.mouse();
        let verts = [
            gui::Vertex::new(0.0, y0, black),
            gui::Vertex::new(f.width, y0, black),
            gui::Vertex::new(0.0, y1, black),
            gui::Vertex::new(f.width, y0, black),
            gui::Vertex::new(f.width, y1, black),
            gui::Vertex::new(0.0, y1, black),
        ];
        // u[1] = (top, bot, mouse x, mouse y); u[2].x = ring frequency.
        f.draw_custom(
            &pipeline,
            &[TOP_STRIP, BOT_STRIP, mx, my, RING_K, 0.0],
            &verts,
        );

        f.text(
            16.0,
            14.0,
            3.0,
            "MOIRE INTERFERENCE - TWO RING FAMILIES",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 16.0 - f.text_width(2.5, "CENTER 2 FOLLOWS THE MOUSE"),
            18.0,
            2.5,
            "CENTER 2 FOLLOWS THE MOUSE",
            gui::Color::CYAN,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "SIN(D1 * K) * SIN(D2 * K) WITH A DRIFTING PHASE - MOVE THE MOUSE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
