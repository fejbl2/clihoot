use actix::Handler;
use common::{
    constants::LOBBY_LOCKED_MSG,
    model::network_messages::{CanJoin, JoinResponse},
};

use crate::{
    messages::client_messages::JoinRequest,
    server::state::{JoinedPlayer, Lobby},
};

impl Handler<JoinRequest> for Lobby {
    type Result = JoinResponse;

    fn handle(&mut self, msg: JoinRequest, _ctx: &mut Self::Context) -> Self::Result {
        let result = JoinResponse {
            uuid: msg.player_data.uuid,
            quiz_name: self.questions.quiz_name.clone(),
            can_join: CanJoin::No(String::new()),
            players: self.get_players(),
        };

        if self.locked {
            return JoinResponse {
                can_join: CanJoin::No(LOBBY_LOCKED_MSG.to_owned()),
                ..result
            };
        }

        let id = msg.player_data.uuid;
        if !self.waiting_players.contains(&id) {
            return JoinResponse {
                can_join: CanJoin::No("Player not in waiting list".to_owned()),
                ..result
            };
        }

        if self
            .joined_players
            .values()
            .any(|x| x.nickname == msg.player_data.nickname)
        {
            return JoinResponse {
                can_join: CanJoin::No("Nickname already taken".to_owned()),
                ..result
            };
        }

        self.waiting_players.retain(|&x| x != id);
        self.joined_players.insert(
            id,
            JoinedPlayer {
                addr: msg.addr,
                color: msg.player_data.color,
                nickname: msg.player_data.nickname,
                uuid: msg.player_data.uuid,
                joined_at: chrono::Utc::now(),
            },
        );

        // do NOT send update to the player that just joined
        self.send_players_update(Some(&id));

        JoinResponse {
            can_join: CanJoin::Yes,
            players: self.get_players(),
            ..result
        }
    }
}
