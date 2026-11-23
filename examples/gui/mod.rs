//! Shared GUI runtime for the 100 Metal examples — a tiny immediate-mode
//! layer on top of the crate's raw Objective-C ABI:
//!
//! * `App` owns an AppKit window (`NSApplication` + layer-hosted
//!   `CAMetalLayer`), pumps the event loop by hand
//!   (`nextEventMatchingMask:untilDate:inMode:dequeue:`) and renders every
//!   frame through the library's Metal wrappers.
//! * `Frame` is what each example's closure receives per frame: a pure
//!   vertex collector (rects, circles, lines, 5x7 bitmap text) plus
//!   keyboard/mouse state. After the closure returns, the collected
//!   vertices are encoded with a stock pipeline and presented to the
//!   layer's drawable.
//! * `draw_custom` lets an example bring its own Metal shader: build a
//!   pipeline with [`make_pipeline`] (name the functions `vertex_main` /
//!   `fragment_main`), then draw [`Vertex`] triangles with an `f32` uniform
//!   array. Uniform layout convention: `u[0] = (width, height, time,
//!   scale)`, `u[1..]` = the floats passed to `draw_custom`.
//!
//! ESC closes every example; the red close button does too.

#![allow(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

mod appkit;
mod font;

use std::collections::HashSet;
use std::time::Instant;

use metal_cpp_rs::metal::device::MTLDevice;
use metal_cpp_rs::metal::enums::{
    MTLBlendFactor, MTLLoadAction, MTLPixelFormat, MTLPrimitiveType, MTLResourceOptions,
    MTLStoreAction,
};
use metal_cpp_rs::metal::pipeline::MTLRenderPipelineState;
use metal_cpp_rs::metal::resources::MTLBuffer;
use metal_cpp_rs::objc_abi::{NSUInteger, Raw, Ref, Strong};
use metal_cpp_rs::prelude::*;
use metal_cpp_rs::quartz_core::{CAMetalLayer, CGSize};

use appkit::{
    ACTIVATION_POLICY_REGULAR, EVENT_FLAGS_CHANGED, EVENT_KEY_DOWN, EVENT_KEY_UP,
    EVENT_LEFT_MOUSE_DOWN, EVENT_LEFT_MOUSE_DRAGGED, EVENT_LEFT_MOUSE_UP, EVENT_MOUSE_MOVED,
    EVENT_RIGHT_MOUSE_DOWN, EVENT_RIGHT_MOUSE_DRAGGED, EVENT_RIGHT_MOUSE_UP, MODIFIER_FLAG_COMMAND,
    MODIFIER_FLAG_CONTROL, MODIFIER_FLAG_OPTION, MODIFIER_FLAG_SHIFT, NSApplication, NSDate,
    NSEvent, NSPoint, NSRect, NSWindow, WINDOW_BACKING_BUFFERED, WINDOW_STYLE_DEFAULT,
    appkit_force_link, set_layer_contents_scale,
};

// ---------------------------------------------------------------------------
// Key codes (HID usage codes, as returned by `-[NSEvent keyCode]`).
// ---------------------------------------------------------------------------

/// HID key codes for the arrow keys, space bar, and so on.
pub mod key {
    /// Left arrow.
    pub const LEFT: u16 = 123;
    /// Right arrow.
    pub const RIGHT: u16 = 124;
    /// Down arrow.
    pub const DOWN: u16 = 125;
    /// Up arrow.
    pub const UP: u16 = 126;
    /// Space bar.
    pub const SPACE: u16 = 49;
    /// Return (Enter).
    pub const ENTER: u16 = 36;
    /// Escape.
    pub const ESC: u16 = 53;
    /// Delete (backspace).
    pub const BACKSPACE: u16 = 51;
    /// Forward delete.
    pub const DELETE: u16 = 117;
    /// Tab.
    pub const TAB: u16 = 48;
    /// A
    pub const A: u16 = 0;
    /// B
    pub const B: u16 = 11;
    /// C
    pub const C: u16 = 8;
    /// D
    pub const D: u16 = 2;
    /// E
    pub const E: u16 = 14;
    /// F
    pub const F: u16 = 3;
    /// G
    pub const G: u16 = 5;
    /// H
    pub const H: u16 = 4;
    /// I
    pub const I: u16 = 34;
    /// J
    pub const J: u16 = 38;
    /// K
    pub const K: u16 = 40;
    /// L
    pub const L: u16 = 37;
    /// M
    pub const M: u16 = 46;
    /// N
    pub const N: u16 = 45;
    /// O
    pub const O: u16 = 31;
    /// P
    pub const P: u16 = 35;
    /// Q
    pub const Q: u16 = 12;
    /// R
    pub const R: u16 = 15;
    /// S
    pub const S: u16 = 1;
    /// T
    pub const T: u16 = 17;
    /// U
    pub const U: u16 = 32;
    /// V
    pub const V: u16 = 9;
    /// W
    pub const W: u16 = 13;
    /// X
    pub const X: u16 = 7;
    /// Y
    pub const Y: u16 = 16;
    /// Z
    pub const Z: u16 = 6;
    /// Digit 0
    pub const NUM_0: u16 = 29;
    /// Digit 1
    pub const NUM_1: u16 = 18;
    /// Digit 2
    pub const NUM_2: u16 = 19;
    /// Digit 3
    pub const NUM_3: u16 = 20;
    /// Digit 4
    pub const NUM_4: u16 = 21;
    /// Digit 5
    pub const NUM_5: u16 = 23;
    /// Digit 6
    pub const NUM_6: u16 = 22;
    /// Digit 7
    pub const NUM_7: u16 = 26;
    /// Digit 8
    pub const NUM_8: u16 = 28;
    /// Digit 9
    pub const NUM_9: u16 = 25;
}

