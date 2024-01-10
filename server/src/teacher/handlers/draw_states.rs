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
