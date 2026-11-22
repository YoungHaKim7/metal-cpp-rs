//! 074 — Raymarch Sphere: a fragment shader raymarcher with an infinite
//! floor plane and one sphere bobbing above it. The light direction tracks
//! the mouse; shading is Lambert plus a cheap fixed-step shadow ray, and
//! distance fog sinks the far floor into the sky color.

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

float sd_scene(float3 p, float t) {
    float3 c = float3(0.0, 1.6 + 0.35 * sin(t * 1.3), 0.0);
    return min(length(p - c) - 1.1, p.y);
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
    out.color = v[vid].color;
    return out;
}

fragment float4 fragment_main(float4 frag [[position]],
                              constant float4* u [[buffer(0)]]) {
    float top = u[1].x;
    float bot = u[1].y;
    float rh = u[0].y - top - bot;
    float2 rel = frag.xy - float2(u[0].x * 0.5, top + rh * 0.5);
    float2 sc = rel / rh;
    sc.y = -sc.y;
    float t = u[0].z;
    float3 ro = float3(0.0, 2.1, -5.5);
    float3 fw = normalize(float3(0.0, 1.0, 0.0) - ro);
    float3 rt = normalize(cross(fw, float3(0.0, 1.0, 0.0)));
    float3 up = cross(rt, fw);
    float3 rd = normalize(fw + sc.x * rt * 1.15 + sc.y * up * 1.15);
    // Light direction from the mouse.
    float2 m = u[1].zw / u[0].xy;
    float3 ldir = normalize(float3((m.x - 0.5) * 3.0, 1.4 - m.y * 1.1, -0.8));
    float3 sky = mix(float3(0.10, 0.12, 0.18), float3(0.02, 0.03, 0.06),
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
            albedo = float3(0.85, 0.42, 0.25);
        } else {
            float ch = abs(fmod(floor(p.x * 0.8) + floor(p.z * 0.8), 2.0));
            albedo = mix(float3(0.16, 0.18, 0.22), float3(0.30, 0.33, 0.40), ch);
        }
        col = albedo * (0.22 + 0.95 * lam * sh);
        col = mix(col, sky, smoothstep(14.0, 55.0, dist));
    }
    col += float3(1.0, 0.85, 0.6) * pow(max(dot(rd, ldir), 0.0), 48.0) * 0.7;
    return float4(col, 1.0);
}
"#;

fn main() {
    let app = gui::App::new("074 - Raymarch Sphere", W, H);
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
        // u[1] = (top, bot, mouse x, mouse y) — mouse drives the light.
        f.draw_custom(&pipeline, &[TOP_STRIP, BOT_STRIP, mx, my], &verts);

        f.text(
            16.0,
            14.0,
            3.0,
            "RAYMARCHER - SPHERE OVER AN INFINITE FLOOR",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 16.0 - f.text_width(2.5, "LIGHT FOLLOWS THE MOUSE"),
            18.0,
            2.5,
            "LIGHT FOLLOWS THE MOUSE",
            gui::Color::YELLOW,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "LAMBERT + FIXED-STEP SHADOW + DISTANCE FOG - MOVE THE MOUSE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
