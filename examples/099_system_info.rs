//! 099 — System Info: a GPU report card for the default Metal device. At
//! startup one `autoreleasepool` + `unsafe` block queries the device —
//! name, unified memory, max buffer length, recommended working set,
//! registry ID, low power / headless / removable — and probes a handful of
//! `MTLGPUFamily` variants. Results land in a plain struct; every frame
//! after that renders from that struct without ever touching Metal.

use metal_cpp_rs::prelude::*;

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 680.0;
const H: f64 = 820.0;

/// The GPU families we probe, Apple silicon generations plus the Metal
/// feature-set levels.
const FAMILIES: [(MTLGPUFamily, &str); 8] = [
    (MTLGPUFamily::GPUFamilyApple3, "APPLE 3 (A9 CLASS)"),
    (MTLGPUFamily::GPUFamilyApple5, "APPLE 5 (A11 CLASS)"),
    (MTLGPUFamily::GPUFamilyApple6, "APPLE 6 (A13 CLASS)"),
    (MTLGPUFamily::GPUFamilyApple7, "APPLE 7 (A14 / M1 CLASS)"),
    (MTLGPUFamily::GPUFamilyApple8, "APPLE 8 (M2 CLASS)"),
    (MTLGPUFamily::GPUFamilyApple9, "APPLE 9 (M3 CLASS)"),
    (MTLGPUFamily::GPUFamilyMetal3, "METAL 3 FEATURE SET"),
    (MTLGPUFamily::GPUFamilyMetal4, "METAL 4 FEATURE SET"),
];

