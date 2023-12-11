use actix::prelude::*;
use actix::{Actor, Context, Message};

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::cell::RefCell;
use std::rc::Rc;

use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite, WebSocketStream};

use common::model::NetworkMessage;
use serde::Serialize;
use url::Url;

// message used for request to send something server, this message should be passed to websocket actor
#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageForWebsocket<T: Serialize + Send>(pub T);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe(pub Recipient<NetworkMessage>);

// actor which represents a gateway to the server, one can send it a request for sending a message or
// just subscribe for incoming messages
pub struct WebsocketActor {
    ws_stream_tx: Rc<
        RefCell<
            SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::protocol::Message>,
        >,
    >,
    ws_stream_rx: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    subscribers: Vec<Recipient<NetworkMessage>>,
}

impl WebsocketActor {
    pub async fn new(url: Url) -> Option<Self> {
        let (ws_stream, _) = connect_async(url).await.ok()?;

        let (tx, rx) = ws_stream.split();

        Some(WebsocketActor {
            ws_stream_rx: Some(rx),
            ws_stream_tx: Rc::new(RefCell::new(tx)),
            subscribers: vec![],
        })
    }
}

// handler for message requests from another local actors
impl<T: Serialize + Send> Handler<MessageForWebsocket<T>> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: MessageForWebsocket<T>, ctx: &mut Context<Self>) {
        let ws_stream_tx = Rc::clone(&self.ws_stream_tx);

        let serialized_message =
            serde_json::to_string(&msg.0).expect("cannot serialize message for websocket");

        async move {
            println!("Client websocket actor: sending message");
            ws_stream_tx
                .borrow_mut()
                .send(tungstenite::Message::Text(serialized_message))
                .await
                .expect("websocket send failed")
        }
        .into_actor(self)
        .wait(ctx);
    }
}

impl Handler<NetworkMessage> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: NetworkMessage, _: &mut Self::Context) -> Self::Result {
        for sub in self.subscribers.iter() {
            sub.do_send(msg.clone());
        }
    }
}

impl Handler<Subscribe> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        println!("Client websocket actor: subscribing");
        self.subscribers.push(msg.0);
    }
}

impl Actor for WebsocketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Websocket actor is alive");

        let mut ws_stream_rx = self
            .ws_stream_rx
            .take()
            .expect("websocket receiver is None");
        let websocket_actor_address = ctx.address().clone();

        async move {
            // listen for messages from server
            while let Ok(incoming_msg) = ws_stream_rx
                .next()
                .await
                .expect("websocket listening failed")
            {
                let incoming_msg: NetworkMessage = serde_json::from_str(
                    incoming_msg
                        .to_text()
                        .expect("cant convert message to text"),
                )
                .expect("cant deserialize message");

                websocket_actor_address.do_send(MessageForWebsocket(incoming_msg));
            }
            println!("Client websocket closed.");
        }
        .into_actor(self)
        .spawn(ctx);
    }
}
