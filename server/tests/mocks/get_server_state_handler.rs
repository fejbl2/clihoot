use actix::{Context, Handler, Message};
use server::lobby::state::Lobby;

#[derive(Message, Debug)]
#[rtype(result = "Lobby")]
pub struct GetServerState;

/// Should only be used for testing
impl Handler<GetServerState> for Lobby {
    type Result = Lobby;

    fn handle(&mut self, _msg: GetServerState, _: &mut Context<Self>) -> Self::Result {
        self.clone()
    }
}
