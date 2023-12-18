// mod fixtures;
// mod mocks;
// mod utils;

// use std::thread::JoinHandle;

// use actix::Addr;
// use common::model::{network_messages::NextQuestion, ServerNetworkMessage};
// use futures_util::StreamExt;
// use rstest::rstest;
// use server::{
//     messages::teacher_messages::{ServerHardStop, StartQuestionMessage, TeacherHardStop},
//     server::{
//         lobby::censor_question,
//         state::{Lobby, Phase},
//     },
//     teacher::init::Teacher,
// };

// use crate::{
//     fixtures::create_server_and_teacher::create_server_and_teacher,
//     mocks::get_server_state_handler::GetServerState, utils::sample_questions,
// };
// use tungstenite::Message;

// #[rstest]
// #[tokio::test]
// async fn round_ends_when_all_players_answered(
//     create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
// ) -> anyhow::Result<()> {
//     let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

//     // TODO

//     server.send(ServerHardStop).await?;
//     server_thread.join().expect("Server thread panicked");

//     teacher.send(TeacherHardStop).await?;
//     teacher_thread.join().expect("Teacher thread panicked");

//     Ok(())
// }
