//! 095 — Bar Chart: 14 live bars whose values random-walk smoothly every
//! frame (velocity damped, clamped to 0..1). Y gridlines carry 0/25/50/75/
//! 100 labels; each bar prints its current value on top and keeps a small
//! peak marker at its historical maximum. SPACE freezes/resumes updates.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 880.0;
const H: f64 = 560.0;
const BARS: usize = 14;

struct Chart {
    vals: [f32; BARS],
    vels: [f32; BARS],
    peaks: [f32; BARS],
    frozen: bool,
    rng: gui::Rng,
}

impl Chart {
    fn new() -> Chart {
        let mut c = Chart {
            vals: [0.0; BARS],
            vels: [0.0; BARS],
            peaks: [0.0; BARS],
            frozen: false,
            rng: gui::Rng::from_entropy(),
        };
        for v in &mut c.vals {
            *v = c.rng.f32();
        }
        c.peaks = c.vals;
        c
    }

    fn update(&mut self, dt: f32) {
        if self.frozen {
            return;
        }
        // Time-scale the per-frame walk so the pace is stable at any fps.
        let steps = (dt / (1.0 / 60.0)).clamp(0.25, 3.0);
        for i in 0..BARS {
            self.vels[i] = self.vels[i] * 0.94 + (self.rng.f32() - 0.5) * 0.05 * steps;
            let mut v = self.vals[i] + self.vels[i] * steps;
            if v < 0.0 {
                v = 0.0;
                self.vels[i] = self.vels[i].abs() * 0.5;
            }
            if v > 1.0 {
                v = 1.0;
                self.vels[i] = -self.vels[i].abs() * 0.5;
            }
            self.vals[i] = v;
            if v > self.peaks[i] {
                self.peaks[i] = v;
            }
        }
    }
}

fn main() {
    let mut chart = Chart::new();
    let app = gui::App::new("095 - Bar Chart", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.08, 0.08, 0.11));
        let s = f.scale;

        // ------------------------------------------------------------- input
        if f.input.key_pressed(gui::key::SPACE) {
            chart.frozen = !chart.frozen;
        }

        chart.update(f.dt);

        // ------------------------------------------------------------- layout
        let margin = 30.0 * s;
        let left = 56.0 * s;
        let px0 = margin + left;
        let py0 = margin + 34.0 * s;
        let pw = f.width - px0 - margin;
        let ph = f.height - py0 - margin - 14.0 * s;

        let map_y = |v: f32| py0 + (1.0 - v) * ph;

        // ------------------------------------------------------------- grid
        f.rect(px0, py0, pw, ph, gui::Color::rgb(0.11, 0.11, 0.15));
        for (i, &label) in ["0", "25", "50", "75", "100"].iter().enumerate() {
            let v = i as f32 * 0.25;
            let y = map_y(v);
            let color = if i == 0 {
                gui::Color::rgb(0.42, 0.44, 0.54)
            } else {
                gui::Color::rgb(0.18, 0.19, 0.24)
            };
            f.line(px0, y, px0 + pw, y, 1.0 * s, color);
            f.text(
                margin + 6.0 * s,
                y - 3.5 * 2.0 * s,
                2.0 * s,
                label,
                gui::Color::GRAY,
            );
        }

        // ------------------------------------------------------------- bars
        let pitch = pw / BARS as f32;
        let bar_w = pitch * 0.62;
        for i in 0..BARS {
            let v = chart.vals[i];
            let x = px0 + i as f32 * pitch + (pitch - bar_w) * 0.5;
            let y = map_y(v);
            let color = gui::Color::hsv(0.62 * (1.0 - v), 0.70, 0.95);
            // Body with a subtle darker cap.
            f.rect(x, y, bar_w, ph - (y - py0), color);
            f.rect(
                x,
                y,
                bar_w,
                4.0 * s,
                gui::Color::lerp(color, gui::Color::BLACK, 0.4),
            );
            // Peak marker: a small line pinned at the historical maximum.
            let py = map_y(chart.peaks[i]);
            f.line(
                x - 3.0 * s,
                py,
                x + bar_w + 3.0 * s,
                py,
                2.5 * s,
                gui::Color::rgb(1.0, 0.95, 0.75),
            );
            // Current value on top.
            let label = format!("{}", (v * 100.0).round() as i32);
            let hover = {
                let (mx, my) = f.mouse();
                mx >= x && mx <= x + bar_w && my >= py0 && my <= py0 + ph
            };
            let text_color = if hover {
                gui::Color::WHITE
            } else {
                gui::Color::rgb(0.72, 0.74, 0.82)
            };
            f.text_center(x + bar_w * 0.5, y - 14.0 * s, 2.2 * s, &label, text_color);
        }
        f.frame_rect(px0, py0, pw, ph, 2.0 * s, gui::Color::rgb(0.30, 0.32, 0.42));

        // ------------------------------------------------------------- header
        f.text(margin, 20.0 * s, 5.0 * s, "BAR CHART", gui::Color::WHITE);
        let (status, color) = if chart.frozen {
            ("FROZEN", gui::Color::YELLOW)
        } else {
            ("LIVE", gui::Color::GREEN)
        };
        f.text(
            f.width - margin - f.text_width(3.0 * s, status),
            26.0 * s,
            3.0 * s,
            status,
            color,
        );
        let sum: f32 = chart.vals.iter().sum();
        let avg = format!("AVG {:.0}", sum / BARS as f32 * 100.0);
        f.text(
            f.width - margin - f.text_width(2.5 * s, &avg),
            52.0 * s,
            2.5 * s,
            &avg,
            gui::Color::GRAY,
        );

        f.text(
            margin,
            f.height - 26.0 * s,
            2.0 * s,
            "SPACE FREEZE/RESUME - BARS RANDOM-WALK - PALE TICK = PEAK",
            gui::Color::GRAY,
        );
    });
}
