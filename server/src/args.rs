use clap::Parser;
use clap_num::number_range;

fn valid_port(s: &str) -> Result<u16, String> {
    number_range(s, 1025u16, u16::MAX)
}

fn valid_file(file: &str) -> Result<String, String> {
    if !std::path::Path::new(file).exists() {
        return Err(format!("File {file} does not exist"));
    }

    Ok(file.to_string())
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Robert Gemrot")]
pub struct Args {
    /// Port which the server will bind to. Must be greater than 1024. Defaults to 8080.
    #[clap(short, long, default_value="8080", value_parser=valid_port)]
    pub port: u16,

    /// Where to load questions from
    #[clap(short, long, value_parser=valid_file, default_value="../common/tests/files/ok_code.yaml")]
    pub questions_file: String,

    /// Whether to randomize questions order (default: false)
    #[clap(short, long, default_value = "false")]
    pub randomize_questions: bool,

    /// Whether to randomize answers order (default: false)
    #[clap(short = 'a', long, default_value = "false")]
    pub randomize_answers: bool,
}
