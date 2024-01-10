use common::terminal::render;
use ratatui::Frame;

pub fn render_teacher_welcome(frame: &mut Frame, quiz_name: &str) {
    render::simple_message(
        frame,
        " Welcome! ",
        "To start the game press ENTER",
        quiz_name,
    );
}

pub fn render_teacher_help(frame: &mut Frame) {
    let help_text = [
        ("ENTER", "Move to the next state"),
        ("CTRL C", "Exit the game"),
        ("x", "Kick a player"),
        ("h", "Show this help"),
        ("↑↓ | jk | ws", "Move up and down"),
    ];
    render::help(frame, &help_text);
}
