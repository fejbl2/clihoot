mod fixtures;
mod utils;

use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use actix::Addr;
use common::{
    constants::{DEFAULT_PORT, DEFAULT_QUIZ_NAME, LOBBY_LOCKED_MSG},
    model::{
        network_messages::{CanJoin, TryJoinRequest, TryJoinResponse},
        ClientNetworkMessage,
    },
};
use futures_util::{SinkExt, StreamExt};
use rstest::rstest;
use server::{messages::teacher_messages::ServerHardStop, server::state::Lobby};

use fixtures::create_server::create_server;
use tungstenite::Message;

use uuid::Uuid;

#[rstest]
#[tokio::test]
async fn lobby_locked_client_cannot_connect(
    create_server: (JoinHandle<()>, Addr<Lobby>),
) -> anyhow::Result<()> {
    let (server_thread, server) = create_server;

    thread::sleep(Duration::from_millis(100));

    let (conn, _) = tokio_tungstenite::connect_async(format!("ws://localhost:{DEFAULT_PORT}"))
        .await
        .expect("Failed to connect to server");

    println!("Connected to server");

    let (mut sender, mut receiver) = conn.split();

    let id = Uuid::new_v4();
    let msg = ClientNetworkMessage::TryJoinRequest(TryJoinRequest { uuid: id });

    sender
        .send(Message::Text(serde_json::to_string(&msg)?))
        .await?;

    println!("Sent TryJoinRequest");

    let msg = receiver.next().await.expect("Failed to receive message")?;

    println!("Received message: {msg:?}");

    assert_eq!(
        msg,
        Message::Text(serde_json::to_string(&TryJoinResponse {
            can_join: CanJoin::No(LOBBY_LOCKED_MSG.to_string()),
            uuid: id,
            quiz_name: DEFAULT_QUIZ_NAME.to_string(),
        })?)
    );

    server.send(ServerHardStop {}).await?;
    server_thread.join().expect("Server thread panicked");

    Ok(())
}
