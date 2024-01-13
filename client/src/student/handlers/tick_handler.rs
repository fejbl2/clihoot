use common::terminal::actor::TerminalHandleTick;

use crate::student::{states::StudentTerminalState, terminal::StudentTerminal};

impl TerminalHandleTick for StudentTerminal {
    fn handle_tick(&mut self) -> anyhow::Result<()> {
        let StudentTerminalState::Question(state) = &mut self.state else {
            return Ok(());
        };

        let current_time = chrono::Utc::now();

        state.duration_from_start = current_time - state.start_time;

        Ok(())
    }
}
