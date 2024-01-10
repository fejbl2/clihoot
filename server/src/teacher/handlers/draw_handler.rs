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
        term.draw(|frame| {
            match &mut self.state {
                TeacherTerminalState::StartGame => {
                    render_teacher_welcome(frame, &self.quiz_name);
                }
                TeacherTerminalState::WaitingForGame {
                    list_state,
                    kick_popup_visible: _,
                } => {
                    render::waiting(frame, &mut self.players, list_state, &self.quiz_name);
                }
                TeacherTerminalState::Question {
                    question,
                    players_answered_count,
                    start_time: _,
                    duration_from_start,
                } => {
                    let mut grid: ChoiceGrid = question.question.clone().into();
                    render::question(
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
                }
                TeacherTerminalState::Answers { answers } => {
                    render::question_answers(frame, answers, self.syntax_theme, &self.quiz_name);
                }
                TeacherTerminalState::Results {
                    results,
                    table_state,
                    kick_popup_visible: _,
                } => {
                    render::results(frame, results, table_state, &self.quiz_name);
                }
                TeacherTerminalState::EndGame => {
                    render::end_game(frame, &self.quiz_name);
                }
                TeacherTerminalState::Error { message } => {
                    render::error(frame, message, &self.quiz_name);
                }
            }

            if self.help_visible {
                // TODO draw the help pop-up
            }
        })?;

        Ok(())
    }
}
