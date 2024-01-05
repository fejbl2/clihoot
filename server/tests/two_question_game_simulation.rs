mod fixtures;
mod mocks;
mod utils;

use std::{collections::HashMap, thread, time::Duration, vec};

use actix::Addr;
use anyhow::anyhow;
use common::{
    messages::network::{ChoiceStats, PlayerData, QuestionEnded},
    questions::{Choice, CodeBlock, Question, QuestionSet},
};
use rstest::rstest;
use server::{
    lobby::state::{Lobby, Phase},
    messages::lobby::{self, StartQuestion, SwitchToLeaderboard},
};
use uuid::Uuid;

use crate::{
    fixtures::{
        create_server::create_server, create_server_and_teacher::create_server_and_teacher,
    },
    mocks::get_server_state_handler::GetServerState,
};

const QUIZ_NAME: &str = "The most epic quiz ever";
const Q1_TEXT: &str = "How old is the creator?";
const Q1_TIME: usize = 10;

const Q2_TEXT: &str = "What will the following code do?";
const Q2_CODEBLOCK: &str = "fn main() {
    println!(\"Hello, world!\");
}";
const Q2_TIME: usize = 5;

fn get_questions() -> QuestionSet {
    //// QUESTION DATA ////
    let q1_choice1 = Choice {
        text: "10".to_string(),
        id: Uuid::new_v4(),
        is_correct: false,
    };

    let q1_choice2 = Choice {
        text: "20".to_string(),
        id: Uuid::new_v4(),
        is_correct: true,
    };

    let q1_choice3 = Choice {
        text: "30".to_string(),
        id: Uuid::new_v4(),
        is_correct: false,
    };

    let q1_choice4 = Choice {
        text: "40".to_string(),
        id: Uuid::new_v4(),
        is_correct: false,
    };

    let q1 = Question {
        code_block: None,
        text: Q1_TEXT.to_string(),
        time_seconds: Q1_TIME,
        choices: vec![q1_choice1, q1_choice2, q1_choice3, q1_choice4],
    };

    let q2_choice1 = Choice {
        id: Uuid::new_v4(),
        is_correct: true,
        text: "Print \"Hello, world!\"".to_string(),
    };

    let q2_choice2 = Choice {
        id: Uuid::new_v4(),
        is_correct: true,
        text: "Print \"Hello, world!\" and exit".to_string(),
    };

    let q2 = Question {
        code_block: Some(CodeBlock {
            language: "C".to_string(),
            code: Q2_CODEBLOCK.to_string(),
        }),
        text: Q2_TEXT.to_string(),
        time_seconds: Q2_TIME,
        choices: vec![q2_choice1, q2_choice2],
    };

    QuestionSet {
        quiz_name: QUIZ_NAME.to_string(),
        randomize_answers: false,
        randomize_questions: false,
        questions: vec![q1.clone(), q2.clone()],
    }
}

struct Game {
    questions: QuestionSet,
    fst_sender: utils::Sender,
    fst_receiver: utils::Receiver,
    fst_player: PlayerData,
    snd_sender: utils::Sender,
    snd_receiver: utils::Receiver,
    snd_player: PlayerData,
    server: Addr<Lobby>,
    server_thread: std::thread::JoinHandle<()>,
    teacher: Addr<server::teacher::init::Teacher>,
    teacher_thread: std::thread::JoinHandle<()>,
}

async fn init_game(questions: QuestionSet) -> anyhow::Result<Game> {
    let (server_thread, server) = create_server(questions.clone());
    let (server_thread, server, teacher_thread, teacher) =
        create_server_and_teacher((server_thread, server));

    let (fst_sender, mut fst_receiver, fst_player) = utils::join_new_player().await?;
    let (snd_sender, snd_receiver, snd_player) = utils::join_new_player().await?;

    utils::receive_players_update(&mut fst_receiver).await?;

    Ok(Game {
        questions,
        fst_sender,
        fst_receiver,
        fst_player,
        snd_sender,
        snd_receiver,
        snd_player,
        server,
        server_thread,
        teacher,
        teacher_thread,
    })
}

