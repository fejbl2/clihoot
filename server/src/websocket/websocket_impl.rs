use actix::AsyncContext;
use actix::{Actor, Addr, Running};

use crate::lobby::state::Lobby;
use crate::messages::websocket::WebsocketGracefulStop;
use common::messages::ClientNetworkMessage;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use std::result::Result::Ok;

use tokio::sync::Mutex;

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use tungstenite::Message;
use uuid::Uuid;

use crate::messages::websocket::{DisconnectFromLobby, WebsocketHardStop};

pub struct Websocket {
    pub lobby_addr: Addr<Lobby>,
    pub player_id: Option<Uuid>,
    pub receiver: Option<SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>>,
    pub sender: Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>,
    pub reader_task: Option<JoinHandle<()>>,
    pub who: SocketAddr,
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
            player_id: None,
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

        if let Some(player_id) = self.player_id {
            self.lobby_addr.do_send(DisconnectFromLobby { player_id });
        }

        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Stopped WsConn for {}", self.who);
    }
}

async fn read_messages_from_socket<'a>(
    mut receiver: SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>,
    _who: SocketAddr,
    addr: Addr<Websocket>,
) {
    while let Some(msg) = receiver.next().await {
        let Ok(msg) = msg else {
            println!("Hanging up on the client bcs reading from socket failed");
            addr.do_send(WebsocketHardStop);
            return;
        };

        match msg {
            Message::Text(msg) => {
                // try to parse the JSON s to a `NetworkMessage`
                match serde_json::from_str::<ClientNetworkMessage>(&msg) {
                    Ok(msg) => {
                        addr.do_send(msg);
                    }
                    Err(e) => {
                        println!("Hanging up on the client bcs parsing message failed: {e}");
                        addr.do_send(WebsocketGracefulStop { reason: None });
                    }
                }
            }
            Message::Close(_) => {
                // cannot call `ctx.stop();` because we are in another Task:
                // instead, we send a message to ourselves to stop
                addr.do_send(WebsocketHardStop);

                // also quit the loop
                return;
            }
            _ => (),
        }
    }
}
