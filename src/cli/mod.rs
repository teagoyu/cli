mod base64;
mod csv;
mod genpass;

use crate::cli::csv::CsvOpts;
use clap::Parser;
use std::path::Path;

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::OutputFormat;
pub use self::genpass::GenPassOpts;

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv,Convert CSV TO JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate a rand password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

#[derive(Debug, Parser)]
#[command(name = "rcli",version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

pub fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("file not found")
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::verify_input_file;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Ok("*".into()));
        assert_eq!(verify_input_file("no-exits"), Ok("no-exits".into()));
    }
}
