mod fixtures;
mod mocks;
mod utils;

use std::{borrow::Cow, thread::JoinHandle, vec};

use actix::Addr;

use common::model::network_messages::PlayersUpdate;

use rstest::rstest;
use server::{
    messages::teacher_messages::{KickPlayer, ServerHardStop, TeacherHardStop},
    server::state::Lobby,
    teacher::init::Teacher,
};
use tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};

use crate::fixtures::create_server_and_teacher::create_server_and_teacher;

#[rstest]
#[tokio::test]
async fn teacher_can_kick_player(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    // join a first player
    let (_fst_sender, mut fst_receiver, fst_data) = utils::join_new_player().await?;

    // join a second player
    let (_snd_sender, mut snd_receiver, snd_data) = utils::join_new_player().await?;

    // drain the players update message which comes after the joining the second player
    let _ = utils::receive_players_update(&mut fst_receiver).await?;

    let kick_reason = "Bad nickname";

    // kick the first player
    server
        .send(KickPlayer {
            player_uuid: fst_data.uuid,
            reason: Some(kick_reason.to_string()),
        })
        .await??;

    // assert that the first player gets a kicked out notice
    let msg = utils::receive_close_frame(&mut fst_receiver).await?;

    assert_eq!(
        msg,
        CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from(kick_reason),
        }
    );

    // assert that the second player gets a players update message
    let msg = utils::receive_players_update(&mut snd_receiver).await?;

    assert_eq!(
        msg,
        PlayersUpdate {
            players: vec![snd_data.clone(),]
        }
    );

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(TeacherHardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
