mod args;
mod server;

use std::{sync::mpsc::Sender, thread};

use crate::args::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    println!("Using input file: {}", args.questions_file);
    println!("Binding to port: {}", args.port);

    // create oneshot channel
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    start_server(tx);

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    Ok(())
}

fn start_server() {
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });
}
