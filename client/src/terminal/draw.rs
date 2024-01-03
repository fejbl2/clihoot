use crate::terminal::draw_states::{
    render_color_selection, render_end_game, render_error, render_name_selection, render_waiting,
    render_welcome,
};
use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::terminal::terminal_actor::TerminalDraw;
use ratatui::backend::Backend;
use ratatui::Terminal;

impl TerminalDraw for StudentTerminal {
    fn redraw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()> {
        match &mut self.state {
            StudentTerminalState::StartGame => {
                term.draw(|frame| {
                    let _ = render_welcome(frame);
                })?;
                Ok(())
            }
            StudentTerminalState::NameSelection { name } => {
                term.draw(|frame| {
                    let _ = render_name_selection(frame, name, true);
                })?;
                Ok(())
            }
            StudentTerminalState::ColorSelection { list_state } => {
                term.draw(|frame| {
                    let _ = render_color_selection(frame, self.color, list_state);
                })?;
                Ok(())
            }
            StudentTerminalState::WaitingForGame {
                list_state,
                players,
            } => {
                term.draw(|frame| {
                    let _ = render_waiting(frame, players, list_state);
                })?;
                Ok(())
            }
            StudentTerminalState::EndGame => {
                term.draw(|frame| {
                    let _ = render_end_game(frame);
                })?;
                Ok(())
            }
            StudentTerminalState::Error { message } => {
                term.draw(|frame| {
                    let _ = render_error(frame, message);
                })?;
                Ok(())
            }
            _ => {
                term.draw(|frame| {
                    let _ = render_error(frame, "The state is not implemented yet");
                })?;
                Ok(())
            }
        }
    }
}
