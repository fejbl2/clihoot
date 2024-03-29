mod fixtures;
mod mocks;
mod utils;

use std::{thread::JoinHandle, time::Duration, vec};

use actix::Addr;

use common::messages::network::QuestionUpdate;

use rstest::rstest;
use server::{
    messages::lobby::{self, StartQuestion},
    Lobby, Teacher,
};

use crate::fixtures::create_server_and_teacher::create_server_and_teacher;

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn answer_can_be_selected(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let (mut fst_sender, mut fst_receiver, fst_player) = utils::join_new_player().await?;
    let (mut snd_sender, mut snd_receiver, snd_player) = utils::join_new_player().await?;

    // first receives PlayersUpdate
    let _fst_players_update = utils::receive_players_update(&mut fst_receiver).await?;

    // start the round
    server.send(StartQuestion).await??;

    // read the question from websocket
    let fst = utils::receive_next_question(&mut fst_receiver).await?;
    let snd = utils::receive_next_question(&mut snd_receiver).await?;
    assert_eq!(fst, snd);

    // the first player sends an answer
    utils::send_question_answer(&mut fst_sender, &fst_player, &fst.question, 0, vec![1]).await?;

    // both players should receive the Question Update
    let fst_update = utils::receive_question_update(&mut fst_receiver).await?;
    let snd_update = utils::receive_question_update(&mut snd_receiver).await?;
    assert_eq!(fst_update, snd_update);
    assert_eq!(
        fst_update,
        QuestionUpdate {
            question_index: 0,
            players_answered_count: 1
        }
    );

    // second player answers
    utils::send_question_answer(&mut snd_sender, &snd_player, &snd.question, 0, vec![0]).await?;

    // and both receive the QuestionEnded
    let mut fst_ended = utils::receive_question_ended(&mut fst_receiver).await?;
    let mut snd_ended = utils::receive_question_ended(&mut snd_receiver).await?;

    // should be exactly the same (except for the player's answer)
    fst_ended.player_answer = None;
    snd_ended.player_answer = None;
    assert_eq!(fst_ended, snd_ended);

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(common::terminal::messages::Stop).await??;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
