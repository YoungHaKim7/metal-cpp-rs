//! 078 — FBM Terrain: five parallax mountain silhouette bands, each an
//! 800-column ridged-fbm ridge line (4 octaves) recomputed on the CPU every
//! frame. Far layers crawl slowly and drown in warm depth fog while the
//! near ridge races by; a low sun burns through the haze.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 640.0;
const TOP_STRIP: f32 = 48.0;
const BOT_STRIP: f32 = 52.0;
const LAYERS: usize = 5;
const COLUMNS: usize = 800;
const OCTAVES: usize = 4;

/// Splitmix-style integer hash -> uniform f32 in 0..1.
fn hash(n: i64) -> f32 {
    let mut x = n as u64;
    x = x.wrapping_mul(0x9E3779B97F4A7C15);
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58476D1CE4E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D049BB133111EB);
    x ^= x >> 31;
    ((x >> 40) as f32) / (1u64 << 24) as f32
}

/// Smooth 1D value noise on the integer lattice.
fn vnoise(x: f32, seed: i64) -> f32 {
    let xi = x.floor();
    let t = x - xi;
    let u = t * t * (3.0 - 2.0 * t);
    let i0 = xi as i64;
    let a = hash(i0.wrapping_add(seed));
    let b = hash(i0.wrapping_add(seed) + 1);
    a + (b - a) * u
}

/// Ridged fbm: 1 - |2n - 1| accumulated over 4 octaves, normalized to 0..1.
fn ridge_fbm(x: f32, seed: i64) -> f32 {
    let mut v = 0.0;
    let mut amp = 1.0;
    let mut freq = 1.0;
    let mut norm = 0.0;
    for o in 0..OCTAVES {
        let n = 1.0 - (2.0 * vnoise(x * freq, seed + o as i64 * 257) - 1.0).abs();
        v += amp * n;
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    v / norm
}

fn main() {
    let app = gui::App::new("078 - FBM Terrain", W, H);
    let mut columns = vec![0.0f32; COLUMNS];
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.04, 0.07));

        let top = TOP_STRIP;
        let bot = f.height - BOT_STRIP;
        let rh = bot - top;

        // Sky: banded gradient from cold zenith to a warm horizon.
        for i in 0..10 {
            let y0 = top + rh * (i as f32 / 10.0);
            let y1 = top + rh * ((i + 1) as f32 / 10.0);
            let t = i as f32 / 9.0;
            let c = gui::Color::lerp(
                gui::Color::rgb(0.04, 0.06, 0.13),
                gui::Color::rgb(0.42, 0.25, 0.22),
                t,
            );
            f.rect(0.0, y0, f.width, y1 - y0 + 1.0, c);
        }

        // Low sun with a two-step glow, painted behind the ridges.
        let sun_x = f.width * 0.68;
        let sun_y = top + rh * 0.30;
        f.circle(sun_x, sun_y, 52.0, gui::Color::rgb(0.35, 0.18, 0.12));
        f.circle(sun_x, sun_y, 40.0, gui::Color::rgb(0.75, 0.32, 0.14));
        f.circle(sun_x, sun_y, 30.0, gui::Color::rgb(1.0, 0.72, 0.35));

        // Five ridge layers, painted far -> near so nearer bands overlap.
        let horizon = top + rh * 0.62;
        let silhouette = gui::Color::rgb(0.09, 0.09, 0.13);
        let haze = gui::Color::rgb(0.52, 0.32, 0.26);
        let colw = f.width / COLUMNS as f32;
        for layer in 0..LAYERS {
            let depth = layer as f32 / (LAYERS - 1) as f32;
            let speed = 6.0 + 26.0 * depth;
            let scroll = f.time * speed;
            let amp = 34.0 + 76.0 * depth;
            let base = horizon - 60.0 + 50.0 * depth;
            let freq = 0.011 + 0.004 * (1.0 - depth);
            let seed = layer as i64 * 7919 + 13;
            for c in 0..COLUMNS {
                let ridge = ridge_fbm((c as f32 + scroll) * freq, seed);
                columns[c] = base - ridge * amp;
            }
            let ink = gui::Color::lerp(silhouette, haze, 0.85 * (1.0 - depth));
            for c in 0..COLUMNS {
                f.rect(
                    c as f32 * colw,
                    columns[c],
                    colw + 1.0,
                    bot - columns[c],
                    ink,
                );
            }
        }

        // Chrome.
        f.text(
            16.0,
            14.0,
            3.0,
            "RIDGED FBM TERRAIN - 5 PARALLAX BANDS",
            gui::Color::WHITE,
        );
        let theta = format!("T = {:.1}S", f.time);
        f.text(
            f.width - 16.0 - f.text_width(2.5, &theta),
            18.0,
            2.5,
            &theta,
            gui::Color::CYAN,
        );
        f.text(
            16.0,
            f.height - 38.0,
            2.0,
            "800 COLUMNS X 4 OCTAVES PER LAYER - CPU NOISE EVERY FRAME - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
