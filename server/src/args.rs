use std::path::PathBuf;

use clap::Parser;
use clap_num::number_range;

use log::info;

fn valid_port(s: &str) -> Result<u16, String> {
    number_range(s, 1025u16, u16::MAX)
}

fn valid_questions_file(file: &str) -> Result<PathBuf, String> {
    // recursively try to find the file from the current directory up to the root
    let mut current_dir = std::env::current_dir().expect("Failed to get current directory");
    loop {
        // if user entered absolute path, this also works thanks to how 'join' works
        let path = current_dir.join(file);
        if path.exists() {
            info!("Using questions file '{}'", path.to_str().unwrap());
            return Ok(path);
        }

        if !current_dir.pop() {
            break;
        }
    }
    Err(format!(
        "File {file} does not exist nowhere in the current directory and its parents"
    ))
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Robert Gemrot")]
pub struct Args {
    /// Port which the server will bind to. Must be greater than 1024. Defaults to 8080.
    #[clap(short, long, default_value="8080", value_parser=valid_port)]
    pub port: u16,

    /// Where to load questions from
    #[clap(short, long, value_parser=valid_questions_file, default_value = "default_questions.yaml")]
    pub questions_file: PathBuf,

    /// Where to write log messages to
    #[clap(short, long, default_value = "clihoot_server_logs.log")]
    pub log_file: PathBuf,

    /// Whether to randomize questions order (default: false)
    #[clap(short, long, default_value = "false")]
    pub randomize_questions: bool,

    /// Whether to randomize answers order (default: false)
    #[clap(short = 'a', long, default_value = "false")]
    pub randomize_answers: bool,
}
