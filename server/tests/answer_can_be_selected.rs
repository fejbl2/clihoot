mod fixtures;
mod mocks;
mod utils;

use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
    time::Duration,
    vec,
};

use actix::Addr;
use anyhow::bail;
use common::{
    model::{
        network_messages::{AnswerSelected, ChoiceStats},
        ClientNetworkMessage, ServerNetworkMessage,
    },
    questions::QuestionCensored,
};

use futures_util::{SinkExt, StreamExt};
use rstest::rstest;
use server::{
    messages::teacher_messages::{ServerHardStop, StartQuestionMessage, TeacherHardStop},
    server::state::{Lobby, Phase},
    teacher::init::Teacher,
};
use tungstenite::Message;

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState,
};

#[rstest]
#[tokio::test]
async fn answer_can_be_selected(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let (mut sender, mut receiver, player) = utils::join_new_player().await?;

    // start the round
    server.send(StartQuestionMessage).await??;

    let state = server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::ActiveQuestion(0));

    // read the question
    let question = match utils::receive_server_network_msg(&mut receiver).await? {
        ServerNetworkMessage::NextQuestion(q) => q,
        _ => bail!("Expected NextQuestion"),
    };

    let answer = ClientNetworkMessage::AnswerSelected(AnswerSelected {
        player_uuid: player.uuid,
        question_index: 0,
        answers: vec![question.choices[0].id],
    });

    // send the answer
    sender
        .send(Message::Text(serde_json::to_string(&answer)?))
        .await?;

    // wait for the server to process the answer
    thread::sleep(Duration::from_millis(100));

    // The server should record the answer and automatically end the round (we are the only player)
    let state = server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::AfterQuestion(0));
    assert!(state.waiting_players.is_empty());
    assert_eq!(state.joined_players.len(), 1);

    assert_eq!(state.results.len(), 1);
    assert!(state.results.contains_key(&0));
    assert!(state.results[&0].contains_key(&player.uuid));
    assert_eq!(state.results[&0][&player.uuid].answer_order, 1);
    assert_eq!(
        state.results[&0][&player.uuid].selected_answers,
        vec![question.choices[0].id]
    );

    let answered = state.results[&0][&player.uuid].timestamp;
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(answered);
    assert!(diff.num_seconds() < 1);

    // The next received message should be an `ServerNetworkMessage::QuestionEnded` message
    let question_ended = receiver.next().await.expect("Failed to receive message")?;
    let question_ended = question_ended.to_text()?;
    let question_ended = serde_json::from_str::<ServerNetworkMessage>(question_ended)?;
    let question_ended = match question_ended {
        ServerNetworkMessage::QuestionEnded(q) => q,
        _ => bail!("Expected QuestionEnded"),
    };

    assert_eq!(question_ended.question_index, 0);
    assert_eq!(
        question_ended.player_answer,
        Some(vec![question.choices[0].id])
    );

    // check the correct question stats
    let mut stats = HashMap::new();
    stats.insert(
        question.choices[0].id,
        ChoiceStats {
            players_answered_count: 1,
        },
    );
    assert_eq!(question_ended.stats, stats);

    assert_eq!(
        QuestionCensored::from(question_ended.question),
        question.question
    );

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(TeacherHardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
