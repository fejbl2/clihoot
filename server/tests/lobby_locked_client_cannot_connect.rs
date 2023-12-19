mod fixtures;
mod mocks;
mod utils;

use std::thread::JoinHandle;

use actix::Addr;
use common::{
    constants::{DEFAULT_QUIZ_NAME, LOBBY_LOCKED_MSG},
    model::{
        network_messages::{CanJoin, TryJoinResponse},
        ServerNetworkMessage,
    },
};
use rstest::rstest;
use server::{messages::teacher_messages::ServerHardStop, server::state::Lobby};

use crate::fixtures::create_server::create_server;
use tungstenite::Message;

#[rstest]
#[tokio::test]
async fn lobby_locked_client_cannot_connect(
    create_server: (JoinHandle<()>, Addr<Lobby>),
) -> anyhow::Result<()> {
    let (server_thread, server) = create_server;

    let (mut sender, mut receiver) = utils::connect_to_server().await;

    let (id, msg) = utils::try_join_server(&mut sender, &mut receiver).await?;

    assert_eq!(
        msg,
        Message::Text(serde_json::to_string(
            &ServerNetworkMessage::TryJoinResponse(TryJoinResponse {
                can_join: CanJoin::No(LOBBY_LOCKED_MSG.to_string()),
                uuid: id,
                quiz_name: DEFAULT_QUIZ_NAME.to_string(),
            })
        )?)
    );

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    Ok(())
}
