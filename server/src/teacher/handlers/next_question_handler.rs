use common::{messages::network::NextQuestion, terminal::actor::TerminalHandleNextQuestion};

use crate::teacher::{
    states::{QuestionState, TeacherTerminalState},
    terminal::TeacherTerminal,
};
use log::debug;

impl TerminalHandleNextQuestion for TeacherTerminal {
    fn handle_next_question(&mut self, question: NextQuestion) -> anyhow::Result<()> {
        debug!(
            "Teacher: handling next question number {}",
            question.question_index
        );

        self.state = TeacherTerminalState::Question(QuestionState {
            question,
            players_answered_count: 0,
            start_time: chrono::Utc::now(),
            duration_from_start: chrono::Duration::zero(),
            skip_popup_visible: false,
        });

        Ok(())
    }
}
