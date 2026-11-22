//! 085 — Paint: a tiny sketchpad. Drag on the canvas to lay down strokes in
//! the selected color and brush size; the eraser paints in the canvas color
//! so it rubs marks back out. The toolbar holds eight color swatches, three
//! brush sizes (S/M/L), ERASER and CLEAR — all clickable with hover
//! highlight. Strokes accumulate as persistent dots, capped at 6000 with the
//! oldest recycled first. `C` also clears; keys `1..8` pick a color.

use std::collections::VecDeque;

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 640.0;

const MAX_DOTS: usize = 6000;

const CANVAS_BG: gui::Color = gui::Color::rgb(0.93, 0.92, 0.88);

const PALETTE: [gui::Color; 8] = [
    gui::Color::rgb(0.10, 0.10, 0.13),
    gui::Color::RED,
    gui::Color::ORANGE,
    gui::Color::YELLOW,
    gui::Color::GREEN,
    gui::Color::CYAN,
    gui::Color::BLUE,
    gui::Color::MAGENTA,
];

/// Brush radius in points for S / M / L.
const BRUSH_R: [f32; 3] = [3.0, 7.5, 15.0];

struct Dot {
    x: f32,
    y: f32,
    r: f32,
    color: gui::Color,
}

struct Paint {
    dots: VecDeque<Dot>,
    color: usize,
    size: usize,
    eraser: bool,
    last: Option<(f32, f32)>,
}

impl Paint {
    fn new() -> Paint {
        Paint {
            dots: VecDeque::new(),
            color: 0,
            size: 1,
            eraser: false,
            last: None,
        }
    }

    fn clear(&mut self) {
        self.dots.clear();
        self.last = None;
    }

    /// Stamp a run of dots from the last stamp position to `(x, y)` so fast
    /// drags leave a solid trail.
    fn stamp(&mut self, x: f32, y: f32, s: f32) {
        let r = BRUSH_R[self.size] * s;
        let color = if self.eraser {
            CANVAS_BG
        } else {
            PALETTE[self.color]
        };
        match self.last {
            Some((lx, ly)) => {
                let (dx, dy) = (x - lx, y - ly);
                let dist = (dx * dx + dy * dy).sqrt();
                let n = ((dist / (r * 0.45).max(1.5)) as usize).min(240);
                for i in 0..=n {
                    let t = (i as f32 + 1.0) / (n as f32 + 1.0);
                    self.dots.push_back(Dot {
                        x: lx + dx * t,
                        y: ly + dy * t,
                        r,
                        color,
                    });
                }
            }
            None => self.dots.push_back(Dot { x, y, r, color }),
        }
        while self.dots.len() > MAX_DOTS {
            self.dots.pop_front();
        }
        self.last = Some((x, y));
    }
}

/// A clickable toolbar button: hover highlight + selection border.
struct Tool {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Tool {
    fn hovered(&self, f: &gui::Frame) -> bool {
        let (mx, my) = f.mouse();
        mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h
    }

    fn clicked(&self, f: &gui::Frame) -> bool {
        self.hovered(f) && f.input.mouse_pressed()
    }

    fn draw(&self, f: &mut gui::Frame, base: gui::Color, selected: bool) {
        let fill = if self.hovered(f) {
            gui::Color::lerp(base, gui::Color::WHITE, 0.22)
        } else {
            base
        };
        f.rect(self.x, self.y, self.w, self.h, fill);
        let border = if selected {
            gui::Color::WHITE
        } else {
            gui::Color::rgb(0.28, 0.29, 0.36)
        };
        f.frame_rect(self.x, self.y, self.w, self.h, 2.0 * f.scale, border);
    }
}

fn main() {
    let mut paint = Paint::new();
    let app = gui::App::new("085 - Paint", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.09, 0.09, 0.12));
        let s = f.scale;

        // ------------------------------------------------------------- layout
        let margin = 14.0 * s;
        let bar_y = 12.0 * s;
        let tool = 40.0 * s;
        let footer_h = 30.0 * s;
        let canvas = (
            margin,
            bar_y + tool + 26.0 * s,
            f.width - 2.0 * margin,
            f.height - bar_y - tool - footer_h - 38.0 * s,
        );

        // ------------------------------------------------------------- toolbar
        let swatch_w = tool + 8.0 * s;
        let mut swatches: Vec<Tool> = Vec::new();
        for i in 0..8 {
            swatches.push(Tool {
                x: margin + i as f32 * swatch_w,
                y: bar_y,
                w: tool,
                h: tool,
            });
        }
        let bx = margin + 8.0 * swatch_w + 12.0 * s;
        let size_btn_w = 34.0 * s;
        let size_btns: [Tool; 3] = [
            Tool {
                x: bx,
                y: bar_y,
                w: size_btn_w,
                h: tool,
            },
            Tool {
                x: bx + size_btn_w + 8.0 * s,
                y: bar_y,
                w: size_btn_w,
                h: tool,
            },
            Tool {
                x: bx + 2.0 * (size_btn_w + 8.0 * s),
                y: bar_y,
                w: size_btn_w,
                h: tool,
            },
        ];
        let ex = size_btns[2].x + size_btn_w + 14.0 * s;
        let eraser_btn = Tool {
            x: ex,
            y: bar_y,
            w: 74.0 * s,
            h: tool,
        };
        let clear_btn = Tool {
            x: ex + 82.0 * s,
            y: bar_y,
            w: 74.0 * s,
            h: tool,
        };

