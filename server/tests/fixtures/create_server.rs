use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

use crate::{fixtures::sample_questions::sample_questions, utils};
use ::server::server::{init::run_server, state::Lobby};
use actix::Addr;
use common::{constants::DEFAULT_PORT, questions::QuestionSet};
use rstest::fixture;

/// A fixture that starts a server thread and returns the join handle and the lobby address.
/// Fixture is run every time is is requested, do not build other fixtures on top of this one (anti-example below)
#[must_use]
#[fixture]
pub fn create_server(sample_questions: QuestionSet) -> (JoinHandle<()>, Addr<Lobby>) {
    assert!(
        utils::is_port_available(DEFAULT_PORT),
        "Port {DEFAULT_PORT} is not available"
    );

    let (tx, rx) = mpsc::channel();
    let addr = format!("0.0.0.0:{DEFAULT_PORT}").parse().unwrap();

    let server_thread = thread::spawn(move || {
        run_server(tx, sample_questions, addr).expect("Failed to run server");
    });

    let server = rx.recv().expect("Failed to receive server address");

    (server_thread, server)
}

/////////////// DO NOT DO THIS: ///////////////
// #[must_use]
// #[fixture]
// pub fn server(create_server: (JoinHandle<()>, Addr<Lobby>)) -> Addr<Lobby> {
//     create_server.1
// }

// #[must_use]
// #[fixture]
// pub fn server_thread(create_server: (JoinHandle<()>, Addr<Lobby>)) -> JoinHandle<()> {
//     create_server.0
// }
///////////////////////////////////////////////
