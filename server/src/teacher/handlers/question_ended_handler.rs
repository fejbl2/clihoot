use anyhow::bail;
use common::{messages::network::QuestionEnded, terminal::actor::TerminalHandleQuestionEnded};
use log::debug;

use crate::teacher::{
    states::{AnswersState, TeacherTerminalState},
    terminal::TeacherTerminal,
};

impl TerminalHandleQuestionEnded for TeacherTerminal {
    fn handle_question_ended(&mut self, question_ended: QuestionEnded) -> anyhow::Result<()> {
        debug!("Teacher: handling question ended");

        let question = match &self.state {
            TeacherTerminalState::Question(state) => &state.question,
            _ => bail!(
                "Teacher: received question ended, but the terminal is not in the Question state"
            ),
        };

        if question.question_index != question_ended.question_index {
            bail!(
                "Teacher: received question ended, but the question index does not match the current question"
            );
        }

        self.state = TeacherTerminalState::Answers(AnswersState {
            answers: question_ended,
        });

        Ok(())
    }
}
