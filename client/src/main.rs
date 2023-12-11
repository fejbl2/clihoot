use actix::prelude::*;

use client::terminal::actor_data::TerminalActorData;
use common::terminal::handle_terminal_events::handle_events;
use common::terminal::messages::Initialize;
use common::terminal::terminal_actor::TerminalActor;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let term = TerminalActor::new(TerminalActorData::new()).start();

    term.send(Initialize).await??;

    tokio::spawn(handle_events(term)).await??;

    Ok(())
}
