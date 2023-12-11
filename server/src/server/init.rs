use actix::{Actor, Addr};

use actix_rt::System;

use common::questions::QuestionSet;
use tokio::net::TcpListener;

use std::{net::SocketAddr, sync::mpsc::Sender};

use crate::websocket::Websocket;

use super::state::Lobby;

fn create_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Could not create tokio runtime") // cannot seem to get rid of this
}

pub fn run_server(
    tx: Sender<Addr<Lobby>>,
    questions: QuestionSet,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let system = actix::System::with_tokio_rt(create_tokio_runtime);

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
    // LOCAL SPAWN is very important here (actors can only be spawned on the same thread)
    let _connection_acceptor =
        tokio::task::spawn_local(accept_connections(addr, lobby_actor.clone()));

    // send the address of the lobby to the main thread
    let _ = tx.send(lobby_actor.clone());

    // handle CTRL+C gracefully
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to register CTRL-C handler");
        println!("CTRL-C received, shutting down");
        System::current().stop();
    });

    Ok(())
}

async fn accept_connections(addr: SocketAddr, lobby: Addr<Lobby>) -> anyhow::Result<()> {
    // create a TCP socket listener

    let listener = TcpListener::bind(addr).await?;

    loop {
        println!("Listening on: {addr:?}, waiting to accept a new connection");

        // accept a connection
        let (socket, who) = listener.accept().await?;

        println!("Accepted connection from: {who:?}");

        // spawn a actor for managing the connection
        let ws = Websocket::new(lobby.clone(), socket, who).await?;
        let _ = ws.start();
    }
}
