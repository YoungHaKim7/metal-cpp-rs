//! 058 — Flappy: a square bird with gravity and one-button flight (SPACE or
//! click). Pipe pairs with a 180 px gap scroll left, spawning every ~1.6 s;
//! each pipe passed scores. Pipes, the ground strip or the ceiling end the
//! run — GAME OVER, then R restarts. The ground scrolls under everything.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 560.0;
const H: f64 = 680.0;
const BIRD: f32 = 26.0;
const BIRD_X: f32 = 150.0;
const GRAVITY: f32 = 1500.0;
const FLAP_VY: f32 = -430.0;
const PIPE_W: f32 = 66.0;
const GAP: f32 = 180.0;
const PIPE_INTERVAL: f32 = 1.6;
const SCROLL: f32 = 185.0;
const GROUND_H: f32 = 76.0;

#[derive(Clone, Copy, PartialEq)]
enum End {
    Waiting,
    Playing,
    Over,
}

struct Flappy {
    y: f32,
    vy: f32,
    pipes: Vec<(f32, f32, bool)>,
    spawn: f32,
    score: u32,
    scroll_off: f32,
    end: End,
    rng: gui::Rng,
}

impl Flappy {
    // `h` is the canvas height in drawing units.
    fn new(h: f32) -> Flappy {
        Flappy {
            y: h * 0.42,
            vy: 0.0,
            pipes: Vec::new(),
            spawn: 0.8,
            score: 0,
            scroll_off: 0.0,
            end: End::Waiting,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn update(&mut self, f: &mut gui::Frame) {
        let ground_y = f.height - GROUND_H;
        self.scroll_off = (self.scroll_off + SCROLL * f.dt) % 26.0;

        match self.end {
            End::Waiting => {
                self.y = f.height * 0.42 + (f.time * 3.0).sin() * 14.0;
                if f.input.key_pressed(gui::key::SPACE) || f.input.mouse_pressed() {
                    self.end = End::Playing;
                    self.vy = FLAP_VY;
                }
            }
            End::Playing => {
                if f.input.key_pressed(gui::key::SPACE) || f.input.mouse_pressed() {
                    self.vy = FLAP_VY;
                }
                self.vy = (self.vy + GRAVITY * f.dt).min(620.0);
                self.y += self.vy * f.dt;

                self.spawn -= f.dt;
                if self.spawn <= 0.0 {
                    let lo = 110.0;
                    let hi = ground_y - 110.0 - GAP;
                    self.pipes
                        .push((f.width + 40.0, lo + self.rng.f32() * (hi - lo), false));
                    self.spawn = PIPE_INTERVAL;
                }
                for p in &mut self.pipes {
                    p.0 -= SCROLL * f.dt;
                }
                self.pipes.retain(|p| p.0 + PIPE_W > -20.0);

                for p in &mut self.pipes {
                    if !p.2 && p.0 + PIPE_W < BIRD_X - BIRD * 0.5 {
                        p.2 = true;
                        self.score += 1;
                    }
                }

                let bx0 = BIRD_X - BIRD * 0.5;
                let by0 = self.y - BIRD * 0.5;
                let hit_pipe = self.pipes.iter().any(|p| {
                    bx0 + BIRD > p.0 && bx0 < p.0 + PIPE_W && (by0 < p.1 || by0 + BIRD > p.1 + GAP)
                });
                if hit_pipe || self.y + BIRD * 0.5 > ground_y || self.y - BIRD * 0.5 < 0.0 {
                    self.end = End::Over;
                    if self.y + BIRD * 0.5 > ground_y {
                        self.y = ground_y - BIRD * 0.5;
                    }
                }
            }
            End::Over => {
                self.vy = (self.vy + GRAVITY * f.dt).min(620.0);
                self.y = (self.y + self.vy * f.dt).min(ground_y - BIRD * 0.5);
                if f.input.key_pressed(gui::key::R) {
                    *self = Flappy::new(f.height);
                }
            }
        }
    }
}

fn main() {
    let mut game = Flappy::new(H as f32 * gui::UNITS_PER_POINT);
    let app = gui::App::new("058 - Flappy", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.07, 0.10, 0.16));
        game.update(f);
        let ground_y = f.height - GROUND_H;

        // Pipe pairs with caps.
        for p in &game.pipes {
            f.rect(p.0, 0.0, PIPE_W, p.1, gui::Color::rgb(0.24, 0.62, 0.30));
            f.frame_rect(
                p.0,
                0.0,
                PIPE_W,
                p.1,
                3.0,
                gui::Color::rgb(0.13, 0.38, 0.18),
            );
            let by = p.1 + GAP;
            f.rect(
                p.0,
                by,
                PIPE_W,
                ground_y - by,
                gui::Color::rgb(0.24, 0.62, 0.30),
            );
            f.frame_rect(
                p.0,
                by,
                PIPE_W,
                ground_y - by,
                3.0,
                gui::Color::rgb(0.13, 0.38, 0.18),
            );
            f.rect(
                p.0 - 5.0,
                p.1 - 14.0,
                PIPE_W + 10.0,
                14.0,
                gui::Color::rgb(0.16, 0.46, 0.22),
            );
            f.rect(
                p.0 - 5.0,
                by,
                PIPE_W + 10.0,
                14.0,
                gui::Color::rgb(0.16, 0.46, 0.22),
            );
        }

        // Scrolling ground strip.
        f.rect(
            0.0,
            ground_y,
            f.width,
            GROUND_H,
            gui::Color::rgb(0.35, 0.24, 0.14),
        );
        f.rect(
            0.0,
            ground_y,
            f.width,
            5.0,
            gui::Color::rgb(0.28, 0.58, 0.26),
        );
        let mut sx = -game.scroll_off;
        while sx < f.width {
            f.rect(
                sx,
                ground_y + 18.0,
                13.0,
                9.0,
                gui::Color::rgb(0.28, 0.19, 0.11),
            );
            sx += 26.0;
        }

        // The square bird.
        let bx = BIRD_X - BIRD * 0.5;
        let by = game.y - BIRD * 0.5;
        f.rect(bx, by, BIRD, BIRD, gui::Color::rgb(1.0, 0.85, 0.25));
        f.rect(
            bx + BIRD - 9.0,
            by + 6.0,
            9.0,
            7.0,
            gui::Color::rgb(1.0, 0.55, 0.15),
        );
        f.rect(bx + BIRD - 15.0, by + 4.0, 6.0, 6.0, gui::Color::WHITE);
        f.rect(bx + BIRD - 13.0, by + 6.0, 3.0, 3.0, gui::Color::BLACK);
        let wing = if game.vy < 0.0 { 3.0 } else { 0.0 };
        f.rect(
            bx + 3.0,
            by + BIRD - 9.0 + wing,
            10.0,
            6.0,
            gui::Color::rgb(0.85, 0.65, 0.10),
        );

        // Score + prompts.
        f.text_center(
            f.width * 0.5,
            26.0,
            8.0,
            &game.score.to_string(),
            gui::Color::WHITE,
        );
        if game.end == End::Waiting {
            let a = 0.5 + 0.4 * (f.time * 4.0).sin();
            f.text_center(
                f.width * 0.5,
                f.height * 0.60,
                3.0,
                "SPACE OR CLICK TO FLAP",
                gui::Color::rgba(1.0, 1.0, 1.0, a),
            );
        }
        if game.end == End::Over {
            f.rect(
                f.width * 0.5 - 180.0,
                f.height * 0.30,
                360.0,
                150.0,
                gui::Color::rgba(0.0, 0.0, 0.0, 0.78),
            );
            f.text_center(
                f.width * 0.5,
                f.height * 0.30 + 30.0,
                7.0,
                "GAME OVER",
                gui::Color::RED,
            );
            f.text_center(
                f.width * 0.5,
                f.height * 0.30 + 82.0,
                3.0,
                &format!("SCORE: {}", game.score),
                gui::Color::YELLOW,
            );
            f.text_center(
                f.width * 0.5,
                f.height * 0.30 + 116.0,
                2.5,
                "PRESS R TO RESTART",
                gui::Color::WHITE,
            );
        }

        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "SPACE OR CLICK FLAPS - DODGE THE PIPES - R RESTARTS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
