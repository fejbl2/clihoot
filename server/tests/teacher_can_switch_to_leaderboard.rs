mod fixtures;
mod mocks;
mod utils;

use std::{
    thread::{self, JoinHandle},
    time::Duration,
    vec,
};

use actix::Addr;

use rstest::rstest;
use server::{
    lobby::{Lobby, Phase},
    messages::lobby::{self, StartQuestion, SwitchToLeaderboard},
    Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher, mocks::GetServerState,
};

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn teacher_can_switch_to_leaderboard(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let (mut sender, mut receiver, player) = utils::join_new_player().await?;

    // start the round
    server.send(StartQuestion).await??;

    // try to send switch to leaderboard, should fail
    assert!(server.send(SwitchToLeaderboard).await?.is_err());

    // read the question from websocket
    let question = utils::receive_next_question(&mut receiver).await?;

    // send the answer
    utils::send_question_answer(&mut sender, &player, &question.question, 0, vec![0]).await?;

    // wait for the server to process the answer
    thread::sleep(Duration::from_millis(100));

    // should receive End question message
    let _end_question = utils::receive_question_ended(&mut receiver).await?;

    // the teacher can now switch to the leaderboard
    server.send(SwitchToLeaderboard).await??;

    // the server should now be in the game ended, because this was the only question
    let state = server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::GameEnded);

    // we should receive the ShowLeaderboard message
    let msg = utils::receive_show_leaderboard(&mut receiver).await?;

    assert!(msg.was_final_round);
    assert_eq!(msg.players, vec![(player, 0)]); // 0 because we have the wrong answer

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(common::terminal::messages::Stop).await??;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
