use actix::{Context, Handler};

use crate::{messages::teacher_messages::GetServerState, server::state::Lobby};

/// Handler for Disconnect message.
impl Handler<GetServerState> for Lobby {
    type Result = Lobby;

    fn handle(&mut self, _msg: GetServerState, _: &mut Context<Self>) -> Self::Result {
        self.clone()
    }
}
