mod fixtures;
mod utils;

use std::thread::JoinHandle;

use actix::Addr;
use common::{
    constants::DEFAULT_QUIZ_NAME,
    model::network_messages::{CanJoin, JoinResponse, TryJoinResponse},
};
use rstest::rstest;
use server::{messages::teacher_messages::ServerHardStop, server::state::Lobby};

use crate::fixtures::create_server_and_teacher::create_server_and_teacher;
use tungstenite::Message;

#[rstest]
#[tokio::test]
async fn client_connects_and_joins(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>),
) -> anyhow::Result<()> {
    let (server_thread, server, _teacher_thread) = create_server_and_teacher;
    let (mut sender, mut receiver) = utils::connect_to_server().await;

    let (id, msg) = utils::try_join_server(&mut sender, &mut receiver).await?;

    assert_eq!(
        msg,
        Message::Text(serde_json::to_string(&TryJoinResponse {
            can_join: CanJoin::Yes,
            uuid: id,
            quiz_name: DEFAULT_QUIZ_NAME.to_string(),
        })?)
    );

    let (player_data, msg) = utils::join_server(&mut sender, &mut receiver, id).await?;

    assert_eq!(
        msg,
        Message::Text(serde_json::to_string(&JoinResponse {
            players: vec![player_data],
            can_join: CanJoin::Yes,
            uuid: id,
            quiz_name: DEFAULT_QUIZ_NAME.to_string(),
        })?)
    );

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    Ok(())

    // // TODO: send HardStop to teacher as well
    // teacher_thread.join().expect("Teacher thread panicked");

    // Ok(())
}