struct GpuReport {
    name: String,
    unified_memory: bool,
    max_buffer_gb: String,
    working_set_gb: String,
    registry_id: u64,
    low_power: bool,
    headless: bool,
    removable: bool,
    families: Vec<(&'static str, bool)>,
}

/// Format a byte count as a gigabyte string.
fn gb(bytes: u64) -> String {
    format!("{:.2} GB", bytes as f64 / 1e9)
}

/// All Objective-C queries happen here, once, inside one autorelease pool.
fn probe(device: &Strong<MTLDevice>) -> GpuReport {
    autoreleasepool(|| unsafe {
        // Long device names would overflow the banner; keep them bounded.
        let raw_name = device.name().to_string();
        let name = if raw_name.chars().count() > 26 {
            format!("{}...", raw_name.chars().take(23).collect::<String>())
        } else {
            raw_name
        };
        GpuReport {
            name,
            unified_memory: device.hasUnifiedMemory(),
            max_buffer_gb: gb(device.maxBufferLength() as u64),
            working_set_gb: gb(device.recommendedMaxWorkingSetSize()),
            registry_id: device.registryID(),
            low_power: device.isLowPower(),
            headless: device.isHeadless(),
            removable: device.isRemovable(),
            families: FAMILIES
                .iter()
                .map(|&(family, label)| (label, device.supportsFamily(family)))
                .collect(),
        }
    })
}

/// One key/value report row.
fn prop_row(f: &mut gui::Frame, y: f32, s: f32, label: &str, value: &str, accent: gui::Color) {
    f.text(40.0 * s, y, 2.5 * s, label, gui::Color::GRAY);
    let vw = f.text_width(3.0 * s, value);
    f.text(f.width - 40.0 * s - vw, y - 2.0 * s, 3.0 * s, value, accent);
}

/// One GPU-family row with a green/red check bar.
fn family_row(f: &mut gui::Frame, x: f32, y: f32, s: f32, w: f32, label: &str, ok: bool) {
    let bar_w = 44.0 * s;
    let bar_h = 14.0 * s;
    let base = if ok {
        gui::Color::rgb(0.20, 0.62, 0.30)
    } else {
        gui::Color::rgb(0.60, 0.20, 0.22)
    };
    let (mx, my) = f.mouse();
    let hover = mx >= x && mx <= x + w && my >= y && my <= y + 34.0 * s;
    f.rect(
        x,
        y,
        w,
        30.0 * s,
        if hover {
            gui::Color::rgb(0.17, 0.17, 0.23)
        } else {
            gui::Color::rgb(0.14, 0.14, 0.18)
        },
    );
    f.rect(x + 10.0 * s, y + 8.0 * s, bar_w, bar_h, base);
    f.text_center(
        x + 10.0 * s + bar_w * 0.5,
        y + 8.0 * s + bar_h * 0.5 - 3.5 * 2.0 * s,
        2.0 * s,
        if ok { "YES" } else { "NO" },
        gui::Color::WHITE,
    );
    f.text(
        x + bar_w + 22.0 * s,
        y + 9.0 * s,
        2.5 * s,
        label,
        if ok {
            gui::Color::WHITE
        } else {
            gui::Color::GRAY
        },
    );
    if ok {
        f.tri(
            (x + bar_w + 8.0 * s, y + 15.0 * s),
            (x + bar_w + 13.0 * s, y + 20.0 * s),
            (x + bar_w + 20.0 * s, y + 9.0 * s),
            gui::Color::GREEN,
        );
    } else {
        let cx = x + bar_w + 14.0 * s;
        f.line(
            cx - 5.0 * s,
            y + 9.0 * s,
            cx + 5.0 * s,
            y + 19.0 * s,
            2.5 * s,
            gui::Color::RED,
        );
        f.line(
            cx + 5.0 * s,
            y + 9.0 * s,
            cx - 5.0 * s,
            y + 19.0 * s,
            2.5 * s,
            gui::Color::RED,
        );
    }
}

fn main() {
    let app = gui::App::new("099 - System Info", W, H);
    let report = probe(app.device());
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.09, 0.09, 0.12));
        let s = f.scale;
        let margin = 40.0 * s;

        // ------------------------------------------------------------- header
        f.text(margin, 24.0 * s, 5.0 * s, "GPU REPORT", gui::Color::WHITE);
        let tag = "DEFAULT METAL DEVICE";
        f.text(
            f.width - margin - f.text_width(2.5 * s, tag),
            32.0 * s,
            2.5 * s,
            tag,
            gui::Color::GRAY,
        );
        // Device name banner.
        let name_h = 46.0 * s;
        f.rect(
            margin,
            66.0 * s,
            f.width - 2.0 * margin,
            name_h,
            gui::Color::rgb(0.16, 0.18, 0.26),
        );
        f.frame_rect(
            margin,
            66.0 * s,
            f.width - 2.0 * margin,
            name_h,
            2.0 * s,
            gui::Color::rgb(0.35, 0.55, 1.0),
        );
        f.text_center(
            f.width * 0.5,
            66.0 * s + name_h * 0.5 - 3.5 * 4.0 * s,
            4.0 * s,
            &report.name,
            gui::Color::WHITE,
        );

        // ------------------------------------------------------------- properties
        let mut y = 132.0 * s;
        let pitch = 34.0 * s;
        f.text(
            margin,
            y,
            3.0 * s,
            "PROPERTIES",
            gui::Color::rgb(0.5, 0.7, 1.0),
        );
        f.line(
            margin,
            y + 16.0 * s,
            f.width - margin,
            y + 16.0 * s,
            1.5 * s,
            gui::Color::rgb(0.28, 0.30, 0.40),
        );
        y += pitch + 6.0 * s;
        prop_row(
            f,
            y,
            s,
            "UNIFIED MEMORY",
            if report.unified_memory { "YES" } else { "NO" },
            if report.unified_memory {
                gui::Color::GREEN
            } else {
                gui::Color::RED
            },
        );
        y += pitch;
        prop_row(
            f,
            y,
            s,
            "MAX BUFFER LENGTH",
            &report.max_buffer_gb,
            gui::Color::YELLOW,
        );
        y += pitch;
        prop_row(
            f,
            y,
            s,
            "RECOMMENDED WORKING SET",
            &report.working_set_gb,
            gui::Color::YELLOW,
        );
        y += pitch;
        prop_row(
            f,
            y,
            s,
            "REGISTRY ID",
            &report.registry_id.to_string(),
            gui::Color::CYAN,
        );
        y += pitch;
        prop_row(
            f,
            y,
            s,
            "LOW POWER",
            if report.low_power { "YES" } else { "NO" },
            if report.low_power {
                gui::Color::YELLOW
            } else {
                gui::Color::GRAY
            },
        );
        y += pitch;
        prop_row(
            f,
            y,
            s,
            "HEADLESS",
            if report.headless { "YES" } else { "NO" },
            if report.headless {
                gui::Color::YELLOW
            } else {
                gui::Color::GRAY
            },
        );
        y += pitch;
        prop_row(
            f,
            y,
            s,
            "REMOVABLE",
            if report.removable { "YES" } else { "NO" },
            if report.removable {
                gui::Color::YELLOW
            } else {
                gui::Color::GRAY
            },
        );

        // ------------------------------------------------------------- families
        y += pitch + 10.0 * s;
        f.text(
            margin,
            y,
            3.0 * s,
            "GPU FAMILY SUPPORT",
            gui::Color::rgb(0.5, 0.7, 1.0),
        );
        f.line(
            margin,
            y + 16.0 * s,
            f.width - margin,
            y + 16.0 * s,
            1.5 * s,
            gui::Color::rgb(0.28, 0.30, 0.40),
        );
        y += pitch + 4.0 * s;
        let row_w = f.width - 2.0 * margin;
        let supported = report.families.iter().filter(|&&(_, ok)| ok).count();
        for (i, &(label, ok)) in report.families.iter().enumerate() {
            family_row(f, margin, y + i as f32 * 34.0 * s, s, row_w, label, ok);
        }
        let summary = format!(
            "{} / {} FAMILIES SUPPORTED",
            supported,
            report.families.len()
        );
        let sum_y = y + report.families.len() as f32 * 34.0 * s + 8.0 * s;
        f.text(margin, sum_y, 2.5 * s, &summary, gui::Color::GRAY);

        f.text(
            margin,
            f.height - 30.0 * s,
            2.0 * s,
            "QUERIED ONCE AT STARTUP IN ONE AUTORELEASE POOL",
            gui::Color::GRAY,
        );
    });
}
