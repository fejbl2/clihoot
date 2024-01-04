use common::{
    messages::network::PlayersUpdate, terminal::terminal_actor::TerminalHandlePlayersUpdate,
};

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};
use log::debug;

impl TerminalHandlePlayersUpdate for TeacherTerminal {
    fn handle_players_update(&mut self, update: PlayersUpdate) -> anyhow::Result<()> {
        debug!("Teacher: handling players update");

        match &mut self.state {
            TeacherTerminalState::WaitingForGame {
                list_state: _,
                players,
            } => {
                *players = update.players;
            }
            TeacherTerminalState::Question {
                question: _,
                players_answered_count: _,
                players,
            } => {
                *players = update.players;
            }
            TeacherTerminalState::Answers {
                answers: _,
                players,
                list_state: _,
            } => {
                *players = update.players;
            }
            TeacherTerminalState::Results {
                results: _,
                list_state: _,
            } => {}
            TeacherTerminalState::EndGame {
                list_state: _,
                results: _,
            } => {}
            TeacherTerminalState::StartGame {} => {}
            TeacherTerminalState::Error { message: _ } => {}
        }

        Ok(())
    }
}
