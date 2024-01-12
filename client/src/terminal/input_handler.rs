use crate::terminal::constants::COLORS;
use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

use crate::music_actor::SoundEffectMessage;
use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::terminal::terminal_actor::TerminalHandleInput;
use common::{
    constants::MAXIMAL_NAME_LENGTH,
    messages::{
        network::{AnswerSelected, JoinRequest, PlayerData},
        ClientNetworkMessage,
    },
};

fn name_in_players(name: &str, players: &[PlayerData]) -> bool {
    players.iter().any(|player| player.nickname == name)
}

impl TerminalHandleInput for StudentTerminal {
    fn handle_input(&mut self, key_code: KeyCode) -> anyhow::Result<()> {
        // hide help pop-up if it is visible and any key is pressed
        if self.help_visible {
            self.help_visible = false;
            return Ok(());
        }

        if key_code == KeyCode::Char('h')
            && !matches!(self.state, StudentTerminalState::NameSelection { .. })
        {
            self.help_visible = true;
            return Ok(());
        }

        match &mut self.state {
            StudentTerminalState::StartGame => {
                if key_code == KeyCode::Enter {
                    self.state = StudentTerminalState::NameSelection {
                        name: String::new(),
                        name_already_used: false,
                    };
                }
            }
            StudentTerminalState::NameSelection {
                name,
                name_already_used,
            } => match key_code {
                KeyCode::Backspace => {
                    name.pop();
                    *name_already_used = name_in_players(name, &self.players);
                }
                KeyCode::Char(char) => {
                    if name.chars().count() < MAXIMAL_NAME_LENGTH {
                        name.push(char);
                        *name_already_used = name_in_players(name, &self.players);
                    }
                }
                KeyCode::Enter if !name.is_empty() => {
                    *name_already_used = name_in_players(name, &self.players);
                    if !*name_already_used {
                        self.music_address.do_send(SoundEffectMessage::EnterPressed);
                        self.name = (*name).to_string();
                        self.state = StudentTerminalState::ColorSelection {
                            list_state: ListState::default().with_selected(Some(0)),
                        }
                    }
                }
                _ => {}
            },
            StudentTerminalState::ColorSelection { list_state } => {
                let mut selected = list_state.selected().unwrap_or(0);

                match key_code {
                    KeyCode::Backspace => {
                        self.state = StudentTerminalState::NameSelection {
                            name: self.name.to_string(),
                            name_already_used: false,
                        };
                    }
                    KeyCode::Enter => {
                        self.music_address.do_send(SoundEffectMessage::EnterPressed);
                        self.color = COLORS[list_state.selected().unwrap_or(0)];
                        self.state = StudentTerminalState::WaitingForGame {
                            list_state: ListState::default().with_selected(Some(0)),
                        };
                        self.ws_actor_address
                            .do_send(ClientNetworkMessage::JoinRequest(JoinRequest {
                                player_data: PlayerData {
                                    color: self.color,
                                    uuid: self.uuid,
                                    nickname: self.name.to_string(),
                                },
                            }));
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        selected += 1;
                        if selected >= COLORS.len() {
                            selected = 0;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        if selected == 0 {
                            selected = COLORS.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        list_state.select(Some(selected));
                    }
                    _ => {}
                };
            }
            StudentTerminalState::WaitingForGame { list_state } => {
                let mut selected = list_state.selected().unwrap_or(0);

                match key_code {
                    KeyCode::Down | KeyCode::Char('s') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        selected += 1;
                        if selected >= self.players.len() {
                            selected = 0;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        if selected == 0 {
                            selected = self.players.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        list_state.select(Some(selected));
                    }
                    _ => {}
                };
            }
            StudentTerminalState::Question {
                question,
                players_answered_count: _,
                answered,
                start_time: _,
                duration_from_start,
                choice_grid,
                choice_selector_state,
            } => {
                if (duration_from_start.num_seconds() as usize) < question.show_choices_after {
                    return Ok(());
                }

                match key_code {
                    KeyCode::Enter => {
                        self.music_address.do_send(SoundEffectMessage::EnterPressed);
                        *answered = true;

                        // allow to send answers quicker in singlechoice questions
                        if !question.is_multichoice && choice_selector_state.selected().is_empty() {
                            choice_selector_state
                                .toggle_selection(choice_grid, question.is_multichoice)
                        }

                        self.ws_actor_address
                            .do_send(ClientNetworkMessage::AnswerSelected(AnswerSelected {
                                player_uuid: self.uuid,
                                question_index: question.question_index,
                                answers: choice_selector_state.selected(),
                            }));
                    }
                    KeyCode::Char(' ') => {
                        choice_selector_state.toggle_selection(choice_grid, question.is_multichoice)
                    } // spacebar
                    KeyCode::Down | KeyCode::Char('s') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        choice_selector_state.move_down(choice_grid);
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        choice_selector_state.move_up(choice_grid);
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        choice_selector_state.move_right(choice_grid);
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        choice_selector_state.move_left(choice_grid);
                    }
                    _ => {}
                };
            }
            StudentTerminalState::Results {
                results,
                table_state: list_state,
            } => {
                let mut selected = list_state.selected().unwrap_or(0);
                match key_code {
                    KeyCode::Down | KeyCode::Char('s') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        selected += 1;
                        if selected >= results.players.len() {
                            selected = 0;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        self.music_address.do_send(SoundEffectMessage::Tap);
                        if selected == 0 {
                            selected = results.players.len() - 1;
                        } else {
                            selected -= 1;
                        }
                        list_state.select(Some(selected));
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }
}
