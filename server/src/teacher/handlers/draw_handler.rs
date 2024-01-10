use common::terminal::{
    render,
    terminal_actor::TerminalDraw,
    widgets::choice::{ChoiceGrid, ChoiceSelectorState},
};

use ratatui::prelude::*;

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};

use super::draw_states::render_teacher_welcome;

impl TerminalDraw for TeacherTerminal {
    fn redraw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()> {
        match &mut self.state {
            TeacherTerminalState::StartGame => {
                term.draw(|frame| {
                    let _ = render_teacher_welcome(frame, &self.quiz_name);
                })?;
                Ok(())
            }
            TeacherTerminalState::WaitingForGame { list_state } => {
                term.draw(|frame| {
                    let _ = render::waiting(frame, &mut self.players, list_state, &self.quiz_name);
                })?;
                Ok(())
            }
            TeacherTerminalState::Question {
                question,
                players_answered_count,
                start_time: _,
                duration_from_start,
            } => {
                term.draw(|frame| {
                    let mut grid: ChoiceGrid = question.question.clone().into();
                    let _ = render::question(
                        frame,
                        question,
                        *players_answered_count,
                        &mut grid,
                        &mut ChoiceSelectorState::default(),
                        duration_from_start.num_seconds() as usize,
                        false,
                        self.syntax_theme,
                        &self.quiz_name,
                    );
                })?;
                Ok(())
            }
            TeacherTerminalState::Answers {
                answers,
                list_state: _,
            } => {
                term.draw(|frame| {
                    let _ = render::question_answers(
                        frame,
                        answers,
                        self.syntax_theme,
                        &self.quiz_name,
                    );
                })?;
                Ok(())
            }
            TeacherTerminalState::Results {
                results,
                table_state,
            } => {
                term.draw(|frame| {
                    let _ = render::results(frame, results, table_state, &self.quiz_name);
                })?;
                Ok(())
            }
            TeacherTerminalState::EndGame => {
                term.draw(|frame| {
                    let _ = render::end_game(frame, &self.quiz_name);
                })?;
                Ok(())
            }
            TeacherTerminalState::Error { message } => {
                term.draw(|frame| {
                    let _ = render::error(frame, message, &self.quiz_name);
                })?;
                Ok(())
            }
        }
    }
}
