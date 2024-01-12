use common::terminal::render;
use ratatui::Frame;

pub fn render_teacher_help(frame: &mut Frame) {
    let help_text = [
        ("ENTER", "Move to the next state"),
        ("CTRL C", "Exit the game"),
        ("x", "Kick a player"),
        ("h", "Show this help"),
        ("↑↓ | ws", "Move up and down"),
    ];
    render::help(frame, &help_text);
}

pub fn render_kick_popup(frame: &mut Frame) {
    let message = "Are you sure you want to kick this player?\n They will not be able to rejoin";
    render::yes_no_popup(frame, message);
}

pub fn render_skip_question_popup(frame: &mut Frame) {
    let message = "Are you sure you want to skip this question?\n Players who haven't answered will get 0 points";
    render::yes_no_popup(frame, message);
}
