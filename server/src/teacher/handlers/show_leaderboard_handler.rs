use common::{messages::network::ShowLeaderboard, terminal::actor::TerminalHandleShowLeaderboard};
use ratatui::widgets::TableState;

use crate::teacher::{
    states::{ResultsState, TeacherTerminalState},
    terminal::TeacherTerminal,
};
use log::debug;

impl TerminalHandleShowLeaderboard for TeacherTerminal {
    fn handle_show_leaderboard(&mut self, show: ShowLeaderboard) -> anyhow::Result<()> {
        debug!("Teacher: handling show leaderboard");

        self.state = TeacherTerminalState::Results(ResultsState {
            results: show,
            table_state: TableState::default().with_selected(Some(0)),
            kick_popup_visible: false,
        });

        Ok(())
    }
}
