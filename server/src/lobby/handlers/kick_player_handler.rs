use actix::{Context, Handler};

use crate::{
    lobby::state::Lobby,
    messages::{lobby::KickPlayer, websocket::WebsocketGracefulStop},
};

impl Handler<KickPlayer> for Lobby {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: KickPlayer, _: &mut Context<Self>) -> Self::Result {
        let socket = match { self.joined_players.get(&msg.player_uuid) } {
            Some(socket) => socket.addr.clone(),
            None => {
                println!("{} was not found in joined_players", msg.player_uuid);
                return Ok(());
            }
        };

        if self.joined_players.remove(&msg.player_uuid).is_none() {
            return Ok(());
        }

        socket.do_send(WebsocketGracefulStop { reason: msg.reason });

        println!("{} was kicked by teacher", msg.player_uuid);

        self.send_players_update(Some(&msg.player_uuid));

        Ok(())
    }
}
