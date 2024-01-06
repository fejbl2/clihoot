use std::sync::Arc;

use actix::{Addr, AsyncContext, Handler};
use common::messages::{
    network::{AnswerSelected, JoinRequest, TryJoinRequest},
    ClientNetworkMessage, ServerNetworkMessage,
};
use futures_util::stream::SplitSink;
use tokio::{net::TcpStream, sync::Mutex};
use tungstenite::Message;

use crate::{
    messages::{client, websocket::GracefulStop},
    websocket::{send_message, Websocket},
    Lobby,
};

use log::error;

pub type Sender = Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>;

async fn handle_try_join_request(
    lobby: Addr<Lobby>,
    msg: TryJoinRequest,
    sender: Sender,
) -> anyhow::Result<()> {
    let res = lobby.send(msg).await?;

    let msg = serde_json::to_string(&ServerNetworkMessage::TryJoinResponse(res))?;

    let () = send_message(sender, Message::Text(msg)).await;

    Ok(())
}

async fn handle_answer_selected(
    lobby: Addr<Lobby>,
    msg: AnswerSelected,
    addr: Addr<Websocket>,
) -> anyhow::Result<()> {
    let res = lobby.send(msg).await?;

    if let Err(e) = res {
        // an error means that the client tries to cheat and therefore, we will disconnect
        error!("Player tried to cheat: {e}");
        addr.do_send(GracefulStop { reason: None });
        return Err(e);
    }

    Ok(())
}

async fn handle_join_request(
    lobby: Addr<Lobby>,
    msg: JoinRequest,
    sender: Sender,
    addr: Addr<Websocket>,
) -> anyhow::Result<()> {
    let res = lobby
        .send(client::JoinRequest {
            player_data: msg.player_data,
            addr,
        })
        .await?;

    let msg = serde_json::to_string(&ServerNetworkMessage::JoinResponse(res))?;

    let () = send_message(sender, Message::Text(msg)).await;

    Ok(())
}

impl Handler<ClientNetworkMessage> for Websocket {
    type Result = ();

    /// Handles mapping of messages
    /// - the websocket --> this function --> lobby
    fn handle(&mut self, msg: ClientNetworkMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClientNetworkMessage::TryJoinRequest(msg) => {
                if self.player_id.is_some() {
                    error!("Player tried to cheat by sending another TryJoinRequest",);
                    ctx.notify(GracefulStop { reason: None });
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
                    error!("Player tried to cheat by sending a different uuid",);
                    ctx.notify(GracefulStop { reason: None });
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
                    error!("Player tried to cheat by sending a different uuid");
                    ctx.notify(GracefulStop { reason: None });
                    return;
                }

                tokio::spawn(handle_answer_selected(
                    self.lobby_addr.clone(),
                    msg,
                    ctx.address(),
                ));
            }
            ClientNetworkMessage::ClientDisconnected(_msg) => {
                ctx.notify(GracefulStop { reason: None });
            }
        }
    }
}
