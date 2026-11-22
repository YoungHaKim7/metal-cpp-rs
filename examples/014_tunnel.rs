//! 014 — Tunnel: a fullscreen custom fragment shader — the classic polar
//! tunnel of log-radius stripes and angular spokes scrolling with time. The
//! tunnel center drifts slightly toward the (smoothed) mouse. Caption strips
//! stay dark for the bitmap text.

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
        return float4(0.02, 0.02, 0.05, 1.0);
    }
    float t = u[0].z;
    float2 base = size * 0.5;
    float2 center = base + (u[1].xy - base) * 0.18;
    float2 p = frag.xy - center;
    float r = max(length(p), 1.5);
    float a = atan2(p.y, p.x);
    float depth = log2(r) * 1.1 - t * 1.5;
    float band = fract(depth);
    float ring = smoothstep(0.0, 0.55, band);
    float spokes = 0.5 + 0.5 * cos(a * 10.0 + t * 0.7);
    float3 col = mix(float3(0.15, 0.30, 0.85), float3(1.00, 0.55, 0.18), ring);
    col *= 0.30 + 0.70 * spokes;
    col *= smoothstep(0.0, 50.0, r);
    col *= 1.0 / (1.0 + r * 0.0012);
    return float4(col, 1.0);
}
"#;

/// One-axis exponential smoother so the tunnel center drifts lazily.
struct Smooth {
    pos: Option<(f32, f32)>,
}

impl Smooth {
    fn new() -> Smooth {
        Smooth { pos: None }
    }

    fn step(&mut self, target: (f32, f32), dt: f32) -> (f32, f32) {
        let k = (dt * 6.0).min(1.0);
        let next = match self.pos {
            None => target,
            Some(p) => (p.0 + (target.0 - p.0) * k, p.1 + (target.1 - p.1) * k),
        };
        self.pos = Some(next);
        next
    }
}

fn main() {
    let app = gui::App::new("014 - Tunnel", 640.0, 440.0);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    let mut mouse = Smooth::new();
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.05));

        let (sx, sy) = mouse.step(f.mouse(), f.dt);
        let (w, h) = (f.width, f.height);
        let verts = [
            gui::Vertex::new(0.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(w * 2.0, 0.0, gui::Color::BLACK),
            gui::Vertex::new(0.0, h * 2.0, gui::Color::BLACK),
        ];
        // u[1] = (mouse_x, mouse_y) in pixels.
        f.draw_custom(&pipeline, &[sx, sy], &verts);

        f.text_center(f.width * 0.5, 14.0, 4.0, "TUNNEL", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "LOG-RADIUS STRIPES + ANGULAR SPOKES - CENTER DRIFTS WITH THE MOUSE",
            gui::Color::GRAY,
        );
    });
}
