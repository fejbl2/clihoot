mod fixtures;
mod mocks;
mod utils;

use std::thread::JoinHandle;

use actix::Addr;
use common::model::{network_messages::NextQuestion, ServerNetworkMessage};
use futures_util::StreamExt;
use rstest::rstest;
use server::{
    messages::teacher_messages::{ServerHardStop, StartQuestionMessage, TeacherHardStop},
    server::{
        lobby::censor_question,
        state::{Lobby, Phase},
    },
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState, utils::sample_questions,
};
use tungstenite::Message;

#[rstest]
#[tokio::test]
async fn next_question_is_delivered(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let (_sender, mut receiver, _data) = utils::join_new_player().await?;

    // The teacher now starts the question
    server.send(StartQuestionMessage).await??;

    // and the client should receive the question message
    let mut questions = sample_questions();
    let question = receiver.next().await.expect("Failed to receive message")?;

    assert_eq!(
        question,
        Message::Text(serde_json::to_string(&ServerNetworkMessage::NextQuestion(
            NextQuestion {
                question: censor_question(&mut questions[0]).clone(),
                question_index: 0u64,
                questions_count: questions.len() as u64,
                show_choices_after: questions[0].get_reading_time_estimate() as u64
            }
        ))?)
    );

    let state = server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::ActiveQuestion(0));

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(TeacherHardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
