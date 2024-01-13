use actix::prelude::*;
use actix::{Actor, Context, Message};

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use log::{debug, info, warn};
use std::cell::RefCell;
use std::rc::Rc;

use url::Url;
use uuid::Uuid;

use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite, WebSocketStream};
use tungstenite::Error::ConnectionClosed;

use common::{
    messages::{
        network::{self, CanJoin::No, TryJoinRequest},
        status::ClientWebsocketStatus,
        ClientNetworkMessage, ServerNetworkMessage,
        ServerNetworkMessage::TryJoinResponse,
    },
    terminal::highlight::Theme,
};

use crate::{music_actor::MusicActor, student::terminal::run_student};

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Subscribe(pub Recipient<ServerNetworkMessage>);

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct SubscribeStatus(pub Recipient<ClientWebsocketStatus>);

type Sender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::protocol::Message>;
type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

// actor which represents a gateway to the server, one can send it a request for sending a message or
// just subscribe for incoming messages
#[allow(clippy::module_name_repetitions)]
pub struct WebsocketActor {
    ws_stream_tx: Rc<RefCell<Sender>>,
    ws_stream_rx: Option<Receiver>,
    subscribers_network_messages: Vec<Recipient<ServerNetworkMessage>>,
    subscribers_status: Vec<Recipient<ClientWebsocketStatus>>,
    music_actor_addr: Addr<MusicActor>,
    syntax_theme: Theme,
}

impl WebsocketActor {
    pub async fn new(
        url: Url,
        uuid: Uuid,
        music_actor_addr: Addr<MusicActor>,
        syntax_theme: Theme,
    ) -> anyhow::Result<Self> {
        let (ws_stream, _) = connect_async(url).await?;

        let (tx, rx) = ws_stream.split();
        let tx_rc = Rc::new(RefCell::new(tx));

        send_message_directly(
            tx_rc.clone(),
            ClientNetworkMessage::TryJoinRequest(TryJoinRequest { uuid }),
        )
        .await?;

        Ok(WebsocketActor {
            ws_stream_rx: Some(rx),
            ws_stream_tx: tx_rc,
            subscribers_network_messages: vec![],
            subscribers_status: vec![],
            music_actor_addr,
            syntax_theme,
        })
    }

    fn handle_try_join_response(
        &mut self,
        message: ServerNetworkMessage,
        ctx: &mut <WebsocketActor as Actor>::Context,
    ) {
        // check if the message is the type we are looking for, otherwise ignore
        let TryJoinResponse(network::TryJoinResponse {
            uuid,
            can_join,
            quiz_name,
        }) = message
        else {
            return;
        };

        if let No(reason) = can_join {
            info!("server does not allow us to join, reason: {}", reason);
        }

        if let Ok(student_actor_addr) = run_student(
            uuid,
            quiz_name,
            ctx.address(),
            &self.music_actor_addr,
            self.syntax_theme,
        ) {
            // register student actor for network messages
            ctx.notify(Subscribe(student_actor_addr.clone().recipient()));

            // register student actor for status messages
            ctx.notify(SubscribeStatus(student_actor_addr.recipient()));
        };
    }
}

// handler for message requests from another local actors
impl Handler<ClientNetworkMessage> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: ClientNetworkMessage, ctx: &mut Context<Self>) {
        let ws_stream_tx = Rc::clone(&self.ws_stream_tx);
        send_message(ws_stream_tx, msg, ctx.address())
            .into_actor(self)
            .wait(ctx);
    }
}

/**
This function try to send message to the server using websocket and if it fails, it will send message to the terminal actor.
*/
async fn send_message(
    stream_tx: Rc<RefCell<Sender>>,
    message: ClientNetworkMessage,
    my_address: Addr<WebsocketActor>,
) {
    if let Err(_error) = send_message_directly(stream_tx, message).await {
        debug!("websocket failed to send message");
        my_address.do_send(ClientWebsocketStatus::CantSendMessage);
    }
}

// I was not able to fix this.. I admit my weakness ... :-(
#[allow(clippy::await_holding_refcell_ref)]
async fn send_message_directly(
    stream_tx: Rc<RefCell<Sender>>,
    message: ClientNetworkMessage,
) -> anyhow::Result<()> {
    let serialized_message = serde_json::to_string(&message)?;

    debug!("client websocket actor: sending message");

    stream_tx
        .borrow_mut()
        .send(tungstenite::Message::Text(serialized_message))
        .await?;

    Ok(())
}

impl Handler<ServerNetworkMessage> for WebsocketActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ServerNetworkMessage, ctx: &mut Self::Context) -> Self::Result {
        debug!("get message from server: {:?}", msg);

        self.handle_try_join_response(msg.clone(), ctx);

        for sub in &self.subscribers_network_messages {
            sub.do_send(msg.clone());
        }

        Ok(())
    }
}

impl Handler<ClientWebsocketStatus> for WebsocketActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ClientWebsocketStatus, ctx: &mut Self::Context) -> Self::Result {
        debug!("get status message: {:?}", msg);

        for sub in &self.subscribers_status {
            sub.do_send(msg.clone());
        }

        match msg {
            ClientWebsocketStatus::ListeningFail
            | ClientWebsocketStatus::CantSendMessage
            | ClientWebsocketStatus::SocketClosed
            | ClientWebsocketStatus::CloseFrameReceived(_) => {
                ctx.stop(); // stop websocket actor
            }
        }
        Ok(())
    }
}

impl Handler<Subscribe> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        debug!("new subscribe request for network messages from: {:?}", msg);
        self.subscribers_network_messages.push(msg.0);
    }
}

impl Handler<SubscribeStatus> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: SubscribeStatus, _: &mut Self::Context) {
        debug!("new subscribe request for error messages from: {:?}", msg);
        self.subscribers_status.push(msg.0);
    }
}

impl Actor for WebsocketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("starting websocket actor");

        let ws_stream_rx = self
            .ws_stream_rx
            .take()
            .expect("websocket receiver is None"); // this cant fail if it is correctly programmed
        let websocket_actor_address = ctx.address().clone();

        async move {
            if let Err(_error) =
                listen_for_messages(ws_stream_rx, websocket_actor_address.clone()).await
            {
                warn!("websocket failed listening");
                websocket_actor_address.do_send(ClientWebsocketStatus::ListeningFail);
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
    while let Ok(incoming_msg) = rx_stream.next().await.ok_or(ConnectionClosed)? {
        match incoming_msg {
            tungstenite::Message::Text(text_msg) => {
                let deserialized_msg: ServerNetworkMessage =
                    serde_json::from_str(text_msg.as_str())?;
                websocket_actor_address.do_send(deserialized_msg);
            }
            tungstenite::Message::Close(content) => {
                let close_reason = match content {
                    None => "Reason not specified.".to_string(),
                    Some(content_some) => content_some.reason.to_string(),
                };

                // close the connection
                websocket_actor_address
                    .do_send(ClientWebsocketStatus::CloseFrameReceived(close_reason));
                return Ok(());
            }
            _ => {}
        }
    }
    info!("client websocket closed.");
    websocket_actor_address.do_send(ClientWebsocketStatus::SocketClosed);
    Ok(())
}
