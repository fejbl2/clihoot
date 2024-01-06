use actix::{ActorContext, Handler};

use crate::{messages::websocket::HardStop, websocket::Websocket};

impl Handler<HardStop> for Websocket {
    type Result = ();

    fn handle(&mut self, _msg: HardStop, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}
