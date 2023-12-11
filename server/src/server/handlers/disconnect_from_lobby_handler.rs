use actix::{Context, Handler};

use crate::{messages::websocket_messages::DisconnectFromLobby, server::state::Lobby};

/// Handler for Disconnect message.
impl Handler<DisconnectFromLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: DisconnectFromLobby, _: &mut Context<Self>) {
        if self.joined_players.remove(&msg.player_id).is_some() {
            println!("{} disconnected", msg.player_id);
            // TODO: send `PlayersUpdate` message
        }
    }
}
