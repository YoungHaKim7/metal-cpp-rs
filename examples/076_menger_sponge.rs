//! 076 — Menger Sponge: a fragment shader raymarcher cutting a four-level
//! Menger sponge out of a box, rotating slowly by spinning the SDF point.
//! Lambert lighting with iteration-count ambient occlusion — pixels whose
//! march took many steps sit deep in the crevices and darken.

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

float sd_box(float3 p, float3 s) {
    float3 d = abs(p) - s;
    return min(max(d.x, max(d.y, d.z)), 0.0) + length(max(d, 0.0));
}

float sd_menger(float3 p) {
    float d = sd_box(p, float3(1.0));
    float s = 1.0;
    for (int m = 0; m < 4; m++) {
        float3 a = p * s;
        a = a - 2.0 * floor(a * 0.5);
        a = a - 1.0;
        s *= 3.0;
        float3 r = abs(1.0 - 3.0 * abs(a));
        float da = max(r.x, r.y);
        float db = max(r.y, r.z);
        float dc = max(r.z, r.x);
        float c = (min(da, min(db, dc)) - 1.0) / s;
        d = max(d, c);
    }
    return d;
}

float3 rot_y(float3 p, float a) {
    float c = cos(a);
    float s = sin(a);
    return float3(c * p.x + s * p.z, p.y, -s * p.x + c * p.z);
}

float3 rot_x(float3 p, float a) {
    float c = cos(a);
    float s = sin(a);
    return float3(p.x, c * p.y - s * p.z, s * p.y + c * p.z);
}

float sd_scene(float3 p, float t) {
    float3 q = rot_x(rot_y(p, t * 0.35), 0.25 * sin(t * 0.5));
    return sd_menger(q * 1.3) / 1.3;
}

float3 sd_normal(float3 p, float t) {
    float e = 0.002;
    return normalize(float3(
        sd_scene(p + float3(e, 0.0, 0.0), t) - sd_scene(p - float3(e, 0.0, 0.0), t),
        sd_scene(p + float3(0.0, e, 0.0), t) - sd_scene(p - float3(0.0, e, 0.0), t),
        sd_scene(p + float3(0.0, 0.0, e), t) - sd_scene(p - float3(0.0, 0.0, e), t)));
}

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
    float2 sc = rel / rh;
    sc.y = -sc.y;
    float t = u[0].z;
    float3 ro = float3(0.0, 1.7, -4.0);
    float3 fw = normalize(-ro);
    float3 rt = normalize(cross(fw, float3(0.0, 1.0, 0.0)));
    float3 up = cross(rt, fw);
    float3 rd = normalize(fw + sc.x * rt * 1.0 + sc.y * up * 1.0);
    float3 ldir = normalize(float3(0.55, 0.85, -0.5));
    float3 bg = float3(0.02, 0.02, 0.05);
    float dist = 0.0;
    float steps = 0.0;
    bool hit = false;
    for (int i = 0; i < 140; i++) {
        steps = float(i);
        float3 p = ro + rd * dist;
        float d = sd_scene(p, t);
        if (d < 0.002) {
            hit = true;
            break;
        }
        dist += d * 0.85;
        if (dist > 24.0) {
            break;
        }
    }
    float3 col = bg;
    if (hit) {
        float3 p = ro + rd * dist;
        float3 n = sd_normal(p, t);
        float lam = max(dot(n, ldir), 0.0);
        // Iteration-count AO: long marches hide deep in the crevices.
        float ao = 1.0 - clamp(steps / 110.0, 0.0, 1.0) * 0.7;
        float3 albedo = float3(0.62, 0.60, 0.56);
        col = albedo * (0.30 + 0.85 * lam) * ao;
        col = mix(col, bg, smoothstep(9.0, 22.0, dist));
    }
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("076 - Menger Sponge", W, H);
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
            "RAYMARCHER - MENGER SPONGE (4 ITERATIONS)",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 16.0 - f.text_width(2.5, "SLOWLY ROTATING"),
            18.0,
            2.5,
            "SLOWLY ROTATING",
            gui::Color::CYAN,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "LAMBERT + ITERATION-COUNT AMBIENT OCCLUSION - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
