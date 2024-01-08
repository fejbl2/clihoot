use crossterm::event::KeyCode;

use common::constants::PLAYER_KICKED_MESSAGE;
use common::terminal::terminal_actor::TerminalHandleInput;
use log::debug;
use ratatui::widgets::ListState;

use crate::{
    messages::lobby::{EndQuestion, KickPlayer, StartQuestion, SwitchToLeaderboard},
    teacher::terminal::{TeacherTerminal, TeacherTerminalState},
};

impl TerminalHandleInput for TeacherTerminal {
    fn handle_input(&mut self, key_code: KeyCode) -> anyhow::Result<()> {
        debug!("Key pressed: {:?}", key_code);
        match &mut self.state {
            TeacherTerminalState::StartGame {} => {
                if key_code == KeyCode::Enter {
                    self.state = TeacherTerminalState::WaitingForGame {
                        list_state: ListState::default().with_selected(Some(0)),
                    };
                }
            }
            TeacherTerminalState::WaitingForGame { list_state } => {
                let mut selected = list_state.selected().unwrap_or(0);

                match key_code {
                    KeyCode::Enter => self.lobby.do_send(StartQuestion),
                    KeyCode::Down | KeyCode::Char('j' | 's') => {
                        selected += 1;
                        if selected >= self.players.len() {
                            selected = 0;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Up | KeyCode::Char('k' | 'w') => {
                        if selected == 0 {
                            selected = self.players.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Char('x') => {
                        let selected = list_state.selected().unwrap_or(0);

                        self.lobby.do_send(KickPlayer {
                            player_uuid: self.players[selected].uuid,
                            reason: Some(PLAYER_KICKED_MESSAGE.to_string()),
                        });
                        list_state.select(Some(selected.saturating_sub(1)));
                    }
                    _ => {}
                };
            }
            TeacherTerminalState::Question {
                question: q,
                players_answered_count: _,
                start_time: _,
                duration_from_start: _,
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
            } => {
                if key_code == KeyCode::Enter {
                    self.lobby.do_send(SwitchToLeaderboard);
                }
            }
            TeacherTerminalState::Results {
                results,
                list_state,
            } => {
                let mut selected = list_state.selected().unwrap_or(0);

                match key_code {
                    KeyCode::Enter => self.lobby.do_send(StartQuestion),
                    KeyCode::Down | KeyCode::Char('j' | 's') => {
                        selected += 1;
                        if selected >= results.players.len() {
                            selected = 0;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Up | KeyCode::Char('k' | 'w') => {
                        if selected == 0 {
                            selected = results.players.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Char('x') => {
                        let selected = list_state.selected().unwrap_or(0);

                        let player_uuid = results.players[selected].0.uuid;
                        self.lobby.do_send(KickPlayer {
                            player_uuid,
                            reason: Some(PLAYER_KICKED_MESSAGE.to_string()),
                        });
                        list_state.select(Some(selected.saturating_sub(1)));
                        results.players = results
                            .clone()
                            .players
                            .into_iter()
                            .filter(|p| p.0.uuid != player_uuid)
                            .collect();
                    }
                    _ => {}
                };
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
