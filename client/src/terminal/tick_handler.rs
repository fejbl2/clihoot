use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::terminal::terminal_actor::TerminalHandleTick;

impl TerminalHandleTick for StudentTerminal {
    fn handle_tick(&mut self) -> anyhow::Result<()> {
        let StudentTerminalState::Question {
            question: _,
            players_answered_count: _,
            answered: _,
            start_time,
            duration_from_start,
            choice_grid: _,
            choice_selector_state: _,
        } = &mut self.state
        else {
            return Ok(());
        };

        let current_time = chrono::Utc::now();

        *duration_from_start = current_time - *start_time;

        Ok(())
    }
}
