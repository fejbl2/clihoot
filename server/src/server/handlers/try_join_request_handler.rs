use actix::{Context, Handler};
use common::model::network_messages::{CanJoin, TryJoinRequest, TryJoinResponse};

use crate::server::state::Lobby;

impl Handler<TryJoinRequest> for Lobby {
    type Result = anyhow::Result<TryJoinResponse>;

    fn handle(&mut self, msg: TryJoinRequest, _: &mut Context<Self>) -> Self::Result {
        if self.locked {
            return Err(anyhow::anyhow!("Lobby is locked"));
        }

        self.waiting_players.push(msg.uuid);

        Ok(TryJoinResponse {
            uuid: msg.uuid,
            can_join: CanJoin::Yes,
            quiz_name: self.questions.quiz_name.clone(),
        })
    }
}
