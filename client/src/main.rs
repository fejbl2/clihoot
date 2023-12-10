use crate::client::ClientActor;
use crate::websocket::{Subscribe, WebsocketActor};
use actix::Actor;
use anyhow::Result;

mod client;
mod websocket;

fn main() -> Result<()> {
    let sys = actix::System::new();

    sys.block_on(async {
        // start websocket actor
        let addr_websocket_actor = WebsocketActor::new().await.start();

        // start our dummy client actor
        let addr_client = ClientActor {
            websocket_astor_addr: addr_websocket_actor.clone(),
        }
        .start();

        // lets set our dummy actor as a subscriber for incoming messages from server
        addr_websocket_actor
            .send(Subscribe(addr_client.recipient()))
            .await
            .unwrap();
    });
    sys.run()?;

    Ok(())
}
