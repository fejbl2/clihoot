mod fixtures;
mod mocks;
mod utils;

use std::time::Duration;
use std::vec;

use common::questions::{Choice, Question, QuestionSet};

use rstest::rstest;
use server::messages::lobby::{self, StartQuestion};
use server::messages::teacher;
use uuid::Uuid;

use crate::fixtures::create_server::create_server;
use crate::fixtures::create_server_and_teacher::create_server_and_teacher;

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn question_ends_itself_after_timeout_test_implementation() -> anyhow::Result<()> {
    let (server_thread, server) = create_server(QuestionSet {
        quiz_name: "test".to_string(),
        randomize_answers: false,
        randomize_questions: false,
        questions: vec![Question {
            choices: vec![Choice {
                id: Uuid::new_v4(),
                is_right: true,
                text: "right".to_string(),
            }],
            code_block: None,
            text: "question".to_string(),
            time_seconds: 2,
        }],
    });

    let (server_thread, server, teacher_thread, teacher) =
        create_server_and_teacher((server_thread, server));

    let (mut fst_sender, mut fst_receiver, fst_player) = utils::join_new_player().await?;
    let (_snd_sender, mut snd_receiver, _snd_player) = utils::join_new_player().await?;

    // first receives PlayersUpdate
    let _fst_players_update = utils::receive_players_update(&mut fst_receiver).await?;

    // start the round
    server.send(StartQuestion).await??;

    // read the question from websocket
    let fst = utils::receive_next_question(&mut fst_receiver).await?;
    let snd = utils::receive_next_question(&mut snd_receiver).await?;
    assert_eq!(fst, snd);

    // the first player sends an answer
    utils::send_question_answer(&mut fst_sender, &fst_player, &fst.question, 0, vec![0]).await?;

    // both players should receive the Question Update
    let _fst_update = utils::receive_question_update(&mut fst_receiver).await?;
    let _snd_update = utils::receive_question_update(&mut snd_receiver).await?;

    // Second player does not answer

    let to_wait = fst.time_seconds + fst.show_choices_after;

    // instead, wait for the question to end itself
    tokio::time::sleep(tokio::time::Duration::from_secs(to_wait.try_into()?)).await;

    // and both receive the QuestionEnded
    let mut fst_ended = utils::receive_question_ended(&mut fst_receiver).await?;
    let snd_ended = utils::receive_question_ended(&mut snd_receiver).await?;

    assert_eq!(snd_ended.player_answer, None);
    assert!(fst_ended.player_answer.is_some());

    fst_ended.player_answer = None;
    assert_eq!(fst_ended, snd_ended);

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(teacher::HardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
