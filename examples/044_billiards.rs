//! 044 — Billiards: sixteen balls on a felt table. Ball-ball hits resolve
//! as elastic impulses along the contact normal, the walls act as cushions
//! and mild rolling friction brings balls to rest under 2 px/s. Click near
//! a ball to kick it away from the cursor; SPACE re-racks the triangle.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 560.0;
const BALL_R: f32 = 12.0;
const WALL: f32 = 22.0;
const RESTITUTION: f32 = 0.94;
const STOP_SPEED: f32 = 2.0;
const KICK_RADIUS: f32 = 80.0;
const KICK_SPEED: f32 = 340.0;

const PALETTE: [gui::Color; 8] = [
    gui::Color::rgb(0.90, 0.25, 0.22),
    gui::Color::rgb(0.95, 0.75, 0.20),
    gui::Color::rgb(0.25, 0.75, 0.35),
    gui::Color::rgb(0.45, 0.30, 0.20),
    gui::Color::rgb(0.25, 0.45, 0.90),
    gui::Color::rgb(0.85, 0.40, 0.80),
    gui::Color::rgb(0.90, 0.50, 0.18),
    gui::Color::rgb(0.25, 0.70, 0.75),
];

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    col: gui::Color,
}

struct Table {
    balls: Vec<Ball>,
}

impl Table {
    fn new() -> Table {
        Table { balls: Vec::new() }
    }

    /// Cue ball left, fifteen balls in a triangle on the head spot.
    fn rack(&mut self, w: f32, h: f32) {
        self.balls.clear();
        self.balls.push(Ball {
            x: w * 0.24,
            y: h * 0.5,
            vx: 0.0,
            vy: 0.0,
            col: gui::Color::rgb(0.94, 0.94, 0.92),
        });
        let ax = w * 0.62;
        let ay = h * 0.5;
        let sx = BALL_R * 2.0 * 0.88;
        let sy = BALL_R * 2.0 + 1.5;
        let mut k = 0usize;
        for row in 0..5usize {
            for j in 0..=row {
                self.balls.push(Ball {
                    x: ax + row as f32 * sx,
                    y: ay + (j as f32 - row as f32 * 0.5) * sy,
                    vx: 0.0,
                    vy: 0.0,
                    col: PALETTE[(k + row) % PALETTE.len()],
                });
                k += 1;
            }
        }
    }

    fn update(&mut self, f: &mut gui::Frame) {
        let dt = f.dt;
        let damp = 0.995f32.powf(dt * 60.0);
        let min_x = WALL + BALL_R;
        let min_y = WALL + BALL_R;
        let max_x = f.width - WALL - BALL_R;
        let max_y = f.height - WALL - BALL_R;
        for b in &mut self.balls {
            b.x += b.vx * dt;
            b.y += b.vy * dt;
            b.vx *= damp;
            b.vy *= damp;
            if b.vx * b.vx + b.vy * b.vy < STOP_SPEED * STOP_SPEED {
                b.vx = 0.0;
                b.vy = 0.0;
            }
            if b.x < min_x {
                b.x = min_x;
                b.vx = -b.vx * RESTITUTION;
            }
            if b.x > max_x {
                b.x = max_x;
                b.vx = -b.vx * RESTITUTION;
            }
            if b.y < min_y {
                b.y = min_y;
                b.vy = -b.vy * RESTITUTION;
            }
            if b.y > max_y {
                b.y = max_y;
                b.vy = -b.vy * RESTITUTION;
            }
        }

        // Elastic ball-ball impulses along the contact normal.
        let d_min = BALL_R * 2.0;
        for i in 0..self.balls.len() {
            for j in (i + 1)..self.balls.len() {
                let dx = self.balls[j].x - self.balls[i].x;
                let dy = self.balls[j].y - self.balls[i].y;
                let d2 = dx * dx + dy * dy;
                if d2 < d_min * d_min && d2 > 1e-9 {
                    let d = d2.sqrt();
                    let (nx, ny) = (dx / d, dy / d);
                    let overlap = (d_min - d) * 0.5;
                    self.balls[i].x -= nx * overlap;
                    self.balls[i].y -= ny * overlap;
                    self.balls[j].x += nx * overlap;
                    self.balls[j].y += ny * overlap;
                    let vn = (self.balls[i].vx - self.balls[j].vx) * nx
                        + (self.balls[i].vy - self.balls[j].vy) * ny;
                    if vn > 0.0 {
                        let imp = vn * (1.0 + RESTITUTION) * 0.5;
                        self.balls[i].vx -= imp * nx;
                        self.balls[i].vy -= imp * ny;
                        self.balls[j].vx += imp * nx;
                        self.balls[j].vy += imp * ny;
                    }
                }
            }
        }

        // Click near a ball: kick it directly away from the cursor.
        if f.input.mouse_pressed() {
            let (mx, my) = f.mouse();
            let mut best: Option<usize> = None;
            let mut best_d2 = KICK_RADIUS * KICK_RADIUS;
            for (idx, b) in self.balls.iter().enumerate() {
                let dx = b.x - mx;
                let dy = b.y - my;
                let d2 = dx * dx + dy * dy;
                if d2 < best_d2 {
                    best_d2 = d2;
                    best = Some(idx);
                }
            }
            if let Some(idx) = best {
                let b = &mut self.balls[idx];
                let dx = b.x - mx;
                let dy = b.y - my;
                let d = (dx * dx + dy * dy).sqrt().max(1.0);
                b.vx += dx / d * KICK_SPEED;
                b.vy += dy / d * KICK_SPEED;
            }
        }

        if f.input.key_pressed(gui::key::SPACE) {
            self.rack(f.width, f.height);
        }
    }
}

fn main() {
    let mut table = Table::new();
    let app = gui::App::new("044 - Billiards", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.05, 0.05, 0.07));

        if table.balls.is_empty() {
            table.rack(f.width, f.height);
        }
        table.update(f);

        // Wooden rail + felt.
        f.frame_rect(
            8.0,
            8.0,
            f.width - 16.0,
            f.height - 16.0,
            10.0,
            gui::Color::rgb(0.30, 0.19, 0.10),
        );
        f.rect(
            WALL,
            WALL,
            f.width - 2.0 * WALL,
            f.height - 2.0 * WALL,
            gui::Color::rgb(0.055, 0.23, 0.12),
        );
        f.line(
            f.width * 0.24,
            WALL,
            f.width * 0.24,
            f.height - WALL,
            1.5,
            gui::Color::rgba(1.0, 1.0, 1.0, 0.08),
        );
        f.circle(
            f.width * 0.62,
            f.height * 0.5,
            2.0,
            gui::Color::rgba(1.0, 1.0, 1.0, 0.15),
        );

        // Balls with a small specular highlight.
        for b in &table.balls {
            f.circle_segments(b.x, b.y, BALL_R, 22, b.col);
            f.circle(
                b.x - 4.0,
                b.y - 4.0,
                3.2,
                gui::Color::rgba(1.0, 1.0, 1.0, 0.5),
            );
        }

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "044 - BILLIARDS",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 130.0,
            26.0,
            2.5,
            &format!("BALLS: {}", table.balls.len()),
            gui::Color::GRAY,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "CLICK NEAR A BALL TO KICK IT - SPACE RE-RACKS THE TRIANGLE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