// ---------------------------------------------------------------------------
// Constants.
// ---------------------------------------------------------------------------

/// 16 MB of vertex scratch — roughly 700k [`Vertex`] triangles per frame.
const VERTEX_BUFFER_BYTES: u64 = 16 << 20;
/// 1 MB of uniform scratch — 4096 chunks of 256 bytes.
const UNIFORM_BUFFER_BYTES: u64 = 1 << 20;
/// Uniform chunks are 256-byte aligned (64 `f32`s).
const UNIFORM_CHUNK_FLOATS: usize = 64;

const DEFAULT_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct GuiVertex {
    float2 pos;
    float4 color;
};

struct GuiOut {
    float4 position [[position]];
    float4 color;
};

vertex GuiOut vertex_main(const device GuiVertex* vertices [[buffer(0)]],
                          constant float4* uniforms [[buffer(1)]],
                          uint vid [[vertex_id]]) {
    float2 size = uniforms[0].xy;
    float2 ndc = float2(vertices[vid].pos.x / size.x * 2.0 - 1.0,
                        1.0 - vertices[vid].pos.y / size.y * 2.0);
    GuiOut out;
    out.position = float4(ndc, 0.0, 1.0);
    out.color = vertices[vid].color;
    return out;
}

fragment float4 fragment_main(GuiOut in [[stage_in]]) {
    return in.color;
}
"#;

// ---------------------------------------------------------------------------
// Data types.
// ---------------------------------------------------------------------------

/// RGBA color, components in `0.0..=1.0`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    /// Red.
    pub r: f32,
    /// Green.
    pub g: f32,
    /// Blue.
    pub b: f32,
    /// Alpha.
    pub a: f32,
}

impl Color {
    /// Opaque color from RGB components.
    pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    /// Color from RGBA components.
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    /// HSV color — `h` in turns (`0.0..=1.0` wraps), `s`/`v` in `0.0..=1.0`.
    pub fn hsv(h: f32, s: f32, v: f32) -> Color {
        let h = h.rem_euclid(1.0) * 6.0;
        let i = h.floor();
        let f = h - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));
        let (r, g, b) = match i as u32 % 6 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };
        Color::rgb(r, g, b)
    }

    /// Linear interpolation between two colors.
    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color::rgba(
            a.r + (b.r - a.r) * t,
            a.g + (b.g - a.g) * t,
            a.b + (b.b - a.b) * t,
            a.a + (b.a - a.a) * t,
        )
    }

    /// White.
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    /// Black.
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    /// Red.
    pub const RED: Color = Color::rgb(1.0, 0.25, 0.25);
    /// Green.
    pub const GREEN: Color = Color::rgb(0.25, 1.0, 0.35);
    /// Blue.
    pub const BLUE: Color = Color::rgb(0.3, 0.5, 1.0);
    /// Yellow.
    pub const YELLOW: Color = Color::rgb(1.0, 0.9, 0.25);
    /// Cyan.
    pub const CYAN: Color = Color::rgb(0.25, 0.9, 1.0);
    /// Magenta.
    pub const MAGENTA: Color = Color::rgb(1.0, 0.35, 0.9);
    /// Orange.
    pub const ORANGE: Color = Color::rgb(1.0, 0.6, 0.2);
    /// Gray.
    pub const GRAY: Color = Color::rgb(0.55, 0.55, 0.6);
    /// Dark gray.
    pub const DARK: Color = Color::rgb(0.12, 0.12, 0.15);
}

/// One triangle-list vertex — position in pixel coordinates plus RGBA color.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    /// X position (pixels, origin top-left).
    pub x: f32,
    /// Y position (pixels, origin top-left).
    pub y: f32,
    /// Red.
    pub r: f32,
    /// Green.
    pub g: f32,
    /// Blue.
    pub b: f32,
    /// Alpha.
    pub a: f32,
}

