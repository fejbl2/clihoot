use actix::Addr;
use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

use common::{
    constants::MAXIMAL_NAME_LENGTH,
    messages::{
        network::{AnswerSelected, JoinRequest, PlayerData},
        ClientNetworkMessage,
    },
    terminal::{
        input_utils::move_in_list,
        terminal_actor::TerminalHandleInput,
        widgets::choice::{ChoiceGrid, ChoiceSelectorState},
    },
};

use crate::{
    music_actor::{MusicActor, SoundEffectMessage},
    terminal::{
        constants::COLORS,
        student::{StudentTerminal, StudentTerminalState},
    },
};

impl TerminalHandleInput for StudentTerminal {
    fn handle_input(&mut self, key_code: KeyCode) {
        // hide help pop-up if it is visible and any key is pressed
        if self.help_visible {
            self.help_visible = false;
            return;
        }

        if key_code == KeyCode::Char('h')
            && !matches!(self.state, StudentTerminalState::NameSelection { .. })
        {
            self.help_visible = true;
            return;
        }

        match &mut self.state {
            StudentTerminalState::StartGame if key_code == KeyCode::Enter => {
                self.state = StudentTerminalState::NameSelection {
                    name: String::new(),
                    name_already_used: false,
                };
            }
            StudentTerminalState::NameSelection {
                name,
                name_already_used,
            } => {
                if input_name(name, key_code, &self.players, name_already_used) {
                    self.music_address.do_send(SoundEffectMessage::EnterPressed);
                    self.name = (*name).to_string();
                    self.state = StudentTerminalState::ColorSelection {
                        list_state: ListState::default().with_selected(Some(0)),
                    }
                }
            }
            StudentTerminalState::ColorSelection { list_state } => {
                if key_code == KeyCode::Backspace {
                    self.state = StudentTerminalState::NameSelection {
                        name: self.name.to_string(),
                        name_already_used: false,
                    };
                    return;
                }

                let mut selected = list_state.selected().unwrap_or(0);

                if key_code == KeyCode::Enter {
                    self.music_address.do_send(SoundEffectMessage::EnterPressed);
                    self.color = COLORS[selected];
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
                    return;
                }

                let moved = move_in_list(&mut selected, COLORS.len(), key_code);
                list_state.select(Some(selected));
                if moved {
                    self.music_address.do_send(SoundEffectMessage::Tap)
                }
            }
            StudentTerminalState::WaitingForGame { list_state } => {
                let mut selected = list_state.selected().unwrap_or(0);
                let moved = move_in_list(&mut selected, self.players.len(), key_code);
                list_state.select(Some(selected));

                if moved {
                    self.music_address.do_send(SoundEffectMessage::Tap)
                }
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
                    return;
                }

                if key_code == KeyCode::Enter {
                    self.music_address.do_send(SoundEffectMessage::EnterPressed);
                    *answered = true;

                    // allow to send answers quicker in singlechoice questions
                    if !question.is_multichoice && choice_selector_state.selected().is_empty() {
                        choice_selector_state.toggle_selection(choice_grid, question.is_multichoice)
                    }

                    self.ws_actor_address
                        .do_send(ClientNetworkMessage::AnswerSelected(AnswerSelected {
                            player_uuid: self.uuid,
                            question_index: question.question_index,
                            answers: choice_selector_state.selected(),
                        }));
                    return;
                }

                move_in_answers(
                    key_code,
                    choice_selector_state,
                    choice_grid,
                    question.is_multichoice,
                    self.music_address.clone(),
                );
            }
            StudentTerminalState::Results {
                results,
                table_state: list_state,
            } => {
                let mut selected = list_state.selected().unwrap_or(0);
                let moved = move_in_list(&mut selected, results.players.len(), key_code);
                list_state.select(Some(selected));

                if moved {
                    self.music_address.do_send(SoundEffectMessage::Tap)
                }
            }
            _ => {}
        };
    }
}

fn name_in_players(name: &str, players: &[PlayerData]) -> bool {
    players.iter().any(|player| player.nickname == name)
}

fn empty_name(name: &str) -> bool {
    name.trim().is_empty()
}

fn input_name(
    name: &mut String,
    key_code: KeyCode,
    players: &[PlayerData],
    name_used: &mut bool,
) -> bool {
    match key_code {
        KeyCode::Backspace => {
            name.pop();
            *name_used = name_in_players(name, players);
            false
        }
        KeyCode::Char(char) => {
            if name.chars().count() < MAXIMAL_NAME_LENGTH {
                name.push(char);
                *name_used = name_in_players(name, players);
            }
            false
        }
        KeyCode::Enter if !empty_name(name) => {
            *name_used = name_in_players(name, players);
            !*name_used
        }
        _ => false,
    }
}

fn move_in_answers(
    key_code: KeyCode,
    choice_selector_state: &mut ChoiceSelectorState,
    choice_grid: &ChoiceGrid,
    is_multichoice: bool,
    music_address: Addr<MusicActor>,
) {
    match key_code {
        KeyCode::Char(' ') => choice_selector_state.toggle_selection(choice_grid, is_multichoice), // spacebar
        KeyCode::Down | KeyCode::Char('s') => {
            music_address.do_send(SoundEffectMessage::Tap);
            choice_selector_state.move_down(choice_grid);
        }
        KeyCode::Up | KeyCode::Char('w') => {
            music_address.do_send(SoundEffectMessage::Tap);
            choice_selector_state.move_up(choice_grid);
        }
        KeyCode::Right | KeyCode::Char('d') => {
            music_address.do_send(SoundEffectMessage::Tap);
            choice_selector_state.move_right(choice_grid);
        }
        KeyCode::Left | KeyCode::Char('a') => {
            music_address.do_send(SoundEffectMessage::Tap);
            choice_selector_state.move_left(choice_grid);
        }
        _ => {}
    };
}
