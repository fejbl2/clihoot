use actix::Addr;
use crossterm::event::KeyCode;
use uuid::Uuid;

use log::debug;
use ratatui::widgets::ListState;

use common::{
    constants::PLAYER_KICKED_MESSAGE,
    terminal::{input_utils::move_in_list, terminal_actor::TerminalHandleInput},
};

use crate::{
    messages::lobby::{EndQuestion, KickPlayer, StartQuestion, SwitchToLeaderboard},
    teacher::{
        state::{TeacherTerminalState, WaitingForGameState},
        terminal::TeacherTerminal,
    },
    Lobby,
};

impl TerminalHandleInput for TeacherTerminal {
    fn handle_input(&mut self, key_code: KeyCode) {
        debug!("Key pressed: {:?}", key_code);

        // hide help pop-up if it is visible and any key is pressed
        if self.help_visible {
            self.help_visible = false;
            return;
        }

        if key_code == KeyCode::Char('h') {
            self.help_visible = true;
            return;
        }

        match &mut self.state {
            TeacherTerminalState::StartGame => {
                if key_code == KeyCode::Enter {
                    self.state = TeacherTerminalState::WaitingForGame(WaitingForGameState {
                        list_state: ListState::default().with_selected(Some(0)),
                        kick_popup_visible: false,
                    });
                }
            }
            TeacherTerminalState::WaitingForGame(state) => {
                let mut selected = state.list_state.selected().unwrap_or(0);

                if state.kick_popup_visible {
                    if !self.players.is_empty() {
                        let player_uuid = self.players[selected].uuid;
                        let kicked = handle_kick_player(self.lobby.clone(), key_code, player_uuid);

                        if kicked {
                            state.list_state.select(Some(selected.saturating_sub(1)));
                        }
                    }

                    state.kick_popup_visible = false;
                    return;
                }

                match key_code {
                    KeyCode::Enter => self.lobby.do_send(StartQuestion),
                    KeyCode::Char('x') => {
                        if !self.players.is_empty() {
                            state.kick_popup_visible = true;
                        }
                    }
                    _ => {}
                };

                move_in_list(&mut selected, self.players.len(), key_code);
                state.list_state.select(Some(selected));
            }
            TeacherTerminalState::Question(state) => {
                if state.skip_popup_visible {
                    if let KeyCode::Char('y') = key_code {
                        self.lobby.do_send(EndQuestion {
                            index: state.question.question_index,
                        });
                    }
                    state.skip_popup_visible = false;
                }
                if key_code == KeyCode::Enter {
                    state.skip_popup_visible = true;
                }
            }
            TeacherTerminalState::Answers(_) => {
                if key_code == KeyCode::Enter {
                    self.lobby.do_send(SwitchToLeaderboard);
                }
            }
            TeacherTerminalState::Results(state) => {
                let mut selected = state.table_state.selected().unwrap_or(0);

                if state.kick_popup_visible {
                    if !self.players.is_empty() {
                        let player_uuid = state.results.players[selected].0.uuid;
                        let kicked = handle_kick_player(self.lobby.clone(), key_code, player_uuid);

                        if kicked {
                            state.table_state.select(Some(selected.saturating_sub(1)));
                            state.results.players = state
                                .results
                                .clone()
                                .players
                                .into_iter()
                                .filter(|p| p.0.uuid != player_uuid)
                                .collect();
                        }
                    }

                    state.kick_popup_visible = false;
                    return;
                }

                if key_code == KeyCode::Enter {
                    if state.results.was_final_round {
                        self.state = TeacherTerminalState::EndGame;
                    }
                    self.lobby.do_send(StartQuestion);
                    return;
                }

                if key_code == KeyCode::Char('x') && !state.results.players.is_empty() {
                    state.kick_popup_visible = true;
                }

                move_in_list(&mut selected, self.players.len(), key_code);
                state.table_state.select(Some(selected));
            }
            TeacherTerminalState::EndGame => {
                debug!("EndGame - doing nothing: {:?}", key_code);
            }
            TeacherTerminalState::Error(_) => {
                debug!("Error - doing nothing: {:?}", key_code);
            }
        };
    }
}

fn handle_kick_player(lobby_addr: Addr<Lobby>, key_code: KeyCode, player_uuid: Uuid) -> bool {
    match key_code {
        KeyCode::Char('y') => {
            lobby_addr.do_send(KickPlayer {
                player_uuid,
                reason: Some(PLAYER_KICKED_MESSAGE.to_string()),
            });
            true
        }
        _ => false,
    }
}
