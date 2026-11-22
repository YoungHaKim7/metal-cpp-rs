//! 096 — Mandelbrot Explorer: a fullscreen-style custom fragment shader
//! rendering the Mandelbrot set with 300 iterations and smooth (continuous)
//! escape-time coloring. Click zooms 2x centered on the click point with a
//! short eased animation; `Z` resets to the default view. The HUD shows the
//! current scale and center; the shader covers the plot area, leaving the
//! HUD strips to the default pipeline.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 880.0;
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

// u[1] = (center_x, center_y, scale, 0) — scale = half-height in complex units.
// u[2] = (rect_x, rect_y, rect_w, rect_h) — plot area in pixels.
fragment float4 fragment_main(float4 frag [[position]],
                              constant float4* u [[buffer(0)]]) {
    float2 q = (frag.xy - u[2].xy) / u[2].zw;
    float aspect = u[2].z / u[2].w;
    float2 c = u[1].xy + float2((q.x - 0.5) * 2.0 * u[1].z * aspect,
                                (0.5 - q.y) * 2.0 * u[1].z);
    float2 z = float2(0.0, 0.0);
    float esc = 300.0;
    for (int i = 0; i < 300; ++i) {
        z = float2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        float m = dot(z, z);
        if (m > 256.0) {
            esc = float(i) + 1.0 - log2(max(0.5 * log2(m), 1e-8));
            break;
        }
    }
    if (esc >= 300.0) {
        return float4(0.015, 0.015, 0.05, 1.0);
    }
    float t = esc / 300.0;
    float3 col = 0.5 + 0.5 * cos(6.28318 * (t * 3.0 + float3(0.0, 0.10, 0.20)) + float3(0.0, 0.6, 1.0));
    // Ease deep-boundary pixels toward bright white-yellow.
    col = mix(col, float3(1.0, 0.95, 0.7), pow(1.0 - t, 6.0) * 0.6);
    return float4(col, 1.0);
}
"#;

/// Default view: a pleasant frame on the set's west notch.
const HOME: (f32, f32, f32) = (-0.6, 0.0, 1.35);
const ANIM_TIME: f32 = 0.3;

#[derive(Copy, Clone)]
struct View {
    cx: f32,
    cy: f32,
    scale: f32,
}

impl View {
    fn lerp_to(a: View, b: View, t: f32) -> View {
        View {
            cx: a.cx + (b.cx - a.cx) * t,
            cy: a.cy + (b.cy - a.cy) * t,
            // Zooms interpolate in log space so the flight feels constant-speed.
            scale: a.scale * (b.scale / a.scale).powf(t),
        }
    }
}

fn fmt_scale(v: f32) -> String {
    if v >= 0.001 {
        format!("{:.3}", v)
    } else {
        format!("{:.2e}", v).to_uppercase()
    }
}

fn fmt_coord(v: f32) -> String {
    let sign = if v < 0.0 { "-" } else { "+" };
    format!("{sign}{:.5}", v.abs())
}

fn main() {
    let app = gui::App::new("096 - Mandelbrot Explorer", W, H);
    let pipeline = gui::make_pipeline(app.device(), SHADER);
    let mut cur = View {
        cx: HOME.0,
        cy: HOME.1,
        scale: HOME.2,
    };
    let mut anim: Option<(View, View, f32)> = None; // (from, to, elapsed)
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.06, 0.06, 0.09));
        let s = f.scale;

        // ------------------------------------------------------------- layout
        let hud_h = 46.0 * s;
        let footer_h = 34.0 * s;
        let rect = (
            10.0 * s,
            hud_h,
            f.width - 20.0 * s,
            f.height - hud_h - footer_h - 10.0 * s,
        );
        // Shader quad is inset so the default-pipeline border stays visible
        // (custom draws always come after the default pass).
        let srect = (
            rect.0 + 3.0 * s,
            rect.1 + 3.0 * s,
            rect.2 - 6.0 * s,
            rect.3 - 6.0 * s,
        );

        // ------------------------------------------------------------- animation
        if let Some((from, to, t)) = anim.as_mut() {
            *t += f.dt;
            let k = (*t / ANIM_TIME).clamp(0.0, 1.0);
            let e = k * k * (3.0 - 2.0 * k);
            cur = View::lerp_to(*from, *to, e);
            if k >= 1.0 {
                anim = None;
            }
        }

        // ------------------------------------------------------------- input
        let (mx, my) = f.mouse();
        let in_rect =
            mx >= rect.0 && mx <= rect.0 + rect.2 && my >= rect.1 && my <= rect.1 + rect.3;
        if f.input.mouse_pressed() && in_rect {
            // Pixel -> complex plane under the *current* animated view, using
            // the same mapping the fragment shader applies to `srect`.
            let qx = (mx - srect.0) / srect.2;
            let qy = (my - srect.1) / srect.3;
            let px = cur.cx + (qx - 0.5) * 2.0 * cur.scale * (srect.2 / srect.3);
            let py = cur.cy + (0.5 - qy) * 2.0 * cur.scale;
            let from = cur;
            let to = View {
                cx: px + (cur.cx - px) * 0.5,
                cy: py + (cur.cy - py) * 0.5,
                scale: cur.scale * 0.5,
            };
            anim = Some((from, to, 0.0));
        }
        if f.input.key_pressed(gui::key::Z) {
            anim = Some((
                cur,
                View {
                    cx: HOME.0,
                    cy: HOME.1,
                    scale: HOME.2,
                },
                0.0,
            ));
        }

        // ------------------------------------------------------------- shader pass
        let verts = [
            gui::Vertex::new(srect.0, srect.1, gui::Color::BLACK),
            gui::Vertex::new(srect.0 + srect.2, srect.1, gui::Color::BLACK),
            gui::Vertex::new(srect.0, srect.1 + srect.3, gui::Color::BLACK),
            gui::Vertex::new(srect.0 + srect.2, srect.1, gui::Color::BLACK),
            gui::Vertex::new(srect.0 + srect.2, srect.1 + srect.3, gui::Color::BLACK),
            gui::Vertex::new(srect.0, srect.1 + srect.3, gui::Color::BLACK),
        ];
        let uniforms = [
            cur.cx, cur.cy, cur.scale, 0.0, srect.0, srect.1, srect.2, srect.3,
        ];
        f.draw_custom(&pipeline, &uniforms, &verts);
        f.frame_rect(
            rect.0,
            rect.1,
            rect.2,
            rect.3,
            2.0 * s,
            gui::Color::rgb(0.30, 0.32, 0.42),
        );

        // ------------------------------------------------------------- HUD
        f.text(
            16.0 * s,
            14.0 * s,
            4.0 * s,
            "MANDELBROT EXPLORER",
            gui::Color::WHITE,
        );
        let scale_text = format!("SCALE {}", fmt_scale(cur.scale));
        f.text(
            f.width - 16.0 * s - f.text_width(3.0 * s, &scale_text),
            12.0 * s,
            3.0 * s,
            &scale_text,
            gui::Color::YELLOW,
        );
        let center_text = format!("CENTER {} {}", fmt_coord(cur.cx), fmt_coord(cur.cy));
        f.text(
            f.width - 16.0 * s - f.text_width(2.0 * s, &center_text),
            30.0 * s,
            2.0 * s,
            &center_text,
            gui::Color::GRAY,
        );

        f.text(
            16.0 * s,
            f.height - footer_h * 0.55,
            2.0 * s,
            "CLICK ZOOM 2X - Z RESET - 300 ITERS, SMOOTH COLOR",
            gui::Color::GRAY,
        );
    });
}
