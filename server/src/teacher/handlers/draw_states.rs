use common::{messages::network::PlayerData, terminal::render};
use ratatui::{widgets::ListState, Frame};

pub fn render_teacher_welcome(frame: &mut Frame) -> anyhow::Result<()> {
    render::simple_message(
        frame,
        "Welcome!".to_string(),
        "To start the game press ENTER",
    )?;

    Ok(())
}

pub fn render_teacher_lobby(
    frame: &mut Frame,
    players: &mut [PlayerData],
    list_state: &mut ListState,
) -> anyhow::Result<()> {
    render::waiting(frame, players, list_state)?;

    Ok(())
}
