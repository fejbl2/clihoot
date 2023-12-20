use crate::websocket::WebsocketActor;
use actix::{Actor, System};
use anyhow::Result;
use clap::Parser;
use std::str::FromStr;
use url::Url;

mod websocket;

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

    sys.block_on(async {
        // start websocket actor
        let Ok(websocket_actor) = WebsocketActor::new(url.clone()).await else {
            println!(
                "I can't contact the specified clihoot server on address: '{url}' I am sorry 😿"
            );
            System::current().stop();
            return;
        };

        let _addr_websocket_actor = websocket_actor.start();
    });
    sys.run()?;
    // mod music_actor;

    // use actix::prelude::*;
    // use client::terminal::student::run_student;
    // use music_actor::MusicActor;
    // use music_actor::MusicMessage;
    // use std::thread;
    // use std::time::Duration;

    // #[actix_rt::main]
    // async fn main() -> anyhow::Result<()> {
    //     let music_actor = MusicActor::new().start();
    //     println!("Music actor is created.");

    //     music_actor.send(MusicMessage::Happy).await.unwrap();
    //     println!("Happy music is playing.");

    //     thread::sleep(Duration::from_millis(5000));

    //     music_actor.send(MusicMessage::Sad).await.unwrap();
    //     println!("Sad music is playing.");

    //     thread::sleep(Duration::from_millis(5000));

    //     music_actor.send(MusicMessage::Angry).await.unwrap();
    //     println!("Angry music is playing.");

    //     thread::sleep(Duration::from_millis(10000));

    //     let (_term, task) = run_student().await?;

    //     task.await??;

    Ok(())
}
