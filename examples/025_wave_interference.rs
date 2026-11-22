//! 025 — Wave Interference: a fullscreen custom fragment shader sums
//! sin(k*r - w*t) ripples from three point sources gliding on fixed
//! lissajous orbits, each pulsing in amplitude; the interference lobes tint
//! the field from deep navy through cyan to white, and each source carries
//! an in-shader glow so its pulse reads even without overlay geometry.
//!
//! The field quad leaves the top and bottom text bands uncovered, because
//! in this runtime every `draw_custom` pass composites over the default
//! pass that paints the bitmap font.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 620.0;

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

// Source i glides on a fixed lissajous orbit around the window center.
float2 source_pos(int i, float t, float2 size) {
    float2 c = size * 0.5;
    if (i == 0) {
        return c + float2(size.x * 0.22 * cos(t * 0.50),
                          size.y * 0.16 * sin(t * 0.70));
    } else if (i == 1) {
        return c + float2(size.x * 0.24 * cos(t * 0.40 + 2.1),
                          -size.y * 0.18 * sin(t * 0.60 + 1.0));
    }
    return c + float2(size.x * 0.20 * cos(-t * 0.60 + 4.0),
                      size.y * 0.20 * sin(t * 0.45 + 3.0));
}

// Each source pulses in amplitude.
float source_amp(int i, float t) {
    return 0.75 + 0.25 * sin(t * 2.0 + float(i) * 2.1);
}

fragment float4 fragment_main(float4 frag [[position]],
                              constant float4* u [[buffer(0)]]) {
    float2 size = u[0].xy;
    float t = u[0].z;
    float v = 0.0;
    float glow = 0.0;
    for (int i = 0; i < 3; i++) {
        float2 d = frag.xy - source_pos(i, t, size);
        float r = length(d);
        v += source_amp(i, t) * sin(0.045 * r - 4.5 * t);
        glow += source_amp(i, t) * exp(-r * r / 900.0);
    }
    float s = clamp(v * 0.26 + 0.5, 0.0, 1.0);
    float3 col = mix(float3(0.012, 0.02, 0.055), float3(0.05, 0.24, 0.52), s);
    col = mix(col, float3(0.18, 0.72, 0.85), smoothstep(0.55, 0.95, s));
    col = mix(col, float3(1.0, 0.96, 0.82), smoothstep(0.88, 1.0, s));
    col += float3(0.3, 0.06, 0.1) * clamp(-v * 0.2, 0.0, 1.0) * (1.0 - s);
    col += float3(1.0, 0.85, 0.6) * glow * 0.55;
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("025 - Wave Interference", W, H);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.04));

        // Interference field: one quad covering everything but the text bands.
        let top = 62.0;
        let bottom = 46.0;
        let y1 = f.height - bottom;
        let verts = [
            gui::Vertex::new(0.0, top, gui::Color::BLACK),
            gui::Vertex::new(f.width, top, gui::Color::BLACK),
            gui::Vertex::new(0.0, y1, gui::Color::BLACK),
            gui::Vertex::new(f.width, top, gui::Color::BLACK),
            gui::Vertex::new(f.width, y1, gui::Color::BLACK),
            gui::Vertex::new(0.0, y1, gui::Color::BLACK),
        ];
        f.draw_custom(&pipeline, &[], &verts);

        f.text(
            24.0,
            12.0,
            4.0,
            "025 - WAVE INTERFERENCE",
            gui::Color::WHITE,
        );
        f.text(
            24.0,
            42.0,
            2.5,
            "BRIGHTNESS = SUM OF SIN(K*R - W*T) FROM 3 SOURCES, CLAMPED",
            gui::Color::GRAY,
        );
        f.text(
            24.0,
            f.height - 30.0,
            2.0,
            "SOURCES GLIDE ON FIXED ORBITS AND PULSE - CUSTOM FRAGMENT SHADER - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
