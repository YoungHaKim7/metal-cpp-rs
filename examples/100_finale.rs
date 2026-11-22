//! 100 — Finale: the grand closer. METAL-CPP-RS headlines the window with
//! 100 GUI EXAMPLES beneath it, ringed by ~300 particles riding Lissajous
//! orbits with cycling hues, while periodic fireworks bursts (60 gravity-
//! driven sparks each, fading out) pop behind the text. SPACE toggles the
//! hue-spin speed between slow and fast.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 640.0;
const PARTICLES: usize = 300;
const SPARKS_PER_BURST: usize = 60;
const BURST_EVERY: f32 = 1.5;
const MAX_SPARKS: usize = 960;

/// Background color sparks fade into (alpha blending is off in this runtime).
const BG: gui::Color = gui::Color::rgb(0.05, 0.05, 0.08);

struct Particle {
    ax: f32,
    ay: f32,
    fx: f32,
    fy: f32,
    ph: f32,
    spd: f32,
    size: f32,
    seed: f32,
}

struct Spark {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    hue: f32,
}

struct Finale {
    particles: Vec<Particle>,
    sparks: Vec<Spark>,
    rng: gui::Rng,
    fast: bool,
    next_burst: f32,
}

impl Finale {
    fn new() -> Finale {
        let mut rng = gui::Rng::from_entropy();
        let golden = 0.618_034_f32;
        let mut particles = Vec::with_capacity(PARTICLES);
        for i in 0..PARTICLES {
            particles.push(Particle {
                ax: 120.0 + rng.f32() * 280.0,
                ay: 70.0 + rng.f32() * 160.0,
                fx: (1.0 + rng.range(1, 4) as f32) * 0.55,
                fy: (1.0 + rng.range(1, 4) as f32) * 0.55,
                ph: rng.f32() * std::f32::consts::TAU,
                spd: 0.5 + rng.f32() * 1.2,
                size: 1.5 + rng.f32() * 2.5,
                seed: i as f32 * golden,
            });
        }
        Finale {
            particles,
            sparks: Vec::new(),
            rng,
            fast: false,
            next_burst: 0.6,
        }
    }

    fn spawn_burst(&mut self) {
        let cx = 120.0 + self.rng.f32() * (W as f32 - 240.0);
        let cy = 70.0 + self.rng.f32() * (H as f32 * 0.42);
        let hue = self.rng.f32();
        for _ in 0..SPARKS_PER_BURST {
            let a = self.rng.f32() * std::f32::consts::TAU;
            let speed = 50.0 + self.rng.f32() * 190.0;
            let life = 1.1 + self.rng.f32() * 0.9;
            self.sparks.push(Spark {
                x: cx,
                y: cy,
                vx: a.cos() * speed,
                vy: a.sin() * speed,
                life,
                max_life: life,
                hue: hue + self.rng.f32() * 0.14,
            });
        }
        if self.sparks.len() > MAX_SPARKS {
            let excess = self.sparks.len() - MAX_SPARKS;
            self.sparks.drain(..excess);
        }
    }
}

fn main() {
    let mut state = Finale::new();
    let app = gui::App::new("100 - Finale", W, H);
    app.run(move |f| {
        f.set_clear(BG);
        let s = f.scale;
        let hue_speed = if state.fast { 0.30 } else { 0.05 };

        // ------------------------------------------------------------- input
        if f.input.key_pressed(gui::key::SPACE) {
            state.fast = !state.fast;
        }

        // ------------------------------------------------------------- update
        state.next_burst -= f.dt;
        if state.next_burst <= 0.0 {
            state.next_burst = BURST_EVERY;
            state.spawn_burst();
        }
        for spark in &mut state.sparks {
            spark.x += spark.vx * f.dt;
            spark.y += spark.vy * f.dt;
            spark.vy += 210.0 * f.dt; // gravity, points/s^2
            spark.life -= f.dt;
        }
        state.sparks.retain(|spark| spark.life > 0.0);

        // ------------------------------------------------------------- fireworks
        for spark in &state.sparks {
            let k = (spark.life / spark.max_life).clamp(0.0, 1.0);
            let hot = gui::Color::hsv(spark.hue % 1.0, 0.55, 1.0);
            let color = gui::Color::lerp(BG, hot, k);
            let size = (1.5 + 2.5 * k) * s;
            f.rect(spark.x * s, spark.y * s, size, size, color);
        }

        // ------------------------------------------------------------- particles
        let cx = f.width * 0.5;
        let cy = f.height * 0.42;
        for p in &state.particles {
            let wt = f.time * 0.6 * p.spd;
            let px = cx + p.ax * s * (p.fx * wt + p.ph).sin();
            let py = cy + p.ay * s * (p.fy * wt + p.ph * 1.7).sin();
            let hue = (f.time * hue_speed + p.seed) % 1.0;
            let color = gui::Color::hsv(hue, 0.72, 1.0);
            let size = p.size * s;
            f.rect(px, py, size, size, color);
        }

        // ------------------------------------------------------------- title
        let title = "METAL-CPP-RS";
        let title_size = 12.0 * s;
        let pulse = 0.75 + 0.25 * (f.time * 2.0).sin();
        f.text_center(
            cx,
            cy - title_size * 3.5,
            title_size,
            title,
            gui::Color::WHITE,
        );
        // Accent rules above and below the title.
        let rule_w = f.text_width(title_size, title) * 0.9 * pulse;
        f.line(
            cx - rule_w * 0.5,
            cy - title_size * 5.6,
            cx + rule_w * 0.5,
            cy - title_size * 5.6,
            3.0 * s,
            gui::Color::hsv((f.time * hue_speed * 2.0) % 1.0, 0.6, 1.0),
        );
        let sub = "100 GUI EXAMPLES";
        let sub_size = 5.0 * s;
        f.text_center(cx, cy + title_size * 4.6, sub_size, sub, gui::Color::YELLOW);
        let tag = "RAW OBJC ABI - ZERO DEPENDENCIES - ONE METAL DEVICE";
        f.text_center(
            cx,
            cy + title_size * 4.6 + sub_size * 9.0 + 10.0 * s,
            2.5 * s,
            tag,
            gui::Color::GRAY,
        );

        // ------------------------------------------------------------- HUD
        let mode = if state.fast { "FAST" } else { "SLOW" };
        let hud = format!("HUE SPIN: {mode}   SPARKS {}", state.sparks.len());
        f.text(
            30.0 * s,
            24.0 * s,
            2.5 * s,
            &hud,
            if state.fast {
                gui::Color::ORANGE
            } else {
                gui::Color::GRAY
            },
        );
        let counter = "EXAMPLE 100 / 100";
        f.text(
            f.width - 30.0 * s - f.text_width(2.5 * s, &counter),
            24.0 * s,
            2.5 * s,
            &counter,
            gui::Color::rgb(0.5, 0.7, 1.0),
        );

        f.text_center(
            f.width * 0.5,
            f.height - 40.0 * s,
            2.0 * s,
            "SPACE TOGGLES HUE-SPIN SPEED - FIREWORKS EVERY 1.5S - ESC OR RED BUTTON TO QUIT",
            gui::Color::GRAY,
        );
    });
}
