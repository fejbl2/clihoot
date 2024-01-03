use common::terminal::terminal_actor::TerminalDraw;

use ratatui::prelude::*;

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};

use super::draw_states::{render_end_game, render_error, render_waiting, render_welcome};

// THIS WAS COPY-PASTED FROM CLIENT

impl TerminalDraw for TeacherTerminal {
    fn redraw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()> {
        match &mut self.state {
            TeacherTerminalState::StartGame => {
                term.draw(|frame| {
                    let _ = render_welcome(frame);
                })?;
                Ok(())
            }
            TeacherTerminalState::WaitingForGame {
                list_state,
                players,
            } => {
                term.draw(|frame| {
                    let _ = render_waiting(frame, players, list_state);
                })?;
                Ok(())
            }
            TeacherTerminalState::EndGame => {
                term.draw(|frame| {
                    let _ = render_end_game(frame);
                })?;
                Ok(())
            }
            TeacherTerminalState::Error { message } => {
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
