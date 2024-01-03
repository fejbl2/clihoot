use common::{
    messages::network::QuestionEnded, terminal::terminal_actor::TerminalHandleQuestionEnded,
};
use log::debug;

use crate::teacher::terminal::TeacherTerminal;

impl TerminalHandleQuestionEnded for TeacherTerminal {
    fn handle_question_ended(&mut self, _question_ended: QuestionEnded) -> anyhow::Result<()> {
        debug!("Teacher: handling question ended");
        // TODO

        Ok(())
    }
}
