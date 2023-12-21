use actix::{ActorContext, Handler};

use crate::{messages::websocket::WebsocketHardStop, websocket::Websocket};

impl Handler<WebsocketHardStop> for Websocket {
    type Result = ();

    fn handle(&mut self, _msg: WebsocketHardStop, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}
