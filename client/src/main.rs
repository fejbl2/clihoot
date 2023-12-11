use crate::websocket::WebsocketActor;
use actix::{Actor, System};
use anyhow::Result;
use clap::Parser;
use std::str::FromStr;
use url::Url;

mod websocket;

fn url_parser(arg: &str) -> Result<Url, String> {
    let destination_addr = format!("ws://{}", arg);
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
        let Some(websocket_actor) = WebsocketActor::new(url.clone()).await else {
            println!(
                "I can't contact the specified clihoot server on address: {} I am sorry :(",
                url
            );
            System::current().stop();
            return;
        };

        let _addr_websocket_actor = websocket_actor.start();

        // TODO start terminal actor + send terminal actor websocket's address

        // TODO register terminal actor at websocket

        /*addr_websocket_actor
        .send(Subscribe(addr_client.recipient()))
        .await
        .unwrap();*/
    });
    sys.run()?;

    Ok(())
}
