use std::sync::Arc;

use actix::{Addr, AsyncContext, Handler};
use common::model::{
    network_messages::{JoinRequest, TryJoinRequest},
    ClientNetworkMessage,
};
use futures_util::stream::SplitSink;
use tokio::{net::TcpStream, sync::Mutex};
use tungstenite::Message;

use crate::{
    messages::{client_messages, websocket_messages::WebsocketGracefulStop},
    server::state::Lobby,
    websocket::{ws_utils::send_message, Websocket},
};

async fn handle_try_join_request(
    lobby: Addr<Lobby>,
    msg: TryJoinRequest,
    sender: Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>,
) -> anyhow::Result<()> {
    let res = lobby.send(msg).await?;
    let msg = serde_json::to_string(&res)?;
    let () = send_message(sender, Message::Text(msg)).await;

    Ok(())
}

async fn handle_join_request(
    lobby: Addr<Lobby>,
    msg: JoinRequest,
    sender: Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>,
    addr: Addr<Websocket>,
) -> anyhow::Result<()> {
    let res = lobby
        .send(client_messages::JoinRequest {
            player_data: msg.player_data,
            addr,
        })
        .await?;

    let msg = serde_json::to_string(&res)?;

    println!("Sending JoinResponse: {msg}");

    let () = send_message(sender, Message::Text(msg)).await;

    Ok(())
}

impl Handler<ClientNetworkMessage> for Websocket {
    type Result = ();

    /// Handles mapping of messages
    /// - the websocket --> this function --> lobby
    /// - Unimplemented stuff are messages that the client should never send us
    fn handle(&mut self, msg: ClientNetworkMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClientNetworkMessage::TryJoinRequest(msg) => {
                if self.player_id.is_some() {
                    println!("Player tried to cheat by sending another TryJoinRequest",);
                    ctx.notify(WebsocketGracefulStop {});
                    return;
                }

                self.player_id = Some(msg.uuid);

                tokio::spawn(handle_try_join_request(
                    self.lobby_addr.clone(),
                    msg,
                    self.sender.clone(),
                ));
            }
            ClientNetworkMessage::JoinRequest(msg) => {
                // If player is cheating by sending a different uuid, just hang up
                if self.player_id != Some(msg.player_data.uuid) {
                    println!("Player tried to cheat by sending a different uuid",);
                    ctx.notify(WebsocketGracefulStop {});
                    return;
                }

                tokio::spawn(handle_join_request(
                    self.lobby_addr.clone(),
                    msg,
                    self.sender.clone(),
                    ctx.address(),
                ));
            }
            ClientNetworkMessage::AnswerSelected(msg) => {
                // If player is cheating by sending a different uuid, just hang up
                if self.player_id != Some(msg.player_uuid) {
                    println!("Player tried to cheat by sending a different uuid");
                    ctx.notify(WebsocketGracefulStop {});
                    return;
                }

                self.lobby_addr.do_send(msg);
            }
            ClientNetworkMessage::ClientDisconnected(_msg) => {
                ctx.notify(WebsocketGracefulStop {});
            }
        }
    }
}
