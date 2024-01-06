mod fixtures;
mod mocks;
mod utils;

use std::{borrow::Cow, thread::JoinHandle, time::Duration, vec};

use actix::Addr;
use common::{
    constants::DEFAULT_GOODBYE_MESSAGE,
    messages::{network::AnswerSelected, ClientNetworkMessage},
};

use futures_util::SinkExt;
use rstest::rstest;
use server::{
    lobby::{Lobby, Phase},
    messages::lobby::{self},
    Teacher,
};
use tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher, mocks::GetServerState,
    utils::sample_questions,
};

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn answer_cannot_be_selected_if_question_is_not_started(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let (mut sender, mut receiver, player) = utils::join_new_player().await?;

    let questions = sample_questions();

    let answer = ClientNetworkMessage::AnswerSelected(AnswerSelected {
        player_uuid: player.uuid,
        question_index: 0,
        answers: vec![questions[0].choices[0].id],
    });

    // try to answer before the question is started
    sender
        .send(Message::Text(serde_json::to_string(&answer)?))
        .await?;

    // The server should disconnect, because the player tried to cheat
    let close = utils::receive_close_frame(&mut receiver).await?;

    assert_eq!(
        close,
        CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from(DEFAULT_GOODBYE_MESSAGE),
        }
    );

    // get server state and make sure they were kicked
    let state = server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::WaitingForPlayers);
    assert!(state.waiting_players.is_empty());
    assert!(state.joined_players.is_empty());

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(common::terminal::messages::Stop).await??;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
