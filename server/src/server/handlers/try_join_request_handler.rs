use actix::{Context, Handler};
use common::model::network_messages::{CanJoin, TryJoinRequest, TryJoinResponse, LOBBY_LOCKED_MSG};

use crate::server::state::Lobby;

impl Handler<TryJoinRequest> for Lobby {
    type Result = TryJoinResponse;

    fn handle(&mut self, msg: TryJoinRequest, _: &mut Context<Self>) -> Self::Result {
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

        self.waiting_players.push(msg.uuid);

        TryJoinResponse {
            can_join: CanJoin::Yes,
            ..response
        }
    }
}