impl Vertex {
    /// Vertex at `(x, y)` painted with `color`.
    pub fn new(x: f32, y: f32, color: Color) -> Vertex {
        Vertex {
            x,
            y,
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

/// Keyboard/mouse state for the current frame.
#[derive(Clone, Default)]
pub struct Input {
    keys_down: HashSet<u16>,
    keys_pressed: HashSet<u16>,
    mouse_pos: (f32, f32),
    mouse_down: bool,
    mouse_pressed: bool,
    mouse_released: bool,
    text: String,
    shift: bool,
    ctrl: bool,
    opt: bool,
    cmd: bool,
}

impl Input {
    /// Is `code` currently held down?
    pub fn key(&self, code: u16) -> bool {
        self.keys_down.contains(&code)
    }

    /// Was `code` pressed this frame?
    pub fn key_pressed(&self, code: u16) -> bool {
        self.keys_pressed.contains(&code)
    }

    /// Characters typed this frame (printable text only).
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Mouse position in pixels, origin top-left.
    pub fn mouse(&self) -> (f32, f32) {
        self.mouse_pos
    }

    /// Left mouse button held down?
    pub fn mouse_down(&self) -> bool {
        self.mouse_down
    }

    /// Left mouse button pressed this frame?
    pub fn mouse_pressed(&self) -> bool {
        self.mouse_pressed
    }

    /// Left mouse button released this frame?
    pub fn mouse_released(&self) -> bool {
        self.mouse_released
    }

    /// Shift modifier held?
    pub fn shift(&self) -> bool {
        self.shift
    }

    /// Control modifier held?
    pub fn ctrl(&self) -> bool {
        self.ctrl
    }

    /// Option/Alt modifier held?
    pub fn opt(&self) -> bool {
        self.opt
    }

    /// Command modifier held?
    pub fn cmd(&self) -> bool {
        self.cmd
    }

    fn begin_frame(&mut self) {
        self.keys_pressed.clear();
        self.mouse_pressed = false;
        self.mouse_released = false;
        self.text.clear();
    }
}

/// One `draw_custom` entry to encode after the default pipeline pass.
struct CustomDraw {
    pso: Raw,
    first_vertex: u32,
    vertex_count: u32,
    uniform_offset_bytes: u32,
}

/// Per-frame drawing surface handed to the example's closure.
pub struct Frame {
    /// Window width in pixels.
    pub width: f32,
    /// Window height in pixels.
    pub height: f32,
    /// Seconds since the app started.
    pub time: f32,
    /// Seconds since the previous frame (clamped to 0.1).
    pub dt: f32,
    /// Backing scale factor (points → pixels).
    pub scale: f32,
    /// Input state for this frame.
    pub input: Input,

    clear: Color,
    verts: Vec<Vertex>,
    customs: Vec<CustomDraw>,
    pending_uniforms: Vec<Vec<f32>>,
    /// Bitmap text is collected separately and drawn LAST so fullscreen
    /// custom shaders can never bury captions.
    text_verts: Vec<Vertex>,
    height_points: f32,
}

impl Frame {
    /// `(mouse_x, mouse_y)` in pixels, origin top-left.
    pub fn mouse(&self) -> (f32, f32) {
        let (x, y) = self.input.mouse_pos;
        (x * self.scale, (self.height_points - y) * self.scale)
    }

    /// Mouse position as `(x, y)` pixels — alias of [`Frame::mouse`].
    pub fn mouse_x(&self) -> f32 {
        self.mouse().0
    }

    /// Mouse Y in pixels.
    pub fn mouse_y(&self) -> f32 {
        self.mouse().1
    }

    /// Set the background color used to clear this frame.
    pub fn set_clear(&mut self, color: Color) {
        self.clear = color;
    }

    /// Add a filled rectangle.
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let (x1, y1) = (x + w, y + h);
        self.verts.push(Vertex::new(x, y, color));
        self.verts.push(Vertex::new(x1, y, color));
        self.verts.push(Vertex::new(x, y1, color));
        self.verts.push(Vertex::new(x1, y, color));
        self.verts.push(Vertex::new(x1, y1, color));
        self.verts.push(Vertex::new(x, y1, color));
    }

    /// Add a rectangle outline `border` pixels thick.
    pub fn frame_rect(&mut self, x: f32, y: f32, w: f32, h: f32, border: f32, color: Color) {
        self.rect(x, y, w, border, color);
        self.rect(x, y + h - border, w, border, color);
        self.rect(x, y + border, border, h - 2.0 * border, color);
        self.rect(x + w - border, y + border, border, h - 2.0 * border, color);
    }

    /// Add a triangle from three points.
    pub fn tri(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32), color: Color) {
        self.verts.push(Vertex::new(a.0, a.1, color));
        self.verts.push(Vertex::new(b.0, b.1, color));
        self.verts.push(Vertex::new(c.0, c.1, color));
    }

    /// Add a thick line segment (`width` in pixels).
    pub fn line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, width: f32, color: Color) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-4 {
            return;
        }
        let (nx, ny) = (-dy / len * width * 0.5, dx / len * width * 0.5);
        self.tri(
            (x0 + nx, y0 + ny),
            (x1 + nx, y1 + ny),
            (x0 - nx, y0 - ny),
            color,
        );
        self.tri(
            (x1 + nx, y1 + ny),
            (x1 - nx, y1 - ny),
            (x0 - nx, y0 - ny),
            color,
        );
    }

    /// Add a filled circle (`segments` edge triangles, default 40).
    pub fn circle(&mut self, cx: f32, cy: f32, r: f32, color: Color) {
        self.circle_segments(cx, cy, r, 40, color);
    }

    /// Add a filled circle with an explicit segment count.
    pub fn circle_segments(&mut self, cx: f32, cy: f32, r: f32, segments: usize, color: Color) {
        if segments < 3 {
            return;
        }
        for i in 0..segments {
            let a0 = (i as f32) / segments as f32 * std::f32::consts::TAU;
            let a1 = ((i + 1) as f32) / segments as f32 * std::f32::consts::TAU;
            self.tri(
                (cx, cy),
                (cx + a0.cos() * r, cy + a0.sin() * r),
                (cx + a1.cos() * r, cy + a1.sin() * r),
                color,
            );
        }
    }

    /// Add an annulus (donut) from `r_inner` to `r_outer`.
    pub fn ring(&mut self, cx: f32, cy: f32, r_outer: f32, r_inner: f32, color: Color) {
        self.ring_segments(cx, cy, r_outer, r_inner, 48, color);
    }

    /// Add an annulus with an explicit segment count.
    pub fn ring_segments(
        &mut self,
        cx: f32,
        cy: f32,
        r_outer: f32,
        r_inner: f32,
        segments: usize,
        color: Color,
    ) {
        if segments < 3 {
            return;
        }
        for i in 0..segments {
            let a0 = (i as f32) / segments as f32 * std::f32::consts::TAU;
            let a1 = ((i + 1) as f32) / segments as f32 * std::f32::consts::TAU;
            let (c0x, c0y) = (cx + a0.cos(), cy + a0.sin());
            let (c1x, c1y) = (cx + a1.cos(), cy + a1.sin());
            let o0 = (c0x * r_outer, c0y * r_outer);
            let o1 = (c1x * r_outer, c1y * r_outer);
            let i0 = (c0x * r_inner, c0y * r_inner);
            let i1 = (c1x * r_inner, c1y * r_inner);
            self.tri(o0, o1, i0, color);
            self.tri(o1, i1, i0, color);
        }
    }

    /// Draw a line of 5x7 bitmap text, top-left anchored, `size` = pixel
    /// height of one glyph row. Text is drawn after every custom shader pass.
    pub fn text(&mut self, x: f32, y: f32, size: f32, s: &str, color: Color) {
        let mut cx = x;
        for ch in s.chars() {
            let rows = font::glyph_rows(if ch.is_ascii() { ch as u8 } else { b' ' });
            for row in 0..7usize {
                let bits = rows[row];
                for col in 0..5usize {
                    if bits & (1 << (4 - col)) != 0 {
                        // FIX: each lit font pixel must be a filled quad
                        // (2 triangles = 6 vertices) covering its `size x size`
                        // cell. The old code pushed a single vertex per lit
                        // pixel, so triangle-list rasterisation grouped
                        // arbitrary triples of pixels into garbage triangles
                        // instead of glyphs.
                        let px = cx + col as f32 * size;
                        let py = y + row as f32 * size;
                        let qx = px + size;
                        let qy = py + size;
                        self.text_verts.push(Vertex::new(px, py, color));
                        self.text_verts.push(Vertex::new(qx, py, color));
                        self.text_verts.push(Vertex::new(px, qy, color));
                        self.text_verts.push(Vertex::new(qx, py, color));
                        self.text_verts.push(Vertex::new(qx, qy, color));
                        self.text_verts.push(Vertex::new(px, qy, color));
                    }
                }
            }
            cx += font::ADVANCE * size;
        }
    }

    /// Draw bitmap text centered horizontally at `cx`.
    pub fn text_center(&mut self, cx: f32, y: f32, size: f32, s: &str, color: Color) {
        let w = font::measure(size, s);
        self.text(cx - w * 0.5, y, size, s, color);
    }

    /// Width of `s` rendered at `size` (for layout math).
    pub fn text_width(&self, size: f32, s: &str) -> f32 {
        font::measure(size, s)
    }

    /// Draw triangles with a custom pipeline (see the module docs for the
    /// vertex/uniform conventions).
    pub fn draw_custom(
        &mut self,
        pso: &MTLRenderPipelineState,
        uniforms: &[f32],
        verts: &[Vertex],
    ) {
        if verts.is_empty() || self.verts.len() + verts.len() > self.vert_capacity() {
            return;
        }
        let first = self.verts.len() as u32;
        self.verts.extend_from_slice(verts);
        let uniform_offset_bytes = self.customs.len() as usize * UNIFORM_CHUNK_FLOATS * 4;
        if (uniform_offset_bytes as u64 + UNIFORM_CHUNK_FLOATS as u64 * 4) > UNIFORM_BUFFER_BYTES {
            self.verts.truncate(first as usize);
            return;
        }
        self.customs.push(CustomDraw {
            pso: pso.as_raw(),
            first_vertex: first,
            vertex_count: verts.len() as u32,
            uniform_offset_bytes: uniform_offset_bytes as u32,
        });
        self.pending_uniforms.push(uniforms.to_vec());
    }

    fn vert_capacity(&self) -> usize {
        (VERTEX_BUFFER_BYTES / std::mem::size_of::<Vertex>() as u64) as usize
    }
}

