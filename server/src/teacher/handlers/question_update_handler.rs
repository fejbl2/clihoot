use anyhow::bail;
use common::{messages::network::QuestionUpdate, terminal::actor::TerminalHandleQuestionUpdate};
use log::debug;

use crate::teacher::{states::TeacherTerminalState, terminal::TeacherTerminal};

impl TerminalHandleQuestionUpdate for TeacherTerminal {
    fn handle_question_update(&mut self, update: QuestionUpdate) -> anyhow::Result<()> {
        debug!("Teacher: handling question update");

        let TeacherTerminalState::Question(state) = &mut self.state else {
            bail!(
                "Teacher: received question update, but the terminal is not in the Question state"
            );
        };

        if state.question.question_index != update.question_index {
            bail!(
                "Teacher: received question update, but the question index does not match the current question"
            );
        }

        state.players_answered_count = update.players_answered_count;

        Ok(())
    }
}
