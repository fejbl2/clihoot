use actix::{Context, Handler};
use actix_rt::System;
use log::debug;

use crate::{messages::lobby::HardStop, Lobby};

impl Handler<HardStop> for Lobby {
    type Result = ();

    fn handle(&mut self, _msg: HardStop, _: &mut Context<Self>) {
        debug!("Received HardStop message in Lobby; stopping server");
        System::current().stop();
    }
}
