use std::borrow::Cow;

use actix::{dev::ContextFutureSpawner, AsyncContext, Handler};
use tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

use crate::{
    messages::websocket_messages::{WebsocketGracefulStop, WebsocketHardStop},
    websocket::{ws_utils::prepare_explicit_message, Websocket},
};

impl Handler<WebsocketGracefulStop> for Websocket {
    type Result = ();

    fn handle(&mut self, _msg: WebsocketGracefulStop, ctx: &mut Self::Context) -> Self::Result {
        // also send close message to the client
        let msg = Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from("Goodbye"),
        }));

        // send a goodbye message
        prepare_explicit_message::<Self>(self.sender.clone(), msg).wait(ctx);

        ctx.notify(WebsocketHardStop {});
    }
}
