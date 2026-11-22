//! 092 — Calendar: month view for the current UTC month, computed from the
//! UNIX epoch with civil-date math (no external date crates). MON..SUN
//! header, weekend tinting, blank slots for other-month days, and a filled
//! circle marking today. LEFT/RIGHT page by month, `T` snaps back to today.

use std::time::SystemTime;

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 760.0;
const H: f64 = 660.0;

const MONTHS: [&str; 12] = [
    "JANUARY",
    "FEBRUARY",
    "MARCH",
    "APRIL",
    "MAY",
    "JUNE",
    "JULY",
    "AUGUST",
    "SEPTEMBER",
    "OCTOBER",
    "NOVEMBER",
    "DECEMBER",
];
const WEEKDAYS: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];

/// Days from the civil date to the epoch (Howard Hinnant's algorithm).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Inverse of [`days_from_civil`].
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// Weekday index with MON = 0 (epoch day 0 was a Thursday).
fn weekday(day_number: i64) -> i64 {
    ((day_number % 7) + 3).rem_euclid(7)
}

fn main() {
    let today_days = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
        / 86400;
    let (ty, tm, _td) = civil_from_days(today_days);

    let mut view = (ty, tm);
    let app = gui::App::new("092 - Calendar", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.09, 0.09, 0.12));
        let s = f.scale;

        // ------------------------------------------------------------- input
        if f.input.key_pressed(gui::key::LEFT) {
            let (y, m) = view;
            view = if m == 1 { (y - 1, 12) } else { (y, m - 1) };
        }
        if f.input.key_pressed(gui::key::RIGHT) {
            let (y, m) = view;
            view = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
        }
        if f.input.key_pressed(gui::key::T) {
            view = (ty, tm);
        }

        // ------------------------------------------------------------- header
        let margin = 40.0 * s;
        let (vy, vm) = view;
        let title = format!("{} {}", MONTHS[(vm - 1) as usize], vy);
        let size = 7.0 * s;
        f.text_center(f.width * 0.5, 26.0 * s, size, &title, gui::Color::WHITE);
        let today_label = format!(
            "TODAY: {} {:02} {} {}",
            ty,
            tm,
            MONTHS[(tm - 1) as usize],
            WEEKDAYS[weekday(today_days) as usize]
        );
        f.text_center(
            f.width * 0.5,
            26.0 * s + size + 12.0 * s,
            2.5 * s,
            &today_label,
            gui::Color::GRAY,
        );

        // Arrows beside the title.
        let arrow_w = 34.0 * s;
        let arrow_h = 34.0 * s;
        let ay = 22.0 * s;
        let left_rect = (
            f.width * 0.5 - f.text_width(size, &title) * 0.5 - arrow_w - 24.0 * s,
            ay,
            arrow_w,
            arrow_h,
        );
        let right_rect = (
            f.width * 0.5 + f.text_width(size, &title) * 0.5 + 24.0 * s,
            ay,
            arrow_w,
            arrow_h,
        );
        let (mx, my) = f.mouse();
        for (rect, glyph, dy_month) in [(left_rect, "<", 0i64), (right_rect, ">", 1i64)] {
            let hover =
                mx >= rect.0 && mx <= rect.0 + rect.2 && my >= rect.1 && my <= rect.1 + rect.3;
            let fill = if hover {
                gui::Color::rgb(0.30, 0.34, 0.48)
            } else {
                gui::Color::rgb(0.20, 0.22, 0.30)
            };
            f.rect(rect.0, rect.1, rect.2, rect.3, fill);
            f.text_center(
                rect.0 + rect.2 * 0.5,
                rect.1 + rect.3 * 0.5 - 3.5 * 4.0 * s,
                4.0 * s,
                glyph,
                gui::Color::WHITE,
            );
            if f.input.mouse_pressed() && hover {
                let (y, m) = view;
                view = if dy_month == 0 {
                    if m == 1 { (y - 1, 12) } else { (y, m - 1) }
                } else if m == 12 {
                    (y + 1, 1)
                } else {
                    (y, m + 1)
                };
            }
        }

        // ------------------------------------------------------------- grid
        let gx = margin;
        let gy = 118.0 * s;
        let gw = f.width - 2.0 * margin;
        let gh = f.height - gy - 64.0 * s;
        let cw = gw / 7.0;
        let ch = gh / 6.0;

        for (i, wd) in WEEKDAYS.iter().enumerate() {
            let weekend = i >= 5;
            f.text_center(
                gx + (i as f32 + 0.5) * cw,
                gy - 22.0 * s,
                3.0 * s,
                wd,
                if weekend {
                    gui::Color::rgb(0.55, 0.60, 0.85)
                } else {
                    gui::Color::GRAY
                },
            );
        }

        let first_day = days_from_civil(vy, vm, 1);
        let first_slot = first_day - weekday(first_day);
        for slot in 0..42i64 {
            let col = slot % 7;
            let row = slot / 7;
            let day_number = first_slot + slot;
            let (dy, dm, dd) = civil_from_days(day_number);
            let in_month = dm == vm && dy == vy;
            if !in_month {
                continue;
            }
            let wd = weekday(day_number) as usize;
            let weekend = wd >= 5;
            let x = gx + col as f32 * cw;
            let y = gy + row as f32 * ch;
            let is_today = day_number == today_days;

            let bg = if is_today {
                gui::Color::rgb(0.20, 0.28, 0.45)
            } else if weekend {
                gui::Color::rgb(0.13, 0.13, 0.18)
            } else {
                gui::Color::rgb(0.15, 0.15, 0.20)
            };
            f.rect(x + 2.0 * s, y + 2.0 * s, cw - 4.0 * s, ch - 4.0 * s, bg);
            if is_today {
                // Filled circle behind today's number.
                let cr = 17.0 * s;
                f.circle(
                    x + 14.0 * s + cr,
                    y + 16.0 * s + cr * 0.6,
                    cr,
                    gui::Color::rgb(0.35, 0.55, 1.0),
                );
            }
            let num_color = if is_today {
                gui::Color::WHITE
            } else if weekend {
                gui::Color::rgb(0.65, 0.70, 0.95)
            } else {
                gui::Color::rgb(0.80, 0.82, 0.90)
            };
            f.text(
                x + 10.0 * s,
                y + 12.0 * s,
                4.0 * s,
                &format!("{dd}"),
                num_color,
            );
            // Small weekday glyph in the cell corner for readability.
            f.text(
                x + cw - f.text_width(2.0 * s, WEEKDAYS[wd]) - 8.0 * s,
                y + 14.0 * s,
                2.0 * s,
                WEEKDAYS[wd],
                gui::Color::rgb(0.38, 0.40, 0.50),
            );
        }
        f.frame_rect(gx, gy, gw, gh, 2.0 * s, gui::Color::rgb(0.28, 0.30, 0.40));

        f.text(
            margin,
            f.height - 34.0 * s,
            2.0 * s,
            "LEFT/RIGHT CHANGE MONTH - T BACK TO TODAY - ALL DATES UTC",
            gui::Color::GRAY,
        );
    });
}
