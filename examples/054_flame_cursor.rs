//! 054 — Flame Cursor: the cursor breathes fire. Particles are emitted at
//! the pointer (the window center when the mouse rests), rise buoyantly,
//! shrink with age and burn white → yellow → orange → red → dark smoke by
//! remaining life. Emission flickers gently with layered noise; 800 max.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const CAP: usize = 800;
const BUOYANCY: f32 = 330.0;
const IDLE_T: f32 = 1.0;

struct Ember {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    life: f32,
    seed: f32,
}

struct Flame {
    embers: Vec<Ember>,
    last: (f32, f32),
    last_move: f32,
    rng: gui::Rng,
}

/// White -> yellow -> orange -> red -> dark smoke as `u` (remaining life)
/// runs from 1 down to 0.
fn flame_color(u: f32) -> gui::Color {
    const STOPS: [(f32, gui::Color); 5] = [
        (1.00, gui::Color::rgb(1.00, 1.00, 1.00)),
        (0.70, gui::Color::rgb(1.00, 0.88, 0.30)),
        (0.45, gui::Color::rgb(1.00, 0.55, 0.15)),
        (0.22, gui::Color::rgb(0.85, 0.18, 0.08)),
        (0.00, gui::Color::rgb(0.16, 0.15, 0.18)),
    ];
    if u >= 1.0 {
        return STOPS[0].1;
    }
    for w in STOPS.windows(2) {
        if u >= w[1].0 {
            let t = (u - w[1].0) / (w[0].0 - w[1].0);
            return gui::Color::lerp(w[1].1, w[0].1, t);
        }
    }
    STOPS[4].1
}

/// Cheap gaussian: the sum of four uniforms is bell-shaped enough.
fn gauss(rng: &mut gui::Rng) -> f32 {
    (rng.f32() + rng.f32() + rng.f32() + rng.f32() - 2.0) * 0.5
}

fn main() {
    let mut flame = Flame {
        embers: Vec::with_capacity(CAP),
        last: (-1000.0, -1000.0),
        last_move: -100.0,
        rng: gui::Rng::from_entropy(),
    };
    let app = gui::App::new("054 - Flame Cursor", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.02, 0.04));

        let mp = f.mouse();
        if flame.last.0 < -900.0 {
            flame.last = mp;
        }
        let moved = (mp.0 - flame.last.0).abs() + (mp.1 - flame.last.1).abs() > 1.5;
        if moved {
            flame.last = mp;
            flame.last_move = f.time;
        }
        let idle = f.time - flame.last_move > IDLE_T;
        let (ex, ey) = if idle {
            (f.width * 0.5, f.height * 0.62)
        } else {
            mp
        };

        // Gentle flicker: two incommensurate sines gate the emission.
        let n = (f.time * 11.0).sin() * 0.6 + (f.time * 17.3 + 1.7).sin() * 0.4;
        let count = 2 + if n > 0.25 { 1 } else { 0 } + if n > 0.75 { 1 } else { 0 };
        for _ in 0..count {
            if flame.embers.len() >= CAP {
                break;
            }
            let jx = gauss(&mut flame.rng) * 3.5;
            let jy = gauss(&mut flame.rng) * 3.5;
            flame.embers.push(Ember {
                x: ex + jx + n * 3.0,
                y: ey + jy,
                vx: (flame.rng.f32() - 0.5) * 55.0 + n * 22.0,
                vy: -(80.0 + flame.rng.f32() * 120.0),
                age: 0.0,
                life: 0.75 + flame.rng.f32() * 0.55,
                seed: flame.rng.f32() * std::f32::consts::TAU,
            });
        }

        flame.embers.retain_mut(|e| {
            e.age += f.dt;
            e.vy -= BUOYANCY * f.dt;
            e.vx *= (-1.6 * f.dt).exp();
            e.vx += (e.age * 9.0 + e.seed).sin() * 60.0 * f.dt;
            e.x += e.vx * f.dt;
            e.y += e.vy * f.dt;
            e.age < e.life
        });

        for e in &flame.embers {
            let t = (e.age / e.life).clamp(0.0, 1.0);
            let u = 1.0 - t;
            let c = flame_color(u);
            let r = 1.6 + 6.2 * u;
            let a = (0.2 + 0.8 * u).min(1.0);
            let segs = if r > 4.0 { 10 } else { 6 };
            f.circle_segments(e.x, e.y, r, segs, gui::Color::rgba(c.r, c.g, c.b, a));
        }

        if !idle {
            let (mx, my) = f.mouse();
            f.ring(mx, my, 14.0, 12.5, gui::Color::rgba(1.0, 0.7, 0.4, 0.4));
        } else {
            // Below the title's band (y 20..48), not across it.
            f.text_center(f.width * 0.5, 54.0, 2.5, "HOME HEARTH", gui::Color::GRAY);
        }

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "054 - FLAME CURSOR",
            gui::Color::WHITE,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "FIRE FOLLOWS THE MOUSE - IDLE BURNS AT CENTER - WHITE TO SMOKE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
