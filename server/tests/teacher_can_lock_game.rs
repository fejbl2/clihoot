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
    messages::lobby::{HardStop, SetLockMessage, StartQuestion},
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState,
};

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn teacher_can_lock_game(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let (_sender, _receiver, _player) = utils::join_new_player().await?;

    // start the round
    server.send(StartQuestion).await??;

    let state = server.send(GetServerState).await?;
    assert!(!state.locked);

    // lock the game
    server.send(SetLockMessage { locked: true }).await?;

    // try to join a new player, should fail
    let (mut sender2, mut receiver2) = utils::connect_to_server().await;
    let (uuid, msg) = utils::try_join_server(&mut sender2, &mut receiver2).await?;
    assert_eq!(
        msg,
        TryJoinResponse {
            can_join: CanJoin::No(LOBBY_LOCKED_MSG.to_string()),
            quiz_name: DEFAULT_QUIZ_NAME.to_string(),
            uuid
        }
    );

    // unlock the game
    server.send(SetLockMessage { locked: false }).await?;

    // should be able to join a new player
    let (_sender, _receiver, _player) = utils::join_new_player().await?;

    let state = server.send(GetServerState).await?;
    assert!(!state.locked);
    assert_eq!(state.joined_players.len(), 2);

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(teacher::HardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
