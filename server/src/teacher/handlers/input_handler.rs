use actix::Addr;
use crossterm::event::KeyCode;
use uuid::Uuid;

use common::constants::PLAYER_KICKED_MESSAGE;
use common::terminal::terminal_actor::TerminalHandleInput;
use log::debug;
use ratatui::widgets::ListState;

use crate::{
    messages::lobby::{EndQuestion, KickPlayer, StartQuestion, SwitchToLeaderboard},
    teacher::terminal::{TeacherTerminal, TeacherTerminalState},
    Lobby,
};

impl TerminalHandleInput for TeacherTerminal {
    fn handle_input(&mut self, key_code: KeyCode) -> anyhow::Result<()> {
        debug!("Key pressed: {:?}", key_code);
        match &mut self.state {
            TeacherTerminalState::StartGame {} => {
                if key_code == KeyCode::Enter {
                    self.state = TeacherTerminalState::WaitingForGame {
                        list_state: ListState::default().with_selected(Some(0)),
                        kick_popup_visible: false,
                    };
                }
            }
            TeacherTerminalState::WaitingForGame {
                list_state,
                kick_popup_visible,
            } => {
                let mut selected = list_state.selected().unwrap_or(0);

                if *kick_popup_visible {
                    let player_uuid = self.players[selected].uuid;
                    let Some(kicked) =
                        handle_kick_player(self.lobby.clone(), key_code, player_uuid)
                    else {
                        return Ok(());
                    };

                    if kicked {
                        list_state.select(Some(selected.saturating_sub(1)));
                    }
                    *kick_popup_visible = false;
                    return Ok(());
                }

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
                    KeyCode::Char('x') => *kick_popup_visible = true,
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
            TeacherTerminalState::Answers { answers: _ } => {
                if key_code == KeyCode::Enter {
                    self.lobby.do_send(SwitchToLeaderboard);
                }
            }
            TeacherTerminalState::Results {
                results,
                table_state,
                kick_popup_visible,
            } => {
                let mut selected = table_state.selected().unwrap_or(0);

                if *kick_popup_visible {
                    let player_uuid = results.players[selected].0.uuid;
                    let Some(kicked) =
                        handle_kick_player(self.lobby.clone(), key_code, player_uuid)
                    else {
                        return Ok(());
                    };

                    if kicked {
                        table_state.select(Some(selected.saturating_sub(1)));
                        results.players = results
                            .clone()
                            .players
                            .into_iter()
                            .filter(|p| p.0.uuid != player_uuid)
                            .collect();
                    }

                    *kick_popup_visible = false;
                    return Ok(());
                }

                match key_code {
                    KeyCode::Enter => self.lobby.do_send(StartQuestion),
                    KeyCode::Down | KeyCode::Char('j' | 's') => {
                        selected += 1;
                        if selected >= results.players.len() {
                            selected = 0;
                        }
                        table_state.select(Some(selected));
                    }
                    KeyCode::Up | KeyCode::Char('k' | 'w') => {
                        if selected == 0 {
                            selected = results.players.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        table_state.select(Some(selected));
                    }
                    KeyCode::Char('x') => *kick_popup_visible = true,
                    _ => {}
                };
            }
            TeacherTerminalState::EndGame => {
                debug!("EndGame - doing nothing: {:?}", key_code);
            }
            TeacherTerminalState::Error { message: _ } => {
                debug!("Error - doing nothing: {:?}", key_code);
            }
        };
        Ok(())
    }
}

fn handle_kick_player(
    lobby_addr: Addr<Lobby>,
    key_code: KeyCode,
    player_uuid: Uuid,
) -> Option<bool> {
    match key_code {
        KeyCode::Char('y') => {
            lobby_addr.do_send(KickPlayer {
                player_uuid,
                reason: Some(PLAYER_KICKED_MESSAGE.to_string()),
            });
            Some(true)
        }
        KeyCode::Char('n') => Some(false),
        _ => None,
    }
}
