mod fixtures;
mod mocks;
mod utils;

use std::{thread::JoinHandle, time::Duration};

use actix::Addr;
use common::{
    constants::{DEFAULT_QUIZ_NAME, LOBBY_LOCKED_MSG},
    messages::network::{CanJoin, TryJoinResponse},
};
use rstest::rstest;
use server::{
    lobby::state::Lobby,
    messages::lobby::{self},
};

use crate::fixtures::create_server::create_server;

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn lobby_locked_client_cannot_connect(
    create_server: (JoinHandle<()>, Addr<Lobby>),
) -> anyhow::Result<()> {
    let (server_thread, server) = create_server;

    let (mut sender, mut receiver) = utils::connect_to_server().await;

    let (id, msg) = utils::try_join_server(&mut sender, &mut receiver).await?;

    assert_eq!(
        msg,
        TryJoinResponse {
            can_join: CanJoin::No(LOBBY_LOCKED_MSG.to_string()),
            uuid: id,
            quiz_name: DEFAULT_QUIZ_NAME.to_string(),
        }
    );

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    Ok(())
}
