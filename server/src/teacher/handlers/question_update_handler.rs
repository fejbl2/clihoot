use common::{
    messages::network::QuestionUpdate, terminal::terminal_actor::TerminalHandleQuestionUpdate,
};
use log::debug;

use crate::teacher::terminal::TeacherTerminal;

impl TerminalHandleQuestionUpdate for TeacherTerminal {
    fn handle_question_update(&mut self, _update: QuestionUpdate) -> anyhow::Result<()> {
        debug!("Teacher: handling question update");
        // TODO

        Ok(())
    }
}
