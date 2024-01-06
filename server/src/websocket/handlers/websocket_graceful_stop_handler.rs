use std::borrow::Cow;

use actix::{dev::ContextFutureSpawner, AsyncContext, Handler};
use common::constants::DEFAULT_GOODBYE_MESSAGE;
use tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

use crate::{
    messages::websocket::{GracefulStop, HardStop},
    websocket::{prepare_explicit_message, Websocket},
};

impl Handler<GracefulStop> for Websocket {
    type Result = ();

    fn handle(&mut self, msg: GracefulStop, ctx: &mut Self::Context) -> Self::Result {
        let reason = msg
            .reason
            .unwrap_or_else(|| DEFAULT_GOODBYE_MESSAGE.to_owned());

        // also send close message to the client
        let msg = Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from(reason),
        }));

        // send a goodbye message
        prepare_explicit_message::<Self>(self.sender.clone(), msg).wait(ctx);

        ctx.notify(HardStop {});
    }
}
