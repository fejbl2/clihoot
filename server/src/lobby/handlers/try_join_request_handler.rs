use actix::{Context, Handler};
use common::{
    constants::LOBBY_LOCKED_MSG,
    messages::network::{CanJoin, TryJoinRequest, TryJoinResponse},
};
use log::debug;

use crate::Lobby;

impl Handler<TryJoinRequest> for Lobby {
    type Result = TryJoinResponse;

    fn handle(&mut self, msg: TryJoinRequest, _: &mut Context<Self>) -> Self::Result {
        debug!("Received TryJoinRequest message in Lobby; trying to join");
        let response = TryJoinResponse {
            uuid: msg.uuid,
            can_join: CanJoin::No(String::new()),
            quiz_name: self.questions.quiz_name.clone(),
        };

        if self.locked {
            return TryJoinResponse {
                can_join: CanJoin::No(LOBBY_LOCKED_MSG.to_owned()),
                ..response
            };
        }

        self.waiting_players.insert(msg.uuid);

        TryJoinResponse {
            can_join: CanJoin::Yes,
            ..response
        }
    }
}
