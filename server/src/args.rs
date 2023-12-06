use clap::Parser;
use clap_num::number_range;

use crate::server::lobby::Question;

fn valid_port(s: &str) -> Result<u16, String> {
    number_range(s, 1025u16, u16::MAX)
}

fn parse_questions(file: &str) -> Result<Vec<Question>, String> {
    if !std::path::Path::new(file).exists() {
        return Err(format!("File {file} does not exist"));
    }

    // read all contents to String
    let contents = std::fs::read_to_string(file).unwrap();

    // parse it as JSON
    let questions: Vec<Question> = serde_json::from_str(&contents).unwrap();

    Ok(questions)
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Robert Gemrot")]
pub struct Args {
    /// Port which the server will bind to. Must be greater than 1024. Defaults to 8080.
    #[clap(short, long, default_value="8080", value_parser=valid_port)]
    pub port: u16,

    /// Where to load questions from
    #[clap(short, long, value_parser=parse_questions)]
    pub questions: Vec<Question>,

    /// Whether to randomize questions order (default: false)
    #[clap(short, long, default_value = "false")]
    pub randomize_questions: bool,

    /// Whether to randomize answers order (default: false)
    #[clap(short = 'a', long, default_value = "false")]
    pub randomize_answers: bool,
}
