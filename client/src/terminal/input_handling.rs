use crate::terminal::constants::COLORS;
use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

use crate::music_actor::SoundEffectMessage;
use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::messages::{
    network::{AnswerSelected, JoinRequest, PlayerData},
    ClientNetworkMessage,
};
use common::terminal::terminal_actor::TerminalHandleInput;

impl TerminalHandleInput for StudentTerminal {
    fn handle_input(&mut self, key_code: KeyCode) -> anyhow::Result<()> {
        match &mut self.state {
            StudentTerminalState::StartGame => {
                if key_code == KeyCode::Enter {
                    self.state = StudentTerminalState::NameSelection {
                        name: String::new(),
                    };
                }
            }
            StudentTerminalState::NameSelection { name } => match key_code {
                KeyCode::Backspace => {
                    name.pop();
                }
                KeyCode::Char(char) => {
                    name.push(char);
                }
                KeyCode::Enter if !name.is_empty() => {
                    self.name = (*name).to_string();
                    self.state = StudentTerminalState::ColorSelection {
                        list_state: ListState::default().with_selected(Some(0)),
                    };
                    self.music_address.do_send(SoundEffectMessage::EnterPressed);
                }
                _ => {}
            },
            StudentTerminalState::ColorSelection { list_state } => {
                let mut selected = list_state.selected().unwrap_or(0);

                match key_code {
                    KeyCode::Backspace => {
                        self.state = StudentTerminalState::NameSelection {
                            name: self.name.to_string(),
                        };
                    }
                    KeyCode::Enter => {
                        self.color = COLORS[list_state.selected().unwrap_or(0)];
                        self.state = StudentTerminalState::WaitingForGame {
                            players: Vec::new(),
                        };
                        self.ws_actor_address
                            .do_send(ClientNetworkMessage::JoinRequest(JoinRequest {
                                player_data: PlayerData {
                                    color: self.color.to_string(),
                                    uuid: self.uuid,
                                    nickname: self.name.to_string(),
                                },
                            }));
                        self.music_address.do_send(SoundEffectMessage::EnterPressed);
                    }
                    KeyCode::Down | KeyCode::Char('j' | 's') => {
                        selected += 1;
                        if selected >= COLORS.len() {
                            selected = 0;
                        }
                        list_state.select(Some(selected));
                        self.music_address.do_send(SoundEffectMessage::Tap);
                    }
                    KeyCode::Up | KeyCode::Char('k' | 'w') => {
                        if selected == 0 {
                            selected = COLORS.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        list_state.select(Some(selected));
                        self.music_address.do_send(SoundEffectMessage::Tap);
                    }
                    _ => {}
                };
            }
            StudentTerminalState::Question {
                question,
                players_answered_count: _,
                answered,
            } => {
                if key_code == KeyCode::Enter {
                    *answered = true;

                    self.ws_actor_address
                        .do_send(ClientNetworkMessage::AnswerSelected(AnswerSelected {
                            player_uuid: self.uuid,
                            question_index: question.question_index,
                            answers: Vec::new(), // TODO send actual answers
                        }))
                }
            }
            _ => {}
        };
        Ok(())
    }
}
