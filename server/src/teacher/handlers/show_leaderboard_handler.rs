use common::{
    messages::network::ShowLeaderboard, terminal::terminal_actor::TerminalHandleShowLeaderboard,
};
use ratatui::widgets::TableState;

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};
use log::debug;

impl TerminalHandleShowLeaderboard for TeacherTerminal {
    fn handle_show_leaderboard(&mut self, show: ShowLeaderboard) -> anyhow::Result<()> {
        debug!("Teacher: handling show leaderboard");

        self.state = TeacherTerminalState::Results {
            results: show,
            table_state: TableState::default().with_selected(Some(0)),
            kick_popup_visible: false,
        };

        Ok(())
    }
}
