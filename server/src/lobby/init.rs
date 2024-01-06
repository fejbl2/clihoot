use actix::{Actor, Addr};

use common::questions::QuestionSet;
use log::info;
use tokio::net::TcpListener;

use std::{net::SocketAddr, sync::mpsc::Sender};

use super::state::Lobby;
use crate::websocket::Websocket;

/// Starts the server and send the address of the lobby through the given channel.
/// # Errors
/// - If the tokio runtime cannot be created
/// - If the server cannot be started
pub fn run_server(
    tx: Sender<Addr<Lobby>>,
    questions: QuestionSet,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let system = actix::System::new();

    system.block_on(init(tx, questions, addr))?;

    system.run()?;

    Ok(())
}

#[allow(clippy::unused_async)]
async fn init(
    tx: Sender<Addr<Lobby>>,
    questions: QuestionSet,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    // spawn an actor for managing the lobby
    let lobby_actor = Lobby::new(questions).start();

    // spawn task for accepting connections
    let _connection_acceptor =
        tokio::task::spawn_local(accept_connections(addr, lobby_actor.clone()));

    // send the address of the lobby to the main thread
    let _ = tx.send(lobby_actor.clone());

    Ok(())
}

async fn accept_connections(addr: SocketAddr, lobby: Addr<Lobby>) -> anyhow::Result<()> {
    // create a TCP socket listener

    let listener = TcpListener::bind(addr).await?;

    loop {
        info!("Listening on: {addr:?}, waiting to accept a new connection");

        // accept a connection
        let (socket, who) = listener.accept().await?;

        info!("Accepted connection from: {who:?}");

        // spawn a actor for managing the connection
        let ws = Websocket::new(lobby.clone(), socket, who).await?;
        let _ = ws.start();
    }
}