        // Swatches.
        for (i, t) in swatches.iter().enumerate() {
            t.draw(f, PALETTE[i], paint.color == i && !paint.eraser);
            let fg = if i == 3 {
                gui::Color::BLACK
            } else {
                gui::Color::WHITE
            };
            f.text(
                t.x + 4.0 * s,
                t.y + 3.0 * s,
                2.0 * s,
                &format!("{}", i + 1),
                fg,
            );
        }
        // Brush size buttons with a preview dot.
        let size_labels = ["S", "M", "L"];
        for (i, t) in size_btns.iter().enumerate() {
            t.draw(
                f,
                gui::Color::rgb(0.24, 0.25, 0.31),
                paint.size == i && !paint.eraser,
            );
            f.text_center(
                t.x + t.w * 0.5,
                t.y + 5.0 * s,
                3.0 * s,
                size_labels[i],
                gui::Color::WHITE,
            );
            f.circle(
                t.x + t.w * 0.5,
                t.y + t.h - 10.0 * s,
                (BRUSH_R[i] * 0.35).max(2.0) * s,
                gui::Color::WHITE,
            );
        }
        // ERASER + CLEAR.
        eraser_btn.draw(
            f,
            if paint.eraser {
                gui::Color::rgb(0.46, 0.49, 0.62)
            } else {
                gui::Color::rgb(0.24, 0.25, 0.31)
            },
            paint.eraser,
        );
        f.text_center(
            eraser_btn.x + eraser_btn.w * 0.5,
            eraser_btn.y + eraser_btn.h * 0.5 - 3.5 * 2.5 * s,
            2.5 * s,
            "ERASER",
            gui::Color::WHITE,
        );
        clear_btn.draw(f, gui::Color::rgb(0.55, 0.22, 0.24), false);
        f.text_center(
            clear_btn.x + clear_btn.w * 0.5,
            clear_btn.y + clear_btn.h * 0.5 - 3.5 * 2.5 * s,
            2.5 * s,
            "CLEAR",
            gui::Color::WHITE,
        );

        // ------------------------------------------------------------- clicks
        for (i, t) in swatches.iter().enumerate() {
            if t.clicked(f) {
                paint.color = i;
                paint.eraser = false;
                paint.last = None;
            }
        }
        for (i, t) in size_btns.iter().enumerate() {
            if t.clicked(f) {
                paint.size = i;
                paint.last = None;
            }
        }
        if eraser_btn.clicked(f) {
            paint.eraser = !paint.eraser;
            paint.last = None;
        }
        if clear_btn.clicked(f) || f.input.key_pressed(gui::key::C) {
            paint.clear();
        }
        let num_keys = [
            gui::key::NUM_1,
            gui::key::NUM_2,
            gui::key::NUM_3,
            gui::key::NUM_4,
            gui::key::NUM_5,
            gui::key::NUM_6,
            gui::key::NUM_7,
            gui::key::NUM_8,
        ];
        for (i, k) in num_keys.iter().enumerate() {
            if f.input.key_pressed(*k) {
                paint.color = i;
                paint.eraser = false;
                paint.last = None;
            }
        }

        // ------------------------------------------------------------- canvas
        f.rect(canvas.0, canvas.1, canvas.2, canvas.3, CANVAS_BG);
        let in_canvas = |f: &gui::Frame| {
            let (mx, my) = f.mouse();
            mx >= canvas.0
                && mx <= canvas.0 + canvas.2
                && my >= canvas.1
                && my <= canvas.1 + canvas.3
        };
        if f.input.mouse_pressed() && in_canvas(f) {
            paint.last = None;
        }
        if f.input.mouse_down() && in_canvas(f) {
            let (mx, my) = f.mouse();
            paint.stamp(mx, my, s);
        }
        if f.input.mouse_released() {
            paint.last = None;
        }
        for dot in &paint.dots {
            if dot.x + dot.r < canvas.0 || dot.x - dot.r > canvas.0 + canvas.2 {
                continue;
            }
            if dot.y + dot.r < canvas.1 || dot.y - dot.r > canvas.1 + canvas.3 {
                continue;
            }
            let segs = ((dot.r * 0.9) as usize).clamp(10, 28);
            f.circle_segments(dot.x, dot.y, dot.r, segs, dot.color);
        }
        f.frame_rect(
            canvas.0,
            canvas.1,
            canvas.2,
            canvas.3,
            2.0 * s,
            gui::Color::rgb(0.28, 0.29, 0.36),
        );

        // ------------------------------------------------------------- status
        let brush = if paint.eraser {
            "ERASER"
        } else {
            ["S", "M", "L"][paint.size]
        };
        let status = format!("DOTS {} / {}   BRUSH {brush}", paint.dots.len(), MAX_DOTS);
        f.text(
            margin,
            bar_y + tool + 4.0 * s,
            2.5 * s,
            &status,
            gui::Color::GRAY,
        );
        f.text(
            f.width - margin - f.text_width(2.5 * s, "PAINT"),
            bar_y + tool + 4.0 * s,
            2.5 * s,
            "PAINT",
            gui::Color::rgb(0.5, 0.7, 1.0),
        );

        f.text(
            margin,
            f.height - footer_h * 0.62,
            2.0 * s,
            "DRAG PAINTS - 1-8 PICK COLOR - C CLEAR - ERASER = CANVAS COLOR - CAP 6000",
            gui::Color::GRAY,
        );
    });
}
