mod music_actor;

use actix::{Actor, System};
use anyhow::Result;
use clap::Parser;
use client::websocket::WebsocketActor;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

use actix::prelude::*;
use client::music_actor::MusicActor;
use client::music_actor::MusicMessage;
use futures::SinkExt;

fn url_parser(arg: &str) -> Result<Url, String> {
    let destination_addr = format!("ws://{arg}");
    Ok(Url::from_str(destination_addr.as_str())
        .map_err(|_| "This is not valid url. Help: <host>:[port]")?)
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Pavol Kycina")]
pub struct Args {
    #[clap(short, long, default_value="localhost:8080", value_parser=url_parser)]
    addr: Url,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let url = args.addr;

    let sys = actix::System::new();

    sys.block_on(async move {
        let addr_music_actor = MusicActor::new().start();

        // start websocket actor
        let Ok(websocket_actor) =
            WebsocketActor::new(url.clone(), Uuid::new_v4(), addr_music_actor.clone()).await
        else {
            println!(
                "I can't contact the specified clihoot server on address: '{url}' I am sorry 😿"
            );
            System::current().stop();
            return;
        };

        let _addr_websocket_actor = websocket_actor.start();

        addr_music_actor.do_send(MusicMessage::Lobby);
    });
    sys.run()?;

    Ok(())
}
