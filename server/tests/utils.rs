use std::{path::Path, thread, time::Duration};

use anyhow::{bail, Ok};
use common::model::network_messages::{CanJoin, JoinRequest, NetworkPlayerData, TryJoinRequest};
use common::model::ServerNetworkMessage;
use common::questions;
use common::{constants::DEFAULT_PORT, model::ClientNetworkMessage};
use futures_util::SinkExt;
use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;
use uuid::Uuid;

use std::net::TcpListener;

#[must_use]
pub fn is_port_available(port: u16) -> bool {
    TcpListener::bind(("0.0.0.0", port)).is_ok()
}

#[must_use]
pub fn sample_questions() -> questions::QuestionSet {
    questions::QuestionSet::from_file(Path::new("../common/tests/files/ok_minimal.yaml")).unwrap()
}

type Sender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[allow(dead_code)]
pub async fn connect_to_server() -> (Sender, Receiver) {
    thread::sleep(Duration::from_millis(100));

    let (conn, _) = tokio_tungstenite::connect_async(format!("ws://localhost:{DEFAULT_PORT}"))
        .await
        .expect("Failed to connect to server");

    let (sender, receiver) = conn.split();

    (sender, receiver)
}

#[allow(dead_code)]
pub async fn try_join_server(
    sender: &mut Sender,
    receiver: &mut Receiver,
) -> anyhow::Result<(Uuid, Message)> {
    thread::sleep(Duration::from_millis(100));

    let id = Uuid::new_v4();
    let msg = ClientNetworkMessage::TryJoinRequest(TryJoinRequest { uuid: id });

    sender
        .send(Message::Text(serde_json::to_string(&msg)?))
        .await?;

    let msg = receiver.next().await.expect("Failed to receive message")?;

    Ok((id, msg))
}

#[allow(dead_code)]
pub async fn join_server(
    sender: &mut Sender,
    receiver: &mut Receiver,
    id: Uuid,
) -> anyhow::Result<(NetworkPlayerData, Message)> {
    thread::sleep(Duration::from_millis(100));

    let random_string_color = Uuid::new_v4().to_string();
    let random_string_nickname = Uuid::new_v4().to_string();

    let player_data = NetworkPlayerData {
        color: random_string_color,
        nickname: random_string_nickname,
        uuid: id,
    };

    let msg = ClientNetworkMessage::JoinRequest(JoinRequest {
        player_data: player_data.clone(),
    });

    sender
        .send(Message::Text(serde_json::to_string(&msg)?))
        .await?;

    let msg = receiver.next().await.expect("Failed to receive message")?;

    Ok((player_data, msg))
}

/// Generates a new player uuid, connects to the server and joins it.
/// # Errors
/// - if the server cannot be joined, will panic.
#[allow(dead_code)]
pub async fn join_new_player() -> anyhow::Result<(Sender, Receiver, NetworkPlayerData)> {
    let (mut sender, mut receiver) = connect_to_server().await;
    let (id, _msg) = try_join_server(&mut sender, &mut receiver).await?;
    let (player_data, msg) = join_server(&mut sender, &mut receiver, id).await?;

    // Message must be Text
    assert!(msg.is_text());
    let msg = msg.to_text()?;

    // deserialize into ServerNetworkMessage
    let msg: ServerNetworkMessage = serde_json::from_str(msg)?;

    // it must be a JoinResponse
    let res = match msg {
        ServerNetworkMessage::JoinResponse(res) => res,
        _ => bail!("Unexpected message"),
    };

    // And it must be correct
    assert_eq!(res.can_join, CanJoin::Yes);
    assert_eq!(res.uuid, id);

    Ok((sender, receiver, player_data))
}

#[allow(dead_code)]
pub async fn receive_server_network_msg(
    receiver: &mut Receiver,
) -> anyhow::Result<ServerNetworkMessage> {
    let msg = receiver.next().await.expect("Failed to receive message")?;
    let msg = msg.to_text()?;
    let msg = serde_json::from_str::<ServerNetworkMessage>(msg)?;

    Ok(msg)
}
