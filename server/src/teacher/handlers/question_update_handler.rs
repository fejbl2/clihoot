use anyhow::bail;
use common::{
    messages::network::QuestionUpdate, terminal::terminal_actor::TerminalHandleQuestionUpdate,
};
use log::debug;

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};

impl TerminalHandleQuestionUpdate for TeacherTerminal {
    fn handle_question_update(&mut self, update: QuestionUpdate) -> anyhow::Result<()> {
        debug!("Teacher: handling question update");

        let TeacherTerminalState::Question {
            question,
            players_answered_count,
            start_time,
            duration_from_start,
        } = &mut self.state
        else {
            bail!(
                "Teacher: received question update, but the terminal is not in the Question state"
            );
        };

        if question.question_index != update.question_index {
            bail!(
                "Teacher: received question update, but the question index does not match the current question"
            );
        }

        *players_answered_count = update.players_answered_count;

        Ok(())
    }
}
