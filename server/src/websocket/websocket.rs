use actix::{fut, ActorContext, ActorFutureExt};
use actix::{Actor, Addr, ContextFutureSpawner, Running, WrapFuture};
use actix::{AsyncContext, Handler};
use common::model::network_messages::KickedOutNotice;
use common::model::NetworkMessage;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use tokio::sync::Mutex;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;

use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use tungstenite::Message;
use uuid::Uuid;

use crate::messages::client_messages::JoinRequest;
use crate::messages::websocket_messages::{
    ConnectToLobby, DisconnectFromLobby, LobbyOutputMessage, WsGracefulCloseConnection,
    WsHardCloseConnection,
};
use crate::server::state::Lobby;
use crate::websocket::ws_utils::prepare_explicit_message;

use super::ws_utils::prepare_message;

pub struct Websocket {
    lobby_addr: Addr<Lobby>,

    player_id: Uuid,

    receiver: Option<SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>>,

    sender: Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>,

    reader_task: Option<JoinHandle<()>>,

    who: SocketAddr,
}

impl Websocket {
    pub async fn new(
        lobby: Addr<Lobby>,
        socket: TcpStream,
        who: SocketAddr,
    ) -> anyhow::Result<Websocket> {
        let socket = tokio_tungstenite::accept_async(socket).await?;

        let (sender, receiver) = socket.split();

        Ok(Websocket {
            player_id: Uuid::new_v4(),
            lobby_addr: lobby,
            receiver: Some(receiver),
            sender: Arc::new(Mutex::new(sender)),
            reader_task: None,
            who,
        })
    }
}

impl Actor for Websocket {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // this is my address
        let addr = ctx.address();

        // First tell the boss that we have a new connection
        self.lobby_addr
            .send(ConnectToLobby {
                addr: addr.clone(),
                player_id: self.player_id,
            })
            .into_actor(self)
            // If we get a response back, then we're good to go
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);

        // in order not to move self into the closure, we need to extract the fields we need
        let who = self.who;
        let receiver = self.receiver.take().expect("Could not take receiver"); // take ownership of the receiver, expect is fine

        // Spawn a Tokio task which will read from the socket and generate messages for this actor
        let reader_task = tokio::spawn(read_messages_from_socket(receiver, who, addr));
        self.reader_task = Some(reader_task);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        if let Some(reader_task) = &self.reader_task {
            reader_task.abort();
        }

        self.lobby_addr.do_send(DisconnectFromLobby {
            player_id: self.player_id,
        });
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Stopped WsConn for {}", self.who);
    }
}

async fn read_messages_from_socket<'a>(
    mut receiver: SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>,
    who: SocketAddr,
    addr: Addr<Websocket>,
) {
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(msg) => {
                println!("Received text message from {who}: {msg}");

                // try to parse the JSON s to a `NetworkMessage`
                match serde_json::from_str::<NetworkMessage>(&msg) {
                    Ok(msg) => {
                        addr.do_send(msg);
                    }
                    Err(e) => println!("Error parsing message: {e}"),
                }
            }
            Message::Close(_) => {
                // cannot call `ctx.stop();` because we are in another Task:
                // instead, we send a message to ourselves to stop
                addr.do_send(WsHardCloseConnection {});

                // also quit the loop
                return;
            }
            _ => (),
        }
    }
}

impl Handler<WsHardCloseConnection> for Websocket {
    type Result = ();

    fn handle(&mut self, _msg: WsHardCloseConnection, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl Handler<WsGracefulCloseConnection> for Websocket {
    type Result = ();

    fn handle(&mut self, _msg: WsGracefulCloseConnection, ctx: &mut Self::Context) -> Self::Result {
        // also send close message to the client
        println!("Sending close to {}...", self.who);

        let msg = Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from("Goodbye"),
        }));

        // send a goodbye message
        prepare_explicit_message::<Self>(self.sender.clone(), msg).wait(ctx);

        ctx.notify(WsHardCloseConnection {});
    }
}

impl Handler<LobbyOutputMessage> for Websocket {
    type Result = ();

    fn handle(&mut self, msg: LobbyOutputMessage, ctx: &mut Self::Context) -> Self::Result {
        let fut = prepare_message::<Self>(self.sender.clone(), msg.0);
        ctx.spawn(fut);
    }
}

impl Handler<NetworkMessage> for Websocket {
    type Result = ();

    /// Handles mapping of messages
    /// - the websocket --> this function --> lobby
    /// - Unimplemented stuff are messages that the client should never send us
    fn handle(&mut self, msg: NetworkMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            NetworkMessage::AnswerSelected(msg) => self.lobby_addr.do_send(msg),
            NetworkMessage::ClientDisconnected(_msg) => {
                ctx.notify(WsGracefulCloseConnection {});
            }
            NetworkMessage::JoinRequest(msg) => self.lobby_addr.do_send(JoinRequest {
                player_data: msg.player_data,
                ws_conn: ctx.address(),
            }),
            NetworkMessage::TryJoinRequest(msg) => self.lobby_addr.do_send(msg),
            NetworkMessage::KickedOutNotice(_msg) => unimplemented!(),
            NetworkMessage::NextQuestion(_msg) => unimplemented!(),
            NetworkMessage::PlayersUpdate(_msg) => unimplemented!(),
            NetworkMessage::QuestionEnded(_msg) => unimplemented!(),
            NetworkMessage::QuestionUpdate(_msg) => unimplemented!(),
            NetworkMessage::ShowLeaderboard(_msg) => unimplemented!(),
            NetworkMessage::TeacherDisconnected(_msg) => unimplemented!(),
        }
    }
}

impl Handler<KickedOutNotice> for Websocket {
    type Result = anyhow::Result<()>;
    fn handle(&mut self, msg: KickedOutNotice, ctx: &mut Self::Context) -> Self::Result {
        let msg = serde_json::to_string(&msg)?;
        let msg = prepare_message(self.sender.clone(), msg);

        ctx.spawn(msg);
        Ok(())
    }
}
