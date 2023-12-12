use actix::{Context, Handler};
use actix_rt::System;

use crate::{messages::teacher_messages::ServerHardStop, server::state::Lobby};

/// Handler for Disconnect message.
impl Handler<ServerHardStop> for Lobby {
    type Result = ();

    fn handle(&mut self, _msg: ServerHardStop, _: &mut Context<Self>) {
        System::current().stop();
    }
}
