use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

use crate::terminal::student::{Color, StudentTerminal, StudentTerminalState};
use common::terminal::terminal_actor::TerminalHandleInput;

const COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

impl TerminalHandleInput for StudentTerminal {
    fn handle_input(&mut self, key_code: KeyCode) -> anyhow::Result<()> {
        // TODO define function that handle input for each state
        match &mut self.state {
            StudentTerminalState::NameSelection { name } => match key_code {
                KeyCode::Backspace => {
                    name.pop();
                }
                KeyCode::Char(char) => {
                    name.push(char);
                }
                KeyCode::Enter => {
                    self.name = (*name).to_string();
                    self.state = StudentTerminalState::ColorSelection {
                        list_state: ListState::default().with_selected(Some(0)),
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
                        };
                    }
                    KeyCode::Enter => {
                        self.color = COLORS[list_state.selected().unwrap_or(0)];
                        self.state = StudentTerminalState::Todo;
                    }
                    KeyCode::Down | KeyCode::Char('j' | 's') => {
                        selected += 1;
                        if selected >= 3 {
                            selected = 0;
                        }
                        list_state.select(Some(selected));
                    }
                    KeyCode::Up | KeyCode::Char('k' | 'w') => {
                        if selected == 0 {
                            selected = 2;
                        } else {
                            selected -= 1;
                        }
                        list_state.select(Some(selected));
                    }
                    _ => {}
                };
            }
            StudentTerminalState::Todo => {}
        };
        Ok(())
    }
}
