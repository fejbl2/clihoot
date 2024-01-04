use std::{
    sync::mpsc::{self},
    thread,
};

use anyhow::anyhow;
use clap::Parser;
use common::questions::QuestionSet;
use server::{args::Args, lobby::init::run_server, teacher::init::run_teacher};

use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

use std::fs::File;

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(args.log_file)?,
        ),
        TermLogger::new(
            LevelFilter::Error,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])?;

    let path = std::path::Path::new(&args.questions_file);
    let mut questions = QuestionSet::from_file(path)?;

    questions.randomize_answers = args.randomize_answers;
    questions.randomize_questions = args.randomize_questions;

    // construct address on which the server will listen
    let addr = format!("0.0.0.0:{}", args.port).parse()?;

    // create oneshot channel, so that spawned server can send us its address
    let (tx_server, rx_server) = mpsc::channel();
    let (tx_teacher, _rx_teacher) = mpsc::channel();

    let quiz_name = questions.quiz_name.clone();

    let server_thread = thread::spawn(move || {
        run_server(tx_server, questions, addr).expect("Failed to run server");
    });

    let teacher_thread = thread::spawn(move || {
        let server = rx_server.recv().expect("Failed to receive server address");
        run_teacher(server, tx_teacher, &quiz_name).expect("Failed to run teacher");
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
