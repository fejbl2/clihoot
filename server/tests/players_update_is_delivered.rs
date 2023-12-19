mod fixtures;
mod mocks;
mod utils;

use std::{thread::JoinHandle, time::Duration};

use actix::Addr;

use common::model::network_messages::PlayersUpdate;

use rstest::rstest;
use server::{
    messages::teacher_messages::{ServerHardStop, TeacherHardStop},
    server::state::Lobby,
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState,
};

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn players_update_is_delivered(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    // join a first player
    let (_fst_sender, mut fst_receiver, fst_data) = utils::join_new_player().await?;

    // join a second player
    let (_snd_sender, _snd_receiver, snd_data) = utils::join_new_player().await?;

    // assert that the first player gets a players update message
    let msg = utils::receive_players_update(&mut fst_receiver).await?;

    assert_eq!(
        msg,
        PlayersUpdate {
            players: vec![fst_data.clone(), snd_data.clone(),]
        }
    );

    // Get server state and assert that both are there
    let state = server.send(GetServerState).await?;

    assert!(state.waiting_players.is_empty());
    assert_eq!(state.joined_players.len(), 2);
    assert!(state.joined_players.contains_key(&fst_data.uuid));
    assert!(state.joined_players.contains_key(&snd_data.uuid));

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(TeacherHardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
