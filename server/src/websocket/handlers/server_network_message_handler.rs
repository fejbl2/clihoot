use actix::{dev::ContextFutureSpawner, Handler};
use common::model::ServerNetworkMessage;

use crate::websocket::{ws_utils::prepare_message, Websocket};

impl Handler<ServerNetworkMessage> for Websocket {
    type Result = anyhow::Result<()>;

    /// Handles mapping of messages
    /// - lobby --> this function --> the websocket  
    fn handle(&mut self, msg: ServerNetworkMessage, ctx: &mut Self::Context) -> Self::Result {
        let msg = serde_json::to_string(&msg)?;

        prepare_message::<Self>(self.sender.clone(), msg).wait(ctx);

        Ok(())
    }
}
