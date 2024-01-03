use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};

use ::server::lobby::state::Lobby;
use actix::Addr;
use common::constants::DEFAULT_QUIZ_NAME;
use rstest::fixture;
use server::teacher::init::{run_teacher, Teacher};

use crate::fixtures::create_server::create_server;

#[must_use]
#[fixture]
pub fn create_server_and_teacher(
    create_server: (JoinHandle<()>, Addr<Lobby>),
) -> (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>) {
    let (server_thread, server) = create_server;

    let (tx, rx) = mpsc::channel();

    let server_address = server.clone();
    let teacher_thread = thread::spawn(move || {
        run_teacher(server_address, tx, DEFAULT_QUIZ_NAME).expect("Failed to run teacher");
    });

    let teacher = rx.recv().expect("Failed to receive teacher address");

    thread::sleep(std::time::Duration::from_millis(100));

    (server_thread, server, teacher_thread, teacher)
}
