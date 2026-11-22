//! 046 — Rain: 250 raindrop streaks fall at a slant set by the wind.
//! Holding LEFT/RIGHT shifts the wind within ±3 px/frame and the value is
//! shown on screen next to a direction arrow. Drops reset at the top; every
//! drop that reaches the ground line spawns a small expanding splash arc
//! that fades away.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 560.0;
const DROPS: usize = 250;
const SPLASH_CAP: usize = 90;
const WIND_MAX: f32 = 3.0;
const WIND_RATE: f32 = 2.2;
const GROUND_H: f32 = 64.0;
const SPLASH_LIFE: f32 = 0.5;

struct Drop {
    x: f32,
    y: f32,
    len: f32,
    spd: f32,
}

struct Splash {
    x: f32,
    y: f32,
    age: f32,
}

struct Storm {
    drops: Vec<Drop>,
    splashes: Vec<Splash>,
    wind: f32,
    rng: gui::Rng,
}

/// Re-seed one drop: anywhere in the sky, or just above the top edge.
fn reset_drop(d: &mut Drop, rng: &mut gui::Rng, w: f32, h: f32, anywhere: bool) {
    d.x = rng.f32() * (w + 240.0) - 120.0;
    d.y = if anywhere {
        rng.f32() * h
    } else {
        -(10.0 + rng.f32() * 90.0)
    };
    d.len = 9.0 + rng.f32() * 14.0;
    d.spd = 520.0 + rng.f32() * 340.0;
}

fn main() {
    let mut storm = Storm {
        drops: Vec::with_capacity(DROPS),
        splashes: Vec::new(),
        wind: 0.9,
        rng: gui::Rng::from_entropy(),
    };
    let app = gui::App::new("046 - Rain", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.04, 0.07));
        let ground_y = f.height - GROUND_H;

        if storm.drops.is_empty() {
            for _ in 0..DROPS {
                let mut d = Drop {
                    x: 0.0,
                    y: 0.0,
                    len: 0.0,
                    spd: 0.0,
                };
                reset_drop(&mut d, &mut storm.rng, f.width, f.height, true);
                storm.drops.push(d);
            }
        }

        // Wind control.
        if f.input.key(gui::key::LEFT) {
            storm.wind -= WIND_RATE * f.dt;
        }
        if f.input.key(gui::key::RIGHT) {
            storm.wind += WIND_RATE * f.dt;
        }
        storm.wind = storm.wind.clamp(-WIND_MAX, WIND_MAX);
        let vx = storm.wind * 60.0; // px/frame -> px/s

        // Integrate drops; splash + recycle at the ground line.
        for d in &mut storm.drops {
            d.x += vx * f.dt;
            d.y += d.spd * f.dt;
            if d.x < -140.0 {
                d.x += f.width + 280.0;
            }
            if d.x > f.width + 140.0 {
                d.x -= f.width + 280.0;
            }
            if d.y >= ground_y {
                if storm.splashes.len() < SPLASH_CAP {
                    storm.splashes.push(Splash {
                        x: d.x,
                        y: ground_y,
                        age: 0.0,
                    });
                }
                reset_drop(d, &mut storm.rng, f.width, f.height, false);
            }
        }
        storm.splashes.retain_mut(|s| {
            s.age += f.dt;
            s.age < SPLASH_LIFE
        });

        // Ground.
        f.rect(
            0.0,
            ground_y,
            f.width,
            GROUND_H,
            gui::Color::rgb(0.05, 0.07, 0.10),
        );
        f.line(
            0.0,
            ground_y,
            f.width,
            ground_y,
            2.0,
            gui::Color::rgb(0.20, 0.24, 0.34),
        );

        // Drops: streaks aligned with their velocity.
        for d in &storm.drops {
            let sp = (vx * vx + d.spd * d.spd).sqrt().max(1.0);
            let (ux, uy) = (vx / sp, d.spd / sp);
            f.line(
                d.x - ux * d.len,
                d.y - uy * d.len,
                d.x,
                d.y,
                1.6,
                gui::Color::rgba(0.62, 0.74, 0.95, 0.65),
            );
        }

        // Splashes: expanding upper-half arcs that fade.
        for s in &storm.splashes {
            let t = s.age / SPLASH_LIFE;
            let r = 3.0 + t * 24.0;
            let col = gui::Color::rgba(0.62, 0.80, 1.0, (1.0 - t) * 0.8);
            let segs = 7;
            for k in 0..segs {
                let a0 = std::f32::consts::PI + (k as f32 / segs as f32) * std::f32::consts::PI;
                let a1 =
                    std::f32::consts::PI + ((k + 1) as f32 / segs as f32) * std::f32::consts::PI;
                f.line(
                    s.x + a0.cos() * r,
                    s.y + a0.sin() * r,
                    s.x + a1.cos() * r,
                    s.y + a1.sin() * r,
                    1.8,
                    col,
                );
            }
        }

        // Wind readout: arrow + signed value.
        let ax = f.width * 0.5;
        let tail = ax - storm.wind * 30.0;
        let tip = ax + storm.wind * 30.0;
        f.line(tail, 56.0, tip, 56.0, 2.0, gui::Color::CYAN);
        if storm.wind.abs() > 0.2 {
            let sgn = storm.wind.signum();
            f.tri(
                (tip, 56.0),
                (tip - sgn * 8.0, 51.0),
                (tip - sgn * 8.0, 61.0),
                gui::Color::CYAN,
            );
        }
        f.text_center(
            f.width * 0.5,
            66.0,
            2.5,
            &format!("WIND: {:+.2} PX/FRAME", storm.wind),
            gui::Color::CYAN,
        );

        f.text_center(f.width * 0.5, 20.0, 4.0, "046 - RAIN", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "LEFT/RIGHT SHIFTS THE WIND IN +-3 PX/FRAME - DROPS RESET AT THE TOP - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
