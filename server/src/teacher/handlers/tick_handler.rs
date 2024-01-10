use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};

impl common::terminal::terminal_actor::TerminalHandleTick for TeacherTerminal {
    fn handle_tick(&mut self) -> anyhow::Result<()> {
        let TeacherTerminalState::Question {
            question: _,
            players_answered_count: _,
            start_time,
            duration_from_start,
            skip_popup_visible: _,
        } = &mut self.state
        else {
            return Ok(());
        };

        let current_time = chrono::Utc::now();
        *duration_from_start = current_time - *start_time;

        Ok(())
    }
}
