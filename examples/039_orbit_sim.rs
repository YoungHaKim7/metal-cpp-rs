//! 039 — Orbit Sim: a pulsing sun, five planets on slightly elliptical
//! parametric orbits (each with a persistent fading trail ring), and
//! click-to-spawn comets that orbit the sun at Kepler-ish speeds (up to 8
//! live); SPACE pauses the clock.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 680.0;
const TAU: f32 = std::f32::consts::TAU;
const COMET_CAP: usize = 8;
const PLANET_TRAIL: usize = 130;
const COMET_TRAIL: usize = 150;

/// (a_frac, b_scale, speed, phase, size, hue) per planet — a_frac is the
/// orbit's horizontal radius as a fraction of the fit radius, b_scale the
/// vertical stretch that makes the ellipse.
const PLANETS: [(f32, f32, f32, f32, f32, f32); 5] = [
    (0.24, 0.94, 1.7, 0.4, 5.0, 0.06),
    (0.38, 0.9, 1.15, 2.3, 7.0, 0.13),
    (0.53, 1.0, 0.8, 4.4, 6.0, 0.33),
    (0.71, 0.93, 0.55, 1.6, 8.0, 0.55),
    (0.92, 0.97, 0.38, 5.4, 6.5, 0.78),
];

struct Planet {
    a: f32,
    b: f32,
    speed: f32,
    phase: f32,
    size: f32,
    hue: f32,
    trail: Vec<(f32, f32)>,
}

struct Comet {
    r: f32,
    speed: f32,
    phase: f32,
    dir: f32,
    trail: Vec<(f32, f32)>,
}

struct Sim {
    rng: gui::Rng,
    t: f32,
    paused: bool,
    planets: Vec<Planet>,
    comets: Vec<Comet>,
}

impl Sim {
    fn new() -> Sim {
        let planets = PLANETS
            .iter()
            .map(|&(a, b, speed, phase, size, hue)| Planet {
                a,
                b,
                speed,
                phase,
                size,
                hue,
                trail: Vec::new(),
            })
            .collect();
        Sim {
            rng: gui::Rng::from_entropy(),
            t: 0.0,
            paused: false,
            planets,
            comets: Vec::new(),
        }
    }

    fn spawn_comet(&mut self, x: f32, y: f32, cx: f32, cy: f32, r_fit: f32) {
        let (dx, dy) = (x - cx, y - cy);
        let r = (dx * dx + dy * dy).sqrt().clamp(r_fit * 0.16, r_fit * 0.98);
        // Kepler-ish: angular speed falls off with r^(-3/2).
        let speed = 1.5 * (r_fit * 0.35 / r).powf(1.5);
        let dir = if self.rng.f32() < 0.5 { 1.0 } else { -1.0 };
        if self.comets.len() >= COMET_CAP {
            self.comets.remove(0);
        }
        self.comets.push(Comet {
            r,
            speed,
            phase: dy.atan2(dx),
            dir,
            trail: Vec::new(),
        });
    }
}

fn main() {
    let mut sim = Sim::new();
    let app = gui::App::new("039 - Orbit Sim", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.05));

        let (cx, cy) = (f.width * 0.5, f.height * 0.54);
        let r_fit = (f.width.min(f.height) * 0.45).max(80.0);

        if f.input.key_pressed(gui::key::SPACE) {
            sim.paused = !sim.paused;
        }
        if f.input.mouse_pressed() {
            let (mx, my) = f.mouse();
            sim.spawn_comet(mx, my, cx, cy, r_fit);
        }
        if !sim.paused {
            sim.t += f.dt;
        }

        // Faint orbit guides showing each planet's ellipse.
        for pl in &sim.planets {
            let (a, b) = (pl.a * r_fit, pl.b * r_fit);
            let mut prev = (cx + a, cy);
            for i in 1..=72 {
                let ang = i as f32 / 72.0 * TAU;
                let p = (cx + ang.cos() * a, cy + ang.sin() * b);
                f.line(
                    prev.0,
                    prev.1,
                    p.0,
                    p.1,
                    1.0,
                    gui::Color::rgb(0.12, 0.13, 0.2),
                );
                prev = p;
            }
        }

        // Planets with persistent fading trails.
        for pl in &mut sim.planets {
            let (a, b) = (pl.a * r_fit, pl.b * r_fit);
            let ang = pl.phase + sim.t * pl.speed;
            let pos = (cx + ang.cos() * a, cy + ang.sin() * b);
            if !sim.paused {
                pl.trail.push(pos);
                if pl.trail.len() > PLANET_TRAIL {
                    pl.trail.remove(0);
                }
            }
            let color = gui::Color::hsv(pl.hue, 0.75, 1.0);
            let n = pl.trail.len();
            for (i, (tx, ty)) in pl.trail.iter().enumerate() {
                let alpha = 0.5 * (i + 1) as f32 / n as f32;
                f.rect(
                    tx - 1.0,
                    ty - 1.0,
                    2.0,
                    2.0,
                    gui::Color::rgba(color.r, color.g, color.b, alpha),
                );
            }
            f.circle(pos.0, pos.1, pl.size, color);
        }

        // Comets: bright, longer trails, orbiting the sun.
        for cm in &mut sim.comets {
            let ang = cm.phase + cm.dir * sim.t * cm.speed;
            let pos = (cx + ang.cos() * cm.r, cy + ang.sin() * cm.r);
            if !sim.paused {
                cm.trail.push(pos);
                if cm.trail.len() > COMET_TRAIL {
                    cm.trail.remove(0);
                }
            }
            let n = cm.trail.len();
            for (i, (tx, ty)) in cm.trail.iter().enumerate() {
                let alpha = 0.55 * (i + 1) as f32 / n as f32;
                f.rect(
                    tx - 1.0,
                    ty - 1.0,
                    2.0,
                    2.0,
                    gui::Color::rgba(0.6, 0.95, 1.0, alpha),
                );
            }
            f.circle_segments(
                pos.0,
                pos.1,
                9.0,
                16,
                gui::Color::rgba(0.6, 0.95, 1.0, 0.35),
            );
            f.circle(pos.0, pos.1, 5.0, gui::Color::rgb(0.85, 0.98, 1.0));
        }

        // The sun, pulsing gently (frozen while paused).
        let pulse = 1.0 + 0.06 * (sim.t * 2.5).sin();
        let r_sun = r_fit * 0.055 * pulse;
        f.circle_segments(
            cx,
            cy,
            r_sun * 2.6,
            40,
            gui::Color::rgba(1.0, 0.75, 0.3, 0.16),
        );
        f.circle_segments(
            cx,
            cy,
            r_sun * 1.7,
            40,
            gui::Color::rgba(1.0, 0.8, 0.35, 0.3),
        );
        f.circle(cx, cy, r_sun, gui::Color::rgb(1.0, 0.88, 0.55));
        f.circle(cx, cy, r_sun * 0.55, gui::Color::rgb(1.0, 0.98, 0.9));

        f.text(24.0, 20.0, 4.0, "039 - ORBIT SIM", gui::Color::WHITE);
        let label = format!(
            "COMETS = {} / {COMET_CAP}   T = {:.1} S",
            sim.comets.len(),
            sim.t
        );
        f.text(24.0, 54.0, 3.0, &label, gui::Color::YELLOW);
        if sim.paused {
            f.text_center(
                f.width * 0.5,
                f.height * 0.12,
                5.0,
                "PAUSED",
                gui::Color::ORANGE,
            );
        }
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "CLICK SPAWNS A COMET (MAX 8) - SPACE PAUSES - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
