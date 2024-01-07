use common::terminal::{render, terminal_actor::TerminalDraw};

use ratatui::prelude::*;

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};

use super::draw_states::{render_teacher_lobby, render_teacher_welcome};

impl TerminalDraw for TeacherTerminal {
    fn redraw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()> {
        match &mut self.state {
            TeacherTerminalState::StartGame => {
                term.draw(|frame| {
                    let _ = render_teacher_welcome(frame);
                })?;
                Ok(())
            }
            TeacherTerminalState::WaitingForGame {
                list_state,
                players,
            } => {
                term.draw(|frame| {
                    let _ = render_teacher_lobby(frame, players, list_state);
                })?;
                Ok(())
            }
            TeacherTerminalState::Question {
                question,
                players_answered_count,
                players: _,
            } => {
                term.draw(|frame| {
                    let _ = render::question_waiting(frame, question, *players_answered_count);
                })?;
                Ok(())
            }
            TeacherTerminalState::Answers {
                answers,
                players: _,
                list_state: _,
                choice_grid,
            } => {
                term.draw(|frame| {
                    let _ = render::question_answers(frame, answers, choice_grid);
                })?;
                Ok(())
            }
            TeacherTerminalState::Results {
                results,
                list_state,
            } => {
                term.draw(|frame| {
                    let _ = render::results(frame, results, list_state);
                })?;
                Ok(())
            }
            TeacherTerminalState::EndGame {
                list_state: _,
                results: _,
            } => {
                term.draw(|frame| {
                    let _ = render::end_game(frame);
                })?;
                Ok(())
            }
            TeacherTerminalState::Error { message } => {
                term.draw(|frame| {
                    let _ = render::error(frame, message);
                })?;
                Ok(())
            }
        }
    }
}
