use crossterm::event::KeyCode;

use common::terminal::terminal_actor::TerminalHandleInput;

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};

impl TerminalHandleInput for TeacherTerminal {
    fn handle_input(&mut self, _key_code: KeyCode) -> anyhow::Result<()> {
        match &mut self.state {
            TeacherTerminalState::WaitingForGame {
                list_state: _,
                players: _,
            } => {
                // TODO
            }
            TeacherTerminalState::Question {
                question: _,
                players_answered_count: _,
                answered: _,
                players: _,
            } => {
                // TODO
            }

            // TODO other branches
            _ => {}
        };
        Ok(())
    }
}
