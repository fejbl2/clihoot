mod args;
mod server;
mod teacher;

use std::{
    sync::mpsc::{self},
    thread,
};

use crate::{args::Args, server::init::run_server, teacher::init::run_teacher};
use clap::Parser;

fn main() {
    let args: Args = Args::parse();

    println!("Using input file: {}", args.questions_file);
    println!("Binding to port: {}", args.port);

    // create oneshot channel, so that spawned server can send us its address
    let (tx, rx) = mpsc::channel();

    let server_thread = thread::spawn(move || {
        run_server(tx);
    });

    let teacher_thread = thread::spawn(move || {
        let server = rx.recv().unwrap();
        run_teacher(server);
    });

    // wait for threads to finish
    server_thread.join().unwrap();
    teacher_thread.join().unwrap();
}
