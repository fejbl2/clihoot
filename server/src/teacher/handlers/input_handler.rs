use crossterm::event::KeyCode;

use common::terminal::terminal_actor::TerminalHandleInput;
use log::debug;
use ratatui::widgets::ListState;

use crate::{
    messages::lobby::{EndQuestion, StartQuestion, SwitchToLeaderboard},
    teacher::terminal::{TeacherTerminal, TeacherTerminalState},
};

impl TerminalHandleInput for TeacherTerminal {
    fn handle_input(&mut self, key_code: KeyCode) -> anyhow::Result<()> {
        debug!("Key pressed: {:?}", key_code);
        match &mut self.state {
            TeacherTerminalState::StartGame {} => {
                if key_code == KeyCode::Enter {
                    self.state = TeacherTerminalState::WaitingForGame {
                        list_state: ListState::default(),
                    };
                }
            }
            TeacherTerminalState::WaitingForGame { list_state: _ } => {
                if key_code == KeyCode::Enter {
                    self.lobby.do_send(StartQuestion);
                }
            }
            TeacherTerminalState::Question {
                question: q,
                players_answered_count: _,
            } => {
                if key_code == KeyCode::Enter {
                    self.lobby.do_send(EndQuestion {
                        index: q.question_index,
                    });
                }
            }
            TeacherTerminalState::Answers {
                answers: _,
                list_state: _,
                choice_grid: _,
            } => {
                if key_code == KeyCode::Enter {
                    self.lobby.do_send(SwitchToLeaderboard);
                }
            }
            TeacherTerminalState::Results {
                results: _,
                list_state: _,
            } => {
                if key_code == KeyCode::Enter {
                    self.lobby.do_send(StartQuestion);
                }
            }
            TeacherTerminalState::EndGame {
                list_state: _,
                results: _,
            } => {
                debug!("EndGame - doing nothing: {:?}", key_code);
            }
            TeacherTerminalState::Error { message: _ } => {
                debug!("Error - doing nothing: {:?}", key_code);
            }
        };
        Ok(())
    }
}