// ---------------------------------------------------------------------------
// App.
// ---------------------------------------------------------------------------

/// Tiny xorshift64\* PRNG — the examples' only randomness source (the crate
/// is zero-dependency, so there is no `rand`). Seed it from
/// [`Rng::from_entropy`].
pub struct Rng(u64);

impl Rng {
    /// Deterministic PRNG from an explicit seed.
    pub fn new(seed: u64) -> Rng {
        Rng(seed.max(1))
    }

    /// Seed from the current wall-clock nanos.
    pub fn from_entropy() -> Rng {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E3779B97F4A7C15);
        Rng::new(nanos ^ 0x2545F4914F6CDD1D)
    }

    /// Next raw 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    /// Uniform `f32` in `0.0..1.0`.
    pub fn f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1 << 24) as f32
    }

    /// Uniform `i64` in `lo..=hi` (inclusive; `hi >= lo`).
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        if hi <= lo {
            return lo;
        }
        lo + (self.next_u64() % (hi - lo + 1) as u64) as i64
    }

    /// Uniform `usize` index in `0..len` (`len > 0`).
    pub fn index(&mut self, len: usize) -> usize {
        self.range(0, len as i64 - 1) as usize
    }
}

/// Windowed Metal application — create one, then hand a per-frame closure to
/// [`App::run`].
pub struct App {
    device: Strong<MTLDevice>,
    queue: Strong<MTLCommandQueue>,
    layer: Strong<CAMetalLayer>,
    window: Strong<NSWindow>,
    ns_app: Strong<NSApplication>,
    default_pso: Strong<MTLRenderPipelineState>,
    vertex_buffer: Strong<MTLBuffer>,
    uniform_buffer: Strong<MTLBuffer>,
    input: Input,
    started: Instant,
    last_frame: Instant,
    width_points: f32,
    height_points: f32,
    scale: f32,
    running: bool,
}

