use common::{
    messages::network::ShowLeaderboard, terminal::terminal_actor::TerminalHandleShowLeaderboard,
};
use ratatui::widgets::ListState;

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};
use log::debug;

impl TerminalHandleShowLeaderboard for TeacherTerminal {
    fn handle_show_leaderboard(&mut self, show: ShowLeaderboard) -> anyhow::Result<()> {
        debug!("Teacher: handling show leaderboard");

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        self.state = TeacherTerminalState::Results {
            results: show,
            list_state,
        };

        Ok(())
    }
}
