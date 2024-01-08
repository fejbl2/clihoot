use common::{
    messages::network::PlayersUpdate, terminal::terminal_actor::TerminalHandlePlayersUpdate,
};

use crate::teacher::terminal::TeacherTerminal;
use log::debug;

impl TerminalHandlePlayersUpdate for TeacherTerminal {
    fn handle_players_update(&mut self, update: PlayersUpdate) -> anyhow::Result<()> {
        debug!("Teacher: handling players update");

        self.players = update.players;

        Ok(())
    }
}
