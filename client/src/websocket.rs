use actix::prelude::*;
use actix::{Actor, Context, Message};

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::cell::RefCell;
use std::rc::Rc;

use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite, WebSocketStream};
use tungstenite::Error::ConnectionClosed;

use common::model;
use common::model::network_messages::TryJoinRequest;
use common::model::ServerNetworkMessage::TryJoinResponse;
use common::model::{ClientNetworkMessage, ServerNetworkMessage};
use url::Url;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe(pub Recipient<ServerNetworkMessage>);

// actor which represents a gateway to the server, one can send it a request for sending a message or
// just subscribe for incoming messages
pub struct WebsocketActor {
    ws_stream_tx: Rc<
        RefCell<
            SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::protocol::Message>,
        >,
    >,
    ws_stream_rx: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    subscribers: Vec<Recipient<ServerNetworkMessage>>,
}

impl WebsocketActor {
    pub async fn new(url: Url) -> anyhow::Result<Self> {
        let (ws_stream, _) = connect_async(url).await?;

        let (tx, rx) = ws_stream.split();
        let tx_rc = Rc::new(RefCell::new(tx));

        send_message_directly(
            tx_rc.clone(),
            ClientNetworkMessage::TryJoinRequest(TryJoinRequest {
                uuid: Uuid::new_v4(),
            }),
        )
        .await?;

        Ok(WebsocketActor {
            ws_stream_rx: Some(rx),
            ws_stream_tx: tx_rc,
            subscribers: vec![],
        })
    }
}

// handler for message requests from another local actors
impl Handler<ClientNetworkMessage> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: ClientNetworkMessage, ctx: &mut Context<Self>) {
        let ws_stream_tx = Rc::clone(&self.ws_stream_tx);
        send_message(ws_stream_tx, msg).into_actor(self).wait(ctx);
    }
}

/**
This function try to send message to the server using websocket and if it fails, it will send message to the terminal actor.
*/
async fn send_message(
    stream_tx: Rc<
        RefCell<
            SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::protocol::Message>,
        >,
    >,
    message: ClientNetworkMessage,
) {
    if let Err(_error) = send_message_directly(stream_tx, message).await {
        // todo: send message to the terminal actor that websocket is failing
    }
}

// I was not able to fix this.. I admit my weakness ... :-(
#[allow(clippy::await_holding_refcell_ref)]
async fn send_message_directly(
    stream_tx: Rc<
        RefCell<
            SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::protocol::Message>,
        >,
    >,
    message: ClientNetworkMessage,
) -> anyhow::Result<()> {
    let serialized_message = serde_json::to_string(&message)?;

    println!("Client websocket actor: sending message");

    stream_tx
        .borrow_mut()
        .send(tungstenite::Message::Text(serialized_message))
        .await?;

    Ok(())
}

impl Handler<ServerNetworkMessage> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: ServerNetworkMessage, _: &mut Self::Context) -> Self::Result {
        if let TryJoinResponse(model::network_messages::TryJoinResponse {}) = msg {
            // todo: spawn terminal actor
            // TODO register terminal actor at websocket
            /*addr_websocket_actor
            .send(Subscribe(addr_client.recipient()))
            .await
            .unwrap();*/

            return;
        }

        for sub in &self.subscribers {
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

        let ws_stream_rx = self
            .ws_stream_rx
            .take()
            .expect("websocket receiver is None"); // this cant fail if it is correctly programmed
        let websocket_actor_address = ctx.address().clone();

        async move {
            if let Err(_error) = listen_for_messages(ws_stream_rx, websocket_actor_address).await {
                // todo: send message to the terminal actor that listening on websocket failed.
            }
        }
        .into_actor(self)
        .spawn(ctx);
    }
}

async fn listen_for_messages(
    mut rx_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    websocket_actor_address: Addr<WebsocketActor>,
) -> anyhow::Result<()> {
    // listen for messages from server
    while let Ok(incoming_msg) = rx_stream.next().await.map_or(Err(ConnectionClosed), Ok)? {
        match incoming_msg {
            tungstenite::Message::Text(text_msg) => {
                let deserialized_msg: ServerNetworkMessage =
                    serde_json::from_str(text_msg.as_str())?;
                websocket_actor_address.do_send(deserialized_msg);
            }
            tungstenite::Message::Close(_) => {
                // close the connection
                return Ok(());
            }
            _ => {}
        }
    }
    println!("Client websocket closed.");
    Ok(())
}
