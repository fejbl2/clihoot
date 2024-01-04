use actix::{Context, Handler};

use crate::{lobby::state::Lobby, messages::websocket::DisconnectFromLobby};
use log::info;

/// Handler for Disconnect message.
impl Handler<DisconnectFromLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: DisconnectFromLobby, _: &mut Context<Self>) {
        if self.joined_players.remove(&msg.player_id).is_some() {
            info!("{} disconnected", msg.player_id);

            let _ = self.send_players_update(Some(&msg.player_id));
        }
    }
}
