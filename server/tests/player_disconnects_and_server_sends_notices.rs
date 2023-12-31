mod fixtures;
mod mocks;
mod utils;

use std::{thread::JoinHandle, time::Duration};

use actix::Addr;

use rstest::rstest;
use server::{
    lobby::state::Lobby,
    messages::{lobby, teacher},
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState,
};

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn player_disconnects_and_server_sends_notices(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    // join a first player
    let (_fst_sender, mut fst_receiver, fst_data) = utils::join_new_player().await?;

    // create scope to drop the sender automatically at the end
    {
        // join and disconnect a second player
        _ = utils::join_new_player().await?;
    }

    let msg = utils::receive_players_update(&mut fst_receiver).await?;
    assert_eq!(msg.players.len(), 2);

    let msg = utils::receive_players_update(&mut fst_receiver).await?;
    assert_eq!(msg.players.len(), 1);

    let state = server.send(GetServerState).await?;

    assert!(state.waiting_players.is_empty());
    assert_eq!(state.joined_players.len(), 1);
    assert!(state.joined_players.contains_key(&fst_data.uuid));

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(teacher::HardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
