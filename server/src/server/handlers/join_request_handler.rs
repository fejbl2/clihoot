use actix::Handler;

use crate::server::{client_messages::JoinRequest, state::Lobby};

impl Handler<JoinRequest> for Lobby {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: JoinRequest, _ctx: &mut Self::Context) -> Self::Result {
        if self.locked {
            return Err(anyhow::anyhow!("Lobby is locked"));
        }

        let id = msg.player_data.uuid;
        if !self.waiting_players.contains(&id) {
            return Err(anyhow::anyhow!("Player {id} not in waiting list"));
        }

        self.waiting_players.retain(|&x| x != id);
        self.joined_players.insert(id, msg.ws_conn);

        Ok(())
    }
}