impl App {
    /// Create a window titled `title` with a `width`×`height` point content
    /// area and a `CAMetalLayer` swapchain attached to its content view.
    ///
    /// # Panics
    /// Panics when no Metal device is available or AppKit fails to
    /// initialize (i.e. not running on macOS).
    pub fn new(title: &str, width: f64, height: f64) -> App {
        // Load AppKit and QuartzCore before any Objective-C class lookup.
        appkit_force_link();
        metal_cpp_rs::quartz_core::quartz_core_force_link();

        unsafe {
            let device = MTLCreateSystemDefaultDevice();
            if device.is_nil() {
                panic!("MTLCreateSystemDefaultDevice() returned nil — Metal is unavailable");
            }
            let queue = device.newCommandQueue();
            if queue.is_nil() {
                panic!("newCommandQueue() returned nil");
            }

            let ns_app = NSApplication::sharedApplication();
            ns_app.setActivationPolicy(ACTIVATION_POLICY_REGULAR);

            let layer = CAMetalLayer::layer();
            layer.setDevice(device.as_ref());
            layer.setPixelFormat(MTLPixelFormat::PixelFormatBGRA8Unorm);
            layer.setFramebufferOnly(true);
            layer.setMaximumDrawableCount(3);

            let window = NSWindow::init_with_content_rect(
                NSWindow::alloc(),
                NSRect {
                    origin: NSPoint { x: 0.0, y: 0.0 },
                    size: CGSize::Make(width, height),
                },
                WINDOW_STYLE_DEFAULT,
                WINDOW_BACKING_BUFFERED,
                false,
            );
            window.setTitle(NSString::from_str(title).as_ref());
            window.setAcceptsMouseMovedEvents(true);
            window.cascadeTopLeftFromPoint(NSPoint { x: 20.0, y: 20.0 });

            let content = window.contentView();
            content.setWantsLayer(true);
            content.setLayer(layer.as_ref().as_raw());

            let bounds = content.bounds();
            let scale = window.backingScaleFactor().max(1.0);
            set_layer_contents_scale(layer.as_ref().as_raw(), scale);
            layer.setDrawableSize(CGSize::Make(
                bounds.size.width * scale,
                bounds.size.height * scale,
            ));

            ns_app.activateIgnoringOtherApps(true);
            window.makeKeyAndOrderFront(std::ptr::null_mut());

            let default_pso = make_pipeline(&device, DEFAULT_SHADER);
            let vertex_buffer = device.newBuffer(
                VERTEX_BUFFER_BYTES,
                MTLResourceOptions::ResourceStorageModeShared,
            );
            let uniform_buffer = device.newBuffer(
                UNIFORM_BUFFER_BYTES,
                MTLResourceOptions::ResourceStorageModeShared,
            );

            App {
                device,
                queue,
                layer,
                window,
                ns_app,
                default_pso,
                vertex_buffer,
                uniform_buffer,
                input: Input::default(),
                started: Instant::now(),
                last_frame: Instant::now(),
                width_points: bounds.size.width as f32,
                height_points: bounds.size.height as f32,
                scale: scale as f32,
                running: true,
            }
        }
    }

