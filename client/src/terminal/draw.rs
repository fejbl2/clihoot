use crate::terminal::draw_states::{render_color_selection, render_name_selection};
use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::terminal::render::{self};
use common::terminal::terminal_actor::TerminalDraw;

use ratatui::backend::Backend;
use ratatui::Terminal;

impl TerminalDraw for StudentTerminal {
    fn redraw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()> {
        term.draw(|frame| {
            match &mut self.state {
                StudentTerminalState::StartGame => {
                    render::welcome(frame, &self.quiz_name);
                }
                StudentTerminalState::NameSelection {
                    name,
                    name_already_used,
                } => {
                    render_name_selection(frame, name, *name_already_used, &self.quiz_name);
                }
                StudentTerminalState::ColorSelection { list_state } => {
                    render_color_selection(frame, self.color, list_state, &self.quiz_name);
                }
                StudentTerminalState::WaitingForGame { list_state } => {
                    render::waiting(frame, &mut self.players, list_state, &self.quiz_name);
                }
                StudentTerminalState::Question {
                    question,
                    players_answered_count,
                    answered,
                    start_time: _,
                    duration_from_start,
                    choice_grid,
                    choice_selector_state,
                } => {
                    render::question(
                        frame,
                        question,
                        *players_answered_count,
                        choice_grid,
                        choice_selector_state,
                        duration_from_start.num_seconds() as usize,
                        *answered,
                        self.syntax_theme,
                        &self.quiz_name,
                    );
                }
                StudentTerminalState::Answers { answers } => {
                    render::question_answers(frame, answers, self.syntax_theme, &self.quiz_name);
                }
                StudentTerminalState::Results {
                    results,
                    table_state,
                } => {
                    render::results(frame, results, table_state, &self.quiz_name);
                }

                StudentTerminalState::EndGame {} => {
                    render::end_game(frame, &self.quiz_name);
                }
                StudentTerminalState::Error { message } => {
                    render::error(frame, message, &self.quiz_name);
                }
            };

            if self.help_visible {
                // TODO draw the help pop-up
            }
        })?;

        Ok(())
    }
}
