use actix::{Context, Handler};

use crate::{lobby::state::Lobby, messages::lobby::SetLockMessage};

impl Handler<SetLockMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: SetLockMessage, _: &mut Context<Self>) -> Self::Result {
        println!(
            "Received SetLockMessage in Lobby; setting `locked` to `{}`",
            msg.locked
        );
        self.locked = msg.locked;
    }
}
