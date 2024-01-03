mod fixtures;
mod mocks;
mod utils;

use std::{thread::JoinHandle, time::Duration};

use actix::Addr;
use common::{
    constants::DEFAULT_QUIZ_NAME,
    messages::network::{CanJoin, JoinResponse, TryJoinResponse},
};
use rstest::rstest;
use server::{
    lobby::state::{Lobby, Phase},
    messages::lobby::{self},
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState,
};

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn client_connects_and_joins(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;
    let (mut sender, mut receiver) = utils::connect_to_server().await;

    let (id, msg) = utils::try_join_server(&mut sender, &mut receiver).await?;

    assert_eq!(
        msg,
        TryJoinResponse {
            can_join: CanJoin::Yes,
            uuid: id,
            quiz_name: DEFAULT_QUIZ_NAME.to_string(),
        }
    );

    let state = server.send(GetServerState).await?;

    assert_eq!(state.waiting_players.len(), 1);
    assert!(state.joined_players.is_empty());
    assert!(!state.locked);
    assert!(state.results.is_empty());
    assert_eq!(state.phase, Phase::WaitingForPlayers);
    assert_ne!(state.teacher, None);

    let (player_data, msg) = utils::join_server(&mut sender, &mut receiver, id).await?;

    assert_eq!(
        msg,
        JoinResponse {
            players: vec![player_data],
            can_join: CanJoin::Yes,
            uuid: id,
            quiz_name: DEFAULT_QUIZ_NAME.to_string(),
        }
    );

    let state = server.send(GetServerState).await?;

    assert!(state.waiting_players.is_empty());
    assert_eq!(state.joined_players.len(), 1);
    assert!(!state.locked);
    assert!(state.results.is_empty());
    assert_eq!(state.phase, Phase::WaitingForPlayers);
    assert_ne!(state.teacher, None);

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(common::terminal::messages::Stop).await??;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
