use actix::{AsyncContext, Handler};
use common::model::network_messages::KickedOutNotice;

use crate::websocket::{ws_utils::prepare_message, Websocket};

impl Handler<KickedOutNotice> for Websocket {
    type Result = anyhow::Result<()>;
    fn handle(&mut self, msg: KickedOutNotice, ctx: &mut Self::Context) -> Self::Result {
        let msg = serde_json::to_string(&msg)?;
        let msg = prepare_message(self.sender.clone(), msg);

        ctx.spawn(msg);
        Ok(())
    }
}
