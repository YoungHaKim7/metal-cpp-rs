//! 059 — Runner: an endless corridor. The square hero jogs in place with a
//! little bounce animation while obstacles — spikes and crates — scroll in
//! from the right with a speed that creeps upward. SPACE jumps; the score
//! is the distance covered. One clip and it is GAME OVER with R to retry.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 520.0;
const SIZE: f32 = 34.0;
const PLAYER_X: f32 = 170.0;
const JUMP_VY: f32 = -740.0;
const GRAVITY: f32 = 2100.0;
const SPEED0: f32 = 280.0;
const SPEED_RAMP: f32 = 9.0;
const SPEED_MAX: f32 = 560.0;
const GROUND_H: f32 = 88.0;
const M_SCALE: f32 = 42.0;

struct Block {
    x: f32,
    w: f32,
    h: f32,
    spike: bool,
}

struct Runner {
    y: f32,
    vy: f32,
    on_ground: bool,
    started: bool,
    blocks: Vec<Block>,
    next_gap: f32,
    dist: f32,
    run_t: f32,
    over: bool,
    rng: gui::Rng,
}

impl Runner {
    fn new() -> Runner {
        Runner {
            y: 0.0,
            vy: 0.0,
            on_ground: true,
            started: false,
            blocks: Vec::new(),
            next_gap: 420.0,
            dist: 0.0,
            run_t: 0.0,
            over: false,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn update(&mut self, f: &mut gui::Frame) {
        let ground_y = f.height - GROUND_H;
        if !self.started {
            self.started = true;
            self.y = ground_y - SIZE;
        }
        if self.over {
            if f.input.key_pressed(gui::key::R) {
                *self = Runner::new();
            }
            return;
        }
        self.run_t += f.dt;
        let speed = (SPEED0 + SPEED_RAMP * self.run_t).min(SPEED_MAX);
        self.dist += speed * f.dt;

        // Jump.
        if self.on_ground && f.input.key_pressed(gui::key::SPACE) {
            self.vy = JUMP_VY;
            self.on_ground = false;
        }
        if !self.on_ground {
            self.vy += GRAVITY * f.dt;
            self.y += self.vy * f.dt;
            if self.y >= ground_y - SIZE {
                self.y = ground_y - SIZE;
                self.vy = 0.0;
                self.on_ground = true;
            }
        }

        // Spawn obstacles by distance ahead, so faster runs keep fair gaps.
        self.next_gap -= speed * f.dt;
        if self.next_gap <= 0.0 {
            let spike = self.rng.f32() < 0.55;
            let (w, h) = if spike {
                (28.0, 34.0)
            } else {
                (34.0 + self.rng.f32() * 22.0, 36.0 + self.rng.f32() * 26.0)
            };
            self.blocks.push(Block {
                x: f.width + 30.0,
                w,
                h,
                spike,
            });
            self.next_gap = 240.0 + self.rng.f32() * 260.0 + speed * 0.35;
        }
        for b in &mut self.blocks {
            b.x -= speed * f.dt;
        }
        self.blocks.retain(|b| b.x + b.w > -30.0);

        // Collision with a slightly shrunk player box (spikes hit low+middle).
        let px0 = PLAYER_X + 5.0;
        let py0 = self.y + 5.0;
        let ps = SIZE - 10.0;
        let hit = self.blocks.iter().any(|b| {
            let (bx0, by0, bw, bh) = if b.spike {
                (b.x + 6.0, ground_y - b.h * 0.72, b.w - 12.0, b.h * 0.72)
            } else {
                (b.x + 3.0, ground_y - b.h + 3.0, b.w - 6.0, b.h - 4.0)
            };
            px0 < bx0 + bw && px0 + ps > bx0 && py0 < by0 + bh && py0 + ps > by0
        });
        if hit {
            self.over = true;
        }
    }
}

fn main() {
    let mut game = Runner::new();
    let app = gui::App::new("059 - Runner", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.04, 0.05, 0.08));
        game.update(f);
        let ground_y = f.height - GROUND_H;

        // Static dim stars.
        for i in 0..50usize {
            let x = (i * 73 % 211) as f32 / 211.0 * f.width;
            let y = (i * 151 % 199) as f32 / 199.0 * (ground_y - 60.0);
            f.rect(
                x,
                y + 10.0,
                1.6,
                1.6,
                gui::Color::rgba(0.75, 0.80, 0.95, 0.35),
            );
        }

        // Ground + scrolling slanted ticks.
        f.rect(
            0.0,
            ground_y,
            f.width,
            GROUND_H,
            gui::Color::rgb(0.09, 0.10, 0.14),
        );
        f.line(
            0.0,
            ground_y,
            f.width,
            ground_y,
            3.0,
            gui::Color::rgb(0.30, 0.34, 0.45),
        );
        let off = game.dist % 40.0;
        let mut tx = -off;
        while tx < f.width + 40.0 {
            f.line(
                tx,
                ground_y + 10.0,
                tx - 14.0,
                ground_y + 28.0,
                2.0,
                gui::Color::rgb(0.20, 0.23, 0.32),
            );
            tx += 40.0;
        }

        // Obstacles.
        for b in &game.blocks {
            if b.spike {
                f.tri(
                    (b.x, ground_y),
                    (b.x + b.w * 0.5, ground_y - b.h),
                    (b.x + b.w, ground_y),
                    gui::Color::rgb(0.92, 0.42, 0.30),
                );
            } else {
                f.rect(
                    b.x,
                    ground_y - b.h,
                    b.w,
                    b.h,
                    gui::Color::rgb(0.72, 0.55, 0.32),
                );
                f.frame_rect(
                    b.x,
                    ground_y - b.h,
                    b.w,
                    b.h,
                    3.0,
                    gui::Color::rgb(0.45, 0.34, 0.19),
                );
            }
        }

        // Player: jogs in place with a small hop when grounded.
        let bounce = if game.on_ground && !game.over {
            (f.time * 11.0).sin().abs() * 5.0
        } else {
            0.0
        };
        let py = game.y - bounce;
        f.rect(PLAYER_X, py, SIZE, SIZE, gui::Color::rgb(0.35, 0.85, 1.0));
        f.rect(
            PLAYER_X + SIZE - 14.0,
            py + 7.0,
            6.0,
            6.0,
            gui::Color::WHITE,
        );
        f.rect(
            PLAYER_X + 4.0,
            py + SIZE - 10.0 + bounce * 0.5,
            8.0,
            4.0,
            gui::Color::rgb(0.15, 0.45, 0.65),
        );

        // HUD.
        let meters = (game.dist / M_SCALE) as u32;
        f.text_center(
            f.width * 0.5,
            24.0,
            5.0,
            &format!("{} M", meters),
            gui::Color::WHITE,
        );
        let speed = (SPEED0 + SPEED_RAMP * game.run_t).min(SPEED_MAX);
        f.text(
            f.width - 190.0,
            30.0,
            2.5,
            &format!("SPEED: {} PX/S", speed as u32),
            gui::Color::GRAY,
        );

        if game.over {
            f.rect(
                f.width * 0.5 - 230.0,
                f.height * 0.32,
                460.0,
                130.0,
                gui::Color::rgba(0.0, 0.0, 0.0, 0.78),
            );
            f.text_center(
                f.width * 0.5,
                f.height * 0.32 + 30.0,
                7.0,
                "GAME OVER",
                gui::Color::RED,
            );
            f.text_center(
                f.width * 0.5,
                f.height * 0.32 + 84.0,
                3.0,
                &format!("DISTANCE: {} M - PRESS R TO RETRY", meters),
                gui::Color::WHITE,
            );
        }

        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "SPACE JUMPS THE SPIKES AND CRATES - SPEED KEEPS CLIMBING - R RESTARTS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
