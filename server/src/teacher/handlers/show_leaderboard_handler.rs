use common::{
    messages::network::ShowLeaderboard, terminal::terminal_actor::TerminalHandleShowLeaderboard,
};

use crate::teacher::terminal::TeacherTerminal;
use log::debug;

impl TerminalHandleShowLeaderboard for TeacherTerminal {
    fn handle_show_leaderboard(&mut self, _show: ShowLeaderboard) -> anyhow::Result<()> {
        debug!("Teacher: handling show leaderboard");
        // TODO

        Ok(())
    }
}
