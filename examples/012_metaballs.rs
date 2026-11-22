//! 012 — Metaballs: a fullscreen custom fragment shader. Four blobs ride
//! lissajous paths while a fifth follows the (smoothed) mouse position; the
//! summed field is thresholded with smoothstep and colored by field
//! strength. Caption strips stay dark for the bitmap text.

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
    float t = u[0].z;
    float m = min(size.x, size.y);
    float2 p = frag.xy;
    float r = m * 0.085;
    float rm = m * 0.075;
    float2 b0 = size * float2(0.5 + 0.33 * sin(t * 0.90), 0.5 + 0.28 * sin(t * 1.30 + 1.0));
    float2 b1 = size * float2(0.5 + 0.36 * cos(t * 0.70 + 2.0), 0.5 + 0.25 * sin(t * 1.10 + 3.0));
    float2 b2 = size * float2(0.5 + 0.30 * sin(t * 1.40 + 4.0), 0.5 + 0.33 * cos(t * 0.60 + 0.5));
    float2 b3 = size * float2(0.5 + 0.26 * sin(t * 0.55 + 5.0), 0.5 + 0.30 * cos(t * 1.60 + 2.5));
    float2 d;
    float field = 0.0;
    d = p - b0;      field += r * r / max(dot(d, d), 1.0);
    d = p - b1;      field += r * r / max(dot(d, d), 1.0);
    d = p - b2;      field += r * r / max(dot(d, d), 1.0);
    d = p - b3;      field += r * r / max(dot(d, d), 1.0);
    d = p - u[1].xy; field += rm * rm / max(dot(d, d), 1.0);
    float3 col = float3(0.04, 0.04, 0.08);
    col = mix(col, float3(0.42, 0.14, 0.72), smoothstep(0.55, 1.05, field));
    col = mix(col, float3(0.95, 0.35, 0.85), smoothstep(1.05, 1.75, field));
    col = mix(col, float3(1.00, 0.95, 0.85), smoothstep(2.10, 3.40, field));
    return float4(col, 1.0);
}
"#;

/// One-axis exponential smoother so the mouse blob glides instead of jumping.
struct Smooth {
    pos: Option<(f32, f32)>,
}

impl Smooth {
    fn new() -> Smooth {
        Smooth { pos: None }
    }

    fn step(&mut self, target: (f32, f32), dt: f32) -> (f32, f32) {
        let k = (dt * 8.0).min(1.0);
        let next = match self.pos {
            None => target,
            Some(p) => (p.0 + (target.0 - p.0) * k, p.1 + (target.1 - p.1) * k),
        };
        self.pos = Some(next);
        next
    }
}

fn main() {
    let app = gui::App::new("012 - Metaballs", 640.0, 440.0);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    let mut mouse = Smooth::new();
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.04, 0.04, 0.08));

        let (sx, sy) = mouse.step(f.mouse(), f.dt);
        let (w, h) = (f.width, f.height);
        let verts = [
            gui::Vertex::new(0.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(w * 2.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(0.0, h * 2.0, gui::Color::BLACK),
        ];
        // u[1] = (mouse_x, mouse_y) in pixels.
        f.draw_custom(&pipeline, &[sx, sy], &verts);

        f.text_center(f.width * 0.5, 14.0, 4.0, "METABALLS", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "FOUR LISSAJOUS BLOBS + ONE ON THE MOUSE - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
