use common::{
    messages::network::NextQuestion, terminal::terminal_actor::TerminalHandleNextQuestion,
};

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};
use log::debug;

impl TerminalHandleNextQuestion for TeacherTerminal {
    fn handle_next_question(&mut self, question: NextQuestion) -> anyhow::Result<()> {
        debug!(
            "Teacher: handling next question number {}",
            question.question_index
        );

        self.state = TeacherTerminalState::Question {
            question,
            players_answered_count: 0,
            players: vec![],
        };

        Ok(())
    }
}
