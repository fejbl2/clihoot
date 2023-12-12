mod fixtures;
mod utils;

use std::thread::JoinHandle;

use actix::Addr;
use common::{assert_questionset_eq, test_utils::compare_question_sets};
use fixtures::create_server::create_server;
use rstest::rstest;
use server::{
    messages::teacher_messages::{GetServerState, ServerHardStop},
    server::state::{Lobby, Phase},
};
use utils::sample_questions;

#[rstest]
#[tokio::test]
async fn server_starts(create_server: (JoinHandle<()>, Addr<Lobby>)) -> anyhow::Result<()> {
    let (server_thread, server) = create_server;

    let state = server.send(GetServerState {}).await?;

    assert!(state.joined_players.is_empty());
    assert!(state.locked); // no players can join if there is no teacher
    assert_questionset_eq!(&state.questions, &sample_questions());
    assert!(state.results.is_empty());
    assert_eq!(state.phase, Phase::WaitingForPlayers);
    assert_eq!(state.teacher, None);
    assert!(state.waiting_players.is_empty());

    server.send(ServerHardStop {}).await?;
    server_thread.join().expect("Server thread panicked");

    Ok(())
}
