use actix::prelude::*;
use actix::{Actor, Context};
use serde::Serialize;
use uuid::Uuid;

use crate::websocket::{MessageFromServer, WebsocketActor, WebsocketMsg};

pub struct ClientActor {
    pub websocket_astor_addr: Addr<WebsocketActor>,
}

impl ClientActor {
    // send message to the clihoot server
    fn send_message_to_server<T: Serialize + Send + Clone + 'static>(&self, message: T) {
        self.websocket_astor_addr
            .do_send(WebsocketMsg { content: message.clone() });
    }
}

impl Actor for ClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Client actor started.");

        self.send_message_to_server(common::model::network_messages::TryJoinRequest {
            uuid: Uuid::new_v4(),
        })
    }
}

impl Handler<MessageFromServer> for ClientActor {
    type Result = ();

    // handle messages from server
    fn handle(&mut self, msg: MessageFromServer, ctx: &mut Context<Self>) -> Self::Result {
        println!("Message from server: {}", msg.content);
    }
}
