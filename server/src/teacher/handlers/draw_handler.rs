use common::{
    constants::{
        MINIMAL_QUESTION_HEIGHT, MINIMAL_QUESTION_WIDTH, MINIMAL_SCREEN_HEIGHT,
        MINIMAL_SCREEN_WIDTH,
    },
    terminal::{
        render,
        terminal_actor::TerminalDraw,
        widgets::choice::{ChoiceGrid, ChoiceSelectorState},
    },
};

use ratatui::prelude::*;

use crate::teacher::{
    draw_states::{render_kick_popup, render_skip_question_popup, render_teacher_help},
    state::TeacherTerminalState,
    terminal::TeacherTerminal,
};

impl TerminalDraw for TeacherTerminal {
    fn redraw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()> {
        term.draw(|frame| {
            if frame.size().height < MINIMAL_SCREEN_HEIGHT
                || frame.size().width < MINIMAL_SCREEN_WIDTH
            {
                render::resize(
                    frame,
                    &self.quiz_name,
                    MINIMAL_SCREEN_HEIGHT,
                    MINIMAL_SCREEN_WIDTH,
                );

                return;
            }

            match &mut self.state {
                TeacherTerminalState::StartGame => {
                    render::welcome(frame, &self.quiz_name);
                }
                TeacherTerminalState::WaitingForGame(state) => {
                    render::waiting(
                        frame,
                        &mut self.players,
                        &mut state.list_state,
                        None,
                        &self.quiz_name,
                    );
                    if state.kick_popup_visible {
                        render_kick_popup(frame);
                    }
                }
                TeacherTerminalState::Question(state) => {
                    if frame.size().height < MINIMAL_QUESTION_HEIGHT
                        || frame.size().width < MINIMAL_QUESTION_WIDTH
                    {
                        render::resize(
                            frame,
                            &self.quiz_name,
                            MINIMAL_QUESTION_HEIGHT,
                            MINIMAL_QUESTION_WIDTH,
                        );
                    } else {
                        let mut grid: ChoiceGrid = state.question.question.clone().into();
                        render::question(
                            frame,
                            &state.question,
                            state.players_answered_count,
                            &mut grid,
                            &mut ChoiceSelectorState::empty(),
                            state.duration_from_start.num_seconds() as usize,
                            false,
                            self.syntax_theme,
                            &self.quiz_name,
                        );

                        if state.skip_popup_visible {
                            render_skip_question_popup(frame);
                        }
                    }
                }
                TeacherTerminalState::Answers(state) => {
                    if frame.size().height < MINIMAL_QUESTION_HEIGHT
                        || frame.size().width < MINIMAL_QUESTION_WIDTH
                    {
                        render::resize(
                            frame,
                            &self.quiz_name,
                            MINIMAL_QUESTION_HEIGHT,
                            MINIMAL_QUESTION_WIDTH,
                        );
                    } else {
                        render::question_answers(
                            frame,
                            &state.answers,
                            self.syntax_theme,
                            &self.quiz_name,
                        );
                    }
                }
                TeacherTerminalState::Results(state) => {
                    render::results(
                        frame,
                        &state.results,
                        &mut state.table_state,
                        None,
                        &self.quiz_name,
                    );
                    if state.kick_popup_visible {
                        render_kick_popup(frame);
                    }
                }
                TeacherTerminalState::EndGame => {
                    render::end_game(frame, &self.quiz_name);
                }
                TeacherTerminalState::Error(state) => {
                    render::error(frame, &state.message, &self.quiz_name);
                }
            }

            if self.help_visible {
                render_teacher_help(frame);
            }
        })?;

        Ok(())
    }
}
