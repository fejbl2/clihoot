use actix::{Context, Handler};

use crate::{lobby::state::Lobby, messages::websocket::DisconnectFromLobby};

/// Handler for Disconnect message.
impl Handler<DisconnectFromLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: DisconnectFromLobby, _: &mut Context<Self>) {
        if self.joined_players.remove(&msg.player_id).is_some() {
            println!("{} disconnected", msg.player_id);

            self.send_players_update(Some(&msg.player_id));
        }
    }
}
