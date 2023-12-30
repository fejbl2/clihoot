use actix::{dev::ContextFutureSpawner, Handler, WrapFuture};
use tungstenite::Message;

use crate::{
    messages::websocket::SendPing,
    websocket::{ws_utils::send_message, Websocket},
};

impl Handler<SendPing> for Websocket {
    type Result = ();

    fn handle(&mut self, _msg: SendPing, ctx: &mut Self::Context) -> Self::Result {
        send_message(self.sender.clone(), Message::Ping(vec![]))
            .into_actor(self)
            .wait(ctx);
    }
}
