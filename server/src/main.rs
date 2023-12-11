mod args;
mod messages;
mod server;
mod teacher;
mod websocket;

use std::{
    sync::mpsc::{self},
    thread,
};

use crate::{args::Args, server::init::run_server, teacher::init::run_teacher};
use anyhow::anyhow;
use clap::Parser;
use common::questions::QuestionSet;

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    let path = std::path::Path::new(&args.questions_file);
    let mut questions = QuestionSet::from_file(path)?;

    questions.randomize_answers = args.randomize_answers;
    questions.randomize_questions = args.randomize_questions;

    // construct address on which the server will listen
    let addr = format!("0.0.0.0:{}", args.port).parse()?;

    // create oneshot channel, so that spawned server can send us its address
    let (tx, rx) = mpsc::channel();

    let server_thread = thread::spawn(move || {
        run_server(tx, questions, addr).expect("Failed to run server");
    });

    let teacher_thread = thread::spawn(move || {
        let server = rx.recv().expect("Failed to receive server address");
        run_teacher(server).expect("Failed to run teacher");
    });

    // wait for threads to finish
    if let Err(_e) = server_thread.join() {
        return Err(anyhow!("Server thread panicked"));
    }

    if let Err(_e) = teacher_thread.join() {
        return Err(anyhow!("Teacher thread panicked"));
    }

    Ok(())
}
