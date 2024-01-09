use crate::terminal::draw_states::{render_color_selection, render_name_selection};
use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::terminal::render::{self};
use common::terminal::terminal_actor::TerminalDraw;

use ratatui::backend::Backend;
use ratatui::Terminal;

impl TerminalDraw for StudentTerminal {
    fn redraw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()> {
        match &mut self.state {
            StudentTerminalState::StartGame => {
                term.draw(|frame| {
                    let _ = render::welcome(frame, &self.quiz_name);
                })?;
                Ok(())
            }
            StudentTerminalState::NameSelection {
                name,
                name_already_used,
            } => {
                term.draw(|frame| {
                    let _ = render_name_selection(frame, name, *name_already_used, &self.quiz_name);
                })?;
                Ok(())
            }
            StudentTerminalState::ColorSelection { list_state } => {
                term.draw(|frame| {
                    let _ = render_color_selection(frame, self.color, list_state, &self.quiz_name);
                })?;
                Ok(())
            }
            StudentTerminalState::WaitingForGame { list_state } => {
                term.draw(|frame| {
                    let _ = render::waiting(frame, &mut self.players, list_state, &self.quiz_name);
                })?;
                Ok(())
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
                term.draw(|frame| {
                    let _ = render::question(
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
                })?;
                Ok(())
            }
            StudentTerminalState::Answers { answers } => {
                term.draw(|frame| {
                    let _ = render::question_answers(frame, answers);
                })?;
                Ok(())
            }
            StudentTerminalState::Results {
                results,
                table_state,
            } => {
                term.draw(|frame| {
                    let _ = render::results(frame, results, table_state, &self.quiz_name);
                })?;
                Ok(())
            }

            StudentTerminalState::EndGame {} => {
                term.draw(|frame| {
                    let _ = render::end_game(frame, &self.quiz_name);
                })?;
                Ok(())
            }
            StudentTerminalState::Error { message } => {
                term.draw(|frame| {
                    let _ = render::error(frame, message, &self.quiz_name);
                })?;
                Ok(())
            }
        }
    }
}
