use ratatui::backend::Backend;
use ratatui::Terminal;

use common::{
    constants::{
        MINIMAL_QUESTION_HEIGHT, MINIMAL_QUESTION_WIDTH, MINIMAL_SCREEN_HEIGHT,
        MINIMAL_SCREEN_WIDTH,
    },
    terminal::{
        render::{self},
        terminal_actor::TerminalDraw,
    },
};

use crate::student::{
    draw_states::{render_color_selection, render_help, render_name_selection},
    state::StudentTerminalState,
    terminal::StudentTerminal,
};

impl TerminalDraw for StudentTerminal {
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
                StudentTerminalState::StartGame => {
                    render::welcome(frame, &self.quiz_name);
                }
                StudentTerminalState::NameSelection(state) => {
                    render_name_selection(frame, state, &self.quiz_name);
                }
                StudentTerminalState::ColorSelection(state) => {
                    render_color_selection(frame, state, &self.quiz_name);
                }
                StudentTerminalState::WaitingForGame(state) => {
                    render::waiting(
                        frame,
                        &mut self.players,
                        &mut state.list_state,
                        Some(self.uuid),
                        &self.quiz_name,
                    );
                }
                StudentTerminalState::Question(state) => {
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
                        render::question(
                            frame,
                            &state.question,
                            state.players_answered_count,
                            &mut state.choice_grid,
                            &mut state.choice_selector_state,
                            state.duration_from_start.num_seconds() as usize,
                            state.answered,
                            self.syntax_theme,
                            &self.quiz_name,
                        );
                    }
                }
                StudentTerminalState::Answers(state) => {
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
                StudentTerminalState::Results(state) => {
                    render::results(
                        frame,
                        &state.results,
                        &mut state.table_state,
                        Some(self.uuid),
                        &self.quiz_name,
                    );
                }

                StudentTerminalState::EndGame => {
                    render::end_game(frame, &self.quiz_name);
                }
                StudentTerminalState::Error(state) => {
                    render::error(frame, &state.message, &self.quiz_name);
                }
            };

            if self.help_visible {
                render_help(frame);
            }
        })?;

        Ok(())
    }
}