    /// The Metal device backing this app.
    pub fn device(&self) -> &Strong<MTLDevice> {
        &self.device
    }

    /// Run the event loop until ESC or the close button ends it. Calls `f`
    /// once per frame with a fresh [`Frame`].
    ///
    /// # Panics
    /// Panics if the AppKit event loop cannot be entered (non-macOS host).
    pub fn run<F: FnMut(&mut Frame)>(mut self, mut f: F) {
        while self.running {
            metal_cpp_rs::objc_abi::autoreleasepool(|| unsafe {
                self.pump_events();
                if !self.running {
                    self.ns_app.terminate(std::ptr::null_mut());
                    return;
                }
                self.sync_size();
                let mut frame = self.make_frame();
                f(&mut frame);
                if self.running {
                    self.render(&frame);
                }
            });
        }
        unsafe { self.ns_app.terminate(std::ptr::null_mut()) };
    }

    unsafe fn pump_events(&mut self) {
        self.input.begin_frame();
        let mode = NSString::from_str("kCFRunLoopDefaultMode");
        // Block briefly for the next event, then drain everything pending.
        let mut until = NSDate::dateWithTimeIntervalSinceNow(1.0 / 120.0);
        loop {
            let event = self.ns_app.nextEventMatchingMask_untilDate_inMode_dequeue(
                NSUInteger::MAX,
                until.as_ref().as_raw(),
                mode.as_ref().as_raw(),
                true,
            );
            if event.is_nil() {
                break;
            }
            if self.handle_event(event.as_ref()) {
                self.ns_app.sendEvent(event.as_ref());
            }
            if !self.running {
                break;
            }
            until = NSDate::distantPast();
        }
        if self.running {
            self.running = self.window.isVisible() || self.window.isMiniaturized();
        }
    }

    /// Handle an event for GUI bookkeeping. Returns `true` when the event
    /// should still be forwarded to the window server (mouse/window events);
    /// keyboard events are consumed here so unhandled keys do not beep.
    unsafe fn handle_event(&mut self, ev: Ref<NSEvent>) -> bool {
        match ev.eventType() {
            EVENT_KEY_DOWN => {
                let code = ev.keyCode();
                if code == key::ESC {
                    self.running = false;
                    return false;
                }
                if self.input.keys_down.insert(code) {
                    self.input.keys_pressed.insert(code);
                }
                let chars = ev.characters();
                if !chars.is_nil() {
                    for ch in chars.to_string().chars() {
                        if ch.is_ascii_graphic() || ch == ' ' {
                            self.input.text.push(ch);
                        }
                    }
                }
                false
            }
            EVENT_KEY_UP => {
                self.input.keys_down.remove(&ev.keyCode());
                false
            }
            EVENT_FLAGS_CHANGED => {
                let flags = ev.modifierFlags();
                self.input.shift = flags & MODIFIER_FLAG_SHIFT != 0;
                self.input.ctrl = flags & MODIFIER_FLAG_CONTROL != 0;
                self.input.opt = flags & MODIFIER_FLAG_OPTION != 0;
                self.input.cmd = flags & MODIFIER_FLAG_COMMAND != 0;
                false
            }
            EVENT_LEFT_MOUSE_DOWN => {
                self.input.mouse_down = true;
                self.input.mouse_pressed = true;
                self.update_mouse(ev);
                true
            }
            EVENT_LEFT_MOUSE_UP => {
                self.input.mouse_down = false;
                self.input.mouse_released = true;
                true
            }
            EVENT_RIGHT_MOUSE_DOWN | EVENT_RIGHT_MOUSE_UP => true,
            EVENT_MOUSE_MOVED | EVENT_LEFT_MOUSE_DRAGGED | EVENT_RIGHT_MOUSE_DRAGGED => {
                self.update_mouse(ev);
                true
            }
            _ => true,
        }
    }

