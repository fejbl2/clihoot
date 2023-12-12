// use std::{
//     sync::mpsc,
//     thread::{self, JoinHandle},
// };

// use ::server::server::{init::run_server, state::Lobby};
// use actix::Addr;
// use common::constants::DEFAULT_PORT;
// use rstest::fixture;

// use crate::utils::{self, sample_questions};

// #[must_use]
// #[fixture]
// pub fn create_teacher(create_server: (JoinHandle<()>, Addr<Lobby>)) -> () {
//     assert!(
//         utils::is_port_available(DEFAULT_PORT),
//         "Port {DEFAULT_PORT} is not available"
//     );

//     let questions = sample_questions();
//     let (tx, rx) = mpsc::channel();
//     let addr = format!("0.0.0.0:{DEFAULT_PORT}").parse().unwrap();

//     let server_thread = thread::spawn(move || {
//         run_server(tx, questions, addr).expect("Failed to run server");
//     });

//     let server = rx.recv().expect("Failed to receive server address");

//     (server_thread, server)
// }
