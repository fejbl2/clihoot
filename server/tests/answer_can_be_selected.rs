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

use common::{messages::network::ChoiceStats, questions::QuestionCensored};

use rstest::rstest;
use server::{
    lobby::state::{Lobby, Phase},
    messages::{
        lobby::{self, StartQuestion},
        teacher,
    },
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState,
};

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn answer_can_be_selected(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let (mut sender, mut receiver, player) = utils::join_new_player().await?;

    // start the round
    server.send(StartQuestion).await??;

    let state = server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::ActiveQuestion(0));

    // read the question from websocket
    let question = utils::receive_next_question(&mut receiver).await?;

    // send the answer
    utils::send_question_answer(&mut sender, &player, &question.question, 0, vec![0]).await?;

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
    let question_ended = utils::receive_question_ended(&mut receiver).await?;

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
    stats.insert(
        question.choices[1].id,
        ChoiceStats {
            players_answered_count: 0,
        },
    );
    stats.insert(
        question.choices[2].id,
        ChoiceStats {
            players_answered_count: 0,
        },
    );
    stats.insert(
        question.choices[3].id,
        ChoiceStats {
            players_answered_count: 0,
        },
    );
    assert_eq!(question_ended.stats, stats);

    assert_eq!(
        QuestionCensored::from(question_ended.question),
        question.question
    );

    server.send(lobby::HardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(teacher::HardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
