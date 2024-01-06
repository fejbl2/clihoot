use actix::{Actor, System};
use anyhow::Result;
use clap::Parser;
use client::websocket::WebsocketActor;
use log::error;
use std::{fs::File, path::PathBuf, str::FromStr};
use url::Url;
use uuid::Uuid;

use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

use client::music_actor::MusicActor;
use common::terminal::highlight::Theme;

fn url_parser(arg: &str) -> Result<Url, String> {
    let destination_addr = format!("ws://{arg}");
    Ok(Url::from_str(destination_addr.as_str())
        .map_err(|_| "This is not valid url. Help: <host>:[port]")?)
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Pavol Kycina")]
pub struct Args {
    /// Url of the clihoot server
    #[clap(short, long, default_value="localhost:8080", value_parser=url_parser)]
    addr: Url,

    /// No music and sounds will be played with this option
    #[clap(short, long)]
    silent: bool,

    /// Where to write log messages to
    #[clap(short, long)]
    log_file: Option<PathBuf>,

    /// Theme for syntax highlighting of code in questions
    #[clap(short('t'), long, default_value_t, value_enum)]
    syntax_theme: Theme,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let uuid = Uuid::new_v4();

    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(
                args.log_file
                    .unwrap_or(format!("clihoot_client_logs_{uuid}.log").into()),
            )?,
        ),
        TermLogger::new(
            LevelFilter::Error,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])?;

    let url = args.addr;
    let silent = args.silent;

    let sys = actix::System::new();

    sys.block_on(async move {
        let addr_music_actor = MusicActor::new(silent).start();

        // start websocket actor
        let Ok(websocket_actor) =
            WebsocketActor::new(url.clone(), uuid, addr_music_actor, args.syntax_theme).await
        else {
            error!(
                "I can't contact the specified clihoot server on address: '{url}' I am sorry 😿"
            );
            System::current().stop();
            return;
        };

        let _addr_websocket_actor = websocket_actor.start();
    });
    sys.run()?;

    Ok(())
}