    unsafe fn update_mouse(&mut self, ev: Ref<NSEvent>) {
        let p = ev.locationInWindow();
        self.input.mouse_pos = (p.x as f32, p.y as f32);
    }

    unsafe fn sync_size(&mut self) {
        let content = self.window.contentView();
        let bounds = content.bounds();
        let scale = self.window.backingScaleFactor().max(1.0);
        if (bounds.size.width as f32) != self.width_points
            || (bounds.size.height as f32) != self.height_points
            || (scale as f32) != self.scale
        {
            self.width_points = bounds.size.width as f32;
            self.height_points = bounds.size.height as f32;
            self.scale = scale as f32;
            set_layer_contents_scale(self.layer.as_ref().as_raw(), scale);
            self.layer.setDrawableSize(CGSize::Make(
                bounds.size.width * scale,
                bounds.size.height * scale,
            ));
        }
    }

    fn make_frame(&mut self) -> Frame {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32().min(0.1);
        self.last_frame = now;
        Frame {
            width: self.width_points * self.scale,
            height: self.height_points * self.scale,
            time: self.started.elapsed().as_secs_f32(),
            dt,
            scale: self.scale,
            input: self.input.clone(),
            clear: Color::rgb(0.05, 0.05, 0.08),
            verts: Vec::with_capacity(4096),
            customs: Vec::new(),
            pending_uniforms: Vec::new(),
            // Text now emits 6 vertices per lit 5x7 font pixel, so a typical
            // caption needs a few thousand vertices — start with enough
            // capacity to avoid reallocating every frame.
            text_verts: Vec::with_capacity(8192),
            height_points: self.height_points,
        }
    }

    unsafe fn render(&mut self, frame: &Frame) {
        let drawable = self.layer.nextDrawable();
        if drawable.is_nil() {
            return;
        }

        // Upload vertices + uniform chunks into the shared scratch buffers.
        let vert_dst = self.vertex_buffer.contents() as *mut Vertex;
        let total_verts = frame.verts.len() + frame.text_verts.len();
        let vert_bytes = total_verts * std::mem::size_of::<Vertex>();
        if vert_bytes > VERTEX_BUFFER_BYTES as usize {
            return;
        }
        if !frame.verts.is_empty() {
            std::ptr::copy_nonoverlapping(frame.verts.as_ptr(), vert_dst, frame.verts.len());
        }
        if !frame.text_verts.is_empty() {
            std::ptr::copy_nonoverlapping(
                frame.text_verts.as_ptr(),
                vert_dst.add(frame.verts.len()),
                frame.text_verts.len(),
            );
        }
        let unif_dst = self.uniform_buffer.contents() as *mut f32;
        let mut written = 0usize;
        for (chunk, uniforms) in frame.pending_uniforms.iter().enumerate() {
            let base = chunk * UNIFORM_CHUNK_FLOATS;
            let floats = [frame.width, frame.height, frame.time, frame.scale];
            for (i, v) in floats.iter().enumerate() {
                *unif_dst.add(base + i) = *v;
            }
            for (i, v) in uniforms.iter().enumerate() {
                if 4 + i < UNIFORM_CHUNK_FLOATS {
                    *unif_dst.add(base + 4 + i) = *v;
                }
            }
            written += 1;
        }
        // Globals for the default pipeline live in chunk 0.
        let globals = [frame.width, frame.height, frame.time, frame.scale];
        for (i, v) in globals.iter().enumerate() {
            *unif_dst.add(i) = *v;
        }
        let _ = written;

        let render_pass = MTLRenderPassDescriptor::renderPassDescriptor();
        let color0 = render_pass.colorAttachments().objectAtIndexedSubscript(0);
        let attachment = color0.as_attachment();
        attachment.setTexture(drawable.as_ref().texture());
        attachment.setLoadAction(MTLLoadAction::LoadActionClear);
        attachment.setStoreAction(MTLStoreAction::StoreActionStore);
        color0.setClearColor(metal_cpp_rs::metal::types::MTLClearColor::Make(
            frame.clear.r as f64,
            frame.clear.g as f64,
            frame.clear.b as f64,
            frame.clear.a as f64,
        ));

        let command_buffer = self.queue.commandBuffer();
        let encoder = command_buffer.renderCommandEncoder(render_pass.as_ref());

        if !frame.verts.is_empty() {
            encoder.setRenderPipelineState(self.default_pso.as_ref());
            encoder.setVertexBuffer(self.vertex_buffer.as_ref(), 0, 0);
            // FIX: the default shader's vertex function reads
            // `constant float4* uniforms [[buffer(1)]]` (screen size for the
            // pixel -> NDC conversion). Without this binding the uniform
            // buffer is unbound at the vertex stage, every position computes
            // to garbage/NaN and NOTHING is drawn (black window).
            encoder.setVertexBuffer(self.uniform_buffer.as_ref(), 0, 1);
            encoder.setFragmentBuffer(self.uniform_buffer.as_ref(), 0, 0);
            encoder.drawPrimitives(
                MTLPrimitiveType::PrimitiveTypeTriangle,
                0,
                frame.verts.len() as u64,
            );
        }

        for entry in &frame.customs {
            let pso = Ref::<MTLRenderPipelineState>::from_raw(entry.pso);
            encoder.setRenderPipelineState(pso);
            encoder.setVertexBuffer(self.vertex_buffer.as_ref(), 0, 0);
            encoder.setVertexBuffer(
                self.uniform_buffer.as_ref(),
                entry.uniform_offset_bytes as u64,
                1,
            );
            encoder.setFragmentBuffer(
                self.uniform_buffer.as_ref(),
                entry.uniform_offset_bytes as u64,
                0,
            );
            encoder.drawPrimitives(
                MTLPrimitiveType::PrimitiveTypeTriangle,
                entry.first_vertex as u64,
                entry.vertex_count as u64,
            );
        }

        // Bitmap text is encoded last so it always sits above custom shaders.
        if !frame.text_verts.is_empty() {
            encoder.setRenderPipelineState(self.default_pso.as_ref());
            encoder.setVertexBuffer(self.vertex_buffer.as_ref(), 0, 0);
            // FIX: same missing vertex-stage uniform binding as above — the
            // text pass uses the default pipeline too.
            encoder.setVertexBuffer(self.uniform_buffer.as_ref(), 0, 1);
            encoder.setFragmentBuffer(self.uniform_buffer.as_ref(), 0, 0);
            encoder.drawPrimitives(
                MTLPrimitiveType::PrimitiveTypeTriangle,
                frame.verts.len() as u64,
                frame.text_verts.len() as u64,
            );
        }

        encoder.endEncoding();
        command_buffer.presentDrawable(drawable.as_ref().as_raw());
        command_buffer.commit();
    }
}

