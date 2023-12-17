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
    server::{lobby::censor_question, state::Lobby},
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher, utils::sample_questions,
};
use tungstenite::Message;

#[rstest]
#[tokio::test]
async fn next_question_is_delivered(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;
    let (mut sender, mut receiver) = utils::connect_to_server().await;
    let (id, _msg) = utils::try_join_server(&mut sender, &mut receiver).await?;
    let (_player_data, _msg) = utils::join_server(&mut sender, &mut receiver, id).await?;

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

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(TeacherHardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