async fn play_first_round(game: &mut Game) -> anyhow::Result<(QuestionEnded, QuestionEnded)> {
    //// GAME - ROUND 1 - both answer correctly ////
    game.server.send(StartQuestion).await??;

    let fst_q1 = utils::receive_next_question(&mut game.fst_receiver).await?;
    let snd_q1 = utils::receive_next_question(&mut game.snd_receiver).await?;

    thread::sleep(Duration::from_millis(100));
    utils::send_question_answer(
        &mut game.fst_sender,
        &game.fst_player,
        &fst_q1.question,
        0,
        vec![1],
    )
    .await?;

    thread::sleep(Duration::from_millis(100));
    utils::receive_question_update(&mut game.fst_receiver).await?;
    utils::receive_question_update(&mut game.snd_receiver).await?;

    thread::sleep(Duration::from_millis(100));
    utils::send_question_answer(&mut game.snd_sender, &game.snd_player, &snd_q1, 0, vec![1])
        .await?;

    thread::sleep(Duration::from_millis(100));
    let fst_end_q1 = utils::receive_question_ended(&mut game.fst_receiver).await?;
    let snd_end_q1 = utils::receive_question_ended(&mut game.snd_receiver).await?;

    Ok((fst_end_q1, snd_end_q1))
}

async fn transition_to_leaderboard(game: &mut Game) -> anyhow::Result<(usize, usize)> {
    // send the SwitchToLeaderboard message
    game.server.send(SwitchToLeaderboard).await??;

    // both players should receive the ShowLeaderboard message
    let fst_ld = utils::receive_show_leaderboard(&mut game.fst_receiver).await?;
    let snd_ld = utils::receive_show_leaderboard(&mut game.snd_receiver).await?;

    assert_eq!(fst_ld, snd_ld);
    assert!(!fst_ld.was_final_round);

    let fst_points = fst_ld.players[0].1;
    let snd_points = snd_ld.players[1].1;

    assert!(fst_points > 0);
    assert!(snd_points > 0);
    assert!(fst_points > snd_points); // fst answered faster

    let state = game.server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::ShowingLeaderboard(0));

    Ok((fst_points, snd_points))
}

async fn conclude_game(game: &mut Game) -> anyhow::Result<(usize, usize)> {
    // send the SwitchToLeaderboard message
    game.server.send(SwitchToLeaderboard).await??;

    // both players should receive the ShowLeaderboard message
    let fst_ld = utils::receive_show_leaderboard(&mut game.fst_receiver).await?;
    let snd_ld = utils::receive_show_leaderboard(&mut game.snd_receiver).await?;

    assert_eq!(fst_ld, snd_ld);
    assert!(fst_ld.was_final_round);

    let fst_points = fst_ld.players[0].1;
    let snd_points = snd_ld.players[1].1;

    assert!(fst_points > 0);
    assert!(snd_points > 0);
    assert!(fst_points > snd_points); // fst answered both faster and more correctly in both rounds

    let state = game.server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::GameEnded);

    Ok((fst_points, snd_points))
}

async fn play_second_round(game: &mut Game) -> anyhow::Result<(QuestionEnded, QuestionEnded)> {
    //// GAME - ROUND 2 - one selects both answers, the second selects only one answer ////
    game.server.send(StartQuestion).await??;

    let fst_q2 = utils::receive_next_question(&mut game.fst_receiver).await?;
    let snd_q2 = utils::receive_next_question(&mut game.snd_receiver).await?;

    thread::sleep(Duration::from_millis(100));

    utils::send_question_answer(
        &mut game.fst_sender,
        &game.fst_player,
        &fst_q2.question,
        1,
        vec![0, 1],
    )
    .await?;

    utils::receive_question_update(&mut game.fst_receiver).await?;
    utils::receive_question_update(&mut game.snd_receiver).await?;

    utils::send_question_answer(&mut game.snd_sender, &game.snd_player, &snd_q2, 1, vec![0])
        .await?;

    thread::sleep(Duration::from_millis(100));

    let fst_end_q1 = utils::receive_question_ended(&mut game.fst_receiver).await?;
    let snd_end_q1 = utils::receive_question_ended(&mut game.snd_receiver).await?;

    Ok((fst_end_q1, snd_end_q1))
}

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn two_question_game_simulation() -> anyhow::Result<()> {
    let questions = get_questions();
    let mut game = init_game(questions).await?;

    let (fst_end_q1, snd_end_q1) = play_first_round(&mut game).await?;
    thread::sleep(Duration::from_millis(100));

    assert_state_after_first_question(&mut game, &fst_end_q1, &snd_end_q1).await?;
    thread::sleep(Duration::from_millis(100));

    let (fst_points_1, snd_points_1) = transition_to_leaderboard(&mut game).await?;
    thread::sleep(Duration::from_millis(100));

    let (fst_end_q2, snd_end_q2) = play_second_round(&mut game).await?;

    thread::sleep(Duration::from_millis(100));
    assert_state_after_second_question(&mut game, &fst_end_q2, &snd_end_q2).await?;

    thread::sleep(Duration::from_millis(100));
    let (fst_points, snd_points) = conclude_game(&mut game).await?;

    let fst_points_2 = fst_points - fst_points_1;
    let snd_points_2 = snd_points - snd_points_1;

    thread::sleep(Duration::from_millis(100));
    assert_state_after_ending(&mut game, fst_points_2, snd_points_2).await?;

    game.server.send(lobby::HardStop).await?;
    game.server_thread.join().expect("Server thread panicked");

    game.teacher
        .send(common::terminal::messages::Stop)
        .await??;
    game.teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}

