//! 075 — Raymarch Torus: a fragment shader raymarcher with a torus that
//! rotates over time by spinning the SDF sample point around Y (with a
//! gentle X wobble), floating over a dark floor. Lambert shading plus a
//! cheap fixed-step shadow; the camera never moves.

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
    float3 q = rot_x(rot_y(p, t * 0.7), 0.35 * sin(t * 0.9));
    float torus = length(float2(length(q.xz) - 1.6, q.y - 1.15)) - 0.55;
    return min(torus, p.y);
}

float3 sd_normal(float3 p, float t) {
    float e = 0.0025;
    return normalize(float3(
        sd_scene(p + float3(e, 0.0, 0.0), t) - sd_scene(p - float3(e, 0.0, 0.0), t),
        sd_scene(p + float3(0.0, e, 0.0), t) - sd_scene(p - float3(0.0, e, 0.0), t),
        sd_scene(p + float3(0.0, 0.0, e), t) - sd_scene(p - float3(0.0, 0.0, e), t)));
}

float cheap_shadow(float3 p, float3 l, float t) {
    float3 q = p + l * 0.06;
    for (int i = 0; i < 24; i++) {
        if (sd_scene(q, t) < 0.02) {
            return 0.35;
        }
        q += l * 0.4;
        if (q.y > 14.0 || q.y < -1.0) {
            break;
        }
    }
    return 1.0;
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
    float3 ro = float3(0.0, 2.3, -5.2);
    float3 fw = normalize(float3(0.0, 0.9, 0.0) - ro);
    float3 rt = normalize(cross(fw, float3(0.0, 1.0, 0.0)));
    float3 up = cross(rt, fw);
    float3 rd = normalize(fw + sc.x * rt * 1.1 + sc.y * up * 1.1);
    float3 ldir = normalize(float3(0.55, 0.9, -0.5));
    float3 sky = mix(float3(0.09, 0.11, 0.16), float3(0.02, 0.03, 0.06),
                     clamp(rd.y * 1.4, 0.0, 1.0));
    float dist = 0.0;
    bool hit = false;
    for (int i = 0; i < 110; i++) {
        float3 p = ro + rd * dist;
        float d = sd_scene(p, t);
        if (d < 0.0025) {
            hit = true;
            break;
        }
        dist += d;
        if (dist > 60.0) {
            break;
        }
    }
    float3 col = sky;
    if (hit) {
        float3 p = ro + rd * dist;
        float3 n = sd_normal(p, t);
        float lam = max(dot(n, ldir), 0.0);
        float sh = cheap_shadow(p + n * 0.04, ldir, t);
        float3 albedo;
        if (p.y > 0.02) {
            albedo = float3(0.25, 0.75, 0.65);
        } else {
            float ch = abs(fmod(floor(p.x * 0.8) + floor(p.z * 0.8), 2.0));
            albedo = mix(float3(0.15, 0.17, 0.21), float3(0.28, 0.31, 0.38), ch);
        }
        col = albedo * (0.22 + 0.95 * lam * sh);
        col = mix(col, sky, smoothstep(14.0, 55.0, dist));
    }
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("075 - Raymarch Torus", W, H);
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
            "RAYMARCHER - ROTATING TORUS",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 16.0 - f.text_width(2.5, "CAMERA STATIC"),
            18.0,
            2.5,
            "CAMERA STATIC",
            gui::Color::CYAN,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "SDF POINT SPUN AROUND Y - LAMBERT + FIXED-STEP SHADOW - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