/// Build a render pipeline whose shaders use the GUI vertex layout: the
/// vertex function reads `const device GuiVertex* [[buffer(0)]]` (24 bytes:
/// `float2 pos, float4 color`) and `constant float4* [[buffer(1)]]` uniforms
/// (`u[0] = (width, height, time, scale)`); name the functions
/// `vertex_main` and `fragment_main`.
///
/// # Panics
/// Panics with the shader-compiler error when the MSL source is invalid.
pub fn make_pipeline(
    device: &Strong<MTLDevice>,
    msl_source: &str,
) -> Strong<MTLRenderPipelineState> {
    unsafe {
        let source = NSString::from_str(msl_source);
        let mut error = Strong::<NSError>::nil();
        let library = device.newLibrary(
            source.as_ref(),
            MTLCompileOptions::new().as_ref(),
            &mut error,
        );
        if library.is_nil() {
            let msg = if error.is_nil() {
                "<no error object>".to_string()
            } else {
                error.localizedDescription().to_string()
            };
            panic!("shader compile failed: {msg}");
        }
        let vertex_fn = library.newFunction(NSString::from_str("vertex_main").as_ref());
        let fragment_fn = library.newFunction(NSString::from_str("fragment_main").as_ref());
        if vertex_fn.is_nil() || fragment_fn.is_nil() {
            panic!("shader is missing vertex_main/fragment_main");
        }
        let desc = MTLRenderPipelineDescriptor::new();
        desc.setVertexFunction(vertex_fn.as_ref());
        desc.setFragmentFunction(fragment_fn.as_ref());
        let color0 = desc.colorAttachments().objectAtIndexedSubscript(0);
        color0.setPixelFormat(MTLPixelFormat::PixelFormatBGRA8Unorm);
        // Straight alpha blending — `rgba(.., a)` overlays and fades work
        // in every pipeline this helper builds (including the default one).
        color0.setBlendingEnabled(true);
        color0.setSourceRGBBlendFactor(MTLBlendFactor::BlendFactorSourceAlpha);
        color0.setDestinationRGBBlendFactor(MTLBlendFactor::BlendFactorOneMinusSourceAlpha);
        color0.setSourceAlphaBlendFactor(MTLBlendFactor::BlendFactorOne);
        color0.setDestinationAlphaBlendFactor(MTLBlendFactor::BlendFactorOneMinusSourceAlpha);
        let mut error = Strong::<NSError>::nil();
        let pso = device.newRenderPipelineState(desc.as_ref(), &mut error);
        if pso.is_nil() {
            let msg = if error.is_nil() {
                "<no error object>".to_string()
            } else {
                error.localizedDescription().to_string()
            };
            panic!("pipeline creation failed: {msg}");
        }
        pso
    }
}