async fn assert_state_after_first_question(
    game: &mut Game,
    fst_end_q1: &QuestionEnded,
    snd_end_q1: &QuestionEnded,
) -> anyhow::Result<()> {
    assert_eq!(fst_end_q1.question, snd_end_q1.question);
    assert_eq!(fst_end_q1.question_index, snd_end_q1.question_index);
    assert_eq!(fst_end_q1.question_index, 0);
    assert_eq!(
        fst_end_q1.player_answer,
        Some(vec![game.questions[0].choices[1].id])
    );
    assert_eq!(snd_end_q1.player_answer, fst_end_q1.player_answer);
    let mut stats = HashMap::new();
    stats.insert(
        game.questions[0].choices[0].id,
        ChoiceStats {
            players_answered_count: 0,
        },
    );
    stats.insert(
        game.questions[0].choices[1].id,
        ChoiceStats {
            players_answered_count: 2,
        },
    );
    stats.insert(
        game.questions[0].choices[2].id,
        ChoiceStats {
            players_answered_count: 0,
        },
    );
    stats.insert(
        game.questions[0].choices[3].id,
        ChoiceStats {
            players_answered_count: 0,
        },
    );
    assert_eq!(fst_end_q1.stats, snd_end_q1.stats);
    assert_eq!(fst_end_q1.stats, stats);

    let state = game.server.send(GetServerState).await?;
    assert_eq!(state.joined_players.len(), 2);
    assert_eq!(state.waiting_players.len(), 0);
    assert!(!state.locked);
    assert_eq!(state.phase, Phase::AfterQuestion(0));
    assert_eq!(state.results.len(), 1);
    assert_eq!(state.results[&0].len(), 2);

    Ok(())
}

async fn assert_state_after_second_question(
    game: &mut Game,
    fst: &QuestionEnded,
    snd: &QuestionEnded,
) -> anyhow::Result<()> {
    assert_eq!(fst.question, snd.question);
    assert_eq!(fst.question_index, snd.question_index);
    assert_eq!(fst.question_index, 1);
    assert_eq!(
        fst.player_answer,
        Some(vec![
            game.questions[1].choices[0].id,
            game.questions[1].choices[1].id
        ])
    );
    assert_eq!(
        snd.player_answer,
        Some(vec![game.questions[1].choices[0].id])
    );
    let mut stats = HashMap::new();
    stats.insert(
        game.questions[1].choices[0].id,
        ChoiceStats {
            players_answered_count: 2,
        },
    );
    stats.insert(
        game.questions[1].choices[1].id,
        ChoiceStats {
            players_answered_count: 1,
        },
    );
    assert_eq!(fst.stats, snd.stats);
    assert_eq!(fst.stats, stats);

    let state = game.server.send(GetServerState).await?;
    assert_eq!(state.joined_players.len(), 2);
    assert_eq!(state.waiting_players.len(), 0);
    assert!(!state.locked);
    assert_eq!(state.phase, Phase::AfterQuestion(1));
    assert_eq!(state.results.len(), 2);
    assert_eq!(state.results[&1].len(), 2);

    Ok(())
}

async fn assert_state_after_ending(
    game: &mut Game,
    fst_points_2: usize,
    snd_points_2: usize,
) -> anyhow::Result<()> {
    let state = game.server.send(GetServerState).await?;
    assert_eq!(state.joined_players.len(), 2);
    assert_eq!(state.waiting_players.len(), 0);
    assert!(!state.locked);
    assert_eq!(state.phase, Phase::GameEnded);
    assert_eq!(state.results.len(), 2);
    assert_eq!(state.results[&1].len(), 2);

    let first_player_results = state.results[&1].get(&game.fst_player.uuid).ok_or(anyhow!(
        "First player's results not found in the results map"
    ))?;

    let second_player_results = state.results[&1].get(&game.snd_player.uuid).ok_or(anyhow!(
        "Second player's results not found in the results map"
    ))?;

    assert_eq!(first_player_results.points_awarded, fst_points_2);

    assert_eq!(second_player_results.points_awarded, snd_points_2);

    // timestamp of the first player should be lower than the second player's
    assert!(first_player_results.timestamp < second_player_results.timestamp);

    assert_eq!(first_player_results.answer_order, 1);
    assert_eq!(second_player_results.answer_order, 2);

    Ok(())
}
