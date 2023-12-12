mod utils;
extern crate common;

use std::{sync::mpsc, thread};

use server::{
    messages::teacher_messages::{GetServerState, ServerHardStop},
    server::{init::run_server, state::Phase},
};
use utils::sample_questions;

#[tokio::test]
async fn server_starts() -> anyhow::Result<()> {
    let questions = sample_questions();
    let (tx, rx) = mpsc::channel();
    let addr = "0.0.0.0:8080".to_string().parse()?;

    let server_thread = thread::spawn(move || {
        run_server(tx, questions, addr).expect("Failed to run server");
    });

    let server = rx.recv().expect("Failed to receive server address");

    let state = server.send(GetServerState {}).await?;

    assert!(state.joined_players.is_empty());
    assert!(state.locked); // no players can join if there is no teacher
    assert!(common::test_utils::compare_question_sets(
        &state.questions,
        &sample_questions()
    ));
    assert!(state.results.is_empty());
    assert_eq!(state.phase, Phase::WaitingForPlayers);
    assert_eq!(state.teacher, None);
    assert!(state.waiting_players.is_empty());

    server.send(ServerHardStop {}).await?;
    server_thread.join().expect("Server thread panicked");

    Ok(())
}
