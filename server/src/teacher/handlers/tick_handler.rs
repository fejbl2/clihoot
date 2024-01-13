use crate::teacher::{states::TeacherTerminalState, terminal::TeacherTerminal};

impl common::terminal::actor::TerminalHandleTick for TeacherTerminal {
    fn handle_tick(&mut self) -> anyhow::Result<()> {
        let TeacherTerminalState::Question(state) = &mut self.state else {
            return Ok(());
        };

        let current_time = chrono::Utc::now();
        state.duration_from_start = current_time - state.start_time;

        Ok(())
    }
}
