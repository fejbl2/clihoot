use std::{path::Path, thread, time::Duration};

use anyhow::Ok;
use common::model::network_messages::{JoinRequest, NetworkPlayerData, TryJoinRequest};
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

    println!("Connected to server");

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

    println!("Sent TryJoinRequest");

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

    let player_data = NetworkPlayerData {
        color: "red".to_owned(),
        nickname: "test".to_owned(),
        uuid: id,
    };

    let msg = ClientNetworkMessage::JoinRequest(JoinRequest {
        player_data: player_data.clone(),
    });

    sender
        .send(Message::Text(serde_json::to_string(&msg)?))
        .await?;

    println!("Sent JoinRequest");

    let msg = receiver.next().await.expect("Failed to receive message")?;

    Ok((player_data, msg))
}
