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
                    let _ = render::welcome(frame);
                })?;
                Ok(())
            }
            StudentTerminalState::NameSelection {
                name,
                name_already_used,
            } => {
                term.draw(|frame| {
                    let _ = render_name_selection(frame, name, *name_already_used);
                })?;
                Ok(())
            }
            StudentTerminalState::ColorSelection { list_state } => {
                term.draw(|frame| {
                    let _ = render_color_selection(frame, self.color, list_state);
                })?;
                Ok(())
            }
            StudentTerminalState::WaitingForGame { list_state } => {
                term.draw(|frame| {
                    let _ = render::waiting(frame, &mut self.players, list_state);
                })?;
                Ok(())
            }
            StudentTerminalState::Question {
                question,
                players_answered_count,
                answered,
                choice_grid,
                choice_selector_state,
            } => {
                term.draw(|frame| {
                    if *answered {
                        let _ = render::question_waiting(frame, question, *players_answered_count);
                    } else {
                        let _ = render::question(
                            frame,
                            question,
                            *players_answered_count,
                            choice_grid,
                            choice_selector_state,
                        );
                    }
                })?;
                Ok(())
            }
            StudentTerminalState::Answers {
                answers,
                choice_grid,
            } => {
                term.draw(|frame| {
                    let _ = render::question_answers(frame, answers, choice_grid);
                })?;
                Ok(())
            }
            StudentTerminalState::Results {
                results,
                list_state,
            } => {
                term.draw(|frame| {
                    let _ = render::results(frame, results, list_state);
                })?;
                Ok(())
            }

            StudentTerminalState::EndGame {} => {
                term.draw(|frame| {
                    let _ = render::end_game(frame);
                })?;
                Ok(())
            }
            StudentTerminalState::Error { message } => {
                term.draw(|frame| {
                    let _ = render::error(frame, message);
                })?;
                Ok(())
            }
        }
    }
}
