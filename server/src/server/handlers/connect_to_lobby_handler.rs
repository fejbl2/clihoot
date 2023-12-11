use actix::{Context, Handler};

use crate::{messages::websocket_messages::ConnectToLobby, server::state::Lobby};

impl Handler<ConnectToLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ConnectToLobby, _: &mut Context<Self>) -> Self::Result {
        // save info that new client joined
        self.joined_players.insert(msg.player_id, msg.addr);
        println!("{} joined", msg.player_id);

        // TODO: remove - just for testing
        self.send_message(&format!("your id is {}", msg.player_id), &msg.player_id);

        // send to all other clients that new client joined
        self.send_to_other(&format!("{} joined", msg.player_id), &msg.player_id, false);
    }
}
