use std::thread::{self, JoinHandle};

use ::server::server::state::Lobby;
use actix::Addr;
use rstest::fixture;
use server::teacher::init::run_teacher;

use crate::fixtures::create_server::create_server;

#[must_use]
#[fixture]
pub fn create_server_and_teacher(
    create_server: (JoinHandle<()>, Addr<Lobby>),
) -> (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>) {
    let (server_thread, server) = create_server;

    let server_address = server.clone();
    let teacher_thread = thread::spawn(move || {
        run_teacher(server_address).expect("Failed to run teacher");
    });

    thread::sleep(std::time::Duration::from_millis(100));

    (server_thread, server, teacher_thread)
}
