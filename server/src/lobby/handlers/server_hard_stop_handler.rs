use actix::{Context, Handler};
use actix_rt::System;
use log::debug;

use crate::{lobby::state::Lobby, messages::lobby::HardStop};

impl Handler<HardStop> for Lobby {
    type Result = ();

    fn handle(&mut self, _msg: HardStop, _: &mut Context<Self>) {
        debug!("Received HardStop message in Lobby; stopping server");
        System::current().stop();
    }
}
