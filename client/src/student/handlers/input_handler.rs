use actix::Addr;
use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

use common::{
    constants::{COLORS, MAXIMAL_NAME_LENGTH},
    messages::{
        network::{AnswerSelected, JoinRequest, PlayerData},
        ClientNetworkMessage,
    },
    terminal::{
        actor::TerminalHandleInput,
        input_utils::move_in_list,
        widgets::choice::{Grid, SelectorState},
    },
};
use uuid::Uuid;

use crate::{
    music_actor::{MusicActor, SoundEffectMessage},
    student::{
        states::{
            ColorSelectionState, NameSelectionState, QuestionState, StudentTerminalState,
            WaitingForGameState,
        },
        terminal::StudentTerminal,
    },
    websocket::WebsocketActor,
};

impl TerminalHandleInput for StudentTerminal {
    #[allow(clippy::too_many_lines)]
    fn handle_input(&mut self, key_code: KeyCode) {
        // hide help pop-up if it is visible and any key is pressed
        if self.help_visible {
            self.help_visible = false;
            return;
        }

        if key_code == KeyCode::Char('h')
            && !matches!(self.state, StudentTerminalState::NameSelection { .. })
        {
            self.music_address.do_send(SoundEffectMessage::Tap);
            self.help_visible = true;
            return;
        }

        match &mut self.state {
            StudentTerminalState::StartGame if key_code == KeyCode::Enter => {
                self.state = StudentTerminalState::NameSelection(NameSelectionState {
                    name: String::new(),
                    name_already_used: false,
                });
            }
            StudentTerminalState::NameSelection(state) => {
                if input_name(
                    &mut state.name,
                    key_code,
                    &self.players,
                    &mut state.name_already_used,
                ) {
                    self.music_address.do_send(SoundEffectMessage::EnterPressed);
                    self.name = (*state.name).to_string();
                    self.state = StudentTerminalState::ColorSelection(ColorSelectionState {
                        list_state: ListState::default().with_selected(Some(0)),
                    });
                }
            }
            StudentTerminalState::ColorSelection(state) => {
                if key_code == KeyCode::Backspace {
                    self.state = StudentTerminalState::NameSelection(NameSelectionState {
                        name: self.name.to_string(),
                        name_already_used: false,
                    });
                    return;
                }

                let mut selected = state.list_state.selected().unwrap_or(0);

                if key_code == KeyCode::Enter {
                    self.music_address.do_send(SoundEffectMessage::EnterPressed);
                    self.color = COLORS[selected];
                    self.state = StudentTerminalState::WaitingForGame(WaitingForGameState {
                        list_state: ListState::default().with_selected(Some(0)),
                    });
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
                state.list_state.select(Some(selected));
                if moved {
                    self.music_address.do_send(SoundEffectMessage::Tap);
                }
            }
            StudentTerminalState::WaitingForGame(state) => {
                let mut selected = state.list_state.selected().unwrap_or(0);
                let moved = move_in_list(&mut selected, self.players.len(), key_code);
                state.list_state.select(Some(selected));

                if moved {
                    self.music_address.do_send(SoundEffectMessage::Tap);
                }
            }
            StudentTerminalState::Question(state) => {
                if (usize::try_from(state.duration_from_start.num_seconds()).unwrap_or(usize::MAX))
                    < state.question.show_choices_after
                {
                    return;
                }

                if state.multichoice_popup_visible {
                    if key_code == KeyCode::Char('y') {
                        self.music_address.do_send(SoundEffectMessage::EnterPressed);
                        state.answered = true;
                        handle_send(&self.ws_actor_address, self.uuid, state);
                    }

                    state.multichoice_popup_visible = false;
                    return;
                }

                if state.answered {
                    return;
                }

                if key_code == KeyCode::Enter {
                    state.answered = true;

                    if state.choice_selector_state.selected().is_empty() {
                        if state.question.is_multichoice {
                            state.multichoice_popup_visible = true;
                            state.answered = false;
                            return;
                        }
                        // allow to send answers quicker in single choice questions
                        state
                            .choice_selector_state
                            .toggle_selection(&state.choice_grid, state.question.is_multichoice);
                    }

                    self.music_address.do_send(SoundEffectMessage::EnterPressed);
                    handle_send(&self.ws_actor_address, self.uuid, state);
                    return;
                }

                move_in_answers(
                    key_code,
                    &mut state.choice_selector_state,
                    &state.choice_grid,
                    state.question.is_multichoice,
                    &self.music_address,
                );
            }
            StudentTerminalState::Results(state) => {
                let mut selected = state.table_state.selected().unwrap_or(0);
                let moved = move_in_list(&mut selected, state.results.players.len(), key_code);
                state.table_state.select(Some(selected));

                if moved {
                    self.music_address.do_send(SoundEffectMessage::Tap);
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
    choice_selector_state: &mut SelectorState,
    choice_grid: &Grid,
    is_multichoice: bool,
    music_address: &Addr<MusicActor>,
) {
    let moved = match key_code {
        KeyCode::Char(' ') => {
            choice_selector_state.toggle_selection(choice_grid, is_multichoice);
            music_address.do_send(SoundEffectMessage::Tap);
            false
        } // spacebar
        KeyCode::Down | KeyCode::Char('s') => {
            choice_selector_state.move_down(choice_grid);
            true
        }
        KeyCode::Up | KeyCode::Char('w') => {
            choice_selector_state.move_up(choice_grid);
            true
        }
        KeyCode::Right | KeyCode::Char('d') => {
            choice_selector_state.move_right(choice_grid);
            true
        }
        KeyCode::Left | KeyCode::Char('a') => {
            choice_selector_state.move_left(choice_grid);
            true
        }
        _ => false,
    };

    if moved {
        music_address.do_send(SoundEffectMessage::Tap);
    }
}

fn handle_send(ws_actor_address: &Addr<WebsocketActor>, uuid: Uuid, state: &mut QuestionState) {
    ws_actor_address.do_send(ClientNetworkMessage::AnswerSelected(AnswerSelected {
        player_uuid: uuid,
        question_index: state.question.question_index,
        answers: state.choice_selector_state.selected(),
    }));
}
