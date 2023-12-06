mod args;
mod server;
mod teacher;

use std::{
    sync::mpsc::{self},
    thread,
};

use crate::{args::Args, server::init::run_server, teacher::init::run_teacher};
use clap::Parser;
use common::questions::QuestionSet;

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    let path = std::path::Path::new(&args.questions_file);
    let mut questions = QuestionSet::from_file(path)?;

    questions.randomize_answers = args.randomize_answers;
    questions.randomize_questions = args.randomize_questions;

    // create oneshot channel, so that spawned server can send us its address
    let (tx, rx) = mpsc::channel();

    let server_thread = thread::spawn(move || {
        run_server(tx, questions);
    });

    let teacher_thread = thread::spawn(move || {
        let server = rx.recv().unwrap();
        run_teacher(server);
    });

    // wait for threads to finish
    server_thread.join().unwrap();
    teacher_thread.join().unwrap();

    Ok(())
}
