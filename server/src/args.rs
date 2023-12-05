use clap::Parser;
use clap_num::number_range;

fn valid_port(s: &str) -> Result<u16, String> {
    number_range(s, 1025u16, u16::MAX)
}

fn valid_file(file: &str) -> Result<String, String> {
    if std::path::Path::new(file).exists() {
        Ok(file.to_string())
    } else {
        Err(format!("File {file} does not exist"))
    }
}

#[derive(Parser)]
#[clap(version = "1.0", author = "Robert Gemrot")]
pub struct Args {
    /// Port which the server will bind to. Must be greater than 1024.
    #[clap(short, long, default_value="8080", value_parser=valid_port)]
    pub port: u16,

    /// Where to load questions from
    #[clap(short, long, value_parser=valid_file)]
    pub questions_file: String,
}
