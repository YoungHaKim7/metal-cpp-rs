//! 001 — Hello Window: a Metal window that greets the world — the "hello
//! world" of the 100-GUI-examples series. Demonstrates [`gui::App`], the
//! 5x7 bitmap text renderer and keyboard input: type to echo, ESC or the
//! red close button quits.

#[path = "gui/mod.rs"]
mod gui;

fn main() {
    let app = gui::App::new("001 - Hello Window", 640.0, 400.0);
    app.run(|f| {
        f.set_clear(gui::Color::rgb(0.04, 0.04, 0.08));

        // Dim vignette frame for a little polish.
        f.frame_rect(
            12.0,
            12.0,
            f.width - 24.0,
            f.height - 24.0,
            2.0,
            gui::Color::rgb(0.16, 0.18, 0.28),
        );

        // The greeting.
        f.text_center(
            f.width * 0.5,
            f.height * 0.22,
            14.0,
            "HELLO, METAL!",
            gui::Color::WHITE,
        );
        f.text_center(
            f.width * 0.5,
            f.height * 0.22 + 110.0,
            3.0,
            "METAL-CPP-RS - 100 GUI EXAMPLES - RAW OBJC ABI, ZERO DEPENDENCIES",
            gui::Color::rgb(0.5, 0.7, 1.0),
        );

        // Echo whatever the user types, with a blinking cursor.
        let typed: String = f.input.text().chars().collect();
        let cursor_visible = (f.time % 1.0) < 0.5;
        let line = if typed.is_empty() {
            String::new()
        } else {
            typed
        };
        let echo = format!("TYPING: {line}");
        f.text_center(
            f.width * 0.5,
            f.height * 0.62,
            4.0,
            &echo,
            gui::Color::YELLOW,
        );
        if cursor_visible {
            let w = f.text_width(4.0, &echo);
            f.rect(
                f.width * 0.5 + w * 0.5 + 6.0,
                f.height * 0.62,
                4.0 * 5.0,
                4.0 * 7.0,
                gui::Color::YELLOW,
            );
        }

        f.text_center(
            f.width * 0.5,
            f.height - 44.0,
            2.0,
            "TYPE TO ECHO - ESC OR CLICK THE RED BUTTON TO QUIT",
            gui::Color::GRAY,
        );
    });
}
