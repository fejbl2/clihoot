use crate::lobby::Lobby;
use crate::messages::{
    ClientActorMessage, ConnectToLobby, DisconnectFromLobby, RelayMessageToClient,
    RelayMessageToLobby, WsGracefulCloseConnection, WsHardCloseConnection,
};
use crate::ws_utils::{prepare_explicit_message, prepare_message};

use actix::{fut, ActorContext, ActorFutureExt};
use actix::{Actor, Addr, ContextFutureSpawner, Running, WrapFuture};
use actix::{AsyncContext, Handler};
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

pub struct WsConn {
    room: Uuid,

    lobby_addr: Arc<Addr<Lobby>>,

    connection_id: Uuid,

    receiver: Option<SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>>,

    sender: Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>,

    reader_task: Option<JoinHandle<()>>,

    who: SocketAddr,
}

impl WsConn {
    pub async fn new(
        room: Uuid,
        lobby: Arc<Addr<Lobby>>,
        socket: TcpStream,
        who: SocketAddr,
    ) -> anyhow::Result<WsConn> {
        let socket = tokio_tungstenite::accept_async(socket).await?;

        let (sender, receiver) = socket.split();

        Ok(WsConn {
            connection_id: Uuid::new_v4(),
            room,
            lobby_addr: lobby,
            receiver: Some(receiver),
            sender: Arc::new(Mutex::new(sender)),
            reader_task: None,
            who,
        })
    }
}

impl Actor for WsConn {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // this is my address
        let addr = ctx.address();

        // First tell the boss that we have a new connection
        self.lobby_addr
            .send(ConnectToLobby {
                addr: addr.clone().recipient(),
                lobby_id: self.room,
                self_id: self.connection_id,
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
        let receiver = self.receiver.take().unwrap();

        // Spawn a Tokio task which will read from the socket and generate messages for this actor
        let reader_task = tokio::spawn(read_messages_from_socket(receiver, who, addr));
        self.reader_task = Some(reader_task);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        if let Some(reader_task) = &self.reader_task {
            reader_task.abort();
        }

        self.lobby_addr.do_send(DisconnectFromLobby {
            id: self.connection_id,
            room_id: self.room,
        });
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Stopped WsConn for {}", self.who);
    }
}

async fn read_messages_from_socket<'a>(
    mut receiver: SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>,
    _who: SocketAddr,
    addr: Addr<WsConn>,
) {
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(s) => addr.do_send(RelayMessageToLobby(s.to_string())),
            Message::Binary(b) => {
                addr.do_send(RelayMessageToLobby(String::from_utf8(b).unwrap()));
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

impl Handler<WsHardCloseConnection> for WsConn {
    type Result = ();

    fn handle(&mut self, _msg: WsHardCloseConnection, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl Handler<WsGracefulCloseConnection> for WsConn {
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

impl Handler<RelayMessageToClient> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: RelayMessageToClient, ctx: &mut Self::Context) -> Self::Result {
        let fut = prepare_message::<Self>(self.sender.clone(), msg.0);
        ctx.spawn(fut);
    }
}

impl Handler<RelayMessageToLobby> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: RelayMessageToLobby, _ctx: &mut Self::Context) -> Self::Result {
        // tell the lobby to send it to everyone else
        self.lobby_addr.do_send(ClientActorMessage {
            id: self.connection_id,
            msg: msg.0,
            room_id: self.room,
        });
    }
}
